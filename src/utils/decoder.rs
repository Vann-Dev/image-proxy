use dotenv::dotenv;
use hex;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::env;

pub fn decode(value: String) -> String {
    dotenv().ok();

    // decode hex to base64 first
    let hex = hex::decode(value).unwrap();

    // base64 decode to get real value
    let mc = new_magic_crypt!(env::var("key").unwrap(), 256);
    let result = mc
        .decrypt_base64_to_string(String::from_utf8(hex.clone()).unwrap())
        .expect("Invalid base64");

    result
}
