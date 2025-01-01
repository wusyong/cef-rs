use cef::{args::Args, rc::Rc, *};

#[derive(Clone)]
struct Application(*mut cef_sys::_cef_app_t);

impl Rc for Application {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe { &*(self.0 as *const _) }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self(App::default().into_raw())
    }
}

impl ImplApp for Application {}

#[derive(Clone)]
struct DemoClient(*mut cef_sys::_cef_client_t);

impl Rc for DemoClient {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe { &*(self.0 as *const _) }
    }
}

impl Default for DemoClient {
    fn default() -> Self {
        Self(Client::default().into_raw())
    }
}

impl ImplClient for DemoClient {}

#[derive(Clone)]
struct DemoWindow {
    base: *mut cef_sys::_cef_window_delegate_t,
    browser_view: BrowserView,
}

impl Rc for DemoWindow {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe { &*(self.base as *const _) }
    }
}

impl Default for DemoWindow {
    fn default() -> Self {
        Self {
            base: ImplWindowDelegate::into_raw(WindowDelegate::default()),
            browser_view: BrowserView::default(),
        }
    }
}

impl ImplViewDelegate for DemoWindow {
    fn on_child_view_changed(
        &self,
        _view: &mut impl ImplView,
        _added: ::std::os::raw::c_int,
        _child: &mut impl ImplView,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }
}
impl ImplPanelDelegate for DemoWindow {}
impl ImplWindowDelegate for DemoWindow {
    fn on_window_created(&self, window: &mut impl ImplWindow) {
        let mut view = self.browser_view.clone();
        window.add_child_view(&mut view);
        window.show();
    }

    fn on_window_destroyed(&self, _window: &mut impl ImplWindow) {
        quit_message_loop();
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let mut app = Application::default();
    let settings = Settings::default();
    dbg!(initialize(args.as_raw(), &settings, &mut app, std::ptr::null_mut()));
    dbg!(execute_process(args.as_raw(), &mut app, std::ptr::null_mut()));

    // let window_info = WindowInfo::new();
    let browser_settings = BrowserSettings::default();
    let mut client = DemoClient::default();
    let url = CefString::from(&CefStringUtf8::from("https://www.google.com"));

    let browser_view = browser_view_create(
        &mut client,
        &url,
        &browser_settings,
        &mut DictionaryValue::default(),
        &mut RequestContext::default(),
        &mut BrowserViewDelegate::default(),
    );
    let mut delegate = DemoWindow {
        browser_view,
        ..Default::default()
    };

    let x = window_create_top_level(&mut delegate);
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
