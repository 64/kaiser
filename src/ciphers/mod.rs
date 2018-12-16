use crate::{Buffer};

mod caesar;
pub use self::caesar::Caesar;

mod affine;
pub use self::affine::Affine;

pub trait Encrypt {
    type Error: std::error::Error;

    fn encrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error>;
}

pub trait Decrypt {
    type Error: std::error::Error;

    fn decrypt(&self, buf: &mut Buffer) -> Result<(), Self::Error>;
}
