use simple_error::SimpleError;
use crate::{Buffer, Char};
use super::{Encrypt, Decrypt};

pub struct Affine {
    a: u8,
    b: u8,
    mmi_a: u8
}

impl Affine {
    pub fn new(a: u8, b: u8) -> Self {
        let mmi_a = (0..Char::MAX).find(|&i| (a * i) % Char::MAX == 1)
            .expect("unable to find modular multiplicative inverse of a");

        Self {
            a,
            b,
            mmi_a
        }
    }
}

impl Encrypt for Affine {
    type Error = SimpleError;

    fn encrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error> {
        for x in buf {
            *x = (*x * self.a) + self.b;
        }

        Ok(())
    }
}

impl Decrypt for Affine {
    type Error = SimpleError;

    fn decrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error> {
        for x in buf {
            *x = (*x - self.b) * self.mmi_a;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let mut buf = Buffer::from("Hello world!");

        let affine = Affine::new(3, 5);

        affine.encrypt(&mut buf).unwrap();
        assert_eq!("Armmv tvemo!", buf.to_string());
        
        affine.decrypt(&mut buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }
}
