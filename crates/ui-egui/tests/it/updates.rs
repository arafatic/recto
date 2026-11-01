//! In-app update checker tests.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use recto_ui_egui::{Dialog, RectoApp, UpdateDetails};
use recto_ui_egui::updater::is_newer_version;

fn harness() -> Harness<'static, RectoApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(|_cc| {
        let mut app = RectoApp::new();
        app.set_option("language", "en").unwrap();
        app
    });
    h.run_steps(8);
    h
}

#[test]
fn test_version_comparison() {
    assert!(is_newer_version("v0.15.0", "0.14.0"));
    assert!(is_newer_version("0.14.1", "0.14.0"));
    assert!(is_newer_version("v1.0.0", "0.14.0"));
    assert!(is_newer_version("v0.15.0-rc1", "0.14.0"));
    assert!(!is_newer_version("v0.14.0", "0.14.0"));
    assert!(!is_newer_version("0.13.9", "0.14.0"));
    assert!(!is_newer_version("v0.14.0", "0.14.1"));
}

#[test]
fn update_command_is_in_palette() {
    let mut h = harness();
    h.state_mut().palette_open = true;
    h.state_mut().palette_query = "check for updates".to_owned();
    h.run_steps(4);
    assert!(h.query_by_label_contains("Check for updates").is_some());
}

#[test]
fn check_updates_command_triggers_check() {
    let mut h = harness();
    assert!(h.state_mut().execute("help.check_updates"));
    assert!(h.state().toast.is_some() || h.state().available_update.is_some());
}

#[test]
fn update_modal_renders_and_opens_download_url() {
    let mut h = harness();
    let test_url = "https://github.com/arafatic/recto/releases/tag/v0.15.0";
    h.state_mut().available_update = Some(UpdateDetails {
        version: "0.15.0".to_string(),
        url: test_url.to_string(),
        notes: "New features and improvements in v0.15.0".to_string(),
    });
    h.state_mut().dialog = Some(Dialog::UpdateAvailable);
    h.run_steps(4);

    assert!(h.query_by_label_contains("Update Available").is_some());
    assert!(h.query_by_label_contains("A new version of Recto is available").is_some());
    assert!(h.query_by_label_contains("New features and improvements").is_some());

    let download_btn = h.get_by_label("Download Update");
    download_btn.click();
    h.run_steps(2);

    assert_eq!(h.state().last_opened_url.as_deref(), Some(test_url));
    assert_eq!(h.state().dialog, None);
}

#[test]
fn update_modal_dismiss_with_later() {
    let mut h = harness();
    h.state_mut().available_update = Some(UpdateDetails {
        version: "0.15.0".to_string(),
        url: "https://github.com/arafatic/recto/releases/latest".to_string(),
        notes: "Notes".to_string(),
    });
    h.state_mut().dialog = Some(Dialog::UpdateAvailable);
    h.run_steps(4);

    let later_btn = h.get_by_label("Later");
    later_btn.click();
    h.run_steps(2);

    assert_eq!(h.state().last_opened_url, None);
    assert_eq!(h.state().dialog, None);
}
