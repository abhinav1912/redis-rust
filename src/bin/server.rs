use std::{net::{SocketAddrV4}, io::{Error}, str::FromStr};

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};

const MESSAGE_LENGTH: usize = 64;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub async fn new(addr: SocketAddrV4) -> Result<Server, Error> {
        let listener = Server::get_listener_for_address(addr).await?;
        Ok(Server { listener })
    }

    pub async fn start_listening(&self) {
        println!("Listening for connections on port {:?}", self.listener.local_addr().unwrap().port());
        loop {
            match self.listener.accept().await {
                Ok((stream, _)) => self.handle_incoming_stream(stream).await,
                Err(err) => println!("{:?}", err),
            }
        }
    }

    async fn handle_incoming_stream(&self, mut stream: TcpStream) {
        let mut read_bytes = [0u8; MESSAGE_LENGTH];
        stream.read(&mut read_bytes).await;

        let received = std::str::from_utf8(&read_bytes).expect("invalid utf8");
        println!("Client says: {}", received);

        let response_data = b"Hello from the server";
        stream.write_all(response_data).await;
        stream.flush().await;
    }

    async fn get_listener_for_address(addr: SocketAddrV4) -> Result<TcpListener, Error> {
        TcpListener::bind(addr).await
    }
}

#[tokio::main]
async fn main() {
    let server_address = SocketAddrV4::from_str("127.0.0.1:8000").expect("Unable to create server address");
    let server = Server::new(server_address).await.expect("Unable to create server!");
    server.start_listening().await;
}