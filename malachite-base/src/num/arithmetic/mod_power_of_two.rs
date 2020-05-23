use num::arithmetic::traits::{
    CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, ModPowerOfTwo, ModPowerOfTwoAssign,
    NegModPowerOfTwo, NegModPowerOfTwoAssign, RemPowerOfTwo, RemPowerOfTwoAssign,
};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::{CheckedFrom, WrappingFrom};
use num::logic::traits::LowMask;

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

macro_rules! impl_mod_power_of_two_unsigned {
    ($t:ident) => {
        impl ModPowerOfTwo for $t {
            type Output = $t;

            /// Calculates `self` mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260u16.mod_power_of_two(8), 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// assert_eq!(1611u32.mod_power_of_two(4), 11);
            /// ```
            #[inline]
            fn mod_power_of_two(self, pow: u64) -> $t {
                if self == 0 || pow >= $t::WIDTH {
                    self
                } else {
                    self & $t::low_mask(pow)
                }
            }
        }

        impl ModPowerOfTwoAssign for $t {
            /// Reduces `self` mod a power of 2. In other words, replaces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAssign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260u16;
            /// x.mod_power_of_two_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// let mut x = 1611u32;
            /// x.mod_power_of_two_assign(4);
            /// assert_eq!(x, 11);
            /// ```
            #[inline]
            fn mod_power_of_two_assign(&mut self, pow: u64) {
                if *self != 0 && pow < $t::WIDTH {
                    *self &= $t::low_mask(pow)
                }
            }
        }

        impl RemPowerOfTwo for $t {
            type Output = $t;

            /// Calculates `self` rem a power of 2. For unsigned integers, rem is equivalent to mod.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOfTwo;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260u16.rem_power_of_two(8), 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// assert_eq!(1611u32.rem_power_of_two(4), 11);
            /// ```
            #[inline]
            fn rem_power_of_two(self, pow: u64) -> $t {
                self.mod_power_of_two(pow)
            }
        }

        impl RemPowerOfTwoAssign for $t {
            /// Reduces `self` rem a power of 2. For unsigned integers, rem is equivalent to mod.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOfTwoAssign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260u16;
            /// x.rem_power_of_two_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// let mut x = 1611u32;
            /// x.rem_power_of_two_assign(4);
            /// assert_eq!(x, 11);
            /// ```
            #[inline]
            fn rem_power_of_two_assign(&mut self, pow: u64) {
                self.mod_power_of_two_assign(pow)
            }
        }

        impl NegModPowerOfTwo for $t {
            type Output = $t;

            /// Calculates `-self` mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> - r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `$t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::NegModPowerOfTwo;
            ///
            /// // 2 * 2^8 - 252 = 260
            /// assert_eq!(260u16.neg_mod_power_of_two(8), 252);
            ///
            /// // 101 * 2^4 - 5 = 1611
            /// assert_eq!(1611u32.neg_mod_power_of_two(4), 5);
            /// ```
            #[inline]
            fn neg_mod_power_of_two(self, pow: u64) -> $t {
                if self != 0 && pow > $t::WIDTH {
                    panic!(ERROR_MESSAGE);
                }
                self.wrapping_neg().mod_power_of_two(pow)
            }
        }

        impl NegModPowerOfTwoAssign for $t {
            /// Reduces `-self` mod a power of 2. In other words, replaces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> - r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `$t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::NegModPowerOfTwoAssign;
            ///
            /// // 2 * 2^8 - 252 = 260
            /// let mut x = 260u16;
            /// x.neg_mod_power_of_two_assign(8);
            /// assert_eq!(x, 252);
            ///
            /// // 101 * 2^4 - 5 = 1611
            /// let mut x = 1611u32;
            /// x.neg_mod_power_of_two_assign(4);
            /// assert_eq!(x, 5);
            /// ```
            #[inline]
            fn neg_mod_power_of_two_assign(&mut self, pow: u64) {
                *self = self.neg_mod_power_of_two(pow)
            }
        }
    };
}

impl_mod_power_of_two_unsigned!(u8);
impl_mod_power_of_two_unsigned!(u16);
impl_mod_power_of_two_unsigned!(u32);
impl_mod_power_of_two_unsigned!(u64);
impl_mod_power_of_two_unsigned!(u128);
impl_mod_power_of_two_unsigned!(usize);

