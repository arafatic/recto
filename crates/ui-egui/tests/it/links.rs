//! Recto omits upstream promotion and keeps contributor credits.

use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use recto_engine::links;
use recto_ui_egui::{Dialog, RectoApp};

fn harness(setup: impl FnOnce(&mut RectoApp) + 'static) -> Harness<'static, RectoApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = RectoApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(6);
    h
}

#[test]
fn workspace_has_no_promotional_links() {
    let h = harness(|_| {});
    assert_eq!(h.root().accesskit_node().label().as_deref(), Some("Recto"));
    h.get_by_label("Welcome to Recto");
    for label in ["Discord", "Join our Discord", "Join the community", "LegacySuite"] {
        assert!(h.query_by_label(label).is_none(), "{label}");
    }
}

#[test]
fn about_retains_credits_without_trademark_graphics() {
    let h = harness(|app| app.dialog = Some(Dialog::About));
    assert!(h.query_all_by_label("Recto").count() >= 1);
    assert!(h.query_by_label("LegacySuite").is_none());
    assert!(h.query_by_label("Join our Discord").is_none());
}

#[test]
fn upstream_promotion_commands_are_inert() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(!h.state_mut().execute(l.command), "{}", l.command);
        assert!(h.state().last_opened_url.is_none());
        assert_eq!(recto_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
#[allow(dead_code)]
fn settled<'a>(h: &'a mut Harness<'static, RectoApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_renders_clean_about_content() {
    let h = harness(|app| app.dialog = Some(Dialog::About));
    assert!(h.query_all_by_label("Recto").count() >= 1);
}
