use num::Signed;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/*-------------------------------------------------------------------------------------------------
  Grid Index
-------------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GridIndex<I> {
    pub row: I,
    pub column: I,
}

impl<I> GridIndex<I> {
    pub const fn new(row: I, column: I) -> Self {
        Self { row, column }
    }
}

/*--------------------------------------------------------------------------------------
  Conversions
--------------------------------------------------------------------------------------*/

impl<I> From<(I, I)> for GridIndex<I> {
    fn from((row, column): (I, I)) -> Self {
        Self { row, column }
    }
}

/*-------------------------------------------------------------------------------------------------
  Arithmetic Traits
-------------------------------------------------------------------------------------------------*/

macro_rules! impl_arithmetic_traits {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident) => {
        // Pairwise arithmetic operations
        impl<I: $trait<Output = I>> $trait for GridIndex<I> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    row: self.row.$method(rhs.row),
                    column: self.column.$method(rhs.column),
                }
            }
        }

        impl<I: $assign_trait> $assign_trait for GridIndex<I> {
            fn $assign_method(&mut self, rhs: Self) {
                self.row.$assign_method(rhs.row);
                self.column.$assign_method(rhs.column);
            }
        }

        // Scalar arithmetic operations
        impl<I: Copy + $trait<Output = I>> $trait<I> for GridIndex<I> {
            type Output = Self;

            fn $method(self, rhs: I) -> Self::Output {
                Self {
                    row: self.row.$method(rhs),
                    column: self.column.$method(rhs),
                }
            }
        }

        impl<I: Copy + $assign_trait> $assign_trait<I> for GridIndex<I> {
            fn $assign_method(&mut self, rhs: I) {
                self.row.$assign_method(rhs);
                self.column.$assign_method(rhs);
            }
        }
    };
}

impl_arithmetic_traits!(Add, add, AddAssign, add_assign);
impl_arithmetic_traits!(Sub, sub, SubAssign, sub_assign);
impl_arithmetic_traits!(Mul, mul, MulAssign, mul_assign);
impl_arithmetic_traits!(Div, div, DivAssign, div_assign);
impl_arithmetic_traits!(Rem, rem, RemAssign, rem_assign);

// Negation unary operation
impl<I: Neg<Output = I>> Neg for GridIndex<I> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            row: self.row.neg(),
            column: self.column.neg(),
        }
    }
}

/*--------------------------------------------------------------------------------------
  Signed
--------------------------------------------------------------------------------------*/

impl<I: Signed> GridIndex<I> {
    pub fn abs(&self) -> Self {
        Self {
            row: self.row.abs(),
            column: self.column.abs(),
        }
    }

    pub fn is_positive(&self) -> bool {
        self.row.is_positive() && self.column.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.row.is_negative() || self.column.is_negative()
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let index = GridIndex::new(3, 4);
        assert_eq!(index.row, 3);
        assert_eq!(index.column, 4);
    }

    #[test]
    fn test_add() {
        let index1 = GridIndex::new(1, 2);
        let index2 = GridIndex::new(3, 4);
        let result = index1 + index2;
        assert_eq!(result.row, 4);
        assert_eq!(result.column, 6);
    }

    #[test]
    fn test_add_assign() {
        let mut index1 = GridIndex::new(1, 2);
        let index2 = GridIndex::new(3, 4);
        index1 += index2;
        assert_eq!(index1.row, 4);
        assert_eq!(index1.column, 6);
    }

    #[test]
    fn test_sub() {
        let index1 = GridIndex::new(5, 7);
        let index2 = GridIndex::new(2, 3);
        let result = index1 - index2;
        assert_eq!(result.row, 3);
        assert_eq!(result.column, 4);
    }

    #[test]
    fn test_sub_assign() {
        let mut index1 = GridIndex::new(5, 7);
        let index2 = GridIndex::new(2, 3);
        index1 -= index2;
        assert_eq!(index1.row, 3);
        assert_eq!(index1.column, 4);
    }

    #[test]
    fn test_mul() {
        let index = GridIndex::new(2, 3);
        let result = index * 2;
        assert_eq!(result.row, 4);
        assert_eq!(result.column, 6);
    }

    #[test]
    fn test_mul_assign() {
        let mut index = GridIndex::new(2, 3);
        index *= 2;
        assert_eq!(index.row, 4);
        assert_eq!(index.column, 6);
    }

    #[test]
    fn test_div() {
        let index = GridIndex::new(6, 8);
        let result = index / 2;
        assert_eq!(result.row, 3);
        assert_eq!(result.column, 4);
    }

    #[test]
    fn test_div_assign() {
        let mut index = GridIndex::new(6, 8);
        index /= 2;
        assert_eq!(index.row, 3);
        assert_eq!(index.column, 4);
    }

    #[test]
    fn test_rem() {
        let index = GridIndex::new(5, 7);
        let result = index % 3;
        assert_eq!(result.row, 2);
        assert_eq!(result.column, 1);
    }

    #[test]
    fn test_rem_assign() {
        let mut index = GridIndex::new(5, 7);
        index %= 3;
        assert_eq!(index.row, 2);
        assert_eq!(index.column, 1);
    }

    #[test]
    fn test_neg() {
        let index = GridIndex::new(3, -4);
        let result = -index;
        assert_eq!(result.row, -3);
        assert_eq!(result.column, 4);
    }

    #[test]
    fn test_abs() {
        let index = GridIndex::new(-3, -4);
        let result = index.abs();
        assert_eq!(result.row, 3);
        assert_eq!(result.column, 4);
    }

    #[test]
    fn test_is_positive() {
        let index = GridIndex::new(3, 4);
        assert!(index.is_positive());
        let index = GridIndex::new(-3, 4);
        assert!(!index.is_positive());
    }

    #[test]
    fn test_is_negative() {
        let index = GridIndex::new(-3, -4);
        assert!(index.is_negative());
        let index = GridIndex::new(3, -4);
        assert!(index.is_negative());
        let index = GridIndex::new(3, 4);
        assert!(!index.is_negative());
    }
}
