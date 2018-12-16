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

    pub fn letter_frequencies(&self) -> [u32; Char::MAX as usize] {
        let mut out = [0; Char::MAX as usize];

        for &b in self {
            out[u8::from(b) as usize] += 1;
        }

        out
    }

    pub fn index_of_coincidence(&self) -> f64 {
        let freqs = self.letter_frequencies();

        let total = freqs.iter()
            .filter(|&f| *f > 0)
            .map(|&f| f * (f - 1))
            .sum::<u32>() as f64;

        let len = self.len();
        let denominator = (len * (len - 1)) as f64 / Char::MAX as f64;

        total / denominator
    }

    pub fn chi_squared(&self) -> f64 {
        let english_freqs = [
           	0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094,
            0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929,
            0.00095, 0.05987, 0.06327, 0.09056, 0.02758, 0.00978, 0.02360, 0.00150,
            0.01974, 0.00074 
        ];

        let freqs = self.letter_frequencies();
        let len_f = self.len() as f64;

        freqs.iter()
            .enumerate()
            .map(|(i, &f)| {
                let e_count = len_f * english_freqs[i];
                let diff = f as f64 - e_count;
                (diff * diff) / e_count
            })
            .sum()
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
            .map(Char::from)
            .collect();

        Buffer::new(bytes, original, 1, 0)
    }
}

impl From<&String> for Buffer {
    fn from(data: &String) -> Buffer {
        Buffer::from(&data[..])
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

    #[test]
    fn test_stats() {
        let buf = Buffer::from("Rust is the best programming language");

        let expected = [3, 1, 0, 0, 3, 0, 4, 1, 2, 0, 0, 1, 2, 2, 1, 1, 0, 3, 3, 3, 2, 0, 0, 0, 0, 0];
        assert_eq!(expected, buf.letter_frequencies());

        assert_eq!(1.310483870967742, buf.index_of_coincidence());
        assert_eq!(29.514280393617323, buf.chi_squared());
    }
}
