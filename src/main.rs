#![allow(unused_imports)]
use std::io::Write;
use std::net::TcpListener;

struct Response {
    message_size: u32,
    header: ResponseHeader,
}

struct ResponseHeader {
    correlation_id: i32,
}

impl Response {
    fn to_bytes(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.message_size.to_be_bytes());
        v.extend_from_slice(&self.header.correlation_id.to_be_bytes());
        v
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let header = Response {
                    message_size: 0,
                    header: ResponseHeader { correlation_id: 7 },
                };
                let bytes = header.to_bytes();
                stream.write_all(&bytes).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
