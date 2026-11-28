#!/usr/bin/env bash
set -euo pipefail
umask 022

repo_root=/opt/sudo4/repo
tag="${RELEASE_TAG:-${1:-latest}}"
asset="${RELEASE_ASSET:-sudo4-linux-x86_64.tar.gz}"
destination="${DEST:-${repo_root}/.artifacts}"

if [[ ! "$tag" =~ ^[A-Za-z0-9._/-]+$ ]]; then
  printf 'Invalid release tag: %s\n' "$tag" >&2
  exit 1
fi

if [[ "$destination" != /* || "$destination" == "/" ]]; then
  printf 'Refusing unsafe destination: %s\n' "$destination" >&2
  exit 1
fi

repo="Interesting0-o/MyBlogProject"
base_url="https://github.com/${repo}/releases/download/${tag}"
temporary_dir="$(mktemp -d)"
trap 'rm -rf "$temporary_dir"' EXIT

read_git_proxy() {
  local key="$1" value home config

  if [[ -n "${SUDO_USER:-}" && "$SUDO_USER" != "root" ]]; then
    home="$(getent passwd "$SUDO_USER" 2>/dev/null | cut -d: -f6)"
    if [[ -n "$home" ]]; then
      for config in "$home/.config/git/config" "$home/.gitconfig"; do
        if [[ -f "$config" ]]; then
          value="$(git config -f "$config" --get "$key" 2>/dev/null || true)"
          if [[ -n "$value" ]]; then
            printf '%s\n' "$value"
            return 0
          fi
        fi
      done
    fi
  fi
}

curl_args=(curl -fL --retry 3 --retry-delay 2 --connect-timeout 15 --max-time 600)
proxy="${RELEASE_PROXY:-}"
if [[ -z "$proxy" ]]; then
  proxy="$(read_git_proxy https.proxy)"
fi
if [[ -z "$proxy" ]]; then
  proxy="$(read_git_proxy http.proxy)"
fi
if [[ -n "$proxy" ]]; then
  curl_args+=(--proxy "$proxy")
fi

printf 'Fetching %s from %s@%s...\n' "$asset" "$repo" "$tag"

if ! "${curl_args[@]}" -o "$temporary_dir/$asset" "$base_url/$asset"; then
  printf 'Download failed.\n' >&2
  printf 'If GitHub is unreachable directly, set https.proxy via git config or RELEASE_PROXY.\n' >&2
  printf 'Wait for the release workflow to finish, or check the tag/repository.\n' >&2
  exit 1
fi

if ! "${curl_args[@]}" -sS -o "$temporary_dir/SHA256SUMS" "$base_url/SHA256SUMS"; then
  printf 'SHA256SUMS is unavailable. Aborting.\n' >&2
  exit 1
fi

expected="$(awk -v name="$asset" '$2 == name { print $1 }' "$temporary_dir/SHA256SUMS")"

if [[ -z "$expected" ]]; then
  printf 'Checksum for %s was not found. Aborting.\n' "$asset" >&2
  exit 1
fi

actual="$(sha256sum "$temporary_dir/$asset" | awk '{ print $1 }')"

if [[ "$actual" != "$expected" ]]; then
  printf 'Checksum mismatch for %s. Aborting.\n' "$asset" >&2
  exit 1
fi

rm -rf "$destination"
mkdir -p "$destination"
tar --no-same-owner --no-same-permissions -xzf "$temporary_dir/$asset" -C "$destination" --strip-components=1
chmod +x "$destination/sudo4-api"

if [[ ! -f "$destination/dist/index.html" ]]; then
  printf 'The release package does not contain the frontend build.\n' >&2
  exit 1
fi

printf 'Artifacts ready in %s\n' "$destination"
printf 'Backend binary: %s\n' "$destination/sudo4-api"
printf 'Frontend files: %s\n' "$destination/dist"
