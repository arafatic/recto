//! Embed desktop executable metadata. On Windows, icons and VERSIONINFO in `recto.exe` mean it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab. A separate PDF document icon
//! uses resource ID 2; the installer references it as `recto.exe,-2`.
//!
//! On other targets this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `RECTO_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

#[path = "src/windows_manifest.rs"]
mod windows_manifest;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/recto.ico");
    println!("cargo:rerun-if-changed=../../assets/app-icon/recto-document.ico");
    println!("cargo:rerun-if-changed=src/windows_manifest.rs");
    println!("cargo:rerun-if-env-changed=RECTO_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=RECTO_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/recto.ico")
        .set_icon_with_id("../../assets/app-icon/recto-document.ico", "2")
        .set_manifest(windows_manifest::WINDOWS_MANIFEST)
        .set("ProductName", "Recto")
        .set("FileDescription", "Recto PDF application")
        .set("LegalCopyright", "Copyright (c) the Recto contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "recto.exe")
        .set("InternalName", "recto");
    if let Err(e) = res.compile() {
        if std::env::var_os("RECTO_REQUIRE_WINRES").is_some() || std::env::var_os("RECTO_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=recto.exe built without icon/version resources: {e}");
    }
}
