use std::ptr::null_mut;

use cef_sys::{
    _XEvent, _cef_browser_t, _cef_key_event_t, _cef_keyboard_handler_t, cef_client_t,
    cef_keyboard_handler_t,
};

use crate::rc::RcImpl;

pub struct KeyBoardHandler;
impl KeyHandlerTrait for KeyBoardHandler {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowsKeyCode(i32);

#[allow(non_upper_case_globals)]
impl WindowsKeyCode {
    pub const LButton: Self = Self(0x01);
    pub const RButton: Self = Self(0x02);
    pub const Cancel: Self = Self(0x03);
    pub const MButton: Self = Self(0x04);
    pub const XButton1: Self = Self(0x05);
    pub const XButton2: Self = Self(0x06);
    pub const Back: Self = Self(0x08);
    pub const Tab: Self = Self(0x09);
    pub const Clear: Self = Self(0x0C);
    pub const Return: Self = Self(0x0D);
    pub const Shift: Self = Self(0x10);
    pub const Control: Self = Self(0x11);
    pub const Menu: Self = Self(0x12);
    pub const Pause: Self = Self(0x13);
    pub const Capital: Self = Self(0x14);
    pub const Kana: Self = Self(0x15);
    pub const Junja: Self = Self(0x17);
    pub const Final: Self = Self(0x18);
    pub const Hanja: Self = Self(0x19);
    pub const Escape: Self = Self(0x1B);
    pub const Convert: Self = Self(0x1C);
    pub const NonConvert: Self = Self(0x1D);
    pub const Accept: Self = Self(0x1E);
    pub const Modechange: Self = Self(0x1F);
    pub const Space: Self = Self(0x20);
    pub const Prior: Self = Self(0x21);
    pub const Next: Self = Self(0x22);
    pub const End: Self = Self(0x23);
    pub const Home: Self = Self(0x24);
    pub const Left: Self = Self(0x25);
    pub const Up: Self = Self(0x26);
    pub const Right: Self = Self(0x27);
    pub const Down: Self = Self(0x28);
    pub const Select: Self = Self(0x29);
    pub const Print: Self = Self(0x2A);
    pub const Execute: Self = Self(0x2B);
    pub const Snapshot: Self = Self(0x2C);
    pub const Insert: Self = Self(0x2D);
    pub const Delete: Self = Self(0x2E);
    pub const Help: Self = Self(0x2F);
    pub const Key0: Self = Self(0x30);
    pub const Key1: Self = Self(0x31);
    pub const Key2: Self = Self(0x32);
    pub const Key3: Self = Self(0x33);
    pub const Key4: Self = Self(0x34);
    pub const Key5: Self = Self(0x35);
    pub const Key6: Self = Self(0x36);
    pub const Key7: Self = Self(0x37);
    pub const Key8: Self = Self(0x38);
    pub const Key9: Self = Self(0x39);
    pub const A: Self = Self(0x41);
    pub const B: Self = Self(0x42);
    pub const C: Self = Self(0x43);
    pub const D: Self = Self(0x44);
    pub const E: Self = Self(0x45);
    pub const F: Self = Self(0x46);
    pub const G: Self = Self(0x47);
    pub const H: Self = Self(0x48);
    pub const I: Self = Self(0x49);
    pub const J: Self = Self(0x4A);
    pub const K: Self = Self(0x4B);
    pub const L: Self = Self(0x4C);
    pub const M: Self = Self(0x4D);
    pub const N: Self = Self(0x4E);
    pub const O: Self = Self(0x4F);
    pub const P: Self = Self(0x50);
    pub const Q: Self = Self(0x51);
    pub const R: Self = Self(0x52);
    pub const S: Self = Self(0x53);
    pub const T: Self = Self(0x54);
    pub const U: Self = Self(0x55);
    pub const V: Self = Self(0x56);
    pub const W: Self = Self(0x57);
    pub const X: Self = Self(0x58);
    pub const Y: Self = Self(0x59);
    pub const Z: Self = Self(0x5A);
    pub const LWin: Self = Self(0x5B);
    pub const RWin: Self = Self(0x5C);
    pub const Apps: Self = Self(0x5D);
    pub const Sleep: Self = Self(0x5F);
    pub const Numpad0: Self = Self(0x60);
    pub const Numpad1: Self = Self(0x61);
    pub const Numpad2: Self = Self(0x62);
    pub const Numpad3: Self = Self(0x63);
    pub const Numpad4: Self = Self(0x64);
    pub const Numpad5: Self = Self(0x65);
    pub const Numpad6: Self = Self(0x66);
    pub const Numpad7: Self = Self(0x67);
    pub const Numpad8: Self = Self(0x68);
    pub const Numpad9: Self = Self(0x69);
    pub const Multiply: Self = Self(0x6A);
    pub const Add: Self = Self(0x6B);
    pub const Separator: Self = Self(0x6C);
    pub const Subtract: Self = Self(0x6D);
    pub const Decimal: Self = Self(0x6E);
    pub const Divide: Self = Self(0x6F);
    pub const F1: Self = Self(0x70);
    pub const F2: Self = Self(0x71);
    pub const F3: Self = Self(0x72);
    pub const F4: Self = Self(0x73);
    pub const F5: Self = Self(0x74);
    pub const F6: Self = Self(0x75);
    pub const F7: Self = Self(0x76);
    pub const F8: Self = Self(0x77);
    pub const F9: Self = Self(0x78);
    pub const F10: Self = Self(0x79);
    pub const F11: Self = Self(0x7A);
    pub const F12: Self = Self(0x7B);
    pub const F13: Self = Self(0x7C);
    pub const F14: Self = Self(0x7D);
    pub const F15: Self = Self(0x7E);
    pub const F16: Self = Self(0x7F);
    pub const F17: Self = Self(0x80);
    pub const F18: Self = Self(0x81);
    pub const F19: Self = Self(0x82);
    pub const F20: Self = Self(0x83);
    pub const F21: Self = Self(0x84);
    pub const F22: Self = Self(0x85);
    pub const F23: Self = Self(0x86);
    pub const F24: Self = Self(0x87);
    pub const NavigationView: Self = Self(0x88);
    pub const NavigationMenu: Self = Self(0x89);
    pub const NavigationUp: Self = Self(0x8A);
    pub const NavigationDown: Self = Self(0x8B);
    pub const NavigationLeft: Self = Self(0x8C);
    pub const NavigationRight: Self = Self(0x8D);
    pub const NavigationAccept: Self = Self(0x8E);
    pub const NavigationCancel: Self = Self(0x8F);
    pub const Numlock: Self = Self(0x90);
    pub const Scroll: Self = Self(0x91);
    pub const OemNecEqual: Self = Self(0x92);
    pub const OemFjMasshou: Self = Self(0x93);
    pub const OemFjTouroku: Self = Self(0x94);
    pub const OemFjLoya: Self = Self(0x95);
    pub const OemFjRoya: Self = Self(0x96);
    pub const LShift: Self = Self(0xA0);
    pub const RShift: Self = Self(0xA1);
    pub const LControl: Self = Self(0xA2);
    pub const RControl: Self = Self(0xA3);
    pub const LMenu: Self = Self(0xA4);
    pub const RMenu: Self = Self(0xA5);
    pub const BrowserBack: Self = Self(0xA6);
    pub const BrowserForward: Self = Self(0xA7);
    pub const BrowserRefresh: Self = Self(0xA8);
    pub const BrowserStop: Self = Self(0xA9);
    pub const BrowserSearch: Self = Self(0xAA);
    pub const BrowserFavorites: Self = Self(0xAB);
    pub const BrowserHome: Self = Self(0xAC);
    pub const VolumeMute: Self = Self(0xAD);
    pub const VolumeDown: Self = Self(0xAE);
    pub const VolumeUp: Self = Self(0xAF);
    pub const MediaNextTrack: Self = Self(0xB0);
    pub const MediaPrevTrack: Self = Self(0xB1);
    pub const MediaStop: Self = Self(0xB2);
    pub const MediaPlayPause: Self = Self(0xB3);
    pub const LaunchMail: Self = Self(0xB4);
    pub const LaunchMediaSelect: Self = Self(0xB5);
    pub const LaunchApp1: Self = Self(0xB6);
    pub const LaunchApp2: Self = Self(0xB7);
    pub const Oem1: Self = Self(0xBA);
    pub const OemPlus: Self = Self(0xBB);
    pub const OemComma: Self = Self(0xBC);
    pub const OemMinus: Self = Self(0xBD);
    pub const OemPeriod: Self = Self(0xBE);
    pub const Oem2: Self = Self(0xBF);
    pub const Oem3: Self = Self(0xC0);
    pub const GamepadA: Self = Self(0xC3);
    pub const GamepadB: Self = Self(0xC4);
    pub const GamepadX: Self = Self(0xC5);
    pub const GamepadY: Self = Self(0xC6);
    pub const GamepadRightShoulder: Self = Self(0xC7);
    pub const GamepadLeftShoulder: Self = Self(0xC8);
    pub const GamepadLeftTrigger: Self = Self(0xC9);
    pub const GamepadRightTrigger: Self = Self(0xCA);
    pub const GamepadDpadUp: Self = Self(0xCB);
    pub const GamepadDpadDown: Self = Self(0xCC);
    pub const GamepadDpadLeft: Self = Self(0xCD);
    pub const GamepadDpadRight: Self = Self(0xCE);
    pub const GamepadMenu: Self = Self(0xCF);
    pub const GamepadView: Self = Self(0xD0);
    pub const GamepadLeftThumbstickButton: Self = Self(0xD1);
    pub const GamepadRightThumbstickButton: Self = Self(0xD2);
    pub const GamepadLeftThumbstickUp: Self = Self(0xD3);
    pub const GamepadLeftThumbstickDown: Self = Self(0xD4);
    pub const GamepadLeftThumbstickRight: Self = Self(0xD5);
    pub const GamepadLeftThumbstickLeft: Self = Self(0xD6);
    pub const GamepadRightThumbstickUp: Self = Self(0xD7);
    pub const GamepadRightThumbstickDown: Self = Self(0xD8);
    pub const GamepadRightThumbstickRight: Self = Self(0xD9);
    pub const GamepadRightThumbstickLeft: Self = Self(0xDA);
    pub const Oem4: Self = Self(0xDB);
    pub const Oem5: Self = Self(0xDC);
    pub const Oem6: Self = Self(0xDD);
    pub const Oem7: Self = Self(0xDE);
    pub const Oem8: Self = Self(0xDF);
    pub const OemAx: Self = Self(0xE1);
    pub const Oem102: Self = Self(0xE2);
    pub const IcoHelp: Self = Self(0xE3);
    pub const Ico00: Self = Self(0xE4);
    pub const Processkey: Self = Self(0xE5);
    pub const IcoClear: Self = Self(0xE6);
    pub const Packet: Self = Self(0xE7);
    pub const OemReset: Self = Self(0xE9);
    pub const OemJump: Self = Self(0xEA);
    pub const OemPa1: Self = Self(0xEB);
    pub const OemPa2: Self = Self(0xEC);
    pub const OemPa3: Self = Self(0xED);
    pub const OemWsctrl: Self = Self(0xEE);
    pub const OemCusel: Self = Self(0xEF);
    pub const OemAttn: Self = Self(0xF0);
    pub const OemFinish: Self = Self(0xF1);
    pub const OemCopy: Self = Self(0xF2);
    pub const OemAuto: Self = Self(0xF3);
    pub const OemEnlw: Self = Self(0xF4);
    pub const OemBacktab: Self = Self(0xF5);
    pub const Attn: Self = Self(0xF6);
    pub const Crsel: Self = Self(0xF7);
    pub const Exsel: Self = Self(0xF8);
    pub const Ereof: Self = Self(0xF9);
    pub const Play: Self = Self(0xFA);
    pub const Zoom: Self = Self(0xFB);
    pub const Noname: Self = Self(0xFC);
    pub const Pa1: Self = Self(0xFD);
    pub const OemClear: Self = Self(0xFE);
    pub const Hangeul: Self = Self::Kana;
    pub const Hangul: Self = Self::Kana;
    pub const Kanji: Self = Self::Hanja;
    pub const OemFjJish: Self = Self::OemNecEqual;
}

pub trait KeyHandlerTrait: Sized {
    // TODO draw the rest owl
    fn into_raw(self) -> *mut _cef_keyboard_handler_t {
        let mut object: cef_keyboard_handler_t = unsafe { std::mem::zeroed() };
        object.on_key_event = Some(keyboard_handler);
        RcImpl::new(object, self) as *mut _
    }
}

pub extern "C" fn get_keyboard_handler(_client: *mut cef_client_t) -> *mut cef_keyboard_handler_t {
    let handler = KeyBoardHandler {};
    handler.into_raw()
}

pub extern "C" fn keyboard_handler(
    kb: *mut _cef_keyboard_handler_t,
    br: *mut _cef_browser_t,
    evt: *const _cef_key_event_t,
    xevt: *mut _XEvent,
) -> i32 {
    unsafe {
        //println!("call_keyboard_handler {}",(*evt).native_key_code);
        if (*evt).windows_key_code == WindowsKeyCode::F12.0 {
            //launch dev tools
            match br.as_ref() {
                Some(brs) => match brs.get_host {
                    Some(cef_sys_fn) => {
                        let host = cef_sys_fn(br);
                        match host.as_ref() {
                            Some(href) => match href.show_dev_tools {
                                Some(cef_sys_fn) => {
                                    cef_sys_fn(
                                        host,
                                        null_mut(),
                                        null_mut(),
                                        null_mut(),
                                        null_mut(),
                                    );
                                }
                                None => {}
                            },
                            None => {}
                        }
                    }
                    None => {}
                },
                None => {}
            }
        }
    }
    1
}
