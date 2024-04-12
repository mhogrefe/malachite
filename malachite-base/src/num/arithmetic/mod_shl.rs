// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModShl, ModShlAssign, UnsignedAbs};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use core::ops::{Shr, ShrAssign};

fn mod_shl_unsigned<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(x: T, other: U, m: T) -> T
where
    u64: ExactFrom<U>,
{
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    if other == U::ZERO {
        x
    } else if m == T::ONE || m == T::TWO {
        T::ZERO
    } else {
        x.mod_mul(T::TWO.mod_pow(u64::exact_from(other), m), m)
    }
}

fn mod_shl_assign_unsigned<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(x: &mut T, other: U, m: T)
where
    u64: ExactFrom<U>,
{
    assert!(*x < m, "x must be reduced mod m, but {x} >= {m}");
    if other == U::ZERO {
    } else if m == T::ONE || m == T::TWO {
        *x = T::ZERO;
    } else {
        x.mod_mul_assign(T::TWO.mod_pow(u64::exact_from(other), m), m);
    }
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        macro_rules! impl_mod_shl_unsigned_inner {
            ($u:ident) => {
                impl ModShl<$u, $t> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2) modulo a number $m$.
                    /// The number must be already reduced modulo $m$.
                    ///
                    /// $f(x, n, m) = y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `other.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `self` is greater than or equal to `m`.
                    ///
                    /// # Examples
                    /// See [here](super::mod_shl#mod_shl).
                    #[inline]
                    fn mod_shl(self, other: $u, m: $t) -> $t {
                        mod_shl_unsigned(self, other, m)
                    }
                }

                impl ModShlAssign<$u, $t> for $t {
                    /// Left-shifts a number (multiplies it by a power of 2) modulo a number $m$, in
                    /// place. The number must be already reduced modulo $m$.
                    ///
                    /// $x \gets y$, where $x, y < m$ and $2^nx \equiv y \mod m$.
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `other.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `self` is greater than or equal to `m`.
                    ///
                    /// # Examples
                    /// See [here](super::mod_shl#mod_shl_assign).
                    #[inline]
                    fn mod_shl_assign(&mut self, other: $u, m: $t) {
                        mod_shl_assign_unsigned(self, other, m);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_mod_shl_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);

fn mod_shl_signed<
    T: ModShl<U, T, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    other: S,
    m: T,
) -> T {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_shl(other_abs, m)
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            T::ZERO
        } else {
            x >> other_abs
        }
    }
}

fn mod_shl_assign_signed<
    T: ModShlAssign<U, T> + PrimitiveUnsigned + ShrAssign<U>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: &mut T,
    other: S,
    m: T,
) {
    assert!(*x < m, "x must be reduced mod m, but {x} >= {m}");
    let other_abs = other.unsigned_abs();
    if other >= S::ZERO {
        x.mod_shl_assign(other_abs, m);
    } else {
        let width = U::wrapping_from(T::WIDTH);
        if width != U::ZERO && other_abs >= width {
            *x = T::ZERO;
        } else {
            *x >>= other_abs;
        }
    }
}

macro_rules! impl_mod_shl_signed {
    ($t:ident) => {
        macro_rules! impl_mod_shl_signed_inner {
            ($u:ident) => {
                impl ModShl<$u, $t> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2) modulo a number $m$.
                    /// The number must be already reduced modulo $m$.
                    ///
                    /// $f(x, n, m) = y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod
                    /// m$.
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `other.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `self` is greater than or equal to `m`.
                    ///
                    /// # Examples
                    /// See [here](super::mod_shl#mod_shl).
                    #[inline]
                    fn mod_shl(self, other: $u, m: $t) -> $t {
                        mod_shl_signed(self, other, m)
                    }
                }

                impl ModShlAssign<$u, $t> for $t {
                    /// Left-shifts a number (multiplies it by a power of 2) modulo a number $m$, in
                    /// place. The number must be already reduced modulo $m$.
                    ///
                    /// $x \gets y$, where $x, y < m$ and $\lfloor 2^nx \rfloor \equiv y \mod m$.
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `other.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `self` is greater than or equal to `m`.
                    ///
                    /// # Examples
                    /// See [here](super::mod_shl#mod_shl_assign).
                    #[inline]
                    fn mod_shl_assign(&mut self, other: $u, m: $t) {
                        mod_shl_assign_signed(self, other, m);
                    }
                }
            };
        }
        apply_to_signeds!(impl_mod_shl_signed_inner);
    };
}
apply_to_unsigneds!(impl_mod_shl_signed);
