use crate::protocol::{frame::KafkaResponseFrame, header::ResponseHeader};
use bytes::{BufMut, BytesMut};

pub fn encode_response(frame: &KafkaResponseFrame) -> BytesMut {
    let mut payload = BytesMut::new();
    match &frame.header {
        ResponseHeader::V0(h) => {
            payload.put_i32(h.correlation_id);
        }
    }
    payload.put_i16(frame.body.error_code);
    payload.put_u8((frame.body.api_keys.len() + 1) as u8);
    for key in &frame.body.api_keys {
        payload.put_i16(key.api_key);
        payload.put_i16(key.min_version);
        payload.put_i16(key.max_version);
        payload.put_u8(0); // tag buffer
    }
    payload.put_i32(frame.body.throttle_time_ms);
    payload.put_u8(0); // tag buffer
    let mut framed = BytesMut::new();
    framed.put_i32(payload.len() as i32);
    framed.extend_from_slice(&payload);
    framed
}
