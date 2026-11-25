#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(git -C "$script_dir" rev-parse --show-toplevel)"

app_root="${APP_ROOT:-/opt/sudo4}"
app_user="${APP_USER:-}"
app_group="${APP_GROUP:-}"
service="${SYSTEMD_UNIT:-sudo4-api}"
tag="${1:-${RELEASE_TAG:-latest}}"
health_url="${HEALTH_URL:-http://127.0.0.1:3000/health}"
health_attempts="${HEALTH_ATTEMPTS:-20}"
keep_releases="${KEEP_RELEASES:-5}"
keep_backups="${KEEP_BACKUPS:-10}"
source_dir="${DEPLOY_SOURCE:-}"

releases_dir="$app_root/releases"
backups_dir="$app_root/backups"
current_link="$app_root/current"

log() { printf '[deploy] %s\n' "$*"; }
die() { printf '[deploy] ERROR: %s\n' "$*" >&2; exit 1; }

if [[ ! "$health_attempts" =~ ^[1-9][0-9]*$ ]]; then
  die "HEALTH_ATTEMPTS must be a positive integer"
fi

if [[ -z "$app_user" ]]; then
  if systemctl cat "$service" >/dev/null 2>&1; then
    app_user="$(systemctl show -p User --value "$service" 2>/dev/null || true)"
    app_user="${app_user:-root}"
    app_group="$(systemctl show -p Group --value "$service" 2>/dev/null || true)"
  else
    app_user="sudo4"
  fi
fi

if [[ -z "$app_group" && -n "$app_user" ]]; then
  app_group="$(id -gn "$app_user" 2>/dev/null || printf '%s' "$app_user")"
fi

log "Service user: $app_user:$app_group"

if [[ "${SKIP_RESTART:-0}" != "1" ]]; then
  command -v systemctl >/dev/null 2>&1 || die "systemctl is required unless SKIP_RESTART=1"

  if ! systemctl cat "$service" >/dev/null 2>&1; then
    die "systemd unit '$service' not found; install deploy/sudo4-api.service, run 'systemctl daemon-reload', or use SKIP_RESTART=1"
  fi
fi

as_root() {
  if [[ "$EUID" -eq 0 ]]; then
    "$@"
  else
    sudo "$@"
  fi
}

staging=""
staging_created=""
cleanup() {
  if [[ -n "$staging_created" && -n "$staging" ]]; then
    rm -rf "$staging"
  fi
}
trap cleanup EXIT

if [[ -n "$source_dir" ]]; then
  [[ -d "$source_dir" ]] || die "DEPLOY_SOURCE does not exist: $source_dir"
  staging="$source_dir"
else
  staging="$(mktemp -d)"
  staging_created=1
  DEST="$staging" "$repo_root/scripts/fetch-release.sh" "$tag"
fi

[[ -x "$staging/sudo4-api" ]] || die "Artifact is missing an executable sudo4-api"
[[ -f "$staging/dist/index.html" ]] || die "Artifact is missing the frontend build (dist/index.html)"

mkdir -p "$app_root" "$releases_dir" "$backups_dir" "$app_root/resource"

if [[ ! -f "$app_root/.env" ]]; then
  install -m 600 "$repo_root/.env.example" "$app_root/.env"
  log "Created $app_root/.env from .env.example; review it before going live."
fi

if [[ "$EUID" -eq 0 ]]; then
  if id -u "$app_user" >/dev/null 2>&1; then
    chown "$app_user":"$app_group" "$app_root/.env" "$app_root/resource" "$backups_dir"
    chmod 600 "$app_root/.env"
  else
    log "Warning: user $app_user does not exist; adjust User= in the systemd unit."
  fi
fi

