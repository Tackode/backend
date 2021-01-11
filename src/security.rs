use sha3::{Digest, Sha3_512};
use rand::prelude::*;

pub fn hash(value: String) -> String {
    let hash = Sha3_512::new().chain(value.into_bytes().as_slice()).finalize();
    hex::encode(hash)
}

pub fn generate_token() -> String {
    let random_bytes: Vec<u8> = (0..64).map(|_| random::<u8>()).collect();
    let hash = Sha3_512::new().chain(&random_bytes.as_slice()).finalize();
    hex::encode(hash)
}
