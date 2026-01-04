#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;
use std::num::NonZeroU32;

enum ResponseHeader {
    V0(ResponseHeaderV0),
}

struct ResponseBody {
    error_code: i16,
}

struct KafkaResponseFrame {
    message_size: i32,
    response_header: ResponseHeader,
    body: ResponseBody,
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
                bytes.extend_from_slice(&header.correlation_id.to_be_bytes());
            }
        }
        match &self.body {
            _ => {
                bytes.extend_from_slice(&self.body.error_code.to_be_bytes());
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
                let request_api_version = i16::from_be_bytes([payload[2], payload[3]]);
                let correlation_id =
                    i32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
                let mut error_code: i16 = 0;
                if request_api_version < 0 || request_api_version > 4 {
                    error_code = 35;
                }
                let response = KafkaResponseFrame {
                    message_size: 0,
                    response_header: ResponseHeader::V0(ResponseHeaderV0 { correlation_id }),
                    body: { ResponseBody { error_code } },
                };
                stream.write_all(&response.to_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