resolve_db_path() {
  if [[ -n "${DB_PATH:-}" ]]; then
    printf '%s\n' "$DB_PATH"
    return 0
  fi

  local url=""
  if [[ -f "$app_root/.env" ]]; then
    url="$(awk -F= '$1 == "DATABASE_URL" { sub(/^[^=]*=/, ""); print; exit }' "$app_root/.env")"
  fi

  case "$url" in
    sqlite://*)
      url="${url#sqlite://}"
      if [[ "$url" == /* ]]; then
        printf '%s\n' "$url"
      else
        printf '%s\n' "$app_root/$url"
      fi
      ;;
    *)
      printf '%s\n' "$app_root/resource/sudo4.db"
      ;;
  esac
}

db_path="$(resolve_db_path)"
if [[ -f "$db_path" ]]; then
  backup_base="$backups_dir/sudo4-$(date -u +%Y%m%d%H%M%S)"
  backup_path="$backup_base.db"
  backup_suffix=0
  while [[ -e "$backup_path" ]]; do
    backup_suffix=$((backup_suffix + 1))
    backup_path="$backup_base-$backup_suffix.db"
  done
  if command -v sqlite3 >/dev/null 2>&1; then
    sqlite3 "$db_path" ".backup '$backup_path'"
    log "Database backed up to $backup_path"
  else
    cp -a "$db_path" "$backup_path"
    if [[ -f "$db_path-wal" ]]; then
      cp -a "$db_path-wal" "$backup_path-wal"
    fi
    if [[ -f "$db_path-shm" ]]; then
      cp -a "$db_path-shm" "$backup_path-shm"
    fi
    log "Database copied to $backup_path (sqlite3 not found; verify the copy)."
  fi
else
  log "Database not found at $db_path; skipping backup."
fi

release_id="${tag//\//-}-$(date -u +%Y%m%d%H%M%S)"
new_release="$releases_dir/$release_id"
while [[ -e "$new_release" ]]; do
  release_id="${release_id}-$(date -u +%s)"
  new_release="$releases_dir/$release_id"
done

mkdir -p "$new_release"
install -m 0755 "$staging/sudo4-api" "$new_release/sudo4-api"
cp -a "$staging/dist" "$new_release/dist"

previous="$(readlink -f "$current_link" 2>/dev/null || true)"
if [[ ! -d "$previous" ]]; then
  previous=""
fi

switch_to() {
  local target="$1" temporary="$app_root/.current.$$"
  ln -sfn "$target" "$temporary"
  mv -Tf "$temporary" "$current_link"
}

restart_service() {
  as_root systemctl restart "$service"
}

wait_for_health() {
  local attempt
  for ((attempt = 1; attempt <= health_attempts; attempt++)); do
    if curl --noproxy '*' -fsS --max-time 2 "$health_url" >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done
  return 1
}

log "Activating release $release_id"
switch_to "$new_release"

if [[ "${SKIP_RESTART:-0}" == "1" ]]; then
  log "SKIP_RESTART=1; skipped service restart and health check."
else
  if ! restart_service || ! wait_for_health; then
    if [[ -n "$previous" && "$previous" != "$new_release" ]]; then
      log "Health check failed; rolling back to $previous"
      switch_to "$previous"
      restart_service || true
      if wait_for_health; then
        die "Deployment failed and was rolled back."
      fi
      die "Deployment failed; rollback also failed. Check: journalctl -u $service"
    fi
    die "Deployment failed; no previous release available. Check: journalctl -u $service"
  fi
  log "Health check passed."
fi

prune_old() {
  local directory="$1" keep="$2"
  local ordered=() i

  if (( keep < 1 )); then
    return 0
  fi

  mapfile -t ordered < <(find "$directory" -mindepth 1 -maxdepth 1 -printf '%T@\t%p\n' | sort -rn | cut -f2-)
  for ((i = keep; i < ${#ordered[@]}; i++)); do
    rm -rf -- "${ordered[$i]}"
    log "Removed old entry ${ordered[$i]}"
  done
}

prune_old "$releases_dir" "$keep_releases"
prune_old "$backups_dir" "$keep_backups"

log "Release $release_id is live."
log "Current: $current_link -> $new_release"
