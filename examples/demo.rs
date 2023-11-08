use std::ptr::null_mut;

use cef::{
    args::Args,
    client::{CefClient, ClientTrait},
    rc::{Rc, RefGuard},
    string::CefString,
    App, BrowserSettings, BrowserView, CefApp, PanelDelegate, Settings, ViewDelegate,
    WindowDelegate, WindowInfo,
};
use cef_sys::_cef_window_t;

#[derive(Debug)]
struct DemoWindow {
    browser_view: BrowserView,
}

impl ViewDelegate for DemoWindow {
    fn on_window_changed(&self, _view: cef::View, _added: bool) {
        println!("on_window_changed.");
    }

    fn on_child_view_changed(&self, view: cef::View, _added: bool, _child: cef::View) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
        println!("on_child_view_changed.");

        /*match view.as_browser_view() {
            Some(bv)=>{
                match bv.get_browser() {
                    Some(host)=>{
                    }
                    None=>{}
                }
            }
            None=>{

            }
        }*/
    }
}
impl PanelDelegate for DemoWindow {}
impl WindowDelegate for DemoWindow {
    fn on_window_created(&self, window: cef::Window) {
        println!("OnWindowCreated.");
        window
            .get_panel()
            .add_child_view(self.browser_view.get_view());
        window.show();
        //window.maximize();
        //window.minimize();
        //window.restore();
        //window.maximize();
        window.set_fullscreen();
    }

    fn on_window_destroyed(&self, _window: cef::Window) {
        cef::quit_message_loop();
    }

    fn can_close(&mut self, _window: cef::Window) -> bool {
        true
    }
}

fn main() {
    println!("DemoRunning");

    let args = Args::new(std::env::args());
    // dbg!(&args);
    let app = CefApp {};

    let settings = Settings::new();

    println!("LogFile={}", settings.log_file.to_string());

    dbg!(cef::initialize(&args, &settings, Some(app)));
    dbg!(cef::execute_process(&args, Some(app)));

    let client = CefClient::new();
    /*
    unsafe {
        let c1 = client.clone().into_raw().as_ref().unwrap();
        match c1.get_keyboard_handler {
            Some(khr)=>{
                println!("Got keyboard_handler");
            }
            None=>{
                println!("None of keyboard_handler");
            }
        }
    }*/
    //c1.as_ref().unwrap().get_keyboard_handler;

    let browser_settings = BrowserSettings::new();

    let url = CefString::new("https://bing.com/");

    let browser_view: BrowserView = cef::create_browser_view(Some(client), url, browser_settings);

    let delegate: DemoWindow = DemoWindow { browser_view };

    //trace_macros!(true);
    let x: cef::Window = dbg!(cef::create_top_level_window(delegate));
    //trace_macros!(false);

    /* 
    unsafe {
        let c2 = client.clone().into_raw().as_ref().unwrap();
        match c2.get_keyboard_handler {
            Some(khr)=>{
                println!("Got keyboard_handler");
            }
            None=>{
                println!("None of keyboard_handler");
            }
        }
    }
    */
    cef::run_message_loop();
    dbg!(x.has_one_ref());

    cef::shutdown();
}
