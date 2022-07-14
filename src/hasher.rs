use crate::dtos::Algorithm;

use digest::{Digest, DynDigest};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

pub struct Hasher {
    hash: Box<dyn DynDigest + Send + Sync>,
}

impl Hasher {
    pub fn new(algorithm: Algorithm) -> Hasher {
        Hasher {
            hash: match algorithm {
                Algorithm::Sha1 => Box::new(Sha1::new()),
                Algorithm::Sha256 => Box::new(Sha256::new()),
                Algorithm::Sha512 => Box::new(Sha512::new()),
            },
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.hash.update(data);
    }

    pub fn finish(self) -> String {
        hex::encode(self.hash.finalize())
    }
}
