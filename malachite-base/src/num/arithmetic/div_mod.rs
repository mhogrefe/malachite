use num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, CheckedDiv,
    DivAssignMod, DivAssignRem, DivMod, DivRem, UnsignedAbs, WrappingNeg,
};
use num::basic::traits::{One, Zero};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub};

fn _div_mod_unsigned<T: Copy + Div<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T>>(
    x: T,
    other: T,
) -> (T, T) {
    let q = x / other;
    (q, x - q * other)
}

fn _div_assign_mod_unsigned<T: Copy + DivAssign<T> + Mul<T, Output = T> + Sub<T, Output = T>>(
    x: &mut T,
    other: T,
) -> T {
    let original = *x;
    *x /= other;
    original - *x * other
}

fn _ceiling_div_neg_mod_unsigned<
    T: Add<T, Output = T>
        + Copy
        + DivMod<T, DivOutput = T, ModOutput = T>
        + Eq
        + One
        + Sub<T, Output = T>
        + Zero,
>(
    x: T,
    other: T,
) -> (T, T) {
    let (quotient, remainder) = x.div_mod(other);
    if remainder == T::ZERO {
        (quotient, T::ZERO)
    } else {
        // Here remainder != 0, so other > 1, so quotient < T::MAX.
        (quotient + T::ONE, other - remainder)
    }
}

fn _ceiling_div_assign_neg_mod_unsigned<
    T: AddAssign<T> + Copy + DivAssignMod<T, ModOutput = T> + Eq + One + Sub<T, Output = T> + Zero,
>(
    x: &mut T,
    other: T,
) -> T {
    let remainder = x.div_assign_mod(other);
    if remainder == T::ZERO {
        T::ZERO
    } else {
        // Here remainder != 0, so other > 1, so self < T::MAX.
        *x += T::ONE;
        other - remainder
    }
}

macro_rules! impl_div_mod_unsigned {
    ($t:ident) => {
        impl DivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a value by another value, returning the quotient and remainder. The quotient
            /// is rounded towards negative infinity. The quotient and remainder satisfy
            /// `self` = q * `other` + r and 0 <= r < `other`.
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
            /// use malachite_base::num::arithmetic::traits::DivMod;
            ///
            /// // 2 * 10 + 3 = 23
            /// assert_eq!(23u8.div_mod(10), (2, 3));
            ///
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.div_mod(5), (9, 0));
            /// ```
            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                _div_mod_unsigned(self, other)
            }
        }

        impl DivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a value by another value in place, returning the remainder. The quotient is
            /// rounded towards negative infinity. The quotient and remainder satisfy
            /// `self` = q * `other` + r and 0 <= r < `other`.
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
            /// use malachite_base::num::arithmetic::traits::DivAssignMod;
            ///
            /// // 2 * 10 + 3 = 23
            /// let mut x = 23u8;
            /// assert_eq!(x.div_assign_mod(10), 3);
            /// assert_eq!(x, 2);
            ///
            /// // 9 * 5 + 0 = 45
            /// let mut x = 45u32;
            /// assert_eq!(x.div_assign_mod(5), 0);
            /// assert_eq!(x, 9);
            /// ```
            #[inline]
            fn div_assign_mod(&mut self, other: $t) -> $t {
                _div_assign_mod_unsigned(self, other)
            }
        }

        impl DivRem<$t> for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            /// Divides a value by another value, returning the quotient and remainder. The quotient
            /// is rounded towards zero. The quotient and remainder satisfy `self` = q * `other` + r
            /// and 0 <= r < `other`. For unsigned integers, rem is equivalent to mod.
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
            /// use malachite_base::num::arithmetic::traits::DivRem;
            ///
            /// // 2 * 10 + 3 = 23
            /// assert_eq!(23u8.div_rem(10), (2, 3));
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.div_rem(5), (9, 0));
            /// ```
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                self.div_mod(other)
            }
        }

        impl DivAssignRem<$t> for $t {
            type RemOutput = $t;

            /// Divides a value by another value in place, returning the remainder. The quotient is
            /// rounded towards zero. The quotient and remainder satisfy `self` = q * `other` + r
            /// and 0 <= r < `other`. For unsigned integers, rem is equivalent to mod.
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
            /// use malachite_base::num::arithmetic::traits::DivAssignRem;
            ///
            /// // 2 * 10 + 3 = 23
            /// let mut x = 23u8;
            /// assert_eq!(x.div_assign_rem(10), 3);
            /// assert_eq!(x, 2);
            ///
            /// // 9 * 5 + 0 = 45
            /// let mut x = 45u32;
            /// assert_eq!(x.div_assign_rem(5), 0);
            /// assert_eq!(x, 9);
            /// ```
            #[inline]
            fn div_assign_rem(&mut self, other: $t) -> $t {
                self.div_assign_mod(other)
            }
        }

        impl CeilingDivNegMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a value by another value, returning the ceiling of the quotient and the
            /// remainder of the negative of the first value divided by the second. The quotient and
            /// remainder satisfy `self` = q * `other` - r and 0 <= r < `other`.
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
            /// use malachite_base::num::arithmetic::traits::CeilingDivNegMod;
            ///
            /// // 3 * 10 - 7 = 23
            /// assert_eq!(23u8.ceiling_div_neg_mod(10), (3, 7));
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.ceiling_div_neg_mod(5), (9, 0));
            /// ```
            #[inline]
            fn ceiling_div_neg_mod(self, other: $t) -> ($t, $t) {
                _ceiling_div_neg_mod_unsigned(self, other)
            }
        }

        impl CeilingDivAssignNegMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a value by another value in place, returning the remainder of the negative
            /// of the first value divided by the second. The quotient and remainder satisfy
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
            /// use malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod;
            ///
            /// // 3 * 10 - 7 = 23
            /// let mut x = 23u8;
            /// assert_eq!(x.ceiling_div_assign_neg_mod(10), 7);
            /// assert_eq!(x, 3);
            ///
            /// // 9 * 5 + 0 = 45
            /// let mut x = 45u32;
            /// assert_eq!(x.ceiling_div_assign_neg_mod(5), 0);
            /// assert_eq!(x, 9);
            /// ```
            #[inline]
            fn ceiling_div_assign_neg_mod(&mut self, other: $t) -> $t {
                _ceiling_div_assign_neg_mod_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_div_mod_unsigned);

