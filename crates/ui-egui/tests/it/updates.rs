//! Recto never invokes the upstream release checker or exposes its update UI.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use recto_ui_egui::RectoApp;

fn harness() -> Harness<'static, RectoApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(|_cc| {
        let mut app = RectoApp::new();
        app.set_option("language", "en").unwrap();
        // Settings left by an upstream build must not re-enable startup requests.
        app.restore(r#"{"check_updates_at_start":true}"#);
        app
    });
    h.run_steps(8);
    h
}

fn assert_no_update_ui(h: &Harness<'static, RectoApp>) {
    for text in ["Check for updates", "Checking for a newer version", "is available", "DOWNLOAD", "LATER"] {
        assert!(h.query_by_label_contains(text).is_none(), "unexpected update UI: {text}");
    }
    assert!(h.state().last_opened_url.is_none());
}

#[test]
fn startup_and_legacy_settings_do_not_offer_updates() {
    let h = harness();
    assert_no_update_ui(&h);
    h.get_by_label("Open PDF");
}

#[test]
fn legacy_update_command_is_inert_and_hidden_from_the_palette() {
    let mut h = harness();
    assert!(!h.state_mut().execute("help.check_updates"));
    h.state_mut().palette_open = true;
    h.state_mut().palette_query = "check for updates".to_owned();
    h.run_steps(4);
    assert_no_update_ui(&h);
    assert!(h.state().toast.is_none(), "a removed command must not open a replacement notification");
}

#[test]
fn desktop_startup_and_frame_loop_have_no_updater_hooks() {
    let main = include_str!("../../../../apps/recto/src/main.rs");
    let app = include_str!("../../src/app.rs");
    let platform = include_str!("../../src/platform.rs");
    let dialogs = include_str!("../../src/dialogs.rs");
    assert!(!main.contains("set_startup_update_check("));
    assert!(!main.contains("craft_update::"), "Windows self-update helper must also be absent");
    assert!(!app.contains("poll_updates("));
    assert!(!app.contains("updates::"));
    assert!(!platform.contains("mod updates;"));
    assert!(!dialogs.contains("updates::dialog("));
}
