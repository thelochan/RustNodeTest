use super::*;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>; //&self - Current instance of struct

    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
