use integer::Integer;
use malachite_base::traits::{NegAssign, Zero};
use std::mem::swap;
use std::ops::{Sub, SubAssign};

/// Subtracts an `Integer` from an `Integer`, taking both `Integer`s by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
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
///     assert_eq!((Integer::ZERO - Integer::from(123)).to_string(), "-123");
///     assert_eq!((Integer::from(123) - Integer::ZERO).to_string(), "123");
///     assert_eq!((Integer::from(456) - Integer::from(-123)).to_string(), "579");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() -
///                 Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl Sub<Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, other: Integer) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts an `Integer` from an `Integer`, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
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
///     assert_eq!((Integer::ZERO - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((Integer::from(123) - &Integer::ZERO).to_string(), "123");
///     assert_eq!((Integer::from(456) - &Integer::from(-123)).to_string(), "579");
///     assert_eq!((Integer::from_str("-1000000000000").unwrap() -
///                 &Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl<'a> Sub<&'a Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, other: &'a Integer) -> Integer {
        self -= other;
        self
    }
}

/// Subtracts an `Integer` from an `Integer`, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
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
///     assert_eq!((&Integer::ZERO - Integer::from(123)).to_string(), "-123");
///     assert_eq!((&Integer::from(123) - Integer::ZERO).to_string(), "123");
///     assert_eq!((&Integer::from(456) - Integer::from(-123)).to_string(), "579");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() -
///                 Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl<'a> Sub<Integer> for &'a Integer {
    type Output = Integer;

    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

/// Subtracts an `Integer` from an `Integer`, taking both `Integer`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
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
///     assert_eq!((&Integer::ZERO - &Integer::from(123)).to_string(), "-123");
///     assert_eq!((&Integer::from(123) - &Integer::ZERO).to_string(), "123");
///     assert_eq!((&Integer::from(456) - &Integer::from(-123)).to_string(), "579");
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap() -
///                 &Integer::from_str("-2000000000000")
///                .unwrap()).to_string(), "1000000000000");
/// }
/// ```
impl<'a, 'b> Sub<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn sub(self, other: &'a Integer) -> Integer {
        if self as *const Integer == other as *const Integer {
            Integer::ZERO
        } else if *self == 0 {
            -other.clone()
        } else if *other == 0 {
            self.clone()
        } else {
            match (self, other) {
                // e.g. 10 - -5 or -10 - 5; sign of result is sign of self
                (&Integer {
                     sign: sx,
                     abs: ref ax,
                 },
                 &Integer {
                     sign: sy,
                     abs: ref ay,
                 }) if sx == (!sy && *ay != 0) => Integer {
                    sign: sx,
                    abs: ax + ay,
                },
                // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of result is sign of self
                (&Integer {
                     sign: sx,
                     abs: ref ax,
                 },
                 &Integer { abs: ref ay, .. }) if sx && *ax == *ay || *ax > *ay => Integer {
                    sign: sx,
                    abs: (ax - ay).unwrap(),
                },
                // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of result is opposite of sign of other
                (&Integer { abs: ref ax, .. },
                 &Integer {
                     sign: sy,
                     abs: ref ay,
                 }) => Integer {
                    sign: !sy,
                    abs: (ay - ax).unwrap(),
                },
            }
        }
    }
}

/// Subtracts an `Integer` from an `Integer` in place, taking the `Integer` on the RHS by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
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
///     let mut x = Integer::ZERO;
///     x -= Integer::from_str("-1000000000000").unwrap();
///     x -= Integer::from_str("2000000000000").unwrap();
///     x -= Integer::from_str("-3000000000000").unwrap();
///     x -= Integer::from_str("4000000000000").unwrap();
///     assert_eq!(x.to_string(), "-2000000000000");
/// }
/// ```
impl SubAssign<Integer> for Integer {
    fn sub_assign(&mut self, mut other: Integer) {
        if other == 0 {
            return;
        } else if *self == 0 {
            *self = other;
            self.neg_assign();
            return;
        }
        let add_strategy = match (&mut (*self), &other) {
            (&mut Integer { sign: sx, .. },
             &Integer {
                 sign: sy,
                 abs: ref ay,
             }) if sx == (!sy && *ay != 0) => 0,
            (&mut Integer {
                 sign: sx,
                 abs: ref mut ax,
             },
             &Integer { abs: ref ay, .. }) if sx && *ax == *ay || *ax > *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            0 => self.abs += other.abs,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            1 => self.abs -= &other.abs,
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= &other.abs;
                self.sign = !self.sign;
            }
        }
    }
}

/// Subtracts an `Integer` from an `Integer` in place, taking the `Integer` on the RHS by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
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
///     let mut x = Integer::ZERO;
///     x -= &Integer::from_str("-1000000000000").unwrap();
///     x -= &Integer::from_str("2000000000000").unwrap();
///     x -= &Integer::from_str("-3000000000000").unwrap();
///     x -= &Integer::from_str("4000000000000").unwrap();
///     assert_eq!(x.to_string(), "-2000000000000");
/// }
/// ```
impl<'a> SubAssign<&'a Integer> for Integer {
    fn sub_assign(&mut self, other: &'a Integer) {
        if *other == 0 {
            return;
        } else if *self == 0 {
            *self = -other.clone();
            return;
        }
        let add_strategy = match (&mut (*self), other) {
            (&mut Integer { sign: sx, .. },
             &Integer {
                 sign: sy,
                 abs: ref ay,
             }) if sx == (!sy && *ay != 0) => 0,
            (&mut Integer {
                 sign: sx,
                 abs: ref mut ax,
             },
             &Integer { abs: ref ay, .. }) if sx && *ax == *ay || *ax > *ay => 1,
            _ => 2,
        };
        match add_strategy {
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            0 => self.abs += &other.abs,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            1 => self.abs -= &other.abs,
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of self is flipped
            _ => {
                *self = Integer {
                    sign: !other.sign,
                    abs: (&other.abs - &self.abs).unwrap(),
                }
            }
        }
    }
}
