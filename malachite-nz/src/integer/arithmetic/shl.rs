// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;

fn shl_unsigned<T>(x: Integer, bits: T) -> Integer
where
    Natural: Shl<T, Output = Natural>,
{
    Integer {
        sign: x.sign,
        abs: x.abs << bits,
    }
}

fn shl_unsigned_ref<'a, T>(x: &'a Integer, bits: T) -> Integer
where
    &'a Natural: Shl<T, Output = Natural>,
{
    Integer {
        sign: x.sign,
        abs: &x.abs << bits,
    }
}

macro_rules! impl_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Integer {
                shl_unsigned(self, bits)
            }
        }

        impl<'a> Shl<$t> for &Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies it by a power of 2), taking it by reference.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Integer {
                shl_unsigned_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Integer {
            /// Left-shifts an [`Integer`] (multiplies it by a power of 2), in place.
            ///
            /// $$
            /// x \gets x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                self.abs <<= bits;
            }
        }
    };
}
apply_to_unsigneds!(impl_shl_unsigned);

fn shl_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
) -> Integer
where
    &'a Integer: Shl<U, Output = Integer> + Shr<U, Output = Integer>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x >> bits.unsigned_abs()
    }
}

fn shl_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Integer, bits: S)
where
    Integer: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies it by a power of 2 or divides it by a power
            /// of 2 and takes the floor), taking it by value.
            ///
            /// $$
            /// f(x, k) = \lfloor x2^k \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> Integer {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies it by a power of 2 or divides it by a power
            /// of 2 and takes the floor), taking it by reference.
            ///
            /// $$
            /// f(x, k) = \lfloor x2^k \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Integer {
                shl_signed_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Integer {
            /// Left-shifts an [`Integer`] (multiplies it by a power of 2 or divides it by a power
            /// of 2 and takes the floor), in place.
            ///
            /// $$
            /// x \gets \lfloor x2^k \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            fn shl_assign(&mut self, bits: $t) {
                shl_assign_signed(self, bits)
            }
        }
    };
}
apply_to_signeds!(impl_shl_signed);
