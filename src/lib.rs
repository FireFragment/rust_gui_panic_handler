#![doc = r#"
# GUI panic handler

This crate allows you to handle panics with a GUI dialog made with [egui](https://github.com/emilk/egui).

The dialog shows panic payload, information about location of the panic and
if you want, an option to report the panic to developer and external links.

You will most likely want to use [`register`] to register the panic handler.

"#]
#![cfg_attr(
    feature = "error-reporting",
    doc = r#"

## A simple example

<img alt="Screenshot of a dialog with crash information" height="400" src="https://raw.githubusercontent.com/FireFragment/rust_gui_panic_handler/v0.1.0/docs/screenshot.png">



```rust,no_run
use gui_panic_handler::AppInfo;
use gui_panic_handler::Link;

gui_panic_handler::register(AppInfo {
    name: "Sample app",
    additional_text: "We are sorry, the application has crashed. To help us fix the crash, please report it using the button below.",
    links: vec![
        Link {
            label: "Browse known crash causes",
            url: "https://example.com",
        },
        Link {
            label: "Get help on our forum",
            url: "https://example.com",
        },
        Link {
            label: "Our website",
            url: "https://example.com",
        },
    ],
    report_bug_url: Some(gui_panic_handler::GitHubBugReporter::new(
        String::from("FireFragment"),
        String::from("rust_gui_panic_handler"),
    )),
});

println!("Reading env var...");
let env_var_value = std::env::var("SUPER_IMPORTANT_ENVIRONMENT_VARIABLE").unwrap();

println!("Read: {env_var_value}")
```
"#
)]

#[cfg(feature = "error-reporting")]
mod error_reporting;
#[cfg(feature = "error-reporting")]
pub use error_reporting::*;

use eframe::egui::{self, Color32, RichText, Vec2};

/// Information about the application used in the error dialog box
#[cfg_attr(
    feature = "error-reporting",
    doc = r#"If you don't want to have bug report button, you can use [`AppInfoNoBugReport`] instead"#
)]
#[derive(Clone, Debug)]
pub struct AppInfo<#[cfg(feature = "error-reporting")] F: ReportBugUrlMaker> {
    /// Name of the application
    pub name: &'static str,
    pub additional_text: &'static str,

    /// Links to be displayed in the error dialog box
    pub links: Vec<Link>,

    /// Used to generate a URL for bug reports.
    /// If you are using GitHub, you can use the ready-made [`GitHubBugReporter`] reporter.
    ///
    /// You can use simple closure like this:
    /// ```
    /// # let report =
    /// Some(|payload: Option<String>, bug_report| {
    ///     format!(
    ///         "https://github.com/FireFragment/rust_gui_panic_handler/issues/new?title=Unhandled panic: {}&body={}",
    ///         gui_panic_handler::urlencoding::encode(&payload.unwrap_or_default()),
    ///         gui_panic_handler::urlencoding::encode(&format!("### Panic report\n{bug_report}"))
    ///     )
    /// })
    /// # ; gui_panic_handler::AppInfo {
    /// #   name: "Sample app",
    /// #   additional_text: "",
    /// #   links: Vec::new(),
    /// #   report_bug_url: report,
    /// # };
    ///
    /// ```
    ///
    /// If you don't want to have bug report button, you can disable the `error-reporting` feature or use [`AppInfoNoBugReport`] instead and set this field to `None`
    #[cfg(feature = "error-reporting")]
    pub report_bug_url: Option<F>,
}

/// A link to be displayed in the error dialog box
#[derive(Clone, Debug)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

/// Puts all details about the crash to a single [string](String).
///
/// Currently used for the `Report crash` and `Copy details` buttons
pub fn details<#[cfg(feature = "error-reporting")] F: ReportBugUrlMaker>(
    panic_payload_display: &Option<String>,
    panic_formatted: &String,
    #[cfg(feature = "error-reporting")] app_info: &AppInfo<F>,
    #[cfg(not(feature = "error-reporting"))] app_info: &AppInfo,
) -> String {
    format!(
        "**Panic report from {}**

{}

Package name: `{}`
Version: `{}`

Panic info:
```
{panic_formatted}
```",
        panic_payload_display
            .as_ref()
            .unwrap_or(&String::from("[PAYLOAD IS NOT A STRING]")),
        app_info.name,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
}

/// Displays the panic dialog using [egui]
pub fn show_gui_egui<#[cfg(feature = "error-reporting")] F: ReportBugUrlMaker>(
    panic_payload_display: Option<String>,
    panic_formatted: String,
    #[cfg(feature = "error-reporting")] app_info: AppInfo<F>,
    #[cfg(not(feature = "error-reporting"))] app_info: AppInfo,
) {
    eframe::run_simple_native(
        "Crash report",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_maximize_button(false)
                .with_always_on_top()
                .with_inner_size([512.0, 256.0]),
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

                                ui.add_space(8.0);

                                ui.label(app_info.additional_text);

                                if let Some(panic_payload_display) = &panic_payload_display {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.strong("Reason:");
                                        ui.monospace(panic_payload_display);
                                    });
                                };

                                ui.add_space(8.0);

                                ui.horizontal_wrapped(|ui| {
                                    if ui.button("📋 Copy details").clicked() {
                                        ui.output_mut(|out| {
                                            out.copied_text = details(
                                                &panic_payload_display,
                                                &panic_formatted,
                                                &app_info,
                                            );
                                        });
                                    }
                                    #[cfg(feature = "error-reporting")]
                                    if let Some(bug_report_url_maker) = &app_info.report_bug_url {
                                        if ui.button("💬 Report crash").clicked() {
                                            ui.output_mut(|out| {
                                                out.open_url = Some(egui::OpenUrl {
                                                    url: bug_report_url_maker.get_report_url(
                                                        panic_payload_display.clone(),
                                                        details(
                                                            &panic_payload_display,
                                                            &panic_formatted,
                                                            &app_info,
                                                        ),
                                                    ),
                                                    new_tab: true,
                                                });
                                            });
                                        }
                                    }
                                });

                                ui.add_space(8.0);

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

/// Register the panic handler. Run at the beggining of your program.
pub fn register<#[cfg(feature = "error-reporting")] F: ReportBugUrlMaker>(
    #[cfg(feature = "error-reporting")] app_info: AppInfo<F>,
    #[cfg(not(feature = "error-reporting"))] app_info: AppInfo,
) {
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

        show_gui_egui(panic_payload_display, panic_formatted, app_info.clone());
    }));
}
