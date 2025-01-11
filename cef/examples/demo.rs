use cef::{args::Args, rc::*, *};

struct DemoApp(*mut RcImpl<cef_sys::_cef_app_t, Self>);

impl DemoApp {
    fn new() -> App {
        App::new(Self(std::ptr::null_mut()))
    }
}

impl WrapApp for DemoApp {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_app_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoApp {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoApp {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplApp for DemoApp {
    fn get_raw(&self) -> *mut cef_sys::_cef_app_t {
        self.0 as *mut cef_sys::_cef_app_t
    }
}

struct DemoClient(*mut RcImpl<cef_sys::_cef_client_t, Self>);

impl DemoClient {
    fn new() -> Client {
        Client::new(Self(std::ptr::null_mut()))
    }
}

impl WrapClient for DemoClient {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_client_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoClient {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoClient {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplClient for DemoClient {
    fn get_raw(&self) -> *mut cef_sys::_cef_client_t {
        self.0 as *mut cef_sys::_cef_client_t
    }
}

struct DemoWindowDelegate {
    base: *mut RcImpl<cef_sys::_cef_window_delegate_t, Self>,
    browser_view: BrowserView,
}

impl DemoWindowDelegate {
    fn new(browser_view: BrowserView) -> WindowDelegate {
        WindowDelegate::new(Self {
            base: std::ptr::null_mut(),
            browser_view,
        })
    }
}

impl WrapWindowDelegate for DemoWindowDelegate {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef_sys::_cef_window_delegate_t, Self>) {
        self.base = object;
    }
}

impl Clone for DemoWindowDelegate {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }

        Self {
            base: self.base,
            browser_view: self.browser_view.clone(),
        }
    }
}

impl Rc for DemoWindowDelegate {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplViewDelegate for DemoWindowDelegate {
    fn on_child_view_changed(
        &self,
        _view: Option<&mut impl ImplView>,
        _added: ::std::os::raw::c_int,
        _child: Option<&mut impl ImplView>,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }

    fn get_raw(&self) -> *mut cef_sys::_cef_view_delegate_t {
        self.base as *mut cef_sys::_cef_view_delegate_t
    }
}

impl ImplPanelDelegate for DemoWindowDelegate {}

impl ImplWindowDelegate for DemoWindowDelegate {
    fn on_window_created(&self, window: Option<&mut impl ImplWindow>) {
        if let Some(window) = window {
            let mut view = self.browser_view.clone();
            window.add_child_view(Some(&mut view));
            window.show();
        }
    }

    fn can_close(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    fn on_window_destroyed(&self, _window: Option<&mut impl ImplWindow>) {
        quit_message_loop();
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let mut app = DemoApp::new();
    dbg!(initialize(
        Some(args.as_main_args()),
        Some(&Default::default()),
        Some(&mut app),
        std::ptr::null_mut()
    ));
    {
        dbg!(execute_process(
            Some(args.as_main_args()),
            Some(&mut app),
            std::ptr::null_mut()
        ));

        // let window_info = WindowInfo::new();
        let mut client = DemoClient::new();
        let url = CefString::from(&CefStringUtf8::from("https://www.google.com"));

        let browser_view = browser_view_create(
            Some(&mut client),
            Some(&url),
            Some(&Default::default()),
            Option::<&mut DictionaryValue>::None,
            Option::<&mut RequestContext>::None,
            Option::<&mut BrowserViewDelegate>::None,
        )
        .expect("Failed to create browser view");
        let mut delegate = DemoWindowDelegate::new(browser_view);

        let x = window_create_top_level(Some(&mut delegate)).expect("Failed to create window");
        // dbg!(cef::create_browser(
        //     window_info,
        //     Some(client),
        //     url,
        //     browser_settings
        // ));

        run_message_loop();
        dbg!(x.has_one_ref());
    }

    shutdown();
}
