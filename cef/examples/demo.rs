use cef::{args::Args, rc::*, *};

struct Application(*mut RcImpl<cef_sys::_cef_app_t, Application>);

impl Application {
    fn new() -> App {
        unsafe {
            let mut cef_object = std::mem::zeroed();
            <Self as ImplApp>::init_methods(&mut cef_object);
            let interface = Self(std::ptr::null_mut());
            let object = RcImpl::new(cef_object, interface);
            (*object).interface.0 = object;
            (object as *mut cef_sys::_cef_app_t).as_wrapper()
        }
    }
}

impl Clone for Application {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for Application {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplApp for Application {
    fn get_raw(&self) -> *mut cef_sys::_cef_app_t {
        self.0 as *mut cef_sys::_cef_app_t
    }
}

struct DemoClient(*mut RcImpl<cef_sys::_cef_client_t, DemoClient>);

impl DemoClient {
    fn new() -> Client {
        unsafe {
            let mut cef_object = std::mem::zeroed();
            <Self as ImplClient>::init_methods(&mut cef_object);
            let interface = Self(std::ptr::null_mut());
            let object = RcImpl::new(cef_object, interface);
            (*object).interface.0 = object;
            (object as *mut cef_sys::_cef_client_t).as_wrapper()
        }
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

struct DemoDictionaryValue(*mut RcImpl<cef_sys::_cef_dictionary_value_t, DemoDictionaryValue>);

impl DemoDictionaryValue {
    fn new() -> DictionaryValue {
        unsafe {
            let mut cef_object = std::mem::zeroed();
            <Self as ImplDictionaryValue>::init_methods(&mut cef_object);
            let interface = Self(std::ptr::null_mut());
            let object = RcImpl::new(cef_object, interface);
            (*object).interface.0 = object;
            (object as *mut cef_sys::_cef_dictionary_value_t).as_wrapper()
        }
    }
}

impl Clone for DemoDictionaryValue {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoDictionaryValue {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplDictionaryValue for DemoDictionaryValue {
    fn get_raw(&self) -> *mut cef_sys::_cef_dictionary_value_t {
        self.0 as *mut cef_sys::_cef_dictionary_value_t
    }
}

struct DemoRequestContext(*mut RcImpl<cef_sys::_cef_request_context_t, DemoRequestContext>);

impl DemoRequestContext {
    fn new() -> RequestContext {
        unsafe {
            let mut cef_object = std::mem::zeroed();
            <Self as ImplRequestContext>::init_methods(&mut cef_object);
            let interface = Self(std::ptr::null_mut());
            let object = RcImpl::new(cef_object, interface);
            (*object).interface.0 = object;
            (object as *mut cef_sys::_cef_request_context_t).as_wrapper()
        }
    }
}

impl Clone for DemoRequestContext {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoRequestContext {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplPreferenceManager for DemoRequestContext {
    fn get_raw(&self) -> *mut cef_sys::_cef_preference_manager_t {
        self.0 as *mut cef_sys::_cef_preference_manager_t
    }
}

impl ImplRequestContext for DemoRequestContext {
    fn get_raw(&self) -> *mut cef_sys::_cef_request_context_t {
        self.0 as *mut cef_sys::_cef_request_context_t
    }
}

struct DemoBrowserViewDelegate(
    *mut RcImpl<cef_sys::_cef_browser_view_delegate_t, DemoBrowserViewDelegate>,
);

impl DemoBrowserViewDelegate {
    fn new() -> BrowserViewDelegate {
        unsafe {
            let mut cef_object = std::mem::zeroed();
            <Self as ImplBrowserViewDelegate>::init_methods(&mut cef_object);
            let interface = Self(std::ptr::null_mut());
            let object = RcImpl::new(cef_object, interface);
            (*object).interface.0 = object;
            (object as *mut cef_sys::_cef_browser_view_delegate_t).as_wrapper()
        }
    }
}

impl Clone for DemoBrowserViewDelegate {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }

        Self(self.0)
    }
}

impl Rc for DemoBrowserViewDelegate {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplViewDelegate for DemoBrowserViewDelegate {
    fn get_raw(&self) -> *mut cef_sys::_cef_view_delegate_t {
        self.0 as *mut cef_sys::_cef_view_delegate_t
    }
}

impl ImplBrowserViewDelegate for DemoBrowserViewDelegate {
    fn get_raw(&self) -> *mut cef_sys::_cef_browser_view_delegate_t {
        self.0 as *mut cef_sys::_cef_browser_view_delegate_t
    }
}

struct DemoWindow {
    base: *mut RcImpl<cef_sys::_cef_window_delegate_t, DemoWindow>,
    browser_view: BrowserView,
}

impl DemoWindow {
    fn new(browser_view: BrowserView) -> WindowDelegate {
        unsafe {
            let mut cef_object: cef_sys::_cef_window_delegate_t = std::mem::zeroed();
            <Self as ImplWindowDelegate>::init_methods(&mut cef_object);
            let interface = Self {
                base: std::ptr::null_mut(),
                browser_view,
            };
            let object = RcImpl::new(cef_object, interface);
            (*object).interface.base = object;
            (object as *mut cef_sys::_cef_window_delegate_t).as_wrapper()
        }
    }
}

impl Clone for DemoWindow {
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

impl Rc for DemoWindow {
    fn as_base(&self) -> &cef_sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
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

    fn get_raw(&self) -> *mut cef_sys::_cef_view_delegate_t {
        self.base as *mut cef_sys::_cef_view_delegate_t
    }
}

impl ImplPanelDelegate for DemoWindow {
    fn get_raw(&self) -> *mut cef_sys::_cef_panel_delegate_t {
        self.base as *mut cef_sys::_cef_panel_delegate_t
    }
}

impl ImplWindowDelegate for DemoWindow {
    fn on_window_created(&self, window: &mut impl ImplWindow) {
        let mut view = self.browser_view.clone();
        window.add_child_view(&mut view);
        window.show();
    }

    fn can_close(&self, _window: &mut impl ImplWindow) -> ::std::os::raw::c_int {
        1
    }

    fn on_window_destroyed(&self, _window: &mut impl ImplWindow) {
        quit_message_loop();
    }

    fn get_raw(&self) -> *mut cef_sys::_cef_window_delegate_t {
        self.base as *mut cef_sys::_cef_window_delegate_t
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let mut app = Application::new();
    dbg!(initialize(
        args.as_main_args(),
        &Default::default(),
        &mut app,
        std::ptr::null_mut()
    ));
    {
        dbg!(execute_process(
            args.as_main_args(),
            &mut app,
            std::ptr::null_mut()
        ));

        // let window_info = WindowInfo::new();
        let mut client = DemoClient::new();
        let url = CefString::from(&CefStringUtf8::from("https://www.google.com"));

        // TODO: Make Default::default() work for this, and make these arguments Option<&mut T>.
        let mut extra_info = DemoDictionaryValue::new();
        let mut request_context = DemoRequestContext::new();
        let mut delegate = DemoBrowserViewDelegate::new();

        let browser_view = browser_view_create(
            &mut client,
            &url,
            &Default::default(),
            &mut extra_info,
            &mut request_context,
            &mut delegate,
        );
        let mut delegate = DemoWindow::new(browser_view);

        let x = window_create_top_level(&mut delegate);
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
