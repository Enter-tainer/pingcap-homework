use std::hash::Hasher;

use ahash::AHasher;

pub fn hash(buf: &[u8]) -> u64 {
    let mut hasher = AHasher::default();
    hasher.write(buf);
    hasher.finish()
}
