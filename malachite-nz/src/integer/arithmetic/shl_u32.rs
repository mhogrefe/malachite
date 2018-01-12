use integer::Integer;
use std::ops::{Shl, ShlAssign};

/// Shifts a `Integer` left (multiplies it by a power of 2), taking the `Integer` by value.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO << 10u32).to_string(), "0");
///     assert_eq!((Integer::from(123) << 2u32).to_string(), "492");
///     assert_eq!((Integer::from(123) << 100u32).to_string(), "155921023828072216384094494261248");
///     assert_eq!((Integer::from(-123) << 2u32).to_string(), "-492");
///     assert_eq!((Integer::from(-123) << 100u32).to_string(),
///         "-155921023828072216384094494261248");
/// }
/// ```
impl Shl<u32> for Integer {
    type Output = Integer;

    fn shl(self, other: u32) -> Integer {
        Integer {
            sign: self.sign,
            abs: self.abs << other,
        }
    }
}

/// Shifts a `Integer` left (multiplies it by a power of 2), taking the `Integer` by reference.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO << 10u32).to_string(), "0");
///     assert_eq!((&Integer::from(123) << 2u32).to_string(), "492");
///     assert_eq!((&Integer::from(123) << 100u32).to_string(),
///         "155921023828072216384094494261248");
///     assert_eq!((&Integer::from(-123) << 2u32).to_string(), "-492");
///     assert_eq!((&Integer::from(-123) << 100u32).to_string(),
///         "-155921023828072216384094494261248");
/// }
/// ```
impl<'a> Shl<u32> for &'a Integer {
    type Output = Integer;

    fn shl(self, other: u32) -> Integer {
        Integer {
            sign: self.sign,
            abs: &self.abs << other,
        }
    }
}

/// Shifts a `Integer` left (multiplies it by a power of 2) in place.
///
/// Time: worst case O(`other`)
///
/// Additional memory: worst case O(`other`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ONE;
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "1024");
///     let mut x = Integer::NEGATIVE_ONE;
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "-1024");
/// }
/// ```
impl ShlAssign<u32> for Integer {
    fn shl_assign(&mut self, other: u32) {
        self.abs <<= other;
    }
}
