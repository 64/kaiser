use simple_error::SimpleError;
use crate::CharStream;
use super::Cipher;
use ascii::AsciiChar;

pub struct Caesar {
    shift: u8,
}

impl Caesar {
    pub fn with_shift(shift: u8) -> Self {
        assert!(shift < 26);
        Self { shift }
    }
}

impl Cipher for Caesar {
    type CipherError = SimpleError;

    fn encrypt(&self, data: &mut CharStream) -> Result<(), Self::CipherError> {
        for byte in data {
            let num = *byte as u8;
            if let Some(ch) = super::char_rotate(num, self.shift) {
                *byte = AsciiChar::from(ch).unwrap();
            }
        }

        Ok(())
    }

    fn decrypt(&self, data: &mut CharStream) -> Result<(), Self::CipherError> {
        for byte in data {
            let num = *byte as u8;
            if let Some(ch) = super::char_rotate(num, 26 - self.shift) {
                *byte = AsciiChar::from(ch).unwrap();
            }
        }

        Ok(())
    }
}
