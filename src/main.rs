use std::alloc::Layout;

use eframe::egui::{self, RichText};

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        let panic_formatted = format!("{:#?}", panic_info);

        let panic_payload_display = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            Some(s.to_string())
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            Some(s.to_owned())
        } else {
            None
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

        eframe::run_simple_native(
            "My egui App",
            eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
                ..Default::default()
            },
            move |ctx, _frame| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("⚠").size(48.0));
                                ui.vertical(|ui| {
                                    ui.heading("App crashed");

                                    if let Some(panic_payload_display) = &panic_payload_display {
                                        ui.label(panic_payload_display);
                                    };

                                    ui.collapsing("Developer information", |ui| {
                                        ui.strong("Panic info");
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
    }));
    println!("Hello, world!");
    panic!("Whaaaaat???");
}
