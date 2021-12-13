use rand::prelude::*;
use sha3::{Digest, Sha3_512};

pub fn hash(value: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(value);
    let hash = hasher.finalize();
    hex::encode(hash)
}

pub fn generate_token() -> String {
    let random_bytes: Vec<u8> = (0..64).map(|_| random::<u8>()).collect();
    hash(random_bytes)
}
