#!/usr/bin/env bash
# Run on macOS and Linux: default/override staging, exact license bytes, and failure cases.
set -eu
root="$(cd "$(dirname "$0")/.." && pwd)"
. "$root/packaging/font-notices.sh"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
unset RECTO_FONTS_DIR RECTO_FONTS_REQUIRED
copy_font_licences "$work/default"
# All local family notices plus the three always-embedded base-font notices.
for notice in "$root"/crates/fonts/assets/fonts/*/OFL.txt; do
  family="$(basename "$(dirname "$notice")")"
  cmp "$notice" "$work/default/OFL-$family.txt"
done
for notice in "$root"/assets/fonts/OFL-*.txt; do
  cmp "$notice" "$work/default/$(basename "$notice")"
done
mkdir -p "$work/override/fonts/custom"
printf 'Custom | Regular | fonts/custom/font.ttf | Latn | OFL-1.1 | fonts/custom/OFL.txt\n' > "$work/override/fonts/manifest.txt"
printf 'Synthetic notice fixture\n' > "$work/override/fonts/custom/OFL.txt"
export RECTO_FONTS_DIR="$work/override"
copy_font_licences "$work/custom"
cmp "$work/override/fonts/custom/OFL.txt" "$work/custom/OFL-custom.txt"
[[ ! -e "$work/custom/OFL-shippori-mincho.txt" ]]
export RECTO_FONTS_REQUIRED=1
if copy_font_licences "$work/missing-chinese" 2>/dev/null; then
  echo 'Expected missing required Chinese manifest to fail' >&2; exit 1
fi
unset RECTO_FONTS_REQUIRED
rm "$work/override/fonts/custom/OFL.txt"
if copy_font_licences "$work/missing-notice" 2>/dev/null; then
  echo 'Expected missing notice to fail' >&2; exit 1
fi
printf 'Custom | Regular | fonts/custom/font.ttf | Latn | OFL-1.1 | ../OFL.txt\n' > "$work/override/fonts/manifest.txt"
if copy_font_licences "$work/escaping-notice" 2>/dev/null; then
  echo 'Expected escaping notice path to fail' >&2; exit 1
fi
echo 'Font notice staging checks passed (default, override, exact bytes, missing and escaping notices)'
