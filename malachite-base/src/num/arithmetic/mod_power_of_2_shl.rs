use num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Shl, ModPowerOf2ShlAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

fn _mod_power_of_2_shl_unsigned<
    T: ModPowerOf2<Output = T> + PrimitiveInt + Shl<U, Output = T>,
    U: ExactFrom<u64> + Ord,
>(
    x: T,
    other: U,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    if other >= U::exact_from(T::WIDTH) {
        T::ZERO
    } else {
        (x << other).mod_power_of_2(pow)
    }
}

fn _mod_power_of_2_shl_assign_unsigned<T: PrimitiveInt + ShlAssign<U>, U: ExactFrom<u64> + Ord>(
    x: &mut T,
    other: U,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    if other >= U::exact_from(T::WIDTH) {
        *x = T::ZERO;
    } else {
        *x <<= other;
        x.mod_power_of_2_assign(pow);
    }
}

macro_rules! impl_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_2_shl_unsigned_inner {
            ($u:ident) => {
                impl ModPowerOf2Shl<$u> for $t {
                    type Output = $t;

                    /// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the input is already
                    /// reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
                    ///
                    /// assert_eq!(12u32.mod_power_of_2_shl(2u8, 5), 16);
                    /// assert_eq!(10u8.mod_power_of_2_shl(100u64, 4), 0);
                    /// ```
                    #[inline]
                    fn mod_power_of_2_shl(self, other: $u, pow: u64) -> $t {
                        _mod_power_of_2_shl_unsigned(self, other, pow)
                    }
                }

                impl ModPowerOf2ShlAssign<$u> for $t {
                    /// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the
                    /// input is already reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
                    ///
                    /// let mut n = 12u32;
                    /// n.mod_power_of_2_shl_assign(2u8, 5);
                    /// assert_eq!(n, 16);
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_power_of_2_shl_assign(100u64, 4);
                    /// assert_eq!(n, 0);
                    /// ```
                    #[inline]
                    fn mod_power_of_2_shl_assign(&mut self, other: $u, pow: u64) {
                        _mod_power_of_2_shl_assign_unsigned(self, other, pow);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);

fn _mod_power_of_2_shl_signed<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Copy + Eq + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    other: S,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_power_of_2_shl(other_abs, pow)
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    }
}

fn _mod_power_of_2_shl_assign_signed<
    T: ModPowerOf2ShlAssign<U> + PrimitiveInt + ShrAssign<U>,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: &mut T,
    other: S,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_power_of_2_shl_assign(other_abs, pow);
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    }
}

macro_rules! impl_mod_power_of_2_shl_signed {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_2_shl_signed_inner {
            ($u:ident) => {
                impl ModPowerOf2Shl<$u> for $t {
                    type Output = $t;

                    /// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the input is already
                    /// reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
                    ///
                    /// assert_eq!(12u32.mod_power_of_2_shl(2i8, 5), 16);
                    /// assert_eq!(10u8.mod_power_of_2_shl(-2i64, 4), 2);
                    /// ```
                    #[inline]
                    fn mod_power_of_2_shl(self, other: $u, pow: u64) -> $t {
                        _mod_power_of_2_shl_signed(self, other, pow)
                    }
                }

                impl ModPowerOf2ShlAssign<$u> for $t {
                    /// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the
                    /// input is already reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
                    ///
                    /// let mut n = 12u32;
                    /// n.mod_power_of_2_shl_assign(2i8, 5);
                    /// assert_eq!(n, 16);
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_power_of_2_shl_assign(-2i64, 4);
                    /// assert_eq!(n, 2);
                    /// ```
                    #[inline]
                    fn mod_power_of_2_shl_assign(&mut self, other: $u, pow: u64) {
                        _mod_power_of_2_shl_assign_signed(self, other, pow);
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_power_of_2_shl_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_signed);
