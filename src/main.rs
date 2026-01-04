#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;

enum ResponseHeader {
    V0(ResponseHeaderV0),
}

struct KafkaResponseFrame {
    message_size: i32,
    response_header: ResponseHeader,
}

struct ResponseHeaderV0 {
    correlation_id: i32,
}

impl KafkaResponseFrame {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend_from_slice(&self.message_size.to_be_bytes());
        match &self.response_header {
            ResponseHeader::V0(header) => {
                // correlation_id (INT32)
                bytes.extend_from_slice(&header.correlation_id.to_be_bytes());
            }
        }
        bytes
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut size_buf = [0u8; 4];
                stream.read_exact(&mut size_buf).unwrap();
                let message_size = i32::from_be_bytes(size_buf);
                let mut payload = vec![0u8; message_size as usize];
                stream.read_exact(&mut payload).unwrap();
                let correlation_id =
                    i32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
                let response = KafkaResponseFrame {
                    message_size: 0,
                    response_header: ResponseHeader::V0(ResponseHeaderV0 { correlation_id }),
                };
                stream.write_all(&response.to_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
