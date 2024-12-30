use cef::{args::Args, bindings::*};

#[derive(Debug, Clone, Copy)]
struct Application;

impl ImplApp for Application {}

#[derive(Debug)]
struct DemoClient;

impl ImplClient for DemoClient {}

#[derive(Debug)]
struct DemoWindow {
    browser_view: BrowserView,
}

impl ImplViewDelegate for DemoWindow {
    fn on_child_view_changed(
        &self,
        _view: &mut View,
        _added: ::std::os::raw::c_int,
        _child: &mut View,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }
}
impl ImplPanelDelegate for DemoWindow {}
impl ImplWindowDelegate for DemoWindow {
    fn on_window_created(&self, window: &mut Window) {
        window.add_child_view(self.browser_view.get_view());
        window.show();
    }

    fn on_window_destroyed(&self, _window: &mut Window) {
        quit_message_loop();
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let mut app = Application;
    let settings = Settings::default();
    dbg!(initialize(&args, &settings, &mut app, std::ptr::null_mut()));
    dbg!(execute_process(&args, &mut app, std::ptr::null_mut()));

    // let window_info = WindowInfo::new();
    let browser_settings = BrowserSettings::default();
    let mut client = DemoClient;
    let url = CefString::from("https://www.google.com");

    let browser_view = dbg!(browser_view_create(
        &mut client,
        &url,
        &browser_settings,
        &mut Default::default(),
        &mut Default::default(),
        &mut Default::default(),
    ));
    let mut delegate = DemoWindow { browser_view };

    let x = dbg!(cef_window_create_top_level(&mut delegate));
    // dbg!(cef::create_browser(
    //     window_info,
    //     Some(client),
    //     url,
    //     browser_settings
    // ));

    run_message_loop();
    dbg!(x.has_one_ref());

    shutdown();
}
