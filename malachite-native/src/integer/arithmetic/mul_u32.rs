use integer::Integer;
use malachite_base::traits::{Assign, Zero};
use std::ops::{Mul, MulAssign};

/// Multiplies an `Integer` by a `u32`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Integer::zero() * 123u32).to_string(), "0");
///     assert_eq!((Integer::from(123i32) * 1u32).to_string(), "123");
///     assert_eq!((Integer::from(-123i32) * 456u32).to_string(), "-56088");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() * 123u32).to_string(),
///                 "-123000000000000");
/// }
/// ```
impl Mul<u32> for Integer {
    type Output = Integer;

    fn mul(mut self, other: u32) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by a `u32`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::zero() * 123u32).to_string(), "0");
///     assert_eq!((&Integer::from(123i32) * 1u32).to_string(), "123");
///     assert_eq!((&Integer::from(-123i32) * 456u32).to_string(), "-56088");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() * 123u32).to_string(),
///                "-123000000000000");
/// }
/// ```
impl<'a> Mul<u32> for &'a Integer {
    type Output = Integer;

    fn mul(self, other: u32) -> Integer {
        if *self == 0 || other == 0 {
            Integer::zero()
        } else {
            Integer {
                sign: self.sign,
                abs: &self.abs * other,
            }
        }
    }
}

/// Multiplies a `u32` by an `Integer`, taking the `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123u32 * Integer::zero()).to_string(), "0");
///     assert_eq!((1u32 * Integer::from(123i32)).to_string(), "123");
///     assert_eq!((456u32 * Integer::from(-123i32)).to_string(), "-56088");
///     assert_eq!((123u32 * Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-123000000000000");
/// }
/// ```
impl Mul<Integer> for u32 {
    type Output = Integer;

    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

/// Multiplies a `u32` by an `Integer`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123u32 * &Integer::zero()).to_string(), "0");
///     assert_eq!((1u32 * &Integer::from(123i32)).to_string(), "123");
///     assert_eq!((456u32 * &Integer::from(-123i32)).to_string(), "-56088");
///     assert_eq!((123u32 * &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-123000000000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for u32 {
    type Output = Integer;

    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}

/// Multiplies an `Integer` by a `u32` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::NegativeOne;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::negative_one();
///     x *= 1u32;
///     x *= 2u32;
///     x *= 3u32;
///     x *= 4u32;
///     assert_eq!(x.to_string(), "-24");
/// }
/// ```
impl MulAssign<u32> for Integer {
    fn mul_assign(&mut self, other: u32) {
        if *self == 0 || other == 0 {
            self.assign(0u32);
        } else {
            self.abs *= other;
        }
    }
}
