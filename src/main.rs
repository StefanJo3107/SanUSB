use crate::actuator::usb_actuator::UsbHidActuator;
use crate::actuator::actuator::Actuator;
use esp_idf_sys::
use san_vm::runner;
mod keycodes;
mod reports;
mod actuator;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let bytecode = include_bytes!("/home/stefan/Dev/SanScript/Payloads/test1.sanb");
    runner::deserialize_bytecode(bytecode);
}
