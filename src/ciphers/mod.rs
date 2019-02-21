use crate::{Buffer, PartialBuffer};

macro_rules! derive_encrypt_decrypt {
    ($name:ident, $err:ident) => {
        impl Encrypt for $name {
            type Error = $err;

            fn encrypt(&mut self, buf: Buffer) -> Result<Buffer, Self::Error> {
                self.encrypt_partial(buf.into()).map(|b| b.into())
            }
        }

        impl Decrypt for $name {
            type Error = $err;

            fn decrypt(&mut self, buf: Buffer) -> Result<Buffer, Self::Error> {
                self.decrypt_partial(buf.into()).map(|b| b.into())
            }
        }
    };
}

mod caesar;
pub use self::caesar::Caesar;

mod affine;
pub use self::affine::Affine;

mod vigenere;
pub use self::vigenere::Vigenere;

mod transposition;
pub use self::transposition::Transposition;

mod substitution;
pub use self::substitution::Substitution;

pub trait Encrypt {
    type Error: std::error::Error;

    fn encrypt(&mut self, buf: Buffer) -> Result<Buffer, Self::Error>;
}

pub trait Decrypt {
    type Error: std::error::Error;

    fn decrypt(&mut self, buf: Buffer) -> Result<Buffer, Self::Error>;
}

pub trait PartialEncrypt: Encrypt {
    fn encrypt_partial(&mut self, buf: PartialBuffer) -> Result<PartialBuffer, Self::Error>;
}

pub trait PartialDecrypt: Decrypt {
    fn decrypt_partial(&mut self, buf: PartialBuffer) -> Result<PartialBuffer, Self::Error>;
}
