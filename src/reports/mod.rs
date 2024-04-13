extern "C" {
    fn usb_util_init();
    fn usb_util_keyboard_report(modifier: u8, keycode: *const u8);
    fn usb_util_abs_mouse_report(buttons: u8, x: u16, y: u16, wheel: i8, pan: i8);
    fn usb_util_consumer_report(code: u16);
}

use crate::keycodes::KeyCode;

pub enum HidReportType {
    KeyPress { key_code: KeyCode },
    KeyRelease { key_code: KeyCode },
    MouseMove { x: u16, y: u16 },
    MouseMoveRelative { x: i16, y: i16 },
    MouseDown { button: u8 },
    MouseUp { button: u8 },
    MouseWheel { scroll: i8, pan: i8 },
}

