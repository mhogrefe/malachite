// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use core::ops::{Shl, ShlAssign};

// Shifts every element left by `bits`.
fn shl_ref<T: Copy>(v: &NaturalVector, bits: T) -> NaturalVector
where
    for<'a> &'a Natural: Shl<T, Output = Natural>,
{
    NaturalVector {
        elements: v.elements.iter().map(|x| x << bits).collect(),
    }
}

// Shifts every element left by `bits`, in place.
fn shl_assign<T: Copy>(v: &mut NaturalVector, bits: T)
where
    Natural: ShlAssign<T>,
{
    for x in &mut v.elements {
        *x <<= bits;
    }
}

macro_rules! impl_natural_vector_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies it by a power of 2), taking it by value.
            /// Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = 2^kv$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            ///
            /// This is equivalent to `_fmpz_vec_scalar_mul_2exp` from `fmpz_vec/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shl(mut self, bits: $t) -> NaturalVector {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies it by a power of 2), taking it by
            /// reference. Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = 2^kv$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            ///
            /// This is equivalent to `_fmpz_vec_scalar_mul_2exp` from `fmpz_vec/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shl(self, bits: $t) -> NaturalVector {
                shl_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for NaturalVector {
            /// Left-shifts a [`NaturalVector`] (multiplies it by a power of 2), in place. Every
            /// element is shifted, and the dimension is unchanged.
            ///
            /// $v \gets 2^kv$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            ///
            /// This is equivalent to `_fmpz_vec_scalar_mul_2exp` from `fmpz_vec/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_vector_shl_unsigned);

macro_rules! impl_natural_vector_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies or divides it by a power of 2), taking
            /// it by value. Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = 2^kv$.
            ///
            /// A negative `bits` shifts right, rounding every element down (toward negative
            /// infinity); to round differently, use
            /// [`entrywise_shl_round`](malachite_base::num::arithmetic::traits::EntrywiseShlRound).
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> NaturalVector {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies or divides it by a power of 2), taking
            /// it by reference. Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = 2^kv$.
            ///
            /// A negative `bits` shifts right, rounding every element down (toward negative
            /// infinity); to round differently, use
            /// [`entrywise_shl_round`](malachite_base::num::arithmetic::traits::EntrywiseShlRound).
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> NaturalVector {
                shl_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for NaturalVector {
            /// Left-shifts a [`NaturalVector`] (multiplies or divides it by a power of 2), in
            /// place. Every element is shifted, and the dimension is unchanged.
            ///
            /// $v \gets 2^kv$.
            ///
            /// A negative `bits` shifts right, rounding every element down (toward negative
            /// infinity); to round differently, use
            /// [`entrywise_shl_round`](malachite_base::num::arithmetic::traits::EntrywiseShlRound).
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_natural_vector_shl_signed);
