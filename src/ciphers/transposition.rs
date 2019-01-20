use super::{Decrypt, Encrypt};
use crate::Buffer;
use crate::Char;
use itertools::Itertools;
use simple_error::SimpleError;
use smallvec::SmallVec;
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Transposition {
    key: SmallVec<[u8; 32]>,
}

impl Transposition {
    pub fn new<T: AsRef<str>>(key: T) -> Self {
        let st = key
            .as_ref()
            .split(',')
            .map(|c| <u8>::from_str(c).expect("Transposition key must be a sequence of integers"))
            .collect::<SmallVec<[u8; 32]>>();
        Self { key: st }
    }
}

impl Encrypt for Transposition {
    type Error = SimpleError;

    fn encrypt(&self, mut buf: Buffer) -> Result<Buffer, Self::Error> {
        let keylen = self.key.len();
        let mut ciphertext = String::from("");

        for chunk in &buf.into_iter().chunks(keylen) {
            let group = chunk.collect::<Vec<_>>();
            let new_key = self.key.clone();
            let mut cipher_group = group.iter().zip(new_key).collect::<Vec<_>>();
            cipher_group.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            let cipher_string = cipher_group.into_iter().map(|a| char::from(**a.0));
            ciphertext += &String::from_iter(cipher_string);
        }

        for (i, x) in (&mut buf).into_iter().enumerate() {
            *x = Char::from(ciphertext.as_bytes()[i] as char);
        }

        Ok(buf)
    }
}

impl Decrypt for Transposition {
    type Error = SimpleError;

    fn decrypt(&self, mut buf: Buffer) -> Result<Buffer, Self::Error> {
        let keylen = self.key.len();
        let mut plaintext = String::from("");

        for chunk in &buf.into_iter().chunks(keylen) {
            let group = chunk.collect::<Vec<_>>();
            let new_key = self.key.clone();
            let mut plain_group = group.iter().zip(new_key).collect::<Vec<_>>();
            plain_group.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            let plain_string = plain_group.into_iter().map(|a| char::from(**a.0));
            plaintext += &String::from_iter(plain_string);
        }

        for (i, x) in (&mut buf).into_iter().enumerate() {
            *x = Char::from(plaintext.as_bytes()[i] as char);
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let transposition = Transposition::new("2,1");
        let buf = Buffer::from("HELLOWORLD");

        let buf = transposition.encrypt(buf).unwrap();
        assert_eq!("EHLLWORODL", buf.to_string());

        let buf = transposition.decrypt(buf).unwrap();
        assert_eq!("HELLOWORLD", buf.to_string());
    }
}
