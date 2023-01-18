use std::env;

use dotenvy::dotenv;
use hex::FromHex;
use itertools::Itertools;
use rand::Rng;

pub fn get_secret() -> [u8; 128] {
    dotenv().ok();
    let raw = env::var("SECRET").ok()
        .filter(|x| x.is_ascii())
        .filter(|x| x.len() == 256);
    if raw.is_some() {
        let asdf = <[u8; 128]>::from_hex(raw.unwrap()).ok();
        if asdf.is_some() {
            return asdf.unwrap();
        }
    }


    return rand::thread_rng().gen::<[u8; 128]>();
}