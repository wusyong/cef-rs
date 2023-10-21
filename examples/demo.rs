use cef::{
    args::Args, client::Client, string::CefString, App, BrowserSettings, BrowserView,
    PanelDelegate, Settings, ViewDelegate, Window, WindowDelegate, WindowInfo,
};

#[derive(Debug, Clone, Copy)]
struct Application;

impl App for Application {}

#[derive(Debug)]
struct DemoClient;

impl Client for DemoClient {}

#[derive(Debug)]
struct DemoWindow {
    browser_view: BrowserView,
    window: Option<Window>,
}

impl ViewDelegate for DemoWindow {}
impl PanelDelegate for DemoWindow {}
impl WindowDelegate for DemoWindow {
    fn on_window_created(&mut self, window: &cef::Window) {
        window
            .as_panel()
            .add_child_view(self.browser_view.as_view());
        window.show();
        // self.window = Some(window.clone());
    }

    fn on_window_destroyed(&mut self, _window: &cef::Window) {
        // self.window = None;
        // dbg!(self.browser_view.0.has_one_ref());
        unsafe { cef_sys::cef_quit_message_loop() };
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let app = Application;
    dbg!(cef::execute_process(&args, Some(app)));

    let settings = Settings::new();
    dbg!(cef::initialize(&args, settings, Some(app)));

    let window_info = WindowInfo::new();
    let browser_settings = BrowserSettings::new();
    let client = DemoClient;
    let url = CefString::new("https://www.google.com");

    let browser_view = dbg!(cef::create_browser_view(
        Some(client),
        url,
        browser_settings
    ));
    let delegate = DemoWindow {
        browser_view,
        window: None,
    };

    let x = dbg!(cef::create_top_level_window(delegate));

    // dbg!(cef::create_browser(
    //     window_info,
    //     Some(client),
    //     url,
    //     browser_settings
    // ));

    cef::run_message_loop();
    // dbg!(x.0.has_one_ref());

    cef::shutdown();
}
