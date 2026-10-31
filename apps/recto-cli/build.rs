//! Windows only: embed version info (VERSIONINFO) into `recto-cli.exe`, so Explorer's
//! Details tab, `(Get-Item recto-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `recto.exe` (`apps/recto/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `RECTO_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RECTO_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=RECTO_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "Recto")
        .set("FileDescription", "Recto command-line tool")
        .set("LegalCopyright", "Copyright (c) 2026 Arafatul Islam. MIT OR Apache-2.0.")
        .set("OriginalFilename", "recto-cli.exe")
        .set("InternalName", "recto-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("RECTO_REQUIRE_WINRES").is_some() || std::env::var_os("RECTO_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=recto-cli.exe built without version resources: {e}");
    }
}
