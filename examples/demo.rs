use cef::{
    args::Args, client::Client, rc::Rc, string::CefString, App, BrowserSettings, BrowserView,
    PanelDelegate, Settings, ViewDelegate, WindowDelegate,
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
}

impl ViewDelegate for DemoWindow {
    fn on_child_view_changed(&self, view: cef::View, _added: bool, _child: cef::View) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }
}
impl PanelDelegate for DemoWindow {}
impl WindowDelegate for DemoWindow {
    fn on_window_created(&self, window: cef::Window) {
        window
            .get_panel()
            .add_child_view(self.browser_view.get_view());
        window.show();
    }

    fn on_window_destroyed(&self, _window: cef::Window) {
        cef::quit_message_loop();
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let app = Application;
    let settings = Settings::new();
    dbg!(cef::initialize(&args, &settings, Some(app)));
    dbg!(cef::execute_process(&args, Some(app)));

    // let window_info = WindowInfo::new();
    let browser_settings = BrowserSettings::new();
    let client = DemoClient;
    let url = CefString::new("https://www.google.com");

    let browser_view = dbg!(cef::create_browser_view(
        Some(client),
        url,
        browser_settings
    ));
    let delegate = DemoWindow { browser_view };

    let x = dbg!(cef::create_top_level_window(delegate));
    // dbg!(cef::create_browser(
    //     window_info,
    //     Some(client),
    //     url,
    //     browser_settings
    // ));

    cef::run_message_loop();
    dbg!(x.has_one_ref());

    cef::shutdown();
}
