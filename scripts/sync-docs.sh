#!/usr/bin/env bash
#
# Regenerate the top-level Markdown docs from their Org sources.
#
# Every org-files/<name>.org is exported to <name>.md at the repo root with
# Emacs' built-in Org Markdown exporter (ox-md). ox-md ships inside Emacs, so
# the only requirement is a reasonably recent Emacs on PATH; no init file,
# packages, or network access are used. The PRNG is seeded before each export
# so the generated heading anchors are stable from one run to the next.
#
# The GitHub Actions "Sync docs" workflow runs this same script on push and is
# the source of truth; running it locally is just a convenience.
#
# Usage:
#   scripts/sync-docs.sh           regenerate the .md files in place
#   scripts/sync-docs.sh --check   don't write; exit non-zero if any .md is stale
#
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

check_only=0
case "${1:-}" in
  --check) check_only=1 ;;
  "")      ;;
  *)       echo "usage: ${0##*/} [--check]" >&2; exit 2 ;;
esac

if ! command -v emacs >/dev/null 2>&1; then
  echo "error: emacs is required but was not found on PATH" >&2
  echo "       install a headless Emacs (e.g. 'sudo dnf install emacs-nox' or" >&2
  echo "       'sudo apt-get install emacs-nox'), or edit the .org files and let" >&2
  echo "       the 'Sync docs' CI workflow regenerate the Markdown on push." >&2
  exit 1
fi

# Keep this in sync with .github/workflows/sync-docs.yml.
emacs_export='(progn
  (require (quote ox-md))
  (setq org-export-with-smart-quotes t)
  (random "zybs-stable-seed")
  (org-md-export-to-markdown))'

status=0
shopt -s nullglob
for org in org-files/*.org; do
  name=$(basename "$org" .org)
  dest="$name.md"
  # ox-md writes the result next to the source; we relocate it to the repo root.
  generated="org-files/$name.md"

  emacs -Q --batch "$org" --eval "$emacs_export" >/dev/null

  if (( check_only )); then
    if ! diff -q "$generated" "$dest" >/dev/null 2>&1; then
      echo "stale: $dest"
      status=1
    fi
    rm -f "$generated"
  else
    mv -f "$generated" "$dest"
    echo "synced $org -> $dest"
  fi
done

if (( check_only && status == 0 )); then
  echo "all Markdown docs are up to date"
fi
exit "$status"
