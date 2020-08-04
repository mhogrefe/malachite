use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoShl, ModPowerOfTwoShlAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInteger;
use num::basic::traits::Zero;
use num::conversion::traits::{ExactFrom, WrappingFrom};

fn _mod_power_of_two_shl_unsigned<T: PrimitiveInteger, U: ExactFrom<u64> + Ord>(
    x: T,
    other: U,
    pow: u64,
) -> T
where
    T: ModPowerOfTwo<Output = T> + Shl<U, Output = T>,
{
    assert!(pow <= T::WIDTH);
    if other >= U::exact_from(T::WIDTH) {
        T::ZERO
    } else {
        (x << other).mod_power_of_two(pow)
    }
}

fn _mod_power_of_two_shl_assign_unsigned<T: PrimitiveInteger, U: ExactFrom<u64> + Ord>(
    x: &mut T,
    other: U,
    pow: u64,
) where
    T: ShlAssign<U>,
{
    assert!(pow <= T::WIDTH);
    if other >= U::exact_from(T::WIDTH) {
        *x = T::ZERO;
    } else {
        *x <<= other;
        x.mod_power_of_two_assign(pow);
    }
}

macro_rules! impl_mod_power_of_two_shl_unsigned {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_two_shl_unsigned_inner {
            ($u:ident) => {
                impl ModPowerOfTwoShl<$u> for $t {
                    type Output = $t;

                    /// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the input is already
                    /// reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
                    ///
                    /// assert_eq!(12u32.mod_power_of_two_shl(2u8, 5), 16);
                    /// assert_eq!(10u8.mod_power_of_two_shl(100u64, 4), 0);
                    /// ```
                    #[inline]
                    fn mod_power_of_two_shl(self, other: $u, pow: u64) -> $t {
                        _mod_power_of_two_shl_unsigned(self, other, pow)
                    }
                }

                impl ModPowerOfTwoShlAssign<$u> for $t {
                    /// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the
                    /// input is already reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShlAssign;
                    ///
                    /// let mut n = 12u32;
                    /// n.mod_power_of_two_shl_assign(2u8, 5);
                    /// assert_eq!(n, 16);
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_power_of_two_shl_assign(100u64, 4);
                    /// assert_eq!(n, 0);
                    /// ```
                    #[inline]
                    fn mod_power_of_two_shl_assign(&mut self, other: $u, pow: u64) {
                        _mod_power_of_two_shl_assign_unsigned(self, other, pow);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mod_power_of_two_shl_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_shl_unsigned);

fn _mod_power_of_two_shl_signed<
    T: PrimitiveInteger,
    U: Copy + Eq + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + Zero,
>(
    x: T,
    other: S,
    pow: u64,
) -> T
where
    S: UnsignedAbs<Output = U>,
    T: ModPowerOfTwoShl<U, Output = T> + Shr<U, Output = T>,
{
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_power_of_two_shl(other_abs, pow)
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    }
}

fn _mod_power_of_two_shl_assign_signed<
    T: PrimitiveInteger,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + Zero,
>(
    x: &mut T,
    other: S,
    pow: u64,
) where
    T: ModPowerOfTwoShlAssign<U> + ShrAssign<U>,
    S: UnsignedAbs<Output = U>,
{
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_power_of_two_shl_assign(other_abs, pow);
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    }
}

macro_rules! impl_mod_power_of_two_shl_signed {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_two_shl_signed_inner {
            ($u:ident) => {
                impl ModPowerOfTwoShl<$u> for $t {
                    type Output = $t;

                    /// Computes `self << other` mod 2<sup>`pow`</sup>. Assumes the input is already
                    /// reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
                    ///
                    /// assert_eq!(12u32.mod_power_of_two_shl(2i8, 5), 16);
                    /// assert_eq!(10u8.mod_power_of_two_shl(-2i64, 4), 2);
                    /// ```
                    #[inline]
                    fn mod_power_of_two_shl(self, other: $u, pow: u64) -> $t {
                        _mod_power_of_two_shl_signed(self, other, pow)
                    }
                }

                impl ModPowerOfTwoShlAssign<$u> for $t {
                    /// Replaces `self` with `self << other` mod 2<sup>`pow`</sup>. Assumes the
                    /// input is already reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShlAssign;
                    ///
                    /// let mut n = 12u32;
                    /// n.mod_power_of_two_shl_assign(2i8, 5);
                    /// assert_eq!(n, 16);
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_power_of_two_shl_assign(-2i64, 4);
                    /// assert_eq!(n, 2);
                    /// ```
                    #[inline]
                    fn mod_power_of_two_shl_assign(&mut self, other: $u, pow: u64) {
                        _mod_power_of_two_shl_assign_signed(self, other, pow);
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_power_of_two_shl_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_shl_signed);
