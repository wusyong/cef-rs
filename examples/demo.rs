use cef::{
    args::Args, client::Client, string::CefString, App, BrowserSettings, BrowserView,
    PanelDelegate, Settings, ViewDelegate, Window, WindowDelegate,
};

#[derive(Debug, Copy, Clone)]
struct Application;

impl App for Application {}

#[derive(Debug, Clone)]
struct DemoClient;

impl Client for DemoClient {}

#[derive(Debug, Clone)]
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
            .add_child_view(&self.browser_view.as_view());
        window.show();
        self.window = Some(window.clone());
    }
}

fn main() {
    unsafe {
        let args = Args::new(std::env::args());
        // dbg!(&args);
        let app = Application;
        dbg!(cef::execute_process(&args, Some(app)));

        let settings = Settings::new();
        dbg!(cef::initialize(&args, settings, Some(app)));

        // let window_info = WindowInfo::new();
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
        dbg!(cef::create_top_level_window(delegate));

        // dbg!(cef::create_browser(
        //     window_info,
        //     Some(client),
        //     url,
        //     browser_settings
        // ));

        cef_sys::cef_run_message_loop();
    }
}
