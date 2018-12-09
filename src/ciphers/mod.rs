use crate::CharStream;

#[inline]
fn char_base(ch: u8) -> Option<u8> {
    match ch {
        b'a' ... b'z' => Some(b'a'),
        b'A' ... b'Z' => Some(b'A'),
        _ => None,
    }
}

#[inline]
fn char_value(ch: u8) -> Option<u8> {
    let base = char_base(ch)?;
    Some(ch - base)
}

#[inline]
fn char_rotate(ch: u8, shift: u8) -> Option<u8> {
    let base = char_base(ch)?;

    let val = ch - base;
    Some(base + (val + shift) % 26)
}

pub trait Cipher {
    type CipherError;

    fn encrypt(&self, data: &mut CharStream) -> Result<(), Self::CipherError>;
    fn decrypt(&self, data: &mut CharStream) -> Result<(), Self::CipherError>;
}

pub mod caesar;
pub use self::caesar::Caesar;

pub mod affine;
pub use self::affine::Affine;
