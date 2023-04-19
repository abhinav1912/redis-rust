use std::{net::{SocketAddrV4, TcpStream}, io::{Write, Read}, str::FromStr};

const MESSAGE_LENGTH: usize = 64;

pub struct Client {
    server_address: SocketAddrV4
}

impl Client {
    pub fn new(server_address: SocketAddrV4) -> Client {
        Client { server_address }
    }

    pub fn connect_to_server(&self) {
        let mut stream = TcpStream::connect(self.server_address).expect("Can't connect to server");
        let write_buf = b"Hello Server!";
        stream.write(write_buf);
        stream.flush();

        let mut read_buf = [0u8; MESSAGE_LENGTH];
        stream.read(&mut read_buf);

        let server_response = std::str::from_utf8(&read_buf).expect("Invalid UTF8 in server response");
        println!("Server said: {}", server_response);
    }
}

fn main() {
    let server_address = SocketAddrV4::from_str("127.0.0.1:8000").expect("Unable to create server address");
    let client_one = Client::new(server_address);
    client_one.connect_to_server();
    println!("Hello, world!");
}
