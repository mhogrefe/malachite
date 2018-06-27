use integer::Integer;
use malachite_base::num::{Assign, NotAssign, UnsignedAbs, Zero};
use std::ops::{Mul, MulAssign};

/// Multiplies an `Integer` by an `i32`, taking the `Integer` by value.
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((Integer::ZERO * 123i32).to_string(), "0");
///     assert_eq!((Integer::from(123i32) * 1i32).to_string(), "123");
///     assert_eq!((Integer::from(123i32) * -456i32).to_string(), "-56088");
///     assert_eq!((-Integer::trillion() * 123i32).to_string(), "-123000000000000");
/// }
/// ```
impl Mul<i32> for Integer {
    type Output = Integer;

    fn mul(mut self, other: i32) -> Integer {
        self *= other;
        self
    }
}

/// Multiplies an `Integer` by an `i32`, taking the `Integer` by reference.
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::ZERO * 123i32).to_string(), "0");
///     assert_eq!((&Integer::from(123i32) * 1i32).to_string(), "123");
///     assert_eq!((&Integer::from(123i32) * -456i32).to_string(), "-56088");
///     assert_eq!((&(-Integer::trillion()) * 123i32).to_string(), "-123000000000000");
/// }
/// ```
impl<'a> Mul<i32> for &'a Integer {
    type Output = Integer;

    fn mul(self, other: i32) -> Integer {
        if *self == 0 || other == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: if other > 0 {
                    self.sign
                } else {
                    !self.sign
                },
                abs: &self.abs * other.unsigned_abs(),
            }
        }
    }
}

/// Multiplies an `i32` by an `Integer`, taking the `Integer` by value.
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((123i32 * Integer::ZERO).to_string(), "0");
///     assert_eq!((1i32 * Integer::from(123i32)).to_string(), "123");
///     assert_eq!((-456i32 * Integer::from(123i32)).to_string(), "-56088");
///     assert_eq!((123i32 * -Integer::trillion()).to_string(), "-123000000000000");
/// }
/// ```
impl Mul<Integer> for i32 {
    type Output = Integer;

    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

/// Multiplies an `i32` by an `Integer`, taking the `Integer` by reference.
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((123i32 * &Integer::ZERO).to_string(), "0");
///     assert_eq!((1i32 * &Integer::from(123i32)).to_string(), "123");
///     assert_eq!((-456i32 * &Integer::from(123i32)).to_string(), "-56088");
///     assert_eq!((123i32 * &(-Integer::trillion())).to_string(), "-123000000000000");
/// }
/// ```
impl<'a> Mul<&'a Integer> for i32 {
    type Output = Integer;

    fn mul(self, other: &'a Integer) -> Integer {
        other * self
    }
}

/// Multiplies an `Integer` by an `i32` in place.
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::NEGATIVE_ONE;
///     x *= -1i32;
///     x *= -2i32;
///     x *= -3i32;
///     x *= -4i32;
///     assert_eq!(x.to_string(), "-24");
/// }
/// ```
impl MulAssign<i32> for Integer {
    fn mul_assign(&mut self, other: i32) {
        if *self == 0 || other == 0 {
            self.assign(0i32);
        } else {
            self.abs *= other.unsigned_abs();
            if other < 0 {
                self.sign.not_assign();
            }
        }
    }
}
