use integer::Integer;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;
use natural::InnerNatural::Small;
use natural::Natural;
use std::mem::swap;
use std::ops::{Sub, SubAssign};

impl Sub<Integer> for Integer {
    type Output = Integer;

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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((Integer::ZERO - Integer::from(123)).to_string(), "-123");
    /// assert_eq!((Integer::from(123) - Integer::ZERO).to_string(), "123");
    /// assert_eq!((Integer::from(456) - Integer::from(-123)).to_string(), "579");
    /// assert_eq!(
    ///     (-Integer::trillion() - -Integer::trillion() * Integer::from(2u32)).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    #[inline]
    fn sub(mut self, other: Integer) -> Integer {
        self -= other;
        self
    }
}

impl<'a> Sub<&'a Integer> for Integer {
    type Output = Integer;

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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((Integer::ZERO - &Integer::from(123)).to_string(), "-123");
    /// assert_eq!((Integer::from(123) - &Integer::ZERO).to_string(), "123");
    /// assert_eq!((Integer::from(456) - &Integer::from(-123)).to_string(), "579");
    /// assert_eq!(
    ///     (-Integer::trillion() - &(-Integer::trillion() * Integer::from(2u32))).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    #[inline]
    fn sub(mut self, other: &'a Integer) -> Integer {
        self -= other;
        self
    }
}

impl<'a> Sub<Integer> for &'a Integer {
    type Output = Integer;

    /// Subtracts an `Integer` from an `Integer`, taking the left `Integer` by reference and the
    /// right `Integer` by value.
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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO - Integer::from(123)).to_string(), "-123");
    /// assert_eq!((&Integer::from(123) - Integer::ZERO).to_string(), "123");
    /// assert_eq!((&Integer::from(456) - Integer::from(-123)).to_string(), "579");
    /// assert_eq!(
    ///     (&(-Integer::trillion()) - -Integer::trillion() * Integer::from(2u32)).to_string(),
    ///     "1000000000000"
    /// );
    /// ```
    fn sub(self, mut other: Integer) -> Integer {
        other -= self;
        -other
    }
}

impl<'a, 'b> Sub<&'a Integer> for &'b Integer {
    type Output = Integer;

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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO - &Integer::from(123)).to_string(), "-123");
    /// assert_eq!((&Integer::from(123) - &Integer::ZERO).to_string(), "123");
    /// assert_eq!((&Integer::from(456) - &Integer::from(-123)).to_string(), "579");
    /// let x: Integer = &-Integer::trillion() - &(-Integer::trillion() * Integer::from(2u32));
    /// assert_eq!(x.to_string(), "1000000000000");
    /// ```
    fn sub(self, other: &'a Integer) -> Integer {
        match (self, other) {
            (x, y) if x as *const Integer == y as *const Integer => Integer::ZERO,
            (integer_zero!(), y) => -y.clone(),
            (x, &integer_zero!()) => x.clone(),
            // e.g. 10 - -5 or -10 - 5; sign of result is sign of self
            (
                &Integer {
                    sign: sx,
                    abs: ref ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (!sy && *ay != 0) => Integer {
                sign: sx,
                abs: ax + ay,
            },
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of result is sign of self
            (
                &Integer {
                    sign: sx,
                    abs: ref ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => Integer {
                sign: sx,
                abs: ax - ay,
            },
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of result is opposite of sign of other
            (
                &Integer { abs: ref ax, .. },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) => Integer {
                sign: !sy,
                abs: ay - ax,
            },
        }
    }
}

impl SubAssign<Integer> for Integer {
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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x -= -Integer::trillion();
    /// x -= Integer::trillion() * Integer::from(2u32);
    /// x -= -Integer::trillion() * Integer::from(3u32);
    /// x -= Integer::trillion() * Integer::from(4u32);
    /// assert_eq!(x.to_string(), "-2000000000000");
    /// ```
    fn sub_assign(&mut self, mut other: Integer) {
        match (&mut *self, &other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), _) => {
                *self = other;
                self.neg_assign();
            }
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (!sy && *ay != 0) => *ax += ay,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            // e.g. 5 - 10, -5 - -10, or -5 - -5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= other.abs;
                self.sign.not_assign();
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::ZERO;
/// x -= &(-Integer::trillion());
/// x -= &(Integer::trillion() * Integer::from(2u32));
/// x -= &(-Integer::trillion() * Integer::from(3u32));
/// x -= &(Integer::trillion() * Integer::from(4u32));
/// assert_eq!(x.to_string(), "-2000000000000");
/// ```
impl<'a> SubAssign<&'a Integer> for Integer {
    fn sub_assign(&mut self, other: &'a Integer) {
        match (&mut *self, other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), y) => *self = -y.clone(),
            // e.g. 10 - -5 or -10 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (!sy && *ay != 0) => *ax += ay,
            // e.g. 10 - 5, -10 - -5, or 5 - 5; sign of self is unchanged
            (
                &mut Integer {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Integer { abs: ref ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            (
                &mut Integer {
                    sign: ref mut sx,
                    abs: ref mut ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) => {
                *sx = !sy;
                *ax = ay - &*ax;
            }
        }
    }
}
