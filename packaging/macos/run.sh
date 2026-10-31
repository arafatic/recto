#!/usr/bin/env bash
# Cargo runner: give desktop development launches the same native identity as the package.
# Tests and other workspace executables run directly.
set -euo pipefail
exe="$1"
shift
if [[ "$(basename "$exe")" != recto ]]; then
  exec "$exe" "$@"
fi
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exe="$(cd "$(dirname "$exe")" && pwd)/$(basename "$exe")"
# Help/version remain fast, headless operations.
for arg in "$@"; do
  if [[ "$arg" = --help || "$arg" = -h || "$arg" = --version ]]; then
    exec "$exe" "$@"
  fi
done
APP="$(dirname "$exe")/Recto.app"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
TEMP_EXE="$APP/Contents/MacOS/.recto.$$"
TEMP_PLIST="$APP/Contents/.Info.plist.$$"
trap 'rm -f "$TEMP_EXE" "$TEMP_PLIST"' EXIT
# Replace atomically so a previous development instance can keep running.
cp -p "$exe" "$TEMP_EXE"
mv -f "$TEMP_EXE" "$APP/Contents/MacOS/recto"
version="$("$exe" --version)"
version="${version#Recto }"
short="${version%%-*}"
short="${short%%+*}"
sed -e "s/@VERSION@/$version/g" -e "s/@SHORT_VERSION@/$short/g" \
  -e "s/@BUILD_SHA@/development/g" "$ROOT/packaging/macos/Info.plist.in" >"$TEMP_PLIST"
mv -f "$TEMP_PLIST" "$APP/Contents/Info.plist"
cp "$ROOT/assets/app-icon/recto.icns" "$APP/Contents/Resources/recto.icns"
for notice in NOTICE LICENSE LICENSE-MIT LICENSE-APACHE ATTRIBUTION.md; do
  if [[ -f "$ROOT/$notice" ]]; then
    cp "$ROOT/$notice" "$APP/Contents/Resources/$notice"
  fi
done
cp "$ROOT/packaging/macos/Credits.rtf" "$APP/Contents/Resources/Credits.rtf"
trap - EXIT
exec "$APP/Contents/MacOS/recto" "$@"
