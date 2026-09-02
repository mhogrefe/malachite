// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::ops::{Shl, ShlAssign};

macro_rules! impl_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for GaussianInteger {
            type Output = GaussianInteger;

            /// Left-shifts a [`GaussianInteger`] (multiplies it by a power of 2), taking it by
            /// value. Both parts are shifted.
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
            /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self`, and $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> GaussianInteger {
                GaussianInteger {
                    real: self.real << bits,
                    imaginary: self.imaginary << bits,
                }
            }
        }

        impl Shl<$t> for &GaussianInteger {
            type Output = GaussianInteger;

            /// Left-shifts a [`GaussianInteger`] (multiplies it by a power of 2), taking it by
            /// reference. Both parts are shifted.
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
            /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self`, and $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> GaussianInteger {
                GaussianInteger {
                    real: &self.real << bits,
                    imaginary: &self.imaginary << bits,
                }
            }
        }

        impl ShlAssign<$t> for GaussianInteger {
            /// Left-shifts a [`GaussianInteger`] (multiplies it by a power of 2), in place. Both
            /// parts are shifted.
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
            /// where $T$ is time, $M$ is additional memory, $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self`, and $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                self.real <<= bits;
                self.imaginary <<= bits;
            }
        }
    };
}
apply_to_unsigneds!(impl_shl_unsigned);
