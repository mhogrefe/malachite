// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign, UnsignedAbs};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

fn mod_power_of_2_shl_unsigned<T: PrimitiveUnsigned + Shl<U, Output = T>, U: PrimitiveUnsigned>(
    x: T,
    other: U,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    if other >= U::exact_from(T::WIDTH) {
        T::ZERO
    } else {
        (x << other).mod_power_of_2(pow)
    }
}

fn mod_power_of_2_shl_assign_unsigned<T: PrimitiveUnsigned + ShlAssign<U>, U: PrimitiveUnsigned>(
    x: &mut T,
    other: U,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
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

                    /// Left-shifts a number (multiplies it by a power of 2) modulo $2^k$. The
                    /// number must be already reduced modulo $2^k$.
                    ///
                    /// $f(x, n, k) = y$, where $x, y < 2^k$ and $2^nx \equiv y \mod 2^k$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than
                    /// or equal to $2^k$.
                    ///
                    /// # Examples
                    /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
                    #[inline]
                    fn mod_power_of_2_shl(self, other: $u, pow: u64) -> $t {
                        mod_power_of_2_shl_unsigned(self, other, pow)
                    }
                }

                impl ModPowerOf2ShlAssign<$u> for $t {
                    /// Left-shifts a number (multiplies it by a power of 2) modulo $2^k$, in place.
                    /// The number must be already reduced modulo $2^k$.
                    ///
                    /// $x \gets y$, where $x, y < 2^k$ and $2^nx \equiv y \mod 2^k$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than
                    /// or equal to $2^k$.
                    ///
                    /// # Examples
                    /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
                    #[inline]
                    fn mod_power_of_2_shl_assign(&mut self, other: $u, pow: u64) {
                        mod_power_of_2_shl_assign_unsigned(self, other, pow);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);

fn mod_power_of_2_shl_signed<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    other: S,
    pow: u64,
) -> T {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
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

fn mod_power_of_2_shl_assign_signed<
    T: ModPowerOf2ShlAssign<U> + PrimitiveInt + ShrAssign<U>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: &mut T,
    other: S,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
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

                    /// Left-shifts a number (multiplies it by a power of 2) modulo $2^k$. The
                    /// number must be already reduced modulo $2^k$.
                    ///
                    /// $f(x, n, k) = y$, where $x, y < 2^k$ and $\lfloor 2^nx \rfloor \equiv y \mod
                    /// 2^k$.
                    ///
                    /// # Panics
                    /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than
                    /// or equal to $2^k$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
                    #[inline]
                    fn mod_power_of_2_shl(self, other: $u, pow: u64) -> $t {
                        mod_power_of_2_shl_signed(self, other, pow)
                    }
                }

                impl ModPowerOf2ShlAssign<$u> for $t {
                    /// Left-shifts a number (multiplies it by a power of 2) modulo $2^k$, in place.
                    /// The number must be already reduced modulo $2^k$.
                    ///
                    /// $x \gets y$, where $x, y < 2^k$ and $\lfloor 2^nx \rfloor \equiv y \mod
                    /// 2^k$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `pow` is greater than `Self::WIDTH` or if `self` is greater than
                    /// or equal to $2^k$.
                    ///
                    /// # Examples
                    /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
                    #[inline]
                    fn mod_power_of_2_shl_assign(&mut self, other: $u, pow: u64) {
                        mod_power_of_2_shl_assign_signed(self, other, pow);
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_power_of_2_shl_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_signed);
