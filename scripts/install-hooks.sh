#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(git -C "$script_dir" rev-parse --show-toplevel)"

git -C "$repo_root" config core.hooksPath .githooks
chmod +x \
  "$repo_root/.githooks/post-merge" \
  "$repo_root/scripts/fetch-release.sh" \
  "$repo_root/scripts/install-hooks.sh"

printf 'Git hooks installed. Future git pulls will update .artifacts automatically.\n'

if ! "$repo_root/scripts/fetch-release.sh"; then
  printf 'Artifacts were not downloaded yet. Run scripts/fetch-release.sh after the first release build.\n' >&2
fi
