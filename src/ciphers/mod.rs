use crate::Buffer;

mod caesar;
pub use self::caesar::Caesar;

mod affine;
pub use self::affine::Affine;

mod vigenere;
pub use self::vigenere::Vigenere;

mod transposition;
pub use self::transposition::Transposition;

pub trait Encrypt {
    type Error: std::error::Error;

    fn encrypt(&self, buf: Buffer) -> Result<Buffer, Self::Error>;
}

pub trait Decrypt {
    type Error: std::error::Error;

    fn decrypt(&self, buf: Buffer) -> Result<Buffer, Self::Error>;
}
