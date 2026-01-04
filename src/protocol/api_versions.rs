#[derive(Debug)]
pub struct ApiKeyVersion {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}
