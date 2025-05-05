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
            Pair::ZeroOne => Self::from((1, 2)), // "01"
            Pair::OneZero => Self::from((2, 2)), // "10"
        }
    }
}
/// typesafe infallible Ladder conversion
impl From<Ladder> for Expression {
    fn from(value: Ladder) -> Self {
        match value {
            Ladder::Normal => Self::from((1, 1)), // "1"
            Ladder::Dagger => Self::from((0, 1)), // "0"
        }
    }
}
/// not typesafe, fallile, string conversion. only use at compile time.
impl<'a> From<&'a str> for Expression {
    fn from(value: &'a str) -> Self {
        assert!(value.len() <= 64);
        Self {
            size: value.len(),
            bits: value
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u64)
                .fold(0u64, |acc, bit| (acc << 1) | bit),
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
                self.bits(),
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
        } else if self.is_imbalanced() {
            Natural::zero()
        } else if self.prefix() == Ladder::Normal {
            Natural::zero()
        } else if self.suffix() == Ladder::Dagger {
            Natural::zero()
        } else if self == Self::from(Pair::ZeroOne) {
            Natural::unit()
        } else {
            let (left, unit, rght) = self.split();
            let removed = left * rght;
            let swapped = left * unit * rght;
            removed.expectation() + swapped.expectation()
        }
    }

    /// read the length
    fn size(&self) -> usize {
        self.size
    }
    /// read the bits
    fn bits(&self) -> u64 {
        self.bits
    }

    /// compare bits, ignore size
    fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// imbalance between Ladder operator variants implies zero expectation
    fn is_imbalanced(self) -> bool {
        self.size() == 2 * self.bits().count_ones() as usize
    }

    /// extract the rightmost digit after the skip
    fn suffix(&self) -> Ladder {
        assert!(self.is_empty().not());
        let lsb = self.bits & 1;
        Ladder::from(lsb as usize)
    }

    /// extract the leftmost digit after the skip
    fn prefix(&self) -> Ladder {
        assert!(self.is_empty().not());
        let msb = self.bits() & (1 << (self.size() - 1));
        Ladder::from(msb as usize)
    }

    /// concatenate
    fn concatenate(a: Self, b: Self) -> Self {
        assert!(a.size() + b.size() <= 64);
        Self {
            bits: a.bits() << b.size() | b.bits(),
            size: a.size() + b.size(),
        }
    }

    /// split the expression into three parts,
    /// assuming there is a middle unit to extract
    /// eagerly
    fn split(self) -> (Self, Self, Self) {
        const MASK: u64 = 0b11;
        const FLAG: u64 = 0b01;
        let i = (0u64..)
            .inspect(|i| assert!(*i < 64))
            .map(|i| ((self.bits() & (MASK << i)), FLAG << i))
            .enumerate()
            .find(|(_, (self_bits, flag_bits))| self_bits == flag_bits)
            .map(|(occurence, _)| occurence)
            .expect("there must be an instance of 01, from upstream assertion");
        let size_rght = i;
        let size_left = self.size() - i - 2;
        let mask_rght = (1 << (i + 0)) - 1;
        let mask_left = (1 << (i + 2)) - 1;
        let mask_left = !mask_left;
        let bits_rght = (self.bits() & mask_rght) >> 0;
        let bits_left = (self.bits() & mask_left) >> (i + 2);
        let left = Self::from((bits_left, size_left));
        let rght = Self::from((bits_rght, size_rght));
        let unit = Self::from(Pair::OneZero);
        (left, unit, rght)
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
    fn test_runaway_substitution_1() {
        let expression = Expression::from("00011110100001111100101101101111");
        let expectation = Natural::zero();
        assert_eq!(expression.expectation(), expectation);
    }

    #[test]
    fn test_runaway_substitution_2() {
        let expression =
            Expression::from("0000000001101110111001111110010101110111100010000011010011111111");
        let expectation = Natural::zero();
        assert_eq!(expression.expectation(), expectation);
    }

    #[test]
    fn test_8_char_expressions() {
        let expression = Expression::from("00100111");
        let expectation = Natural::from(12);
        assert_eq!(expression.expectation(), expectation);

        let expression = Expression::from("01001101");
        let expectation = Natural::from(2);
        assert_eq!(expression.expectation(), expectation);
    }

    #[test]
    fn test_from_str() {
        let expr = Expression::from("101");
        assert_eq!(expr.size(), 3);
        assert_eq!(expr.bits, 0b101);
    }

    #[test]
    fn test_concatenation() {
        let a = Expression::from("010");
        let b = Expression::from("100");
        let c = Expression::from("010100");
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_split() {
        let (left, unit, rght) = Expression::from("110100").split();
        assert_eq!(left, Expression::from("11"));
        assert_eq!(unit, Expression::from("10"));
        assert_eq!(rght, Expression::from("00"));
        // constants that are bit-wise representations of Pairs,
        // but not actually typed as Pairs, useful to find
        // position of first occurence of FLAG
        //
        // self = 0b_1010_1111 (binary representation)
        // FLAG = 0b_01
        // MASK = 0b_11
        // i    = 3 (position of first occurrence of `01`)
        //
        // LEFT CALCULATION:
        // self.0                         = 0b_1010_1111
        // i                              =         3...
        // u64::MAX                       = 0b_1111_1111
        // u64::MAX << (i + 2)            = 0b_1111_1111 << 5
        //                                = 0b_1110_0000
        // self.0 & (u64::MAX << (i + 2)) = 0b_1010_1111
        //                                & 0b_1110_0000
        //                                = 0b_1010_0000
        //
        // RIGHT CALCULATION:
        // 1 << i                         = 1 << 3
        //                                = 0b______1000
        // (1 << i) - 1                   = 0b______1000 - 1
        //                                = 0b______0111
        // self.0 & ((1 << i) - 1)        = 0b_1010_1111
        //                                & 0b______0111
        //                                = 0b______0111
        //
        // RESULT:
        // left = 0b_101_00_000 => Operator(_)
        // pair = 0b_____01_000 => Pair::OneZero
        // rght = 0b________111 => Operator(_)
    }
}
