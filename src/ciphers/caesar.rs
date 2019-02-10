use super::{Decrypt, Encrypt, PartialEncrypt, PartialDecrypt};
use crate::{Buffer, PartialBuffer};
use simple_error::SimpleError;

pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }
}

impl PartialEncrypt for Caesar {
    fn encrypt_partial(&self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        for b in &mut buf {
            *b += self.shift;
        }

        Ok(buf)
    }
}

impl PartialDecrypt for Caesar {
    fn decrypt_partial(&self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        for b in &mut buf {
            *b -= self.shift;
        }

        Ok(buf)
    }
}

derive_encrypt_decrypt!(Caesar, SimpleError);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let caesar = Caesar::new(5);
        let buf = Buffer::from("Hello world!");

        let buf = caesar.encrypt(buf).unwrap();
        assert_eq!("Mjqqt btwqi!", buf.to_string());

        let buf = caesar.decrypt(buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }
}
