use cef::{
    app::App, args::Args, browser::BrowserSettings, client::Client, settings::Settings,
    string::CefString, window::WindowInfo,
};

#[derive(Debug, Copy, Clone)]
struct Application;

impl App for Application {}

#[derive(Debug, Clone)]
struct DemoClient;

impl Client for DemoClient {}

fn main() {
    unsafe {
        let args = Args::new(std::env::args());
        let app = Application;
        dbg!(cef::app::execute_process(&args, Some(app)));

        let settings = Settings::new();
        dbg!(cef::app::initialize(&args, settings, Some(app)));

        let window_info = WindowInfo::new();
        let browser_settings = BrowserSettings::new();
        let client = DemoClient;
        let url = CefString::new("https://www.google.com");

        dbg!(cef::browser::create_browser(
            window_info,
            Some(client),
            url,
            browser_settings
        ));

        cef_sys::cef_run_message_loop();
    }
}
