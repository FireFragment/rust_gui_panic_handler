use std::convert::Infallible;
pub use urlencoding;

use eframe::egui::{self, Color32, RichText, Vec2};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct GitHubBugReporter {
    pub repo_owner: String,
    pub repo_name: String,
}

impl GitHubBugReporter {
    pub fn new(repo_owner: String, repo_name: String) -> Self {
        Self {
            repo_owner,
            repo_name,
        }
    }
}

impl ReportBugUrlMaker for GitHubBugReporter {
    fn get_report_url(&self, payload: Option<String>, bug_report: String) -> String {
        format!(
            "https://github.com/{}/{}/issues/new?title=Unhandled panic: {}&body={}",
            self.repo_owner,
            self.repo_name,
            urlencoding::encode(&payload.unwrap_or_default()),
            urlencoding::encode(&format!("### Panic report\n{bug_report}"))
        )
    }
}

/// Generates a URL for bug reports - to be used as [`AppInfo::report_bug_url`]
///
/// If you are using GitHub, you can use the [`github_report_bug_url`] function.
pub trait ReportBugUrlMaker: Clone + Send + Sync + 'static {
    fn get_report_url(&self, payload: Option<String>, bug_report: String) -> String;
}

impl<T: Fn(Option<String>, String) -> String + Clone + Send + Sync + 'static> ReportBugUrlMaker
    for T
{
    fn get_report_url(&self, payload: Option<String>, bug_report: String) -> String {
        self(payload, bug_report)
    }
}

impl ReportBugUrlMaker for Infallible {
    fn get_report_url(&self, _payload: Option<String>, _bug_report: String) -> String {
        eprintln!("Called `get_report_url` of `Infallible` - `Infallible` should never exist, but here it is: {self:?}");
        String::new()
    }
}

pub type AppInfoNoBugReport = AppInfo<Infallible>;

/// Information about the application used in the error dialog box
///
/// If you don't want to have bug report button, you can use [`AppInfoNoBugReport`] instead
#[derive(Clone, Debug)]
pub struct AppInfo<F: ReportBugUrlMaker = Infallible> {
    /// Name of the application
    pub name: &'static str,
    pub additional_text: &'static str,

    /// Links to be displayed in the error dialog box
    pub links: Vec<Link>,

    /// Used to generate a URL for bug reports
    /// If you are using GitHub, you can use the ready-made [`github_report_bug_url`] reporter
    ///
    /// You can use simple closure like this:
    /// ```
    /// |payload: Option<String>, bug_report| {
    /// format!(
    ///     "https://github.com/FireFragment/rust_gui_panic_handler/issues/new?title=Unhandled panic: {}&body={}",
    ///     gui_panic_handler::urlencoding::encode(&payload.unwrap_or_default()),
    ///     gui_panic_handler::urlencoding::encode(&format!("### Panic report\n{bug_report}"))
    /// )
    /// ```
    ///
    /// If you don't want to have bug report button, you can use [`AppInfoNoBugReport`] instead and set this field to `None`
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
pub fn details<F: ReportBugUrlMaker>(
    panic_payload_display: &Option<String>,
    panic_formatted: &String,
    app_info: &AppInfo<F>,
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

pub fn show_gui_egui<F: ReportBugUrlMaker>(
    panic_payload_display: Option<String>,
    panic_formatted: String,
    app_info: AppInfo<F>,
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

pub fn register<F: ReportBugUrlMaker>(info: AppInfo<F>) {
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
