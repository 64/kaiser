use super::{Decrypt, Encrypt, PartialDecrypt, PartialEncrypt};
use crate::{Buffer, PartialBuffer};
use simple_error::SimpleError;
use smallvec::SmallVec;

pub struct Vigenere {
    key: SmallVec<[u8; 32]>,
}

impl Vigenere {
    pub fn new<T: AsRef<str>>(key: T) -> Self {
        let sv = key
            .as_ref()
            .chars()
            .filter(|c| c.is_ascii())
            .map(|c| (c.to_ascii_uppercase() as u8) - b'A')
            .collect::<SmallVec<[u8; 32]>>();

        Self { key: sv }
    }

    pub unsafe fn new_unchecked(key: &[u8]) -> Self {
        Self {
            key: SmallVec::from(key),
        }
    }
}

impl PartialEncrypt for Vigenere {
    fn encrypt_partial(&mut self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        let keylen = self.key.len();

        for (i, b) in (&mut buf).into_iter().enumerate() {
            *b += self.key[i % keylen];
        }

        Ok(buf)
    }
}

impl PartialDecrypt for Vigenere {
    fn decrypt_partial(&mut self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        let keylen = self.key.len();

        for (i, b) in (&mut buf).into_iter().enumerate() {
            *b -= self.key[i % keylen];
        }

        Ok(buf)
    }
}

derive_encrypt_decrypt!(Vigenere, SimpleError);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let mut vigenere = Vigenere::new("KEY");
        let buf = Buffer::from("Hello world!");

        let buf = vigenere.encrypt(buf).unwrap();
        assert_eq!("Rijvs uyvjn!", buf.to_string());

        let buf = vigenere.decrypt(buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }
}
