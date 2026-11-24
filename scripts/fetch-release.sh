#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(git -C "$script_dir" rev-parse --show-toplevel)"
tag="${RELEASE_TAG:-${1:-latest}}"
asset="${RELEASE_ASSET:-sudo4-linux-x86_64.tar.gz}"
destination="${DEST:-${repo_root}/.artifacts}"

if [[ -n "${GITHUB_REPOSITORY:-}" ]]; then
  repo="$GITHUB_REPOSITORY"
else
  remote="$(git -C "$repo_root" config --get remote.origin.url 2>/dev/null || true)"

  case "$remote" in
    git@github.com:*)
      repo="${remote#git@github.com:}"
      ;;
    ssh://git@github.com/*)
      repo="${remote#ssh://git@github.com/}"
      ;;
    https://github.com/*)
      repo="${remote#https://github.com/}"
      ;;
    http://github.com/*)
      repo="${remote#http://github.com/}"
      ;;
    *)
      printf 'Cannot detect a GitHub repository from remote: %s\n' "${remote:-<none>}" >&2
      printf 'Set GITHUB_REPOSITORY=owner/repo and try again.\n' >&2
      exit 1
      ;;
  esac
fi

repo="${repo%.git}"

if [[ ! "$repo" =~ ^[^/]+/[^/]+$ ]]; then
  printf 'Invalid GitHub repository: %s\n' "$repo" >&2
  exit 1
fi

base_url="https://github.com/${repo}/releases/download/${tag}"
temporary_dir="$(mktemp -d)"
trap 'rm -rf "$temporary_dir"' EXIT

printf 'Fetching %s from %s@%s...\n' "$asset" "$repo" "$tag"

if ! curl -fL --retry 3 --retry-delay 2 -o "$temporary_dir/$asset" "$base_url/$asset"; then
  printf 'Download failed.\n' >&2
  printf 'Wait for the release workflow to finish, or check the tag/repository.\n' >&2
  exit 1
fi

if curl -fsSL --retry 3 --retry-delay 2 -o "$temporary_dir/SHA256SUMS" "$base_url/SHA256SUMS"; then
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
else
  printf 'Warning: SHA256SUMS is unavailable; skipping verification.\n' >&2
fi

rm -rf "$destination"
mkdir -p "$destination"
tar -xzf "$temporary_dir/$asset" -C "$destination" --strip-components=1
chmod +x "$destination/sudo4-api"

if [[ ! -f "$destination/dist/index.html" ]]; then
  printf 'The release package does not contain the frontend build.\n' >&2
  exit 1
fi

printf 'Artifacts ready in %s\n' "$destination"
printf 'Backend binary: %s\n' "$destination/sudo4-api"
printf 'Frontend files: %s\n' "$destination/dist"
