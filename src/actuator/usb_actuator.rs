use log::{debug, info, warn};
use crate::actuator::actuator::Actuator;
use crate::keycodes::KeyCode;
use crate::reports::{HidReport, HidReportType};

pub struct UsbHidActuator {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,

    hid_report: HidReport,
}

impl UsbHidActuator {
    pub fn new(width: u16, height: u16) -> UsbHidActuator {
        UsbHidActuator{
            width,
            height,
            x: 0,
            y: 0,
            hid_report: HidReport::new()
        }
    }

    fn clear(&mut self) {
        self.hid_report.clear();
        info!("Cleared HID report!");
    }
}

impl Actuator for UsbHidActuator {
    fn connected(&mut self) {
        self.hid_report.init();
        info!("Initialized HID report!");
    }

    fn disconnected(&mut self) {
        info!("Disconnected!");
    }

    fn get_screen_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_cursor_position(&self) -> (u16, u16) {
        self.hid_report.get_mouse_position()
    }

    fn set_cursor_position(&mut self, x: u16, y: u16) {
        debug!("Set cursor position to {x} {y}");
        self.hid_report.send(HidReportType::MouseMove { x, y });
    }

    fn move_cursor(&mut self, x: i16, y: i16) {
        debug!("Move cursor by {x} {y}");
        self.hid_report
            .send(HidReportType::MouseMoveRelative { x, y });
    }

    fn mouse_down(&mut self, button: u8) {
        debug!("Mouse down {button}");
        self.hid_report.send(HidReportType::MouseDown {
            button,
        });
    }

    fn mouse_up(&mut self, button: u8) {
        debug!("Mouse up {button}");
        self.hid_report.send(HidReportType::MouseUp {
            button
        });
    }

    fn mouse_wheel(&mut self, x: i16, y: i16) {
        todo!()
    }

    fn key_down(&mut self, key: KeyCode) {
        if matches!(key, KeyCode::None) {
            warn!("Keycode not found");
            return;
        }
        self.hid_report
            .send(HidReportType::KeyPress { key_code: key });
    }

    fn key_repeat(&mut self, key: KeyCode) {
        todo!()
    }

    fn key_up(&mut self, key: KeyCode) {
        if matches!(key, KeyCode::None) {
            warn!("Keycode not found");
            return;
        }
        self.hid_report
            .send(HidReportType::KeyRelease { key_code: key });
    }

    fn set_clipboard(&mut self, data: Vec<u8>) {
        todo!()
    }

    fn set_options(&mut self, heartbeat: u32) {
        todo!()
    }

    fn reset_options(&mut self) {
        todo!()
    }

    fn enter(&mut self) {
        self.clear();
    }

    fn leave(&mut self) {
        self.clear();
    }

    fn hid_key_down(&mut self, key: u8) {
        self.hid_report.send(HidReportType::KeyPress {
            key_code: KeyCode::Key(key),
        });
    }

    fn hid_key_up(&mut self, key: u8) {
        self.hid_report.send(HidReportType::KeyRelease {
            key_code: KeyCode::Key(key),
        });
    }
}