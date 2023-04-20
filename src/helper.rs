use std::{vec};

const MAX_MESSAGE_LENGTH: usize = 4096;

fn buffer_to_array(buf: [u8; MAX_MESSAGE_LENGTH]) -> Vec<String> {
    let string = std::str::from_utf8(&buf).expect("invalid utf8");
    let mut arr: Vec<String> = vec![];
    for part in string.trim().split_whitespace() {
        arr.push(part.to_string())
    }
    arr
}