macro_rules! impl_mod_power_of_two_signed {
    ($t:ident, $u:ident) => {
        impl ModPowerOfTwo for $t {
            type Output = $u;

            /// Calculates `self` mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Unlike rem_power_of_two, this function always returns a non-negative number.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is negative and `pow` is greater than `$t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260i16.mod_power_of_two(8), 4);
            ///
            /// // -101 * 2^4 + 5 = -1611
            /// assert_eq!((-1611i32).mod_power_of_two(4), 5);
            /// ```
            #[inline]
            fn mod_power_of_two(self, pow: u64) -> $u {
                if self < 0 && pow > $t::WIDTH {
                    panic!(ERROR_MESSAGE);
                }
                $u::wrapping_from(self).mod_power_of_two(pow)
            }
        }

        impl ModPowerOfTwoAssign for $t {
            /// Reduces `self` mod a power of 2. In other words, replsces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Unlike rem_power_of_two, this function always assigns a non-negative number.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is negative and `pow` is greater than or equal to `$t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoAssign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260i16;
            /// x.mod_power_of_two_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // -101 * 2^4 + 5 = -1611
            /// let mut x = -1611i32;
            /// x.mod_power_of_two_assign(4);
            /// assert_eq!(x, 5);
            /// ```
            #[inline]
            fn mod_power_of_two_assign(&mut self, pow: u64) {
                *self = $t::checked_from(self.mod_power_of_two(pow)).expect(ERROR_MESSAGE);
            }
        }

        impl RemPowerOfTwo for $t {
            type Output = $t;

            /// Calculates `self` rem a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
            /// 0 <= |r| < 2<sup>`pow`</sup>.
            ///
            /// Unlike `mod_power_of_two`, this function always returns zero or a number with the
            /// same sign as `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOfTwo;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260i16.rem_power_of_two(8), 4);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// assert_eq!((-1611i32).rem_power_of_two(4), -11);
            /// ```
            #[inline]
            fn rem_power_of_two(self, pow: u64) -> $t {
                if self >= 0 {
                    $t::wrapping_from($u::wrapping_from(self).mod_power_of_two(pow))
                } else {
                    $t::wrapping_from($u::wrapping_from(self.wrapping_neg()).mod_power_of_two(pow))
                        .wrapping_neg()
                }
            }
        }

        impl RemPowerOfTwoAssign for $t {
            /// Reduces `self` rem a power of 2. In other words, replaces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
            /// 0 <= |r| < 2<sup>`pow`</sup>.
            ///
            /// Unlike `mod_power_of_two`, this function always assigns zero or a number with the
            /// same sign as `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOfTwoAssign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260i16;
            /// x.rem_power_of_two_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// let mut x = -1611i32;
            /// x.rem_power_of_two_assign(4);
            /// assert_eq!(x, -11);
            /// ```
            #[inline]
            fn rem_power_of_two_assign(&mut self, pow: u64) {
                *self = self.rem_power_of_two(pow)
            }
        }

        impl CeilingModPowerOfTwo for $t {
            type Output = $t;

            /// Calculates `self` ceiling-mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= -r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is positive or `$t::MIN`, and `pow` is greater than or equal to
            /// `$t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingModPowerOfTwo;
            ///
            /// // 2 * 2^8 + -252 = 260
            /// assert_eq!(260i16.ceiling_mod_power_of_two(8), -252);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// assert_eq!((-1611i32).ceiling_mod_power_of_two(4), -11);
            /// ```
            #[inline]
            fn ceiling_mod_power_of_two(self, pow: u64) -> $t {
                let abs_result = if self >= 0 {
                    $u::wrapping_from(self).neg_mod_power_of_two(pow)
                } else {
                    $u::wrapping_from(self.wrapping_neg()).mod_power_of_two(pow)
                };
                $t::checked_from(abs_result)
                    .expect(ERROR_MESSAGE)
                    .checked_neg()
                    .expect(ERROR_MESSAGE)
            }
        }

        impl CeilingModPowerOfTwoAssign for $t {
            /// Reduces `self` ceiling-mod a power of 2. In other words, replaces `self` with r,
            /// where `self` = q * 2<sup>`pow`</sup> + r and 0 <= -r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is positive or `$t::MIN`, and `pow` is greater than or equal to
            /// `$t::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CeilingModPowerOfTwoAssign;
            ///
            /// // 2 * 2^8 + -252 = 260
            /// let mut x = 260i16;
            /// x.ceiling_mod_power_of_two_assign(8);
            /// assert_eq!(x, -252);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// let mut x = -1611i32;
            /// x.ceiling_mod_power_of_two_assign(4);
            /// assert_eq!(x, -11);
            /// ```
            #[inline]
            fn ceiling_mod_power_of_two_assign(&mut self, pow: u64) {
                *self = self.ceiling_mod_power_of_two(pow)
            }
        }
    };
}

impl_mod_power_of_two_signed!(i8, u8);
impl_mod_power_of_two_signed!(i16, u16);
impl_mod_power_of_two_signed!(i32, u32);
impl_mod_power_of_two_signed!(i64, u64);
impl_mod_power_of_two_signed!(i128, u128);
impl_mod_power_of_two_signed!(isize, usize);
