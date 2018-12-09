use std::num::NonZeroUsize;
use std::{slice, iter, fmt};
use std::str::FromStr;
use ascii::{AsciiChar, AsciiString};

#[derive(Clone, Debug)]
pub struct CharStream {
    data: AsciiString,
    stride: NonZeroUsize,
    offset: usize,
}

impl CharStream {
    pub fn new(data: AsciiString) -> Self {
        Self::new_with_params(data, NonZeroUsize::new(1).unwrap(), 0)
    }

    pub fn new_with_params(data: AsciiString, stride: NonZeroUsize, offset: usize) -> Self {
        Self {
            data,
            stride,
            offset,
        }
    }

    pub fn stride(&self) -> usize {
        self.stride.get() 
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl From<&str> for CharStream {
    fn from(data: &str) -> Self {
        CharStream::new(AsciiString::from_str(data).unwrap())
    }
}

impl<'a> IntoIterator for &'a CharStream {
    type Item = &'a AsciiChar;
    type IntoIter = iter::StepBy<iter::Skip<slice::Iter<'a, AsciiChar>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.as_slice().iter()
            .skip(self.offset())
            .step_by(self.stride())
    }
}

impl<'a> IntoIterator for &'a mut CharStream {
    type Item = &'a mut AsciiChar;
    type IntoIter = iter::StepBy<iter::Skip<slice::IterMut<'a, AsciiChar>>>;

    fn into_iter(self) -> Self::IntoIter {
        let (offset, stride) = (self.offset(), self.stride());

        self.data.as_mut_slice().iter_mut()
            .skip(offset)
            .step_by(stride)
    }
}

impl fmt::Display for CharStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.data, f)
    }
}
