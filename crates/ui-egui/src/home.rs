//! Recto workspace: document actions, pinned folders and local recent files.

use egui::{Align2, CornerRadius, Rect, Sense, Stroke, vec2};

use crate::theme::{self, Tokens};
use crate::{RectoApp, icons, panels::human_size, widgets};

pub mod folders_ui;

/// Draw the workspace and recent documents. Actions reuse the existing workflows.
pub fn show(app: &mut RectoApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        egui::Frame::NONE.inner_margin(egui::Margin::symmetric(28, 28)).show(ui, |ui| {
            ui.label(egui::RichText::new(tl!("Welcome to Recto")).font(theme::semibold(30.0)).color(t.text));
            ui.add_space(6.0);
            ui.label(egui::RichText::new("Your document workspace.").font(theme::regular(14.0)).color(t.text_muted));
            ui.add_space(28.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(12.0, 12.0);
                for (label, icon, detail) in [
                    ("Open PDF", "folder-open", "Browse your files"),
                    ("Edit PDF", "type", "Edit text and images"),
                    ("Merge PDFs", "files", "Combine documents"),
                    ("Convert", "file-output", "Word, images and more"),
                    ("Compress PDF", "file-down", "Reduce file size"),
                ] {
                    let width = ui.max_rect().width().clamp(1.0, 204.0);
                    let (rect, resp) = ui.allocate_exact_size(vec2(width, 116.0), Sense::click());
                    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
                    let press = theme::Press::track(ui, &resp);
                    ui.painter().rect(
                        rect,
                        CornerRadius::same(3),
                        press.fill(t.card, t.hover, t.pressed),
                        Stroke::new(1.0, t.border),
                        egui::StrokeKind::Inside,
                    );
                    let painter = ui.painter().with_clip_rect(rect);
                    icons::paint(ui, Rect::from_min_size(rect.min + vec2(16.0, 14.0), vec2(22.0, 22.0)), icon, 21.0, t.icon);
                    painter.text(rect.min + vec2(16.0, 58.0), Align2::LEFT_CENTER, label, theme::semibold(14.0), t.text);
                    painter.text(rect.min + vec2(16.0, 84.0), Align2::LEFT_CENTER, detail, theme::regular(12.0), t.text_muted);
                    let resp = theme::hand(resp).on_hover_text(detail);
                    if label == "Convert" {
                        egui::Popup::menu(&resp).show(|ui| {
                            for (format, command) in [
                                ("Word (.docx)", "export.docx"),
                                ("PowerPoint (.pptx)", "export.pptx"),
                                ("Images", "export.image"),
                                ("Plain text", "export.text"),
                                ("HTML", "export.html"),
                                ("Rich text (.rtf)", "export.rtf"),
                            ] {
                                if ui.button(format).clicked() {
                                    app.run_command(command);
                                    ui.close();
                                }
                            }
                        });
                    } else if resp.clicked() {
                        match label {
                            "Open PDF" => app.open_dialog(),
                            "Merge PDFs" => {
                                app.execute("page.combine");
                            }
                            "Edit PDF" => app.run_command("edit.edit_text"),
                            "Compress PDF" => app.run_command("optimize.reduce"),
                            _ => {}
                        }
                    }
                }
            });

            #[cfg(not(target_arch = "wasm32"))]
            {
                ui.add_space(26.0);
                crate::folders_ui::section(app, ui);
            }

            ui.add_space(26.0);
            let mut clear = false;
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(tl!("Recent")).font(theme::semibold(17.0)));
                if !app.recent.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let resp = widgets::ghost_button(ui, "trash-2", tl!("Clear"));
                        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Clear Recent Files")));
                        clear = resp.on_hover_text(tl!("Clear Recent Files")).clicked();
                    });
                }
            });
            ui.add_space(8.0);
            if app.recent.is_empty() {
                ui.label(egui::RichText::new(tl!("Open a PDF to get started. Your recent files will appear here.")).color(t.text_muted));
            }
            let mut open = None;
            let mut remove = None;
            for r in &app.recent {
                let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 46.0), Sense::click());
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &r.name));
                let press = theme::Press::track(ui, &resp);
                // The row, not its response: the pointer stays "in" the row over its remove button.
                let hovered = ui.rect_contains_pointer(rect);
                if hovered && press.hover < 0.04 && press.down < 0.04 {
                    // The remove button sits in the row, so the row response may not be hovered.
                    ui.painter().rect_filled(rect, CornerRadius::same(3), t.hover);
                } else {
                    press.wash(ui, rect, 3, false);
                }
                if hovered {
                    // Remove just this file from the list (#430); the file itself is untouched.
                    let x_rect = Rect::from_center_size(rect.right_center() - vec2(26.0, 0.0), vec2(28.0, 28.0));
                    let x = ui.interact(x_rect, ui.scope_id().with(("remove-recent", &r.path)), Sense::click());
                    let label = crate::i18n::fmt(tl!("Remove {name} from Recent"), &[("name", &r.name)]);
                    x.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &label));
                    if x.hovered() {
                        ui.painter().rect_filled(x_rect, CornerRadius::same(3), t.pressed);
                    }
                    icons::paint(ui, x_rect.shrink(6.0), "x", 16.0, t.icon);
                    if x.on_hover_text(tl!("Remove from Recent")).clicked() {
                        remove = Some(r.path.clone());
                    }
                }
                let body = rect.translate(press.offset());
                icons::paint(ui, Rect::from_min_size(body.min + vec2(10.0, 11.0), vec2(24.0, 24.0)), "file-text", 22.0, t.icon);
                // Room on the right for the remove button.
                let detail = ui.painter().text(
                    body.right_center() - vec2(48.0, 0.0),
                    Align2::RIGHT_CENTER,
                    format!("{}  ·  {}", crate::i18n::trn(crate::i18n::current(), r.pages as u64, "{n} page", "{n} pages"), human_size(r.size)),
                    theme::regular(12.0),
                    t.text_muted,
                );
                // The name and path stop short of the page count; hovering shows the whole path.
                let width = detail.left() - 16.0 - (body.left() + 46.0);
                widgets::row_text(ui, body.min + vec2(46.0, 15.0), r.name.as_str(), theme::medium(13.5), t.text, width);
                widgets::row_text(ui, body.min + vec2(46.0, 32.0), r.path.as_str(), theme::regular(11.0), t.text_faint, width);
                if theme::hand(resp).on_hover_text(&r.path).clicked() {
                    open = Some(r.path.clone());
                }
            }
            if let Some(p) = open {
                app.open_recent(&p);
            }
            if let Some(p) = remove {
                app.recent.retain(|r| r.path != p);
            }
            if clear {
                app.execute("file.clear_recent");
            }
        });
    });
}
