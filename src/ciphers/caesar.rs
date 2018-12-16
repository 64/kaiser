use simple_error::SimpleError;
use crate::Buffer;
use super::{Encrypt, Decrypt};

pub struct Caesar {
    shift: u8
}

impl Caesar {
    pub fn new(shift: u8) -> Self {
        Self { shift }
    }
}

impl Encrypt for Caesar {
    type Error = SimpleError;

    fn encrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error> {
        for b in buf {
            *b += self.shift;
        }

        Ok(())
    }
}

impl Decrypt for Caesar {
    type Error = SimpleError;

    fn decrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error> {
        for b in buf {
            *b -= self.shift;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let caesar = Caesar::new(5);
        let mut buf = Buffer::from("Hello world!"); 

        caesar.encrypt(&mut buf).unwrap();
        assert_eq!("Mjqqt btwqi!", buf.to_string());

        caesar.decrypt(&mut buf).unwrap();
        assert_eq!("Hello world!", buf.to_string());
    }
}
