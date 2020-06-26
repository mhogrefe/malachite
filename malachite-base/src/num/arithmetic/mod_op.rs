use std::ops::{Neg, Rem, RemAssign, Sub};

use num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign, UnsignedAbs,
};
use num::basic::traits::Zero;
use num::conversion::traits::ExactFrom;

#[inline]
pub fn _neg_mod_unsigned<T: Copy + Eq + Zero>(x: T, other: T) -> T
where
    T: Rem<T, Output = T> + Sub<T, Output = T>,
{
    let remainder = x % other;
    if remainder == T::ZERO {
        T::ZERO
    } else {
        other - remainder
    }
}

#[inline]
pub fn _neg_mod_assign_unsigned<T: Copy + Eq + Zero>(x: &mut T, other: T)
where
    T: RemAssign<T> + Sub<T, Output = T>,
{
    *x %= other;
    if *x != T::ZERO {
        *x = other - *x;
    }
}

macro_rules! impl_mod_unsigned {
    ($t:ident) => {
        impl Mod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning the remainder. The quotient and
            /// remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::Mod;
            ///
            /// // 2 * 10 + 3 = 23
            /// assert_eq!(23u8.mod_op(10), 3);
            ///
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.mod_op(5), 0);
            /// ```
            #[inline]
            fn mod_op(self, other: $t) -> $t {
                self % other
            }
        }

        impl ModAssign<$t> for $t {
            /// Divides a value by another value in place, replacing `self` with the remainder. The
            /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= r < `other`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAssign;
            ///
            /// // 2 * 10 + 3 = 23
            /// let mut x = 23u8;
            /// x.mod_assign(10);
            /// assert_eq!(x, 3);
            ///
            /// // 9 * 5 + 0 = 45
            /// let mut x = 45u32;
            /// x.mod_assign(5);
            /// assert_eq!(x, 0);
            /// ```
            #[inline]
            fn mod_assign(&mut self, other: $t) {
                *self %= other;
            }
        }

        impl NegMod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning the remainder of the negative of the
            /// first value divided by the second. The quotient and remainder satisfy
            /// `self` = q * `other` - r and 0 <= r < `other`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::NegMod;
            ///
            /// // 3 * 10 - 7 = 23
            /// assert_eq!(23u8.neg_mod(10), 7);
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.neg_mod(5), 0);
            /// ```
            #[inline]
            fn neg_mod(self, other: $t) -> $t {
                _neg_mod_unsigned(self, other)
            }
        }

        impl NegModAssign<$t> for $t {
            /// Divides a value by another value in place, replacing `self` with the remainder of
            /// the negative of the first value divided by the second. The quotient and remainder
            /// satisfy `self` = q * `other` - r and 0 <= r < `other`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::NegModAssign;
            ///
            /// // 3 * 10 - 7 = 23
            /// let mut x = 23u8;
            /// x.neg_mod_assign(10);
            /// assert_eq!(x, 7);
            ///
            /// // 9 * 5 + 0 = 45
            /// let mut x = 45u32;
            /// x.neg_mod_assign(5);
            /// assert_eq!(x, 0);
            /// ```
            #[inline]
            fn neg_mod_assign(&mut self, other: $t) {
                _neg_mod_assign_unsigned(self, other);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_unsigned);

pub fn _mod_op_signed<U, T: Copy + Ord + Zero>(x: T, other: T) -> T
where
    T: ExactFrom<U> + Neg<Output = T> + UnsignedAbs<Output = U>,
    U: NegMod<U, Output = U> + Rem<U, Output = U>,
{
    let remainder = if (x >= T::ZERO) == (other >= T::ZERO) {
        x.unsigned_abs() % other.unsigned_abs()
    } else {
        x.unsigned_abs().neg_mod(other.unsigned_abs())
    };
    if other >= T::ZERO {
        T::exact_from(remainder)
    } else {
        -T::exact_from(remainder)
    }
}

#[inline]
pub fn _ceiling_mod_signed<U, T: Copy + Ord + Zero>(x: T, other: T) -> T
where
    T: ExactFrom<U> + Neg<Output = T> + UnsignedAbs<Output = U>,
    U: NegMod<U, Output = U> + Rem<U, Output = U>,
{
    let remainder = if (x >= T::ZERO) == (other >= T::ZERO) {
        x.unsigned_abs().neg_mod(other.unsigned_abs())
    } else {
        x.unsigned_abs() % other.unsigned_abs()
    };
    if other >= T::ZERO {
        -T::exact_from(remainder)
    } else {
        T::exact_from(remainder)
    }
}

macro_rules! impl_mod_signed {
    ($t:ident) => {
        impl Mod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning the remainder. The remainder has the
            /// same sign as the divisor. The quotient and remainder satisfy
            /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::Mod;
            ///
            /// // 2 * 10 + 3 = 23
            /// assert_eq!(23i8.mod_op(10), 3);
            ///
            /// // -3 * -10 + -7 = 23
            /// assert_eq!(23i16.mod_op(-10), -7);
            ///
            /// // -3 * 10 + 7 = -23
            /// assert_eq!((-23i32).mod_op(10), 7);
            ///
            /// // 2 * -10 + -3 = -23
            /// assert_eq!((-23i64).mod_op(-10), -3);
            /// ```
            #[inline]
            fn mod_op(self, other: $t) -> $t {
                _mod_op_signed(self, other)
            }
        }

        impl ModAssign<$t> for $t {
            /// Divides a value by another value in place, replacing `self` with the remainder. The
            /// remainder has the same sign as the divisor. The quotient and remainder satisfy
            /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAssign;
            ///
            /// // 2 * 10 + 3 = 23
            /// let mut x = 23i8;
            /// x.mod_assign(10);
            /// assert_eq!(x, 3);
            ///
            /// // -3 * -10 + -7 = 23
            /// let mut x = 23i16;
            /// x.mod_assign(-10);
            /// assert_eq!(x, -7);
            ///
            /// // -3 * 10 + 7 = -23
            /// let mut x = -23i32;
            /// x.mod_assign(10);
            /// assert_eq!(x, 7);
            ///
            /// // 2 * -10 + -3 = -23
            /// let mut x = -23i64;
            /// x.mod_assign(-10);
            /// assert_eq!(x, -3);
            /// ```
            #[inline]
            fn mod_assign(&mut self, other: $t) {
                *self = self.mod_op(other);
            }
        }

        impl CeilingMod<$t> for $t {
            type Output = $t;

            /// Divides a value by another value, returning the remainder. The remainder has the
            /// opposite sign of the divisor. The quotient and remainder satisfy
            /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingMod;
            ///
            /// // 3 * 10 + -7 = 23
            /// assert_eq!(23i8.ceiling_mod(10), -7);
            ///
            /// // -2 * -10 + 3 = 23
            /// assert_eq!(23i16.ceiling_mod(-10), 3);
            ///
            /// // -2 * 10 + -3 = -23
            /// assert_eq!((-23i32).ceiling_mod(10), -3);
            ///
            /// // 3 * -10 + 7 = -23
            /// assert_eq!((-23i64).ceiling_mod(-10), 7);
            /// ```
            #[inline]
            fn ceiling_mod(self, other: $t) -> $t {
                _ceiling_mod_signed(self, other)
            }
        }

        impl CeilingModAssign<$t> for $t {
            /// Divides a value by another value in place, taking the second `Integer` by value,
            /// replacing `self` with the remainder. The remainder has the opposite sign of the
            /// divisor. The quotient and remainder satisfy `self` = q * `other` + r and
            /// 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
            ///
            /// // 3 * 10 + -7 = 23
            /// let mut x = 23i8;
            /// x.ceiling_mod_assign(10);
            /// assert_eq!(x, -7);
            ///
            /// // -2 * -10 + 3 = 23
            /// let mut x = 23i16;
            /// x.ceiling_mod_assign(-10);
            /// assert_eq!(x, 3);
            ///
            /// // -2 * 10 + -3 = -23
            /// let mut x = -23i32;
            /// x.ceiling_mod_assign(10);
            /// assert_eq!(x, -3);
            ///
            /// // 3 * -10 + 7 = -23
            /// let mut x = -23i64;
            /// x.ceiling_mod_assign(-10);
            /// assert_eq!(x, 7);
            /// ```
            #[inline]
            fn ceiling_mod_assign(&mut self, other: $t) {
                *self = self.ceiling_mod(other);
            }
        }
    };
}
apply_to_signeds!(impl_mod_signed);
