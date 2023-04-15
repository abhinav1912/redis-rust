use std::{net::{TcpListener, SocketAddrV4, TcpStream}, io::{Error, Read, Write}};

use crate::node::Node;

const MESSAGE_LENGTH: usize = 64;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new(&self, addr: SocketAddrV4) -> Result<&Self, Error> {
        let listener = <Server as Node>::get_listener_for_address(addr);
        match listener {
            Ok(listener) => Ok(&Server { listener }),
            Err(error) => Err(error)
        }
    }

    pub fn start_listening(&self) {
        for stream in self.listener.incoming() {
            match stream.ok() {
                Some(incoming_stream) => self.handle_incoming_stream(incoming_stream),
                None => println!("Unable to unwrap stream!")
            }
        }
    }

    fn handle_incoming_stream(&self, stream: TcpStream) {
        let mut read_bytes = [0u8; MESSAGE_LENGTH];
        stream.read(&mut read_bytes);

        let received = std::str::from_utf8(&read_bytes).expect("invalid utf8");
        println!("Client says: {}", received);

        let response_data = b"Hello from the server";
        stream.write(response_data);
        stream.flush();
    }
}

impl Node for Server { }