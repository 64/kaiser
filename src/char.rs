use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char {
    c: u8
}

impl Char {
    pub const MAX: u8 = 26;

    pub fn to_upper(self) -> char {
        (self.c as u8 + b'A') as char
    }

    pub fn to_lower(self) -> char {
        (self.c as u8 + b'a') as char
    }
}

impl Add<u8> for Char {
    type Output = Char;

    fn add(self, other: u8) -> Char {
        Char { c: (self.c + other) % Char::MAX } 
    }
}

impl AddAssign<u8> for Char {
    fn add_assign(&mut self, other: u8) {
        *self = *self + other; 
    }
}

impl Sub<u8> for Char {
    type Output = Char;

    fn sub(self, other: u8) -> Char {
        let a = self.c as i32 - other as i32;
        let b = a + (Char::MAX as i32 * std::u8::MAX as i32);
        let c = b % (Char::MAX as i32);
        Char { c: c as u8 }
    }
}

impl SubAssign<u8> for Char {
    fn sub_assign(&mut self, other: u8) {
        *self = *self - other;
    }
}

impl From<u8> for Char {
    fn from(c: u8) -> Char {
        debug_assert!(c < Char::MAX);
        Char { c }
    }
}

impl From<char> for Char {
    fn from(c: char) -> Char {
        debug_assert!(c.is_ascii() && c.is_alphabetic());
        if c.is_uppercase() {
            Char { c: c as u8 - b'A' }
        } else {
            Char { c: c as u8 - b'a' }
        }
    }
}

impl From<Char> for u8 {
    fn from(c: Char) -> u8 {
        c.c
    }
}

impl From<Char> for char {
    fn from(c: Char) -> char {
        c.to_upper()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_lower() {
        let c = Char { c: 0 };

        assert_eq!(c.to_upper(), 'A');
        assert_eq!(c.to_lower(), 'a');
    }

    #[test]
    fn test_conversions() {
        let a = Char { c: 0 };
        let k = Char { c: 10 };

        assert_eq!(0, u8::from(a));
        assert_eq!(10, u8::from(k));
        assert_eq!('A', char::from(a));
        assert_eq!('K', char::from(k));
        assert_eq!(a, Char::from(0));
        assert_eq!(k, Char::from(10));
        assert_eq!(a, Char::from('A'));
        assert_eq!(a, Char::from('a'));
        assert_eq!(k, Char::from('K'));
        assert_eq!(k, Char::from('k'));
    }

    #[test]
    fn test_ops() {
        let a = Char { c: 0 };

        assert_eq!(a, a + 0);
        assert_eq!(a, a - 0);
        assert_eq!(a, a + Char::MAX);
        assert_eq!(a, a - Char::MAX);
        assert_eq!(Char::from(1), a + 1);
        assert_eq!(Char::from(10), a + 10);
        assert_eq!(Char::from(23), a - 3);
        assert_eq!(Char::from(10), a + Char::MAX + 10);
        assert_eq!(Char::from(16), a - Char::MAX - 10);
    }
}
