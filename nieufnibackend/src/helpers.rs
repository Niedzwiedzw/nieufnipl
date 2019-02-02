use blake2::{ Digest, Blake2b };
use crate::config::HASH_SALT;

pub fn to_hash(input: String) -> String {
    let mut hasher = Blake2b::new();
    hasher.input(HASH_SALT);
    hasher.input(input.as_str());

    format!("{:x}", hasher.result())
}
