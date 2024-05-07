use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use log::info;

pub struct TcpClient {
    address: String,
    port: usize,
    stream: Option<TcpStream>,
}

impl TcpClient {
    pub fn new(address: String, port: usize) -> TcpClient {
        TcpClient {
            address,
            port,
            stream: None,
        }
    }

    pub fn connect_to_server(&mut self) -> anyhow::Result<()> {
        let stream = TcpStream::connect(format!("{}:{}", self.address, self.port))?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn send_command(&mut self, command: &str) -> anyhow::Result<()> {
        self.stream?.write_all(command.as_bytes())?;
        info!("Sent command: {}", command);
        Ok(())
    }

    pub fn receive_response(&mut self) -> anyhow::Result<String> {
        let mut result = String::from("");
        self.stream?.read_to_string(&mut result)?;
        info!("Received response: {}", result);
        Ok(result)
    }

    pub fn close_stream(&mut self) -> anyhow::Result<()> {
        self.stream?.shutdown(Shutdown::Both)?;
        Ok(())
    }
}