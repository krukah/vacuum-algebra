#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// represents composition of exactly two ladder operators
pub enum Pair {
    ZeroOne,
    OneZero,
}
impl std::ops::Not for Pair {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::ZeroOne => Self::OneZero,
            Self::OneZero => unreachable!("we should never invert a OneZero"),
        }
    }
}
