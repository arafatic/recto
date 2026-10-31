# Recto app icon

Original white geometric lowercase `r.` on black, drawn entirely with paths; no fonts or
upstream brand artwork. Licensed under MIT OR Apache-2.0 (LICENSE-recto.txt).
Historical licence notices are retained separately and do not describe the current artwork.

- `recto.svg`: master; `recto-small.svg`: compact copy.
- `recto-1024.png` and `recto.icns`: macOS runtime and bundle icons.
- `recto.ico`: Windows application icon.
- `recto-document.svg` and `recto-document.ico`: Windows PDF document icon.
- `hicolor/*/apps/local.recto.desktop.*`: Linux/FreeBSD theme icons.

The existing icon design and generated resource bytes are preserved.
`packaging/icons.sh` generates the PNG, ICO and ICNS files when artwork changes.
Update affected hashes in `ATTRIBUTION.toml`, then run `cargo xtask assets --write`
and `cargo xtask assets` after any future regeneration.
