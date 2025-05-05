#[derive(Clone, Debug, Copy, PartialEq, Eq)]
/// represents a single ladder operator
pub enum Ladder {
    T,
    F,
}

impl std::ops::Not for Ladder {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::T => Self::F,
            Self::F => Self::T,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladder_not() {
        assert_eq!(!Ladder::T, Ladder::F);
        assert_eq!(!Ladder::F, Ladder::T);
    }
}
