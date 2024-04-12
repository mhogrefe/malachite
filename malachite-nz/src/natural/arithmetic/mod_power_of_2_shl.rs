// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::ops::{Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2Shl, ModPowerOf2ShlAssign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

fn mod_power_of_2_shl_unsigned_nz<T: PrimitiveUnsigned>(x: &Natural, bits: T, pow: u64) -> Natural
where
    u64: ExactFrom<T>,
{
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    let bits = u64::exact_from(bits);
    if bits >= pow {
        Natural::ZERO
    } else {
        x.mod_power_of_2(pow - bits) << bits
    }
}

fn mod_power_of_2_shl_assign_unsigned_nz<T: PrimitiveUnsigned>(x: &mut Natural, bits: T, pow: u64)
where
    u64: ExactFrom<T>,
{
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    let bits = u64::exact_from(bits);
    if bits >= pow {
        *x = Natural::ZERO;
    } else {
        x.mod_power_of_2_assign(pow - bits);
        *x <<= bits;
    }
}

macro_rules! impl_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        impl ModPowerOf2Shl<$t> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo $2^k$. The
            /// [`Natural`] must be already reduced modulo $2^k$. The [`Natural`] is taken by value.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $2^nx \equiv y \mod 2^k$.
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
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_2_shl_assign(bits, pow);
                self
            }
        }

        impl<'a> ModPowerOf2Shl<$t> for &'a Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo $2^k$. The
            /// [`Natural`] must be already reduced modulo $2^k$. The [`Natural`] is taken by
            /// reference.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $2^nx \equiv y \mod 2^k$.
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
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(self, bits: $t, pow: u64) -> Natural {
                mod_power_of_2_shl_unsigned_nz(self, bits, pow)
            }
        }

        impl ModPowerOf2ShlAssign<$t> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo $2^k$, in place.
            /// The [`Natural`] must be already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $2^nx \equiv y \mod 2^k$.
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
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
            #[inline]
            fn mod_power_of_2_shl_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shl_assign_unsigned_nz(self, bits, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);

fn mod_power_of_2_shl_signed_nz<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
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
        x.mod_power_of_2_shl(bits.unsigned_abs(), pow)
    } else {
        x >> bits.unsigned_abs()
    }
}

fn mod_power_of_2_shl_assign_signed_nz<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
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
        x.mod_power_of_2_shl_assign(bits.unsigned_abs(), pow);
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_mod_power_of_2_shl_signed {
    ($t:ident) => {
        impl ModPowerOf2Shl<$t> for Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo $2^k$. The
            /// [`Natural`] must be already reduced modulo $2^k$. The [`Natural`] is taken by value.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $\lfloor 2^nx \rfloor \equiv y \mod 2^k$.
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
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_2_shl_assign(bits, pow);
                self
            }
        }

        impl<'a> ModPowerOf2Shl<$t> for &'a Natural {
            type Output = Natural;

            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo $2^k$. The
            /// [`Natural`] must be already reduced modulo $2^k$. The [`Natural`] is taken by
            /// reference.
            ///
            /// $f(x, n, k) = y$, where $x, y < 2^k$ and $\lfloor 2^nx \rfloor \equiv y \mod 2^k$.
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
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(self, bits: $t, pow: u64) -> Natural {
                mod_power_of_2_shl_signed_nz(self, bits, pow)
            }
        }

        impl ModPowerOf2ShlAssign<$t> for Natural {
            /// Left-shifts a [`Natural`] (multiplies it by a power of 2) modulo $2^k$, in place.
            /// The [`Natural`] must be already reduced modulo $2^k$.
            ///
            /// $x \gets y$, where $x, y < 2^k$ and $\lfloor 2^nx \rfloor \equiv y \mod 2^k$.
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
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
            #[inline]
            fn mod_power_of_2_shl_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shl_assign_signed_nz(self, bits, pow)
            }
        }
    };
}
apply_to_signeds!(impl_mod_power_of_2_shl_signed);
