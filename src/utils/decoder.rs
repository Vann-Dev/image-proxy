use dotenv::dotenv;
use hex;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::env;

pub fn decode(value: String) -> String {
    dotenv().ok();

    let hex = hex::decode(value).unwrap();

    let mc = new_magic_crypt!(env::var("key").unwrap(), 256, env::var("iv").unwrap());
    let result = mc
        .decrypt_base64_to_string(String::from_utf8(hex.clone()).unwrap())
        .expect("Invalid base64");

    result
}

pub fn validate(value: String) -> bool {
    dotenv().ok();

    let hex = match hex::decode(value) {
        Ok(val) => val,
        Err(_) => return false,
    };

    let mc = new_magic_crypt!(env::var("key").unwrap(), 256, env::var("iv").unwrap());

    let result = match String::from_utf8(hex.clone()) {
        Ok(val) => val,
        Err(_) => return false,
    };

    match mc.decrypt_base64_to_string(result) {
        Ok(_) => true,
        Err(_) => false,
    }
}
