use simple_error::SimpleError;
use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;
use std::{fmt, iter, slice};

use crate::{score, Char};

#[derive(Clone, Debug)]
pub struct Buffer {
    data: Vec<Char>,
    original: Arc<String>,
}

#[derive(Clone, Debug)]
pub struct PartialBuffer {
    buf: Buffer,
    offset: usize,
    stride: NonZeroUsize,
}

pub trait IntoBorrowingIterator<'a> {
    type IntoIter: Iterator<Item = &'a Char>;
    type IntoIterMut: Iterator<Item = &'a mut Char>;

    fn iter(&'a self) -> Self::IntoIter;
    fn iter_mut(&'a mut self) -> Self::IntoIterMut;
}

impl<'a, T: 'a> IntoBorrowingIterator<'a> for T
where
    for<'b> &'b T: IntoIterator<Item = &'b Char>,
    for<'b> &'b mut T: IntoIterator<Item = &'b mut Char>,
{
    type IntoIter = <&'a T as IntoIterator>::IntoIter;
    type IntoIterMut = <&'a mut T as IntoIterator>::IntoIter;

    fn iter(&'a self) -> Self::IntoIter {
        self.into_iter()
    }

    fn iter_mut(&'a mut self) -> Self::IntoIterMut {
        self.into_iter()
    }
}

pub trait CharStream<'a>: IntoBorrowingIterator<'a> {
    fn original(&'a self) -> &'a str;

    fn len(&'a self) -> usize;

    fn collect_original(&'a self) -> String {
        let mut char_stream = self.iter();
        let mut next_char = char_stream.next();

        self.original()
            .chars()
            .map(|c| {
                if let Some(entry) = next_char {
                    if c.is_alphabetic() {
                        next_char = char_stream.next();

                        if c.is_uppercase() {
                            return entry.to_upper();
                        } else {
                            return entry.to_lower();
                        };
                    }
                }

                c
            })
            .collect::<String>()
    }
}

impl<'a> IntoIterator for &'a Buffer {
    type Item = &'a Char;
    type IntoIter = slice::Iter<'a, Char>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a mut Buffer {
    type Item = &'a mut Char;
    type IntoIter = slice::IterMut<'a, Char>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.as_mut_slice().iter_mut()
    }
}

impl<'a> IntoIterator for &'a PartialBuffer {
    type Item = &'a Char;
    type IntoIter = iter::StepBy<iter::Skip<<&'a Buffer as IntoIterator>::IntoIter>>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf
            .data
            .iter()
            .skip(self.offset)
            .step_by(self.stride.get())
    }
}

impl<'a> IntoIterator for &'a mut PartialBuffer {
    type Item = &'a mut Char;
    type IntoIter = iter::StepBy<iter::Skip<<&'a mut Buffer as IntoIterator>::IntoIter>>;

    fn into_iter(self) -> Self::IntoIter {
        self.buf
            .data
            .iter_mut()
            .skip(self.offset)
            .step_by(self.stride.get())
    }
}

impl Buffer {
    pub fn new(data: Vec<Char>, original: Arc<String>) -> Self {
        Self { data, original }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn partial(self, offset: usize, stride: NonZeroUsize) -> PartialBuffer {
        PartialBuffer {
            buf: self,
            offset,
            stride,
        }
    }

    pub fn score(&self, method: score::ScoreMethod) -> score::Score {
        score::score(&self, method)
    }

    pub fn original(&self) -> &str {
        &self.original
    }
}

impl std::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(c1, c2)| c1 == c2)
    }
}

impl FromStr for Buffer {
    type Err = SimpleError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let original = Arc::new(data.to_owned());

        let bytes: Result<Vec<Char>, _> = original
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| {
                if !c.is_ascii() {
                    Err(SimpleError::new("string contains non-ascii characters"))
                } else {
                    Ok(Char::from(c))
                }
            })
            .collect();

        bytes.map(|b| Buffer::new(b, original))
    }
}

impl From<&str> for Buffer {
    fn from(data: &str) -> Self {
        Buffer::from_str(data).unwrap()
    }
}

impl From<&String> for Buffer {
    fn from(data: &String) -> Buffer {
        Buffer::from(&data[..])
    }
}

impl From<Buffer> for PartialBuffer {
    fn from(buf: Buffer) -> PartialBuffer {
        PartialBuffer {
            buf,
            offset: 0,
            stride: NonZeroUsize::new(1).unwrap(), // TODO: Check assembly - do we need new_unchecked?
        }
    }
}

impl From<PartialBuffer> for Buffer {
    fn from(pbuf: PartialBuffer) -> Buffer {
        pbuf.buf
    }
}

impl CharStream<'_> for PartialBuffer {
    fn original(&self) -> &str {
        &*self.buf.original
    }

    fn len(&self) -> usize {
        let stride = self.stride.get();

        // Take ceiling of integer division: (len - offset) / stride
        (self.buf.len() + stride - self.offset - 1) / stride
    }
}

impl CharStream<'_> for Buffer {
    fn original(&self) -> &str {
        &*self.original
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for PartialBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.collect_original(), f)
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.collect_original(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let s = "Hello World!".to_owned();
        let buf = Buffer::from("Hello World!");
        assert_eq!(s, buf.to_string());
    }

    #[test]
    fn test_offset_stride() {
        let buffer = Buffer::from("ABCDEFGHIJ").partial(3, NonZeroUsize::new(2).unwrap());

        let expected = buffer
            .iter()
            .map(|&c| -> char { c.into() })
            .collect::<String>();

        assert_eq!("DFHJ", expected);
        assert_eq!(4, buffer.len());
    }
}
