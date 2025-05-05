#[derive(Clone, Debug, Copy, PartialEq, Eq)]
/// represents a single ladder operator
pub enum Ladder {
    Normal,
    Dagger,
}
impl From<usize> for Ladder {
    fn from(x: usize) -> Self {
        match x.count_ones() {
            0 => Self::Dagger,
            1 => Self::Normal,
            _ => panic!("invalid ladder value: {}", x),
        }
    }
}
impl std::ops::Not for Ladder {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Normal => Self::Dagger,
            Self::Dagger => Self::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ladder_not() {
        assert_eq!(!Ladder::Normal, Ladder::Dagger);
        assert_eq!(!Ladder::Dagger, Ladder::Normal);
    }
}
