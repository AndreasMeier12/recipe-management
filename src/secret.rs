use std::env;

use dotenvy::dotenv;
use hex::FromHex;
use itertools::Itertools;
use log::info;
use rand::Rng;

pub fn get_secret() -> [u8; 128] {
    dotenv().ok();
    let maybe_secret = env::var("SECRET").ok();
    if maybe_secret.is_none() {
        info!("No SECRET found in environment variable, generating one");
        return rand::thread_rng().gen::<[u8; 128]>();
    }

    let raw = env::var("SECRET").ok()
        .filter(|x| x.is_ascii())
        .filter(|x| x.len() == 256);

    if raw.is_some() {
        let asdf = <[u8; 128]>::from_hex(raw.unwrap()).ok();
        if asdf.is_some() {
            info!("Setting your SECRET variable");
            return asdf.unwrap();
        }
    }
    info!("SECRET variable is a 128-byte hex-encoded secret");

    return rand::thread_rng().gen::<[u8; 128]>();
}