#![allow(unused_imports)]
use std::io::Write;
use std::net::TcpListener;

struct HeaderV0 {
    correlation_id: u32,
}

struct Message {
    message_size: u32,
    header: HeaderV0,
}

impl Message {
    fn new(message_size: u32, correlation_id: u32) -> Message {
        Message {
            message_size,
            header: HeaderV0 { correlation_id },
        }
    }
}

fn main() {
    let message = Message::new(2, 7);
    let message_id = message
        .message_size
        .to_be_bytes()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");
    let correlation_id = message
        .header
        .correlation_id
        .to_be_bytes()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                use std::io::Write;
                let response = format!("{message_id}\n{correlation_id}\n");
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
