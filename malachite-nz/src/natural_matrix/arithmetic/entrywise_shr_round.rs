// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{
    EntrywiseShrRound, EntrywiseShrRoundAssign, ShrRound, ShrRoundAssign,
};
use malachite_base::rounding_modes::RoundingMode;

// Shifts every element right by `bits`, rounding each according to `rm`.
fn entrywise_shr_round_ref<T: Copy>(a: &NaturalMatrix, bits: T, rm: RoundingMode) -> NaturalMatrix
where
    for<'a> &'a Natural: ShrRound<T, Output = Natural>,
{
    NaturalMatrix {
        height: a.height,
        width: a.width,
        elements: a.elements.iter().map(|x| x.shr_round(bits, rm).0).collect(),
    }
}

// Shifts every element right by `bits`, rounding each according to `rm`, in place.
fn entrywise_shr_round_assign<T: Copy>(a: &mut NaturalMatrix, bits: T, rm: RoundingMode)
where
    Natural: ShrRoundAssign<T>,
{
    for x in &mut a.elements {
        x.shr_round_assign(bits, rm);
    }
}

macro_rules! impl_natural_matrix_entrywise_shr_round_unsigned {
    ($t:ident) => {
        impl EntrywiseShrRound<$t> for NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides it by a power of 2), taking it by value,
            /// and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`.
            ///
            /// $f(A, k, r)_{ij} = \operatorname{round}(a_{ij}/2^k)$, where $\operatorname{round}$
            /// rounds according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n, k) = O(n + k)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, and $k$ is the number of elements.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round).
            ///
            /// With `rm` equal to `Down`, this is equivalent to `fmpz_mat_scalar_tdiv_q_2exp` from
            /// `fmpz_mat/scalar.c`, FLINT 3.6.0.
            #[inline]
            fn entrywise_shr_round(mut self, bits: $t, rm: RoundingMode) -> NaturalMatrix {
                self.entrywise_shr_round_assign(bits, rm);
                self
            }
        }

        impl EntrywiseShrRound<$t> for &NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides it by a power of 2), taking it by
            /// reference, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`.
            ///
            /// $f(A, k, r)_{ij} = \operatorname{round}(a_{ij}/2^k)$, where $\operatorname{round}$
            /// rounds according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n, k) = O(n + k)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, and $k$ is the number of elements.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round).
            ///
            /// With `rm` equal to `Down`, this is equivalent to `fmpz_mat_scalar_tdiv_q_2exp` from
            /// `fmpz_mat/scalar.c`, FLINT 3.6.0.
            #[inline]
            fn entrywise_shr_round(self, bits: $t, rm: RoundingMode) -> NaturalMatrix {
                entrywise_shr_round_ref(self, bits, rm)
            }
        }

        impl EntrywiseShrRoundAssign<$t> for NaturalMatrix {
            /// Right-shifts a [`NaturalMatrix`] (divides it by a power of 2), in place, and rounds
            /// every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`.
            ///
            /// $a_{ij} \gets \operatorname{round}(a_{ij}/2^k)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(k) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, and $k$ is the number of elements.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round_assign).            ///
            /// With `rm` equal to `Down`, this is equivalent to `fmpz_mat_scalar_tdiv_q_2exp` from
            /// `fmpz_mat/scalar.c`, FLINT 3.6.0.
            #[inline]
            fn entrywise_shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                entrywise_shr_round_assign(self, bits, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_matrix_entrywise_shr_round_unsigned);

macro_rules! impl_natural_matrix_entrywise_shr_round_signed {
    ($t:ident) => {
        impl EntrywiseShrRound<$t> for NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides or multiplies it by a power of 2), taking
            /// it by value, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`.
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$; no rounding is
            /// needed then.
            ///
            /// $f(A, k, r)_{ij} = \operatorname{round}(a_{ij}/2^k)$, where $\operatorname{round}$
            /// rounds according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is the number of elements.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round).
            #[inline]
            fn entrywise_shr_round(mut self, bits: $t, rm: RoundingMode) -> NaturalMatrix {
                self.entrywise_shr_round_assign(bits, rm);
                self
            }
        }

        impl EntrywiseShrRound<$t> for &NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides or multiplies it by a power of 2), taking
            /// it by reference, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`.
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$; no rounding is
            /// needed then.
            ///
            /// $f(A, k, r)_{ij} = \operatorname{round}(a_{ij}/2^k)$, where $\operatorname{round}$
            /// rounds according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is the number of elements.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round).
            #[inline]
            fn entrywise_shr_round(self, bits: $t, rm: RoundingMode) -> NaturalMatrix {
                entrywise_shr_round_ref(self, bits, rm)
            }
        }

        impl EntrywiseShrRoundAssign<$t> for NaturalMatrix {
            /// Right-shifts a [`NaturalMatrix`] (divides or multiplies it by a power of 2), in
            /// place, and rounds every element according to the specified rounding mode.
            ///
            /// Each element is rounded as by
            /// [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound), but no
            /// [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different
            /// elements may be rounded in different directions. Passing `Floor` or `Down` is
            /// equivalent to using `>>`.
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$; no rounding is
            /// needed then.
            ///
            /// $a_{ij} \gets \operatorname{round}(a_{ij}/2^k)$, where $\operatorname{round}$ rounds
            /// according to `rm`.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is the number of elements.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` and some element is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::entrywise_shr_round#entrywise_shr_round_assign).
            #[inline]
            fn entrywise_shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                entrywise_shr_round_assign(self, bits, rm);
            }
        }
    };
}
apply_to_signeds!(impl_natural_matrix_entrywise_shr_round_signed);
