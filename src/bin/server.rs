use std::{net::{TcpListener, SocketAddrV4, TcpStream}, io::{Error, Read, Write}, thread, str::FromStr};

const MESSAGE_LENGTH: usize = 64;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new(addr: SocketAddrV4) -> Result<Server, Error> {
        let listener = Server::get_listener_for_address(addr);
        match listener {
            Ok(listener) => {
                listener.set_nonblocking(true)?;
                Ok(Server { listener })
            },
            Err(error) => Err(error)
        }
    }

    pub fn start_listening(&self) {
        println!("Listening for connections on port {:?}", self.listener.local_addr().unwrap().port());
        for stream in self.listener.incoming() {
            match stream.ok() {
                Some(incoming_stream) => self.handle_incoming_stream(incoming_stream),
                None => ()
            }
        }
    }

    fn handle_incoming_stream(&self, mut stream: TcpStream) {
        let mut read_bytes = [0u8; MESSAGE_LENGTH];
        stream.read(&mut read_bytes);

        let received = std::str::from_utf8(&read_bytes).expect("invalid utf8");
        println!("Client says: {}", received);

        let response_data = b"Hello from the server";
        stream.write(response_data);
        stream.flush();
    }

    fn get_listener_for_address(addr: SocketAddrV4) -> Result<TcpListener, Error> {
        TcpListener::bind(addr)
    }
}

fn main() {
    let server_address = SocketAddrV4::from_str("127.0.0.1:8000").expect("Unable to create server address");
    let server = Server::new(server_address).expect("Unable to create server.");
    server.start_listening();
}