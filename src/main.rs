#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::num::NonZeroU32;

enum ResponseHeader {
    V0(ResponseHeaderV0),
}

#[derive(Serialize, Deserialize)]
struct ResponseAPIKey {
    api_key: i16,
    min_version: i16,
    max_version: i16,
    #[serde(rename = "TAG_BUFFER")]
    tag_buffer: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    error_code: i16,
    api_keys: Vec<ResponseAPIKey>,
    // throttle_time_ms: i32,
    // #[serde(rename = "TAG_BUFFER")]
    // tag_buffer: Option<String>,
}

struct KafkaResponseFrame {
    response_header: ResponseHeader,
    body: ResponseBody,
}

struct ResponseHeaderV0 {
    correlation_id: i32,
}

impl KafkaResponseFrame {
    fn to_bytes(&self) -> Vec<u8> {
        let body = self.body_and_header_bytes();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(body.len() as i32).to_be_bytes());
        bytes.extend_from_slice(&body);
        bytes
    }
    fn body_and_header_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        if let ResponseHeader::V0(h) = &self.response_header {
            bytes.extend_from_slice(&h.correlation_id.to_be_bytes());
        }
        bytes.extend_from_slice(&self.body.error_code.to_be_bytes());
        bytes.extend_from_slice(&(self.body.api_keys.len() as i32).to_be_bytes());
        for key in &self.body.api_keys {
            bytes.extend_from_slice(&key.api_key.to_be_bytes());
            bytes.extend_from_slice(&key.min_version.to_be_bytes());
            bytes.extend_from_slice(&key.max_version.to_be_bytes());
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
                let api_key = ResponseAPIKey {
                    api_key: 18,
                    min_version: 0,
                    max_version: 4,
                    tag_buffer: None,
                };
                let mut api_keys = Vec::new();
                api_keys.push(api_key);
                let response = KafkaResponseFrame {
                    response_header: ResponseHeader::V0(ResponseHeaderV0 { correlation_id }),
                    body: {
                        ResponseBody {
                            error_code,
                            api_keys,
                        }
                    },
                };
                stream.write_all(&response.to_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
