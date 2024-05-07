use std::io;
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
        info!("Trying to connect to {}:{}", self.address, self.port);
        let stream = TcpStream::connect(format!("{}:{}", self.address, self.port))?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn send_command(&mut self, command: String) -> anyhow::Result<()> {
        let mut stream = self.stream.as_mut().expect("TCP stream is not initialized");
        stream.write_all(command.as_bytes())?;
        info!("Sent command: {}", command);
        Ok(())
    }

    pub fn receive_motd(&mut self) -> anyhow::Result<String> {
        let mut result: [u8;5000] = [0;5000];
        let mut stream = self.stream.as_mut().expect("TCP stream is not initialized");
        stream.read(&mut result)?;
        info!("Received response: {}", std::str::from_utf8(&result)?);
        Ok(std::str::from_utf8(&result)?.parse()?)
    }

    pub fn receive_response(&mut self) -> anyhow::Result<String> {
        let mut result: [u8;400] = [0;400];
        let mut stream = self.stream.as_mut().expect("TCP stream is not initialized");
        stream.read(&mut result)?;
        info!("Received response: {}", std::str::from_utf8(&result)?);
        Ok(std::str::from_utf8(&result)?.parse()?)
    }

    pub fn close_stream(&mut self) -> anyhow::Result<()> {
        let mut stream = self.stream.as_mut().expect("TCP stream is not initialized");
        stream.shutdown(Shutdown::Both)?;
        Ok(())
    }
}