use num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem, UnsignedAbs,
};
use num::conversion::traits::{ExactFrom, WrappingFrom};

macro_rules! impl_div_mod_unsigned {
    ($t:ident) => {
        impl DivMod for $t {
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
                let q = self / other;
                (q, self - q * other)
            }
        }

        impl DivAssignMod for $t {
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
                let original = *self;
                *self /= other;
                original - *self * other
            }
        }

        impl DivRem for $t {
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
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.div_rem(5), (9, 0));
            /// ```
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                self.div_mod(other)
            }
        }

        impl DivAssignRem for $t {
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

        impl CeilingDivNegMod for $t {
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
            ///
            /// // 9 * 5 + 0 = 45
            /// assert_eq!(45u32.ceiling_div_neg_mod(5), (9, 0));
            /// ```
            #[inline]
            fn ceiling_div_neg_mod(self, other: $t) -> ($t, $t) {
                let (quotient, remainder) = self.div_mod(other);
                if remainder == 0 {
                    (quotient, 0)
                } else {
                    // Here remainder != 0, so other > 1, so quotient < $t::MAX.
                    (quotient + 1, other - remainder)
                }
            }
        }

        impl CeilingDivAssignNegMod for $t {
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
                let remainder = self.div_assign_mod(other);
                if remainder == 0 {
                    0
                } else {
                    // Here remainder != 0, so other > 1, so self < $t::MAX.
                    *self += 1;
                    other - remainder
                }
            }
        }
    };
}
impl_div_mod_unsigned!(u8);
impl_div_mod_unsigned!(u16);
impl_div_mod_unsigned!(u32);
impl_div_mod_unsigned!(u64);
impl_div_mod_unsigned!(u128);
impl_div_mod_unsigned!(usize);

macro_rules! impl_div_mod_signed {
    ($t:ident) => {
        impl DivMod for $t {
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
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    ($t::exact_from(quotient), remainder)
                } else {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    ($t::wrapping_from(quotient).wrapping_neg(), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        $t::exact_from(remainder)
                    } else {
                        -$t::exact_from(remainder)
                    },
                )
            }
        }

        impl DivAssignMod for $t {
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

        impl DivRem for $t {
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
                let q = self.checked_div(other).unwrap();
                (q, self - q * other)
            }
        }

        impl DivAssignRem for $t {
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
                let original = *self;
                *self = self.checked_div(other).unwrap();
                original - *self * other
            }
        }

        impl CeilingDivMod for $t {
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
                let (quotient, remainder) = if (self >= 0) == (other >= 0) {
                    let (quotient, remainder) = self
                        .unsigned_abs()
                        .ceiling_div_neg_mod(other.unsigned_abs());
                    ($t::exact_from(quotient), remainder)
                } else {
                    let (quotient, remainder) = self.unsigned_abs().div_mod(other.unsigned_abs());
                    ($t::wrapping_from(quotient).wrapping_neg(), remainder)
                };
                (
                    quotient,
                    if other >= 0 {
                        -$t::exact_from(remainder)
                    } else {
                        $t::exact_from(remainder)
                    },
                )
            }
        }

        impl CeilingDivAssignMod for $t {
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
impl_div_mod_signed!(i8);
impl_div_mod_signed!(i16);
impl_div_mod_signed!(i32);
impl_div_mod_signed!(i64);
impl_div_mod_signed!(i128);
impl_div_mod_signed!(isize);
