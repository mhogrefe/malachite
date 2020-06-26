use num::arithmetic::traits::{
    CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, CheckedNeg, ModPowerOfTwo,
    ModPowerOfTwoAssign, NegModPowerOfTwo, NegModPowerOfTwoAssign, RemPowerOfTwo,
    RemPowerOfTwoAssign, WrappingNeg,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;
use num::conversion::traits::{CheckedFrom, WrappingFrom};

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

#[inline]
pub fn _mod_power_of_two_unsigned<T: PrimitiveInteger>(x: T, pow: u64) -> T {
    if x == T::ZERO || pow >= T::WIDTH {
        x
    } else {
        x & T::low_mask(pow)
    }
}

#[inline]
pub fn _mod_power_of_two_assign_unsigned<T: PrimitiveInteger>(x: &mut T, pow: u64) {
    if *x != T::ZERO && pow < T::WIDTH {
        *x &= T::low_mask(pow)
    }
}

#[inline]
fn _neg_mod_power_of_two_unsigned<T: PrimitiveInteger>(x: T, pow: u64) -> T
where
    T: ModPowerOfTwo<Output = T>,
{
    if x != T::ZERO && pow > T::WIDTH {
        panic!(ERROR_MESSAGE);
    }
    x.wrapping_neg().mod_power_of_two(pow)
}

macro_rules! impl_mod_power_of_two_unsigned {
    ($s:ident) => {
        impl ModPowerOfTwo for $s {
            type Output = $s;

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
            fn mod_power_of_two(self, pow: u64) -> $s {
                _mod_power_of_two_unsigned(self, pow)
            }
        }

        impl ModPowerOfTwoAssign for $s {
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
                _mod_power_of_two_assign_unsigned(self, pow);
            }
        }

        impl RemPowerOfTwo for $s {
            type Output = $s;

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
            fn rem_power_of_two(self, pow: u64) -> $s {
                self.mod_power_of_two(pow)
            }
        }

        impl RemPowerOfTwoAssign for $s {
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

        impl NegModPowerOfTwo for $s {
            type Output = $s;

            /// Calculates `-self` mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> - r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `$s::WIDTH`.
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
            fn neg_mod_power_of_two(self, pow: u64) -> $s {
                _neg_mod_power_of_two_unsigned(self, pow)
            }
        }

        impl NegModPowerOfTwoAssign for $s {
            /// Reduces `-self` mod a power of 2. In other words, replaces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> - r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `$s::WIDTH`.
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
apply_to_unsigneds!(impl_mod_power_of_two_unsigned);

#[inline]
pub fn _mod_power_of_two_signed<U, S: PrimitiveInteger>(x: S, pow: u64) -> U
where
    U: ModPowerOfTwo<Output = U> + WrappingFrom<S>,
{
    if x < S::ZERO && pow > S::WIDTH {
        panic!(ERROR_MESSAGE);
    }
    U::wrapping_from(x).mod_power_of_two(pow)
}

#[inline]
pub fn _mod_power_of_two_assign_signed<U, S: Copy>(x: &mut S, pow: u64)
where
    S: CheckedFrom<U> + ModPowerOfTwo<Output = U>,
{
    *x = S::checked_from(x.mod_power_of_two(pow)).expect(ERROR_MESSAGE);
}

pub fn _rem_power_of_two_signed<U, S: Copy + Ord + Zero>(x: S, pow: u64) -> S
where
    U: ModPowerOfTwo<Output = U> + WrappingFrom<S>,
    S: WrappingFrom<U> + WrappingNeg<Output = S>,
{
    if x >= S::ZERO {
        S::wrapping_from(U::wrapping_from(x).mod_power_of_two(pow))
    } else {
        S::wrapping_from(U::wrapping_from(x.wrapping_neg()).mod_power_of_two(pow)).wrapping_neg()
    }
}

pub fn _ceiling_mod_power_of_two_signed<U, S: Copy + Ord + Zero>(x: S, pow: u64) -> S
where
    U: ModPowerOfTwo<Output = U> + NegModPowerOfTwo<Output = U> + WrappingFrom<S>,
    S: CheckedFrom<U> + CheckedNeg<Output = S> + WrappingNeg<Output = S>,
{
    let abs_result = if x >= S::ZERO {
        U::wrapping_from(x).neg_mod_power_of_two(pow)
    } else {
        U::wrapping_from(x.wrapping_neg()).mod_power_of_two(pow)
    };
    S::checked_from(abs_result)
        .expect(ERROR_MESSAGE)
        .checked_neg()
        .expect(ERROR_MESSAGE)
}

macro_rules! impl_mod_power_of_two_signed {
    ($u:ident, $s:ident) => {
        impl ModPowerOfTwo for $s {
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
            /// Panics if `self` is negative and `pow` is greater than `$s::WIDTH`.
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
                _mod_power_of_two_signed(self, pow)
            }
        }

        impl ModPowerOfTwoAssign for $s {
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
            /// Panics if `self` is negative and `pow` is greater than or equal to `$s::WIDTH`.
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
                _mod_power_of_two_assign_signed(self, pow);
            }
        }

        impl RemPowerOfTwo for $s {
            type Output = $s;

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
            fn rem_power_of_two(self, pow: u64) -> $s {
                _rem_power_of_two_signed::<$u, $s>(self, pow)
            }
        }

        impl RemPowerOfTwoAssign for $s {
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

        impl CeilingModPowerOfTwo for $s {
            type Output = $s;

            /// Calculates `self` ceiling-mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= -r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is positive or `$s::MIN`, and `pow` is greater than or equal to
            /// `$s::WIDTH`.
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
            fn ceiling_mod_power_of_two(self, pow: u64) -> $s {
                _ceiling_mod_power_of_two_signed::<$u, $s>(self, pow)
            }
        }

        impl CeilingModPowerOfTwoAssign for $s {
            /// Reduces `self` ceiling-mod a power of 2. In other words, replaces `self` with r,
            /// where `self` = q * 2<sup>`pow`</sup> + r and 0 <= -r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `self` is positive or `$s::MIN`, and `pow` is greater than or equal to
            /// `$s::WIDTH`.
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
apply_to_unsigned_signed_pair!(impl_mod_power_of_two_signed);
