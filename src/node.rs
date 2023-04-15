use std::net::{TcpListener, SocketAddrV4};

pub trait Node {
    fn get_listener_for_address(addr: SocketAddrV4) -> TcpListener {
        let listener = TcpListener::bind(addr).unwrap();
        listener
    }

    fn get_listener() -> TcpListener {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener
    }
}