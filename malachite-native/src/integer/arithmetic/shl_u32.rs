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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::zero() << 10).to_string(), "0");
///     assert_eq!((Integer::from(123) << 2).to_string(), "492");
///     assert_eq!((Integer::from(123) << 100).to_string(), "155921023828072216384094494261248");
///     assert_eq!((Integer::from(-123) << 2).to_string(), "-492");
///     assert_eq!((Integer::from(-123) << 100).to_string(), "-155921023828072216384094494261248");
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::zero() << 10).to_string(), "0");
///     assert_eq!((&Integer::from(123) << 2).to_string(), "492");
///     assert_eq!((&Integer::from(123) << 100).to_string(), "155921023828072216384094494261248");
///     assert_eq!((&Integer::from(-123) << 2).to_string(), "-492");
///     assert_eq!((&Integer::from(-123) << 100).to_string(), "-155921023828072216384094494261248");
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::{NegativeOne, One};
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::one();
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "1024");
///     let mut x = Integer::negative_one();
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
