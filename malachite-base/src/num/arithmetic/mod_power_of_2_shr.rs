use num::arithmetic::traits::{
    ModPowerOf2Shl, ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, UnsignedAbs,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use std::ops::{Shr, ShrAssign};

fn mod_power_of_2_shr_signed<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
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
        x.mod_power_of_2_shl(other_abs, pow)
    }
}

fn mod_power_of_2_shr_assign_signed<
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
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    } else {
        x.mod_power_of_2_shl_assign(other_abs, pow);
    }
}

macro_rules! impl_mod_power_of_2_shr_signed {
    ($t:ident) => {
        macro_rules! impl_mod_power_of_2_shr_signed_inner {
            ($u:ident) => {
                impl ModPowerOf2Shr<$u> for $t {
                    type Output = $t;

                    /// Computes `self >> other` mod $2^p$. Assumes the input is already reduced
                    /// mod $2^p$.
                    ///
                    /// $f(x, n, p) = y$, where $x, y < 2^p$ and
                    /// $\lfloor 2^{-n}x \rfloor \equiv y \mod 2^p$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::mod_power_of_2_shr` module.
                    #[inline]
                    fn mod_power_of_2_shr(self, other: $u, pow: u64) -> $t {
                        mod_power_of_2_shr_signed(self, other, pow)
                    }
                }

                impl ModPowerOf2ShrAssign<$u> for $t {
                    /// Replaces `self` with `self >> other` mod $2^p$. Assumes the input is
                    /// already reduced mod $2^p$.
                    ///
                    /// $x \gets y$, where $x, y < 2^p$ and
                    /// $\lfloor 2^{-n}x \rfloor \equiv y \mod 2^p$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::mod_power_of_2_shr` module.
                    #[inline]
                    fn mod_power_of_2_shr_assign(&mut self, other: $u, pow: u64) {
                        mod_power_of_2_shr_assign_signed(self, other, pow)
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_power_of_2_shr_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shr_signed);
