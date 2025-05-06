#[derive(Clone, Debug, Copy, PartialEq, Eq)]
/// represents a single ladder operator
pub enum Ladder {
    Lower,
    Raise,
}
impl From<usize> for Ladder {
    fn from(x: usize) -> Self {
        match x.count_ones() {
            0 => Self::Raise,
            1 => Self::Lower,
            _ => panic!("invalid ladder value: {}", x),
        }
    }
}
impl std::ops::Not for Ladder {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Lower => Self::Raise,
            Self::Raise => Self::Lower,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ladder_not() {
        assert_eq!(!Ladder::Lower, Ladder::Raise);
        assert_eq!(!Ladder::Raise, Ladder::Lower);
    }
}
