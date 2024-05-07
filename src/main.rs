use san_vm::runner;
use sanscript_common::hid_actuator::HidActuator;
use crate::actuator::esp_actuator::EspActuator;

mod actuator;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let bytecode = include_bytes!("/home/stefan/Dev/SanScript/Payloads/test3.sanb");
    let mut actuator = EspActuator::new();
    actuator.init_actuator();
    actuator.sleep(3000);
    runner::deserialize_bytecode(actuator, bytecode);
}
