use super::{Decrypt, Encrypt};
use crate::Buffer;
use simple_error::SimpleError;

pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }
}

impl Encrypt for Caesar {
    type Error = SimpleError;

    fn encrypt(&self, mut buf: Buffer) -> Result<Buffer, Self::Error> {
        for b in &mut buf {
            *b += self.shift;
        }

        Ok(buf)
    }
}

impl Decrypt for Caesar {
    type Error = SimpleError;

    fn decrypt(&self, mut buf: Buffer) -> Result<Buffer, Self::Error> {
        for b in &mut buf {
            *b -= self.shift;
        }

        Ok(buf)
    }
}

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
