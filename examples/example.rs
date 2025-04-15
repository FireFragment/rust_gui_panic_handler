fn main() {
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
}
