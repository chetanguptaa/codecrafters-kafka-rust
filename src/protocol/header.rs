#[derive(Debug)]
pub struct ResponseHeaderV0 {
    pub correlation_id: i32,
}

#[derive(Debug)]
pub enum ResponseHeader {
    V0(ResponseHeaderV0),
}
