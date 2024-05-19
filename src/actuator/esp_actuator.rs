use std::ffi::{c_char, CString};
use std::ptr;
use esp_idf_hal::delay::FreeRtos;
use san_common::hid_actuator::HidActuator;
use esp_idf_sys::esptinyusb::{tinyusb_config_t, tinyusb_config_t__bindgen_ty_1, tinyusb_config_t__bindgen_ty_2, tinyusb_config_t__bindgen_ty_2__bindgen_ty_1, tinyusb_driver_install, tud_hid_n_keyboard_report, tud_hid_n_mouse_report};
use esp_idf_sys::tinyusb::{hid_report_type_t, tud_mounted};
use san_common::keycodes::{HID_KEY_SHIFT_LEFT, HID_KEY_SHIFT_RIGHT};

const CONFIG_DESC: [u8; 34] = [9, 2, 34, 0, 1, 1, 0, 160, 50, 9, 4, 0, 0, 1, 3, 0, 0, 4, 9, 33, 17, 1, 0, 1, 34, 146, 0, 7, 5, 129, 3, 16, 0, 10];
const REPORT_DESC: [u8; 146] = [5, 1, 9, 6, 161, 1, 133, 1, 5, 7, 25, 224, 41, 231, 21, 0, 37, 1, 149, 8, 117, 1, 129, 2, 149, 1, 117, 8, 129, 1, 5, 8, 25, 1, 41, 5, 149, 5, 117, 1, 145, 2, 149, 1, 117, 3, 145, 1, 5, 7, 25, 0, 42, 255, 0, 21, 0, 38, 255, 0, 149, 6, 117, 8, 129, 0, 192, 5, 1, 9, 2, 161, 1, 133, 2, 9, 1, 161, 0, 5, 9, 25, 1, 41, 5, 21, 0, 37, 1, 149, 5, 117, 1, 129, 2, 149, 1, 117, 3, 129, 1, 5, 1, 9, 48, 9, 49, 21, 129, 37, 127, 149, 2, 117, 8, 129, 6, 9, 56, 21, 129, 37, 127, 149, 1, 117, 8, 129, 6, 5, 12, 10, 56, 2, 21, 129, 37, 127, 149, 1, 117, 8, 129, 6, 192, 192];


#[no_mangle]
extern "C" fn tud_hid_descriptor_report_cb(instance: u8) -> *const u8 {
    return REPORT_DESC.as_ptr();
}

#[no_mangle]
extern "C" fn tud_hid_get_report_cb(instance: u8, report_id: u8, report_type: hid_report_type_t, buffer: *const u8, reqlen: u16) -> u16 {
    return 0;
}

#[no_mangle]
extern "C" fn tud_hid_set_report_cb(instance: u8, report_id: u8, report_type: hid_report_type_t, buffer: *const u8, buffsize: u16) {}

fn get_hid_string_descriptor() -> (*mut *const c_char, usize) {
    // Create a vector of CString objects
    let strings = vec![
        CString::new(String::from_utf8(vec![0x09, 0x04]).unwrap()).unwrap(),
        CString::new("TinyUSB").unwrap(),
        CString::new("TinyUSB Device").unwrap(),
        CString::new("123456").unwrap(),
        CString::new("Example HID interface").unwrap(),
    ];

    // Create a vector of raw pointers to the C strings
    let mut raw_pointers: Vec<*const c_char> = strings.iter()
        .map(|c_str| c_str.as_ptr())
        .collect();

    // Get a raw pointer to the beginning of the raw pointers array
    let raw_ptr = raw_pointers.as_mut_ptr();

    // Return the raw pointer and its length to be used in C or other operations
    (raw_ptr, raw_pointers.len())
}

pub struct EspActuator {
    mouse_button_pressed: u8
}

impl EspActuator {
    pub fn new() -> EspActuator {
        EspActuator {
            mouse_button_pressed: 0
        }
    }

    pub fn init_actuator(&self) {
        // let (string_hid_desc, string_hid_desc_len) = get_hid_string_descriptor();

        log::info!("USB initialization!");
        let tusb_cfg = tinyusb_config_t {
            string_descriptor: ptr::null_mut(),
            string_descriptor_count: 0,
            external_phy: false,
            __bindgen_anon_1: unsafe { tinyusb_config_t__bindgen_ty_1 { device_descriptor: ptr::null_mut() } },
            __bindgen_anon_2: unsafe { tinyusb_config_t__bindgen_ty_2 { __bindgen_anon_1: tinyusb_config_t__bindgen_ty_2__bindgen_ty_1 { configuration_descriptor: CONFIG_DESC.as_ptr() } } },
            self_powered: false,
            vbus_monitor_io: 0,
        };

        unsafe { tinyusb_driver_install(&tusb_cfg); }
    }
}

impl HidActuator for EspActuator {
    fn move_cursor(&mut self, x: i8, y: i8) {
        unsafe {
            tud_hid_n_mouse_report(0, 2, self.mouse_button_pressed, x, y, 0, 0);
        }
    }

    fn mouse_down(&mut self, button: u8) {
        unsafe {
            tud_hid_n_mouse_report(0, 2, button, 0, 0, 0, 0);
        }

        self.mouse_button_pressed = button;
    }

    fn mouse_up(&mut self) {
        unsafe {
            tud_hid_n_mouse_report(0, 2, 0, 0, 0, 0, 0);
        }

        self.mouse_button_pressed = 0;
    }

    fn scroll_mouse_wheel(&mut self, x: i8, y: i8) {
        unsafe {
            tud_hid_n_mouse_report(0, 2, self.mouse_button_pressed, 0, 0, x, y);
        }
    }

    fn key_down(&mut self, key: &Vec<u8>) {
        let mut keys_slice: [u8; 6] = [0; 6];
        if key.len() <= 6 {
            for i in 0..key.len() {
                keys_slice[i] = key[i];
            }
        }

        unsafe {
            if tud_mounted() {
                if keys_slice.contains(&HID_KEY_SHIFT_LEFT) {
                    tud_hid_n_keyboard_report(0, 1, 0, [HID_KEY_SHIFT_LEFT, 0, 0, 0, 0, 0].as_mut_ptr());
                    self.sleep(10);
                } else if keys_slice.contains(&HID_KEY_SHIFT_RIGHT) {
                    tud_hid_n_keyboard_report(0, 1, 0, [HID_KEY_SHIFT_RIGHT, 0, 0, 0, 0, 0].as_mut_ptr());
                    self.sleep(10);
                }
                tud_hid_n_keyboard_report(0, 1, 0, keys_slice.as_mut_ptr());
            }
        }
    }

    fn clear_keys(&mut self) {
        unsafe {
            if tud_mounted() {
                tud_hid_n_keyboard_report(0, 1, 0, [0, 0, 0, 0, 0, 0].as_mut_ptr());
            }
        }
    }

    fn sleep(&mut self, duration_ms: usize) {
        FreeRtos::delay_ms(duration_ms as u32);
    }
}