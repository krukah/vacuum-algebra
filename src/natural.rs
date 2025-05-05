#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
/// wrapper type around integers
pub struct Natural(usize);

impl std::ops::Add for Natural {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Natural {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn unit() -> Self {
        Self(1)
    }

    pub fn size(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_zero() {
        assert_eq!(Natural::zero().size(), 0);
    }

    #[test]
    fn test_natural_unit() {
        assert_eq!(Natural::unit().size(), 1);
    }

    #[test]
    fn test_natural_add() {
        assert_eq!(Natural::unit() + Natural::unit(), Natural(2));
        assert_eq!(Natural::zero() + Natural::zero(), Natural::zero());
        assert_eq!(Natural::zero() + Natural::unit(), Natural::unit());
    }
}
