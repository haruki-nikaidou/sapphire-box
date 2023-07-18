use base64::Engine;
use base64::engine::general_purpose;

pub fn encode_utf8(input: Vec<u8>) -> String {
    String::from_utf8(input).unwrap()
}
pub fn decode_utf8(input: String) -> Vec<u8> {
    input.as_bytes().to_vec()
}

pub fn encode_base64(input: Vec<u8>) -> String {
    general_purpose::STANDARD_NO_PAD.encode(&input)
}
pub fn decode_base64(input: String) -> Vec<u8> {
    general_purpose::STANDARD_NO_PAD.decode(input.as_bytes()).unwrap()
}