fn _div_mod_signed<
    U: CeilingDivNegMod<U, DivOutput = U, ModOutput = U> + DivMod<U, DivOutput = U, ModOutput = U>,
    S: Copy
        + ExactFrom<U>
        + Neg<Output = S>
        + Ord
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>
        + WrappingNeg<Output = S>
        + Zero,
>(
    x: S,
    other: S,
) -> (S, S) {
    let (quotient, remainder) = if (x >= S::ZERO) == (other >= S::ZERO) {
        let (quotient, remainder) = x.unsigned_abs().div_mod(other.unsigned_abs());
        (S::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) = x.unsigned_abs().ceiling_div_neg_mod(other.unsigned_abs());
        (S::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= S::ZERO {
            S::exact_from(remainder)
        } else {
            -S::exact_from(remainder)
        },
    )
}

fn _div_rem_signed<
    T: CheckedDiv<T, Output = T> + Copy + Mul<T, Output = T> + Sub<T, Output = T>,
>(
    x: T,
    other: T,
) -> (T, T) {
    let q = x.checked_div(other).unwrap();
    (q, x - q * other)
}

fn _div_assign_rem_signed<
    T: CheckedDiv<T, Output = T> + Copy + Mul<T, Output = T> + Sub<T, Output = T>,
>(
    x: &mut T,
    other: T,
) -> T {
    let original = *x;
    *x = x.checked_div(other).unwrap();
    original - *x * other
}

fn _ceiling_div_mod_signed<
    U: CeilingDivNegMod<U, DivOutput = U, ModOutput = U> + DivMod<U, DivOutput = U, ModOutput = U>,
    T: Copy
        + ExactFrom<U>
        + Neg<Output = T>
        + Ord
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>
        + WrappingNeg<Output = T>
        + Zero,
>(
    x: T,
    other: T,
) -> (T, T) {
    let (quotient, remainder) = if (x >= T::ZERO) == (other >= T::ZERO) {
        let (quotient, remainder) = x.unsigned_abs().ceiling_div_neg_mod(other.unsigned_abs());
        (T::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) = x.unsigned_abs().div_mod(other.unsigned_abs());
        (T::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= T::ZERO {
            -T::exact_from(remainder)
        } else {
            T::exact_from(remainder)
        },
    )
}

macro_rules! impl_div_mod_signed {
    ($t:ident) => {
        impl DivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a value by another value, returning the quotient and remainder. The quotient
            /// is rounded towards negative infinity, and the remainder has the same sign as the
            /// divisor. The quotient and remainder satisfy `self` = q * `other` + r and
            /// 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivMod;
            ///
            /// // 2 * 10 + 3 = 23
            /// assert_eq!(23i8.div_mod(10), (2, 3));
            ///
            /// // -3 * -10 + -7 = 23
            /// assert_eq!(23i16.div_mod(-10), (-3, -7));
            ///
            /// // -3 * 10 + 7 = -23
            /// assert_eq!((-23i32).div_mod(10), (-3, 7));
            ///
            /// // 2 * -10 + -3 = -23
            /// assert_eq!((-23i64).div_mod(-10), (2, -3));
            /// ```
            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                _div_mod_signed(self, other)
            }
        }

        impl DivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a value by another value in place, returning the remainder. The quotient is
            /// rounded towards negative infinity, and the remainder has the same sign as the
            /// divisor. The quotient and remainder satisfy `self` = q * `other` + r and
            /// 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivAssignMod;
            ///
            /// // 2 * 10 + 3 = 23
            /// let mut x = 23i8;
            /// assert_eq!(x.div_assign_mod(10), 3);
            /// assert_eq!(x, 2);
            ///
            /// // -3 * -10 + -7 = 23
            /// let mut x = 23i16;
            /// assert_eq!(x.div_assign_mod(-10), -7);
            /// assert_eq!(x, -3);
            ///
            /// // -3 * 10 + 7 = -23
            /// let mut x = -23i32;
            /// assert_eq!(x.div_assign_mod(10), 7);
            /// assert_eq!(x, -3);
            ///
            /// // 2 * -10 + -3 = -23
            /// let mut x = -23i64;
            /// assert_eq!(x.div_assign_mod(-10), -3);
            /// assert_eq!(x, 2);
            /// ```
            #[inline]
            fn div_assign_mod(&mut self, other: $t) -> $t {
                let (q, r) = self.div_mod(other);
                *self = q;
                r
            }
        }

        impl DivRem<$t> for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            /// Divides a value by another value, returning the quotient and remainder. The quotient
            /// is rounded towards zero and the remainder has the same sign as the dividend. The
            /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivRem;
            ///
            /// // 2 * 10 + 3 = 23
            /// assert_eq!(23i8.div_rem(10), (2, 3));
            ///
            /// // -2 * -10 + 3 = 23
            /// assert_eq!(23i16.div_rem(-10), (-2, 3));
            ///
            /// // -2 * 10 + -3 = -23
            /// assert_eq!((-23i32).div_rem(10), (-2, -3));
            ///
            /// // 2 * -10 + -3 = -23
            /// assert_eq!((-23i64).div_rem(-10), (2, -3));
            /// ```
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                _div_rem_signed(self, other)
            }
        }

        impl DivAssignRem<$t> for $t {
            type RemOutput = $t;

            /// Divides a value by another value in place, returning the remainder. The quotient is
            /// rounded towards zero and the remainder has the same sign as the dividend. The
            /// quotient and remainder satisfy `self` = q * `other` + r and 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::DivAssignRem;
            ///
            /// // 2 * 10 + 3 = 23
            /// let mut x = 23i8;
            /// assert_eq!(x.div_assign_rem(10), 3);
            /// assert_eq!(x, 2);
            ///
            /// // -2 * -10 + 3 = 23
            /// let mut x = 23i16;
            /// assert_eq!(x.div_assign_rem(-10), 3);
            /// assert_eq!(x, -2);
            ///
            /// // -2 * 10 + -3 = -23
            /// let mut x = -23i32;
            /// assert_eq!(x.div_assign_rem(10), -3);
            /// assert_eq!(x, -2);
            ///
            /// // 2 * -10 + -3 = -23
            /// let mut x = -23i64;
            /// assert_eq!(x.div_assign_rem(-10), -3);
            /// assert_eq!(x, 2);
            /// ```
            #[inline]
            fn div_assign_rem(&mut self, other: $t) -> $t {
                _div_assign_rem_signed(self, other)
            }
        }

        impl CeilingDivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a value by another value, returning the quotient and remainder. The quotient
            /// is rounded towards positive infinity and the remainder has the opposite sign of the
            /// divisor. The quotient and remainder satisfy `self` = q * `other` + r and
            /// 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
            ///
            /// // 3 * 10 + -7 = 23
            /// assert_eq!(23i8.ceiling_div_mod(10), (3, -7));
            ///
            /// // -2 * -10 + 3 = 23
            /// assert_eq!(23i16.ceiling_div_mod(-10), (-2, 3));
            ///
            /// // -2 * 10 + -3 = -23
            /// assert_eq!((-23i32).ceiling_div_mod(10), (-2, -3));
            ///
            /// // 3 * -10 + 7 = -23
            /// assert_eq!((-23i64).ceiling_div_mod(-10), (3, 7));
            /// ```
            #[inline]
            fn ceiling_div_mod(self, other: $t) -> ($t, $t) {
                _ceiling_div_mod_signed(self, other)
            }
        }

        impl CeilingDivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a value by another value in place, taking the second `Integer` by value,
            /// returning the remainder. The quotient is rounded towards positive infinity and the
            /// remainder has the opposite sign of the divisor. The quotient and remainder satisfy
            /// `self` = q * `other` + r and 0 <= |r| < |`other`|.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingDivAssignMod;
            ///
            /// // 3 * 10 + -7 = 23
            /// let mut x = 23i8;
            /// assert_eq!(x.ceiling_div_assign_mod(10), -7);
            /// assert_eq!(x, 3);
            ///
            /// // -2 * -10 + 3 = 23
            /// let mut x = 23i16;
            /// assert_eq!(x.ceiling_div_assign_mod(-10), 3);
            /// assert_eq!(x, -2);
            ///
            /// // -2 * 10 + -3 = -23
            /// let mut x = -23i32;
            /// assert_eq!(x.ceiling_div_assign_mod(10), -3);
            /// assert_eq!(x, -2);
            ///
            /// // 3 * -10 + 7 = -23
            /// let mut x = -23i64;
            /// assert_eq!(x.ceiling_div_assign_mod(-10), 7);
            /// assert_eq!(x, 3);
            /// ```
            #[inline]
            fn ceiling_div_assign_mod(&mut self, other: $t) -> $t {
                let (q, r) = self.ceiling_div_mod(other);
                *self = q;
                r
            }
        }
    };
}
apply_to_signeds!(impl_div_mod_signed);
