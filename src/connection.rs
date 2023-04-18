const MAX_MESSAGE_LENGTH: usize = 4096;

pub enum ConnectionState {
    ReadRequest,
    SendResponse,
    DeleteConnection
}

pub struct Connection {
    state: ConnectionState,
    rbuf_size: usize,
    rbuf: [u8; 4 + MAX_MESSAGE_LENGTH],
    wbuf_size: usize,
    wbuf_sent: usize,
    wbuf: [u8; 4 + MAX_MESSAGE_LENGTH]
}

impl Connection {
    pub fn new() -> Connection {
        Connection { state: ConnectionState::ReadRequest, rbuf_size: 0, rbuf: [0u8; 4 + MAX_MESSAGE_LENGTH], wbuf_size: 0, wbuf_sent: 0, wbuf: [0u8; 4 + MAX_MESSAGE_LENGTH] }
    }
}