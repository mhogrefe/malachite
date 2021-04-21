use num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, CheckedNeg, ModPowerOf2, ModPowerOf2Assign,
    NegModPowerOf2, NegModPowerOf2Assign, RemPowerOf2, RemPowerOf2Assign, WrappingNeg,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::{CheckedFrom, WrappingFrom};

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

fn _mod_power_of_2_unsigned<T: PrimitiveInt>(x: T, pow: u64) -> T {
    if x == T::ZERO || pow >= T::WIDTH {
        x
    } else {
        x & T::low_mask(pow)
    }
}

fn _mod_power_of_2_assign_unsigned<T: PrimitiveInt>(x: &mut T, pow: u64) {
    if *x != T::ZERO && pow < T::WIDTH {
        *x &= T::low_mask(pow)
    }
}

#[inline]
fn _neg_mod_power_of_2_unsigned<T: ModPowerOf2<Output = T> + PrimitiveInt>(x: T, pow: u64) -> T {
    if x != T::ZERO && pow > T::WIDTH {
        panic!("{}", ERROR_MESSAGE);
    }
    x.wrapping_neg().mod_power_of_2(pow)
}

macro_rules! impl_mod_power_of_2_unsigned {
    ($s:ident) => {
        impl ModPowerOf2 for $s {
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260u16.mod_power_of_2(8), 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// assert_eq!(1611u32.mod_power_of_2(4), 11);
            /// ```
            #[inline]
            fn mod_power_of_2(self, pow: u64) -> $s {
                _mod_power_of_2_unsigned(self, pow)
            }
        }

        impl ModPowerOf2Assign for $s {
            /// Reduces `self` mod a power of 2. In other words, replaces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260u16;
            /// x.mod_power_of_2_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// let mut x = 1611u32;
            /// x.mod_power_of_2_assign(4);
            /// assert_eq!(x, 11);
            /// ```
            #[inline]
            fn mod_power_of_2_assign(&mut self, pow: u64) {
                _mod_power_of_2_assign_unsigned(self, pow);
            }
        }

        impl RemPowerOf2 for $s {
            type Output = $s;

            /// Calculates `self` rem a power of 2. For unsigned integers, rem is equivalent to mod.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260u16.rem_power_of_2(8), 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// assert_eq!(1611u32.rem_power_of_2(4), 11);
            /// ```
            #[inline]
            fn rem_power_of_2(self, pow: u64) -> $s {
                self.mod_power_of_2(pow)
            }
        }

        impl RemPowerOf2Assign for $s {
            /// Reduces `self` rem a power of 2. For unsigned integers, rem is equivalent to mod.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260u16;
            /// x.rem_power_of_2_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // 100 * 2^4 + 11 = 1611
            /// let mut x = 1611u32;
            /// x.rem_power_of_2_assign(4);
            /// assert_eq!(x, 11);
            /// ```
            #[inline]
            fn rem_power_of_2_assign(&mut self, pow: u64) {
                self.mod_power_of_2_assign(pow)
            }
        }

        impl NegModPowerOf2 for $s {
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
            /// use malachite_base::num::arithmetic::traits::NegModPowerOf2;
            ///
            /// // 2 * 2^8 - 252 = 260
            /// assert_eq!(260u16.neg_mod_power_of_2(8), 252);
            ///
            /// // 101 * 2^4 - 5 = 1611
            /// assert_eq!(1611u32.neg_mod_power_of_2(4), 5);
            /// ```
            #[inline]
            fn neg_mod_power_of_2(self, pow: u64) -> $s {
                _neg_mod_power_of_2_unsigned(self, pow)
            }
        }

        impl NegModPowerOf2Assign for $s {
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
            /// use malachite_base::num::arithmetic::traits::NegModPowerOf2Assign;
            ///
            /// // 2 * 2^8 - 252 = 260
            /// let mut x = 260u16;
            /// x.neg_mod_power_of_2_assign(8);
            /// assert_eq!(x, 252);
            ///
            /// // 101 * 2^4 - 5 = 1611
            /// let mut x = 1611u32;
            /// x.neg_mod_power_of_2_assign(4);
            /// assert_eq!(x, 5);
            /// ```
            #[inline]
            fn neg_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.neg_mod_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_unsigned);

fn _mod_power_of_2_signed<U: ModPowerOf2<Output = U> + WrappingFrom<S>, S: PrimitiveInt>(
    x: S,
    pow: u64,
) -> U {
    if x < S::ZERO && pow > S::WIDTH {
        panic!("{}", ERROR_MESSAGE);
    }
    U::wrapping_from(x).mod_power_of_2(pow)
}

fn _mod_power_of_2_assign_signed<U, S: CheckedFrom<U> + Copy + ModPowerOf2<Output = U>>(
    x: &mut S,
    pow: u64,
) {
    *x = S::checked_from(x.mod_power_of_2(pow)).expect(ERROR_MESSAGE);
}

fn _rem_power_of_2_signed<
    U: ModPowerOf2<Output = U> + WrappingFrom<S>,
    S: Copy + Ord + WrappingFrom<U> + WrappingNeg<Output = S> + Zero,
>(
    x: S,
    pow: u64,
) -> S {
    if x >= S::ZERO {
        S::wrapping_from(U::wrapping_from(x).mod_power_of_2(pow))
    } else {
        S::wrapping_from(U::wrapping_from(x.wrapping_neg()).mod_power_of_2(pow)).wrapping_neg()
    }
}

fn _ceiling_mod_power_of_2_signed<
    U: ModPowerOf2<Output = U> + NegModPowerOf2<Output = U> + WrappingFrom<S>,
    S: CheckedFrom<U> + CheckedNeg<Output = S> + Copy + Ord + WrappingNeg<Output = S> + Zero,
>(
    x: S,
    pow: u64,
) -> S {
    let abs_result = if x >= S::ZERO {
        U::wrapping_from(x).neg_mod_power_of_2(pow)
    } else {
        U::wrapping_from(x.wrapping_neg()).mod_power_of_2(pow)
    };
    S::checked_from(abs_result)
        .expect(ERROR_MESSAGE)
        .checked_neg()
        .expect(ERROR_MESSAGE)
}

macro_rules! impl_mod_power_of_2_signed {
    ($u:ident, $s:ident) => {
        impl ModPowerOf2 for $s {
            type Output = $u;

            /// Calculates `self` mod a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Unlike rem_power_of_2, this function always returns a non-negative number.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260i16.mod_power_of_2(8), 4);
            ///
            /// // -101 * 2^4 + 5 = -1611
            /// assert_eq!((-1611i32).mod_power_of_2(4), 5);
            /// ```
            #[inline]
            fn mod_power_of_2(self, pow: u64) -> $u {
                _mod_power_of_2_signed(self, pow)
            }
        }

        impl ModPowerOf2Assign for $s {
            /// Reduces `self` mod a power of 2. In other words, replsces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> + r and 0 <= r < 2<sup>`pow`</sup>.
            ///
            /// Unlike rem_power_of_2, this function always assigns a non-negative number.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260i16;
            /// x.mod_power_of_2_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // -101 * 2^4 + 5 = -1611
            /// let mut x = -1611i32;
            /// x.mod_power_of_2_assign(4);
            /// assert_eq!(x, 5);
            /// ```
            #[inline]
            fn mod_power_of_2_assign(&mut self, pow: u64) {
                _mod_power_of_2_assign_signed(self, pow);
            }
        }

        impl RemPowerOf2 for $s {
            type Output = $s;

            /// Calculates `self` rem a power of 2. In other words, returns r, where
            /// `self` = q * 2<sup>`pow`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
            /// 0 <= |r| < 2<sup>`pow`</sup>.
            ///
            /// Unlike `mod_power_of_2`, this function always returns zero or a number with the
            /// same sign as `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// assert_eq!(260i16.rem_power_of_2(8), 4);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// assert_eq!((-1611i32).rem_power_of_2(4), -11);
            /// ```
            #[inline]
            fn rem_power_of_2(self, pow: u64) -> $s {
                _rem_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl RemPowerOf2Assign for $s {
            /// Reduces `self` rem a power of 2. In other words, replaces `self` with r, where
            /// `self` = q * 2<sup>`pow`</sup> + r, r == 0 or (sgn(r) == sgn(`self`)), and
            /// 0 <= |r| < 2<sup>`pow`</sup>.
            ///
            /// Unlike `mod_power_of_2`, this function always assigns zero or a number with the
            /// same sign as `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
            ///
            /// // 1 * 2^8 + 4 = 260
            /// let mut x = 260i16;
            /// x.rem_power_of_2_assign(8);
            /// assert_eq!(x, 4);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// let mut x = -1611i32;
            /// x.rem_power_of_2_assign(4);
            /// assert_eq!(x, -11);
            /// ```
            #[inline]
            fn rem_power_of_2_assign(&mut self, pow: u64) {
                *self = self.rem_power_of_2(pow)
            }
        }

        impl CeilingModPowerOf2 for $s {
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
            /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2;
            ///
            /// // 2 * 2^8 + -252 = 260
            /// assert_eq!(260i16.ceiling_mod_power_of_2(8), -252);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// assert_eq!((-1611i32).ceiling_mod_power_of_2(4), -11);
            /// ```
            #[inline]
            fn ceiling_mod_power_of_2(self, pow: u64) -> $s {
                _ceiling_mod_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl CeilingModPowerOf2Assign for $s {
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
            /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2Assign;
            ///
            /// // 2 * 2^8 + -252 = 260
            /// let mut x = 260i16;
            /// x.ceiling_mod_power_of_2_assign(8);
            /// assert_eq!(x, -252);
            ///
            /// // -100 * 2^4 + -11 = -1611
            /// let mut x = -1611i32;
            /// x.ceiling_mod_power_of_2_assign(4);
            /// assert_eq!(x, -11);
            /// ```
            #[inline]
            fn ceiling_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.ceiling_mod_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_mod_power_of_2_signed);
