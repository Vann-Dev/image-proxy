use dotenv::dotenv;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::env;

pub fn encrypt(value: String) -> String {
    dotenv().ok();

    let mc = new_magic_crypt!(env::var("key").unwrap(), 256, env::var("iv").unwrap());
    let base = mc.encrypt_str_to_base64(value);

    let result = hex::encode(base);

    result
}
