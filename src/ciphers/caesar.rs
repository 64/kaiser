use super::{Decrypt, Encrypt, PartialDecrypt, PartialEncrypt};
use crate::metaheuristic::HeuristicTarget;
use crate::{Buffer, Char, PartialBuffer};
use rand::Rng;
use simple_error::SimpleError;

#[derive(Clone)]
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

impl HeuristicTarget for Caesar {
    type KeyParam = ();

    fn rand_key<R: Rng + ?Sized>(_param: Self::KeyParam, _rng: &mut R) -> Self {
        unimplemented!()
    }
    fn tweak_key<R: Rng + ?Sized>(&mut self, _param: Self::KeyParam, _rng: &mut R) {
        unimplemented!()
    }

    // Used for brute force (linear search) - pass 1st param None to get 1st key
    // TODO: Can we use iterators somehow?
    fn next_key(key: Option<Self>, _param: Self::KeyParam) -> Option<Self> {
        match key {
            Some(k) => {
                if k.shift == Char::MAX - 1 {
                    None
                } else {
                    Some(Caesar { shift: k.shift + 1 })
                }
            }
            None => Some(Caesar { shift: 0 }),
        }
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
