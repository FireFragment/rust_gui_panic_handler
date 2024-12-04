use eframe::egui::{self, Color32, RichText, Vec2};

#[derive(Clone, Debug)]
pub struct AppInfo {
    pub name: &'static str,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

pub fn show_gui_egui(
    panic_payload_display: Option<String>,
    panic_formatted: String,
    app_info: AppInfo,
) {
    eframe::run_simple_native(
        "Crash report",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        },
        move |ctx, _frame| {
            ctx.style_mut(|style| {
                style.spacing.item_spacing = Vec2::new(4.0, 0.0);
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("⚠").size(48.0).color(Color32::RED));
                            ui.add_space(16.0);
                            ui.vertical(|ui| {
                                ui.heading(format!("{} crashed", app_info.name));

                                if let Some(panic_payload_display) = &panic_payload_display {
                                    ui.horizontal_wrapped(|ui| {
                                        //ui.strong("Reason:");
                                        ui.label(panic_payload_display);
                                    });
                                };

                                ui.horizontal_wrapped(|ui| {
                                    let mut links = app_info.links.iter();
                                    if let Some(link) = links.next() {
                                        if ui.link(link.label).clicked() {
                                            ctx.open_url(egui::OpenUrl {
                                                url: link.url.to_owned(),
                                                new_tab: true,
                                            });
                                        };
                                    }
                                    for link in links {
                                        ui.separator();
                                        if ui.link(link.label).clicked() {
                                            ctx.open_url(egui::OpenUrl {
                                                url: link.url.to_owned(),
                                                new_tab: true,
                                            });
                                        }
                                    }
                                });

                                ui.add_space(16.0);
                                ui.horizontal_wrapped(|ui| {
                                    ui.strong("Package name:");
                                    ui.monospace(env!("CARGO_PKG_NAME"));
                                });
                                ui.horizontal_wrapped(|ui| {
                                    ui.strong("Version:");
                                    ui.label(env!("CARGO_PKG_VERSION"));
                                });

                                ui.collapsing("Developer information", |ui| {
                                    ui.monospace(&panic_formatted);

                                    /*ui.strong("Panic payload");
                                    if let Some(panic_payload_debug) = &panic_payload_debug {
                                        ui.monospace(panic_payload_debug);
                                    } else {
                                        ui.label("Panic payload doesn't implement");
                                        ui.monospace("Debug");
                                    }*/
                                });
                            })
                        });
                    });
                });
            });
        },
    )
    .unwrap();
}

pub fn show_gui_dialog(
    panic_payload_display: Option<String>,
    panic_formatted: String,
    info: AppInfo,
) {
    native_dialog::MessageDialog::new()
        .set_title("Crash report")
        .set_text(&format!(
            "App crashed.\n{}",
            panic_payload_display.unwrap_or_default()
        ))
        .set_type(native_dialog::MessageType::Error)
        .show_alert()
        .unwrap();
}

pub fn register(info: AppInfo) {
    std::panic::set_hook(Box::new(move |panic_info| {
        let panic_formatted = format!("{:#?}", panic_info);

        let panic_payload_display = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            Some(s.to_string())
        } else {
            panic_info
                .payload()
                .downcast_ref::<String>()
                .map(|s| s.to_owned())
        };

        /*let panic_payload_display = Box::new(panic_info.payload())
            .downcast_ref::<Box<dyn std::fmt::Display>>()
            .map(|payload| format!("{payload}"));

        let panic_payload_debug = Box::new(panic_info.payload())
            .downcast_ref::<Box<dyn std::fmt::Debug>>()
            .map(|payload| format!("{payload:#?}"));*/

        println!("The app panicked.");
        println!("Panic info: {panic_formatted}");

        if let Some(panic_payload_display) = &panic_payload_display {
            println!("Panic payload: {panic_payload_display}");
        } else {
            println!("Panic payload doesn't implement `Display`")
        }
        /*if let Some(panic_payload_debug) = &panic_payload_debug {
            println!("Payload debug info: {panic_payload_debug}");
        } else {
            println!("Panic payload doesn't implement `Debug`")
        }*/

        show_gui_egui(panic_payload_display, panic_formatted, info.clone());
    }));
}
