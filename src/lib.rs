#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod double_array;
pub mod staged_hash;

fn decode_utf16(chars: &[u16]) -> String {
    use std::char;

    char::decode_utf16(chars.iter().cloned())
        .filter_map(Result::ok)
        .collect()
}
