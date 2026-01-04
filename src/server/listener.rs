use crate::codec::encoder::encode_response;
use crate::protocol::api_versions::ApiKeyVersion;
use crate::protocol::frame::{ApiVersionsResponse, KafkaResponseFrame};
use crate::protocol::header::{ResponseHeader, ResponseHeaderV0};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn run(addr: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    loop {
        let mut size_buf = [0u8; 4];
        if let Err(e) = stream.read_exact(&mut size_buf).await {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(e.into());
        }
        let size = i32::from_be_bytes(size_buf) as usize;
        if size < 8 {
            anyhow::bail!("Invalid request size");
        }
        let mut payload = vec![0u8; size];
        stream.read_exact(&mut payload).await?;
        let api_version = i16::from_be_bytes([payload[2], payload[3]]);
        let correlation_id = i32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);
        let error_code = if !(0..=4).contains(&api_version) {
            35
        } else {
            0
        };
        let response = KafkaResponseFrame {
            header: ResponseHeader::V0(ResponseHeaderV0 { correlation_id }),
            body: ApiVersionsResponse {
                error_code,
                api_keys: vec![
                    ApiKeyVersion {
                        api_key: 18,
                        min_version: 0,
                        max_version: 4,
                    },
                    ApiKeyVersion {
                        api_key: 75,
                        min_version: 0,
                        max_version: 0,
                    },
                ],
                throttle_time_ms: 0,
            },
        };
        let bytes = encode_response(&response);
        stream.write_all(&bytes).await?;
    }
    Ok(())
}
