use std::path::PathBuf;

use gui_panic_handler::AppInfo;
use gui_panic_handler::Link;

fn main() {
    gui_panic_handler::register(AppInfo {
        name: "GUI panic handler testing app",
        links: vec![
            Link {
                label: "Get help",
                url: "https://example.com",
            },
            Link {
                label: "Report bug",
                url: "https://example.com",
            },
            Link {
                label: "Application website",
                url: "https://example.com",
            },
        ],
    });

    println!("Hello, world!");
    panic!("Whaaaaat???");
}
