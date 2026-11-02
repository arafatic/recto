# shellcheck shell=bash
# Source independently of env.sh, including from release archive assembly.
copy_font_licences() {
  local dest="$1" root font_root manifest line family style file scripts licence notice rest count=0
  root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  font_root="${RECTO_FONTS_DIR-$root/crates/fonts/assets}"
  font_root="${font_root:-.}"
  mkdir -p "$dest" || return 1
  # These faces are embedded independently of the supplemental manifest.
  for notice in "$root"/assets/fonts/OFL-*.txt; do
    [[ -s "$notice" ]] || { echo "Missing base font notice: $notice" >&2; return 1; }
    cp "$notice" "$dest/" || return 1
  done
  for manifest in manifest.txt simplified-chinese-manifest.txt; do
    if [[ ! -f "$font_root/fonts/$manifest" ]]; then
      if [[ "$manifest" == manifest.txt || "${RECTO_FONTS_REQUIRED+x}" == x ]]; then
        echo "Missing font manifest: $font_root/fonts/$manifest" >&2; return 1
      fi
      continue
    fi
    while IFS= read -r line || [[ -n "$line" ]]; do
      [[ "$line" =~ ^[[:space:]]*(#|$) ]] && continue
      IFS='|' read -r family style file scripts licence notice rest <<< "$line"
      file="${file#"${file%%[![:space:]]*}"}"
      file="${file%"${file##*[![:space:]]}"}"
      licence="${licence#"${licence%%[![:space:]]*}"}"
      licence="${licence%"${licence##*[![:space:]]}"}"
      notice="${notice#"${notice%%[![:space:]]*}"}"
      notice="${notice%"${notice##*[![:space:]]}"}"
      if [[ "$licence" != OFL-1.1 || ! "$notice" =~ ^fonts/([a-zA-Z0-9_-]+)/OFL\.txt$ ]]; then
        echo "Invalid font notice metadata in $manifest: $line" >&2; return 1
      fi
      family="${BASH_REMATCH[1]}"
      if [[ "$file" != "fonts/$family/"* || "${file#fonts/$family/}" == */* || ! -s "$font_root/$notice" ]]; then
        echo "Missing or mismatched font notice: $notice" >&2; return 1
      fi
      count=$((count + 1))
      cp "$font_root/$notice" "$dest/OFL-$family.txt" || return 1
    done < "$font_root/fonts/$manifest"
  done
  [[ "$count" -gt 0 ]] || { echo "No declared font notices in $font_root" >&2; return 1; }
}
