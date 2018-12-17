use smallvec::SmallVec;
use simple_error::SimpleError;
use crate::Buffer;
use super::{Encrypt, Decrypt};

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

        Self {
            key: sv,
        }
    }

    pub unsafe fn new_unchecked(key: &[u8]) -> Self {
        Self {
            key: SmallVec::from(key),
        }
    }
}

impl Encrypt for Vigenere {
    type Error = SimpleError;

    fn encrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error> {
        let keylen = self.key.len();

        for (i, b) in buf.into_iter().enumerate() {
            *b += self.key[i % keylen];
        }

        Ok(())
    }
}

impl Decrypt for Vigenere {
    type Error = SimpleError;

    fn decrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error> {
        let keylen = self.key.len();

        for (i, b) in buf.into_iter().enumerate() {
            *b -= self.key[i % keylen];
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let vigenere = Vigenere::new("KEY");
        let mut buf = Buffer::from("Hello world!");

        vigenere.encrypt(&mut buf).unwrap();
        assert_eq!("Rijvs uyvjn!", buf.to_string());

        vigenere.decrypt(&mut buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }
}
