use crate::protocol::{api_versions::ApiKeyVersion, header::ResponseHeader};

#[derive(Debug)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub api_keys: Vec<ApiKeyVersion>,
    pub throttle_time_ms: i32,
}

#[derive(Debug)]
pub struct KafkaResponseFrame {
    pub header: ResponseHeader,
    pub body: ApiVersionsResponse,
}
