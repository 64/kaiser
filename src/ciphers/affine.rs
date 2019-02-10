use super::{Decrypt, Encrypt, PartialEncrypt, PartialDecrypt};
use crate::{Buffer, PartialBuffer, Char};
use simple_error::SimpleError;

pub struct Affine {
    a: u8,
    b: u8,
    mmi_a: u8,
}

impl Affine {
    pub fn new(a: u8, b: u8) -> Self {
        let mmi_a = (0..Char::MAX)
            .find(|&i| (a * i) % Char::MAX == 1)
            .expect("unable to find modular multiplicative inverse of a");

        Self { a, b, mmi_a }
    }
}

impl PartialEncrypt for Affine {
    fn encrypt_partial(&self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        for x in &mut buf {
            *x = (*x * self.a) + self.b;
        }

        Ok(buf)
    }
}

impl PartialDecrypt for Affine {
    fn decrypt_partial(&self, mut buf: PartialBuffer) -> Result<PartialBuffer, Self::Error> {
        for x in &mut buf {
            *x = (*x - self.b) * self.mmi_a;
        }

        Ok(buf)
    }
}

derive_encrypt_decrypt!(Affine, SimpleError);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let buf = Buffer::from("Hello world!");

        let affine = Affine::new(3, 5);

        let buf = affine.encrypt(buf).unwrap();
        assert_eq!("Armmv tvemo!", buf.to_string());

        let buf = affine.decrypt(buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }
}
