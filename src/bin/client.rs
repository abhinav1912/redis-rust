use std::{net::{SocketAddrV4}, io::{Write, Read}, str::FromStr};

use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

const MESSAGE_LENGTH: usize = 64;

pub struct Client {
    server_address: SocketAddrV4
}

impl Client {
    pub fn new(server_address: SocketAddrV4) -> Client {
        Client { server_address }
    }

    pub async fn connect_to_server(&self) {
        let mut stream = TcpStream::connect(self.server_address).await.expect("Can't connect to server");
        let write_buf = b"Hello Server!";
        stream.write(write_buf).await;
        stream.flush().await;

        let mut read_buf = [0u8; MESSAGE_LENGTH];
        stream.read(&mut read_buf).await;

        let server_response = std::str::from_utf8(&read_buf).expect("Invalid UTF8 in server response");
        println!("Server said: {}", server_response);
    }
}

#[tokio::main]
async fn main() {
    let server_address = SocketAddrV4::from_str("127.0.0.1:8000").expect("Unable to create server address");
    let client_one = Client::new(server_address);
    client_one.connect_to_server().await;
}
