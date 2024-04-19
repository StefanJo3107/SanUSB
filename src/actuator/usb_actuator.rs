use crate::reports::HidReport;

pub struct UsbHidActuator {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub flip_mouse_wheel: bool,
    pub v_scroll_scale: f32,
    pub h_scroll_scale: f32,

    hid_report: HidReport,
}
