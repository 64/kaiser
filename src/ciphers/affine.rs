use simple_error::SimpleError;
use crate::CharStream;
use super::Cipher;
use ascii::AsciiChar;

pub struct Affine {
    a: u8,
    b: u8,
    mmi_a: u8,
}

impl Affine {
    pub fn with_params(a: u8, b: u8) -> Self {
        // Calculate modular multiplicative inverse of a modulo m (= 26)
        let mmi_a = (0..26).find(|i| (a * i) % 26 == 1)
            .expect("unable to find modular multiplicative inverse of a");

        Self { 
            a,
            b,
            mmi_a
        }
    }
}

impl Cipher for Affine {
    type CipherError = SimpleError;

    fn encrypt(&self, data: &mut CharStream) -> Result<(), Self::CipherError> {
        for byte in data {
            let num = *byte as u8;

            match super::char_value(num) {
                Some(x) => {
                    let new = super::char_rotate(num, self.a * x + self.b).unwrap();
                    *byte = AsciiChar::from(new).unwrap();
                },
                None => (),
            }
        }

        Ok(())
    }

    fn decrypt(&self, data: &mut CharStream) -> Result<(), Self::CipherError> {
        for byte in data {
            let num = *byte as u8;

            match super::char_value(num) {
                Some(x) => {
                    let new = super::char_rotate(num, self.mmi_a * (x - self.b)).unwrap();
                    *byte = AsciiChar::from(new).unwrap();
                },
                None => (),
            }
        }

        Ok(())
    }
}
