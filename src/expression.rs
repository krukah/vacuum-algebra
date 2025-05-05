use crate::ladder::Ladder;
use crate::natural::Natural;
use crate::pair::Pair;
use std::ops::Not;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
/// represents composition of ladder operators
pub struct Expression {
    bits: u64,
    size: usize,
}

/// typesafe infallible Pair conversion
impl From<Pair> for Expression {
    fn from(value: Pair) -> Self {
        match value {
            Pair::ZeroOne => Self::from("01"),
            Pair::OneZero => Self::from("10"),
        }
    }
}

/// typesafe infallible Ladder conversion
impl From<Ladder> for Expression {
    fn from(value: Ladder) -> Self {
        match value {
            Ladder::T => Self::from("1"),
            Ladder::F => Self::from("0"),
        }
    }
}

impl<'a> From<&'a str> for Expression {
    fn from(value: &'a str) -> Self {
        assert!(value.len() <= 64);
        Self {
            size: value.len(),
            bits: value.chars().fold(0u64, |acc, char| {
                (acc << 1) | char.to_digit(2).unwrap() as u64
            }),
        }
    }
}

/// typesafe infallible bit-map conversion
impl From<(u64, usize)> for Expression {
    fn from((bits, size): (u64, usize)) -> Self {
        assert!(size <= 64);
        Self { bits, size }
    }
}

/// helpful for debugging
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "{:_>w$}", "", w = 64 - self.size())
        } else {
            write!(
                f,
                "{:_>width1$}{:0>width2$b}",
                "",
                self.bits,
                width1 = 64 - self.size(),
                width2 = 00 + self.size(),
            )
        }
    }
}

/// concatenation
impl std::ops::Mul for Expression {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::concatenate(self, rhs)
    }
}

impl Expression {
    /// calculate the expectation value recursively
    pub fn expectation(self) -> Natural {
        if self.is_empty() {
            Natural::zero()
        } else if self == Self::from(Pair::OneZero) {
            Natural::zero()
        } else if self == Self::from(Pair::ZeroOne) {
            Natural::unit()
        } else {
            match (self.prefix(), self.suffix()) {
                (Ladder::T, _) | (_, Ladder::F) => Natural::zero(),
                (Ladder::F, Ladder::T) => {
                    let (left, unit, rght) = self.split();
                    let removed = left * rght;
                    let swapped = left * unit * rght;
                    swapped.expectation() + removed.expectation()
                }
            }
        }
    }

    /// read the length
    pub fn size(&self) -> usize {
        self.size
    }

    /// split the expression into three parts,
    /// assuming there is a middle unit to extract
    /// eagerly
    pub fn split(self) -> (Self, Self, Self) {
        const MASK: u64 = 0b11;
        const FLAG: u64 = 0b01;
        let i = (0u64..64u64)
            .map(|i| ((self.bits & (MASK << i)), FLAG << i))
            .enumerate()
            .find(|(_, (self_bits, flag_bits))| self_bits == flag_bits)
            .map(|(occurence, _)| occurence)
            .expect("there must be an instance of 01, from upstream assertion");
        let size_rght = i;
        let size_left = self.size() - i - 2;
        let mask_rght = (1 << (i + 0)) - 1;
        let mask_left = (1 << (i + 2)) - 1;
        let mask_left = !mask_left;
        let bits_rght = (self.bits & mask_rght) >> 0;
        let bits_left = (self.bits & mask_left) >> (i + 2);
        let left = Expression::from((bits_left, size_left));
        let rght = Expression::from((bits_rght, size_rght));
        let unit = Expression::from(Pair::OneZero);
        (left, unit, rght)
    }

    /// compare bits, ignore size
    pub fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// extract the rightmost digit after the skip
    pub fn suffix(&self) -> Ladder {
        assert!(self.is_empty().not());
        match self.bits & 1 {
            0 => Ladder::F,
            _ => Ladder::T,
        }
    }

    /// extract the leftmost digit after the skip
    pub fn prefix(&self) -> Ladder {
        assert!(self.is_empty().not());
        match (self.bits) & (1 << (self.size - 1)) {
            0 => Ladder::F,
            _ => Ladder::T,
        }
    }

    /// concatenate
    pub fn concatenate(a: Self, b: Self) -> Self {
        assert!(a.size() + b.size() <= 64);
        Self {
            bits: a.bits << b.size() | b.bits,
            size: a.size() + b.size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_char_expressions() {
        assert_eq!(Expression::from("01").expectation(), Natural::unit()); // non-zero
        assert_eq!(Expression::from("10").expectation(), Natural::zero());
        assert_eq!(Expression::from("00").expectation(), Natural::zero());
        assert_eq!(Expression::from("11").expectation(), Natural::zero());
    }

    #[test]
    fn test_3_char_expressions() {
        assert_eq!(Expression::from("001").expectation(), Natural::zero());
        assert_eq!(Expression::from("011").expectation(), Natural::zero());
        assert_eq!(Expression::from("000").expectation(), Natural::zero());
        assert_eq!(Expression::from("100").expectation(), Natural::zero());
        assert_eq!(Expression::from("010").expectation(), Natural::zero());
        assert_eq!(Expression::from("101").expectation(), Natural::zero());
        assert_eq!(Expression::from("110").expectation(), Natural::zero());
        assert_eq!(Expression::from("111").expectation(), Natural::zero());
    }

    #[test]
    fn test_from_str() {
        let expr = Expression::from("101");
        assert_eq!(expr.size(), 3);
        assert_eq!(expr.bits, 0b101);
    }

    #[test]
    fn test_concatenation() {
        let a = Expression::from("01");
        let b = Expression::from("10");
        assert_eq!(a * b, Expression::from("0110"));
    }

    #[test]
    fn test_split() {
        let (left, unit, rght) = Expression::from("110100").split();
        assert_eq!(left, Expression::from("11"));
        assert_eq!(unit, Expression::from("10"));
        assert_eq!(rght, Expression::from("00"));
    }
}
