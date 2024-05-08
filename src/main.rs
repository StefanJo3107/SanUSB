use san_vm::runner;
use sanscript_common::hid_actuator::HidActuator;
use sanusb::actuator::esp_actuator::EspActuator;
use sanusb::irc_client::irc_client::IRClient;
use sanusb::irc_client::wifi_handler::WiFiHandler;

const SSID: &str = "quarks11";
const PASS: &str = "1104maja";
const SERVER_ADDR: &str = "192.168.138.131";
const SERVER_PORT: usize = 6667;
const SERVER_NAME: &str = "badusb.test.org";
const CHANNEL_NAME: &str = "#badusb";
const USERNAME: &str = "sanusb";

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    //
    // let bytecode = include_bytes!("/home/stefan/Dev/SanScript/Payloads/test3.sanb");
    // let mut actuator = EspActuator::new();
    // actuator.init_actuator();
    // actuator.sleep(3000);
    // runner::deserialize_bytecode(actuator, bytecode);
    let mut wifi = WiFiHandler::new(String::from(SSID), String::from(PASS)).expect("Error creating wifi handler");
    wifi.connect_wifi().expect("Error while connecting to wifi");
    let mut irc = IRClient::new(String::from(SERVER_ADDR), SERVER_PORT, String::from(SERVER_NAME), String::from(CHANNEL_NAME), String::from(USERNAME)).expect("Error creating irc client");
    irc.client_loop();
}
