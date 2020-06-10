use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rand::prelude::*;

pub fn hash(value: String) -> String {
    let mut hasher = Sha3::sha3_512();
    hasher.input(value.into_bytes().as_slice());
    hasher.result_str()
}

pub fn generate_token() -> String {
    let random_bytes: Vec<u8> = (0..64).map(|_| random::<u8>()).collect();
    let mut hasher = Sha3::sha3_512();
    hasher.input(&random_bytes.as_slice());
    hasher.result_str()
}
