use std::ops::{Shr, ShrAssign};

use num::arithmetic::traits::{
    ModPowerOfTwoShl, ModPowerOfTwoShlAssign, ModPowerOfTwoShr, ModPowerOfTwoShrAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;

fn _mod_power_of_two_shr_signed<
    T: ModPowerOfTwoShl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    other: S,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    } else {
        x.mod_power_of_two_shl(other_abs, pow)
    }
}

fn _mod_power_of_two_shr_assign_signed<
    T: ModPowerOfTwoShlAssign<U> + PrimitiveInt + ShrAssign<U>,
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
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    } else {
        x.mod_power_of_two_shl_assign(other_abs, pow);
    }
}

macro_rules! impl_mod_power_of_two_shr_signed {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_two_shr_signed_inner {
            ($u:ident) => {
                impl ModPowerOfTwoShr<$u> for $t {
                    type Output = $t;

                    /// Computes `self >> other` mod 2<sup>`pow`</sup>. Assumes the input is already
                    /// reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShr;
                    ///
                    /// assert_eq!(10u8.mod_power_of_two_shr(2i64, 4), 2);
                    /// assert_eq!(12u32.mod_power_of_two_shr(-2i8, 5), 16);
                    /// ```
                    #[inline]
                    fn mod_power_of_two_shr(self, other: $u, pow: u64) -> $t {
                        _mod_power_of_two_shr_signed(self, other, pow)
                    }
                }

                impl ModPowerOfTwoShrAssign<$u> for $t {
                    /// Replaces `self` with `self >> other` mod 2<sup>`pow`</sup>. Assumes the
                    /// input is already reduced mod 2<sup>`pow`</sup>.
                    ///
                    /// Time: worst case O(1)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// # Example
                    /// ```
                    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShrAssign;
                    ///
                    /// let mut n = 10u8;
                    /// n.mod_power_of_two_shr_assign(2i64, 4);
                    /// assert_eq!(n, 2);
                    ///
                    /// let mut n = 12u32;
                    /// n.mod_power_of_two_shr_assign(-2i8, 5);
                    /// assert_eq!(n, 16);
                    /// ```
                    #[inline]
                    fn mod_power_of_two_shr_assign(&mut self, other: $u, pow: u64) {
                        _mod_power_of_two_shr_assign_signed(self, other, pow)
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_power_of_two_shr_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_shr_signed);
