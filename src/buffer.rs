use std::num::NonZeroUsize;
use std::{slice, iter, fmt};
use std::sync::Arc;

use crate::Char;

#[derive(Clone, Debug)]
pub struct Buffer {
    data: Vec<Char>,
    original: Arc<String>,
    stride: NonZeroUsize,
    offset: usize
}

impl Buffer {
    pub fn new(data: Vec<Char>, original: Arc<String>, stride: usize, offset: usize) -> Self {
        Self {
            data,
            original,
            stride: NonZeroUsize::new(stride).expect("non-zero stride"),
            offset,
        }
    }

    pub fn len(&self) -> usize {
        let stride = self.stride();

        // Take ceiling of integer division: (len - offset) / stride
        (self.data.len() + stride - self.offset - 1) / stride 
    }

    pub fn set_stride(&mut self, stride: usize) {
        self.stride = NonZeroUsize::new(stride).expect("non-zero stride");
    }

    pub fn stride(&self) -> usize {
        self.stride.get()
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn original(&self) -> &str {
        &self.original
    }
}

impl From<&str> for Buffer {
    fn from(data: &str) -> Self {
        assert!(data.is_ascii()); 

        let original = Arc::new(data.to_owned());

        let bytes: Vec<Char> = original
            .chars()
            .filter(|c| c.is_alphabetic() && c.is_ascii())
            .map(|c| Char::from(c))
            .collect();

        Buffer::new(bytes, original, 1, 0)
    }
}

impl<'a> IntoIterator for &'a Buffer {
    type Item = &'a Char;
    type IntoIter = iter::StepBy<iter::Skip<slice::Iter<'a, Char>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.as_slice().iter()
            .skip(self.offset())
            .step_by(self.stride())
    }
}

impl<'a> IntoIterator for &'a mut Buffer {
    type Item = &'a mut Char;
    type IntoIter = iter::StepBy<iter::Skip<slice::IterMut<'a, Char>>>;

    fn into_iter(self) -> Self::IntoIter {
        let (offset, stride) = (self.offset(), self.stride());

        self.data.as_mut_slice().iter_mut()
            .skip(offset)
            .step_by(stride)
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = self.original
            .chars()
            .scan(0, |index, c| {
                if let Some(entry) = self.data.get(*index) {
                    if c.is_alphabetic() {
                        *index += 1;

                        if c.is_uppercase() {
                            return Some(entry.to_upper());
                        } else {
                            return Some(entry.to_lower());
                        };
                    }
                }

                Some(c)
            })
            .collect::<String>();

        fmt::Display::fmt(&out, f)
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
        let mut buffer = Buffer::from("ABCDEFGHIJ");
        buffer.set_stride(2);
        buffer.set_offset(3);

        let expected = buffer
            .into_iter()
            .map(|&c| -> char { c.into() })
            .collect::<String>();

        assert_eq!("DFHJ", expected);
        assert_eq!(4, buffer.len());
    }
}
