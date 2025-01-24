// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::ops::{Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Shl, ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::logic::traits::SignificantBits;

fn mod_power_of_2_shr_ref<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    pow: u64,
) -> Natural
where
    &'a Natural: ModPowerOf2Shl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x.mod_power_of_2_shl(bits.unsigned_abs(), pow)
    }
}

fn mod_power_of_2_shr_assign<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    pow: u64,
) where
    Natural: ModPowerOf2ShlAssign<U> + ShrAssign<U>,
{
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        x.mod_power_of_2_shl_assign(bits.unsigned_abs(), pow);
    }
}

macro_rules! impl_mod_power_of_2_shr_signed {
    ($t:ident) => {
        impl ModPowerOf2Shr<$t> for Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo $2^k$. The
            /// [`Natural`] must be already reduced modulo $2^k$. The [`Natural`] is taken by value.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod
            /// 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shr#mod_power_of_2_shr).
            #[inline]
            fn mod_power_of_2_shr(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_2_shr_assign(bits, pow);
                self
            }
        }

        impl ModPowerOf2Shr<$t> for &Natural {
            type Output = Natural;

            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo $2^k$. The
            /// [`Natural`] must be already reduced modulo $2^k$. The [`Natural`] is taken by
            /// reference.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod
            /// 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shr#mod_power_of_2_shr).
            #[inline]
            fn mod_power_of_2_shr(self, bits: $t, pow: u64) -> Natural {
                mod_power_of_2_shr_ref(self, bits, pow)
            }
        }

        impl ModPowerOf2ShrAssign<$t> for Natural {
            /// Right-shifts a [`Natural`] (divides it by a power of 2) modulo $2^k$, in place. The
            /// [`Natural`] must be already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $\lfloor 2^{-n}x \rfloor \equiv y \mod 2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shr#mod_power_of_2_shr_assign).
            #[inline]
            fn mod_power_of_2_shr_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shr_assign(self, bits, pow);
            }
        }
    };
}
apply_to_signeds!(impl_mod_power_of_2_shr_signed);
