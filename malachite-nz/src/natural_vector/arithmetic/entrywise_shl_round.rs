// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    EntrywiseShlRound, EntrywiseShlRoundAssign, ShlRound, ShlRoundAssign,
};
use malachite_base::rounding_modes::RoundingMode;

// Shifts every element left by `bits`, rounding each according to `rm`.
fn entrywise_shl_round_ref<T: Copy>(v: &NaturalVector, bits: T, rm: RoundingMode) -> NaturalVector
where
    for<'a> &'a Natural: ShlRound<T, Output = Natural>,
{
    NaturalVector {
        elements: v.elements.iter().map(|x| x.shl_round(bits, rm).0).collect(),
    }
}

// Shifts every element left by `bits`, rounding each according to `rm`, in place.
fn entrywise_shl_round_assign<T: Copy>(v: &mut NaturalVector, bits: T, rm: RoundingMode)
where
    Natural: ShlRoundAssign<T>,
{
    for x in &mut v.elements {
        x.shl_round_assign(bits, rm);
    }
}

macro_rules! impl_natural_vector_entrywise_shl_round_signed {
    ($t:ident) => {
        impl EntrywiseShlRound<$t> for NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies or divides it by a power of 2), taking
            /// it by value, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `<<`.
            ///
            /// Rounding might only be necessary if `bits` is negative.
            ///
            /// $f(v, k, r)_i = \operatorname{round}(2^kv_i)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact`, `bits` is negative, and some element is not divisible by
            /// $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shl_round#entrywise_shl_round).
            #[inline]
            fn entrywise_shl_round(mut self, bits: $t, rm: RoundingMode) -> NaturalVector {
                self.entrywise_shl_round_assign(bits, rm);
                self
            }
        }

        impl EntrywiseShlRound<$t> for &NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies or divides it by a power of 2), taking
            /// it by reference, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `<<`.
            ///
            /// Rounding might only be necessary if `bits` is negative.
            ///
            /// $f(v, k, r)_i = \operatorname{round}(2^kv_i)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact`, `bits` is negative, and some element is not divisible by
            /// $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shl_round#entrywise_shl_round).
            #[inline]
            fn entrywise_shl_round(self, bits: $t, rm: RoundingMode) -> NaturalVector {
                entrywise_shl_round_ref(self, bits, rm)
            }
        }

        impl EntrywiseShlRoundAssign<$t> for NaturalVector {
            /// Left-shifts a [`NaturalVector`] (multiplies or divides it by a power of 2), in
            /// place, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `<<`.
            ///
            /// Rounding might only be necessary if `bits` is negative.
            ///
            /// $v_i \gets \operatorname{round}(2^kv_i)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact`, `bits` is negative, and some element is not divisible by
            /// $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shl_round#entrywise_shl_round_assign).
            #[inline]
            fn entrywise_shl_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                entrywise_shl_round_assign(self, bits, rm);
            }
        }
    };
}
apply_to_signeds!(impl_natural_vector_entrywise_shl_round_signed);
