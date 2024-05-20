// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering::{self, *};
use core::ops::{Shl, ShlAssign};
use malachite_base::num::arithmetic::traits::{
    ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;

fn shl_round_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
    rm: RoundingMode,
) -> (Integer, Ordering)
where
    &'a Integer: Shl<U, Output = Integer> + ShrRound<U, Output = Integer>,
{
    if bits >= S::ZERO {
        (x << bits.unsigned_abs(), Equal)
    } else {
        x.shr_round(bits.unsigned_abs(), rm)
    }
}

fn shl_round_assign_i<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Integer,
    bits: S,
    rm: RoundingMode,
) -> Ordering
where
    Integer: ShlAssign<U> + ShrRoundAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
        Equal
    } else {
        x.shr_round_assign(bits.unsigned_abs(), rm)
    }
}

macro_rules! impl_shl_round_signed {
    ($t:ident) => {
        impl ShlRound<$t> for Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies or divides it by a power of 2), taking it by
            /// value, and rounds according to the specified rounding mode. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the exact value. If `bits` is non-negative, then the returned [`Ordering`] is
            /// always `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
            /// necessary if `bits` is negative.
            ///
            /// Let $q = x2^k$, and let $g$ be the function that just returns the first element of
            /// the pair, without the [`Ordering`]:
            ///
            /// $g(x, k, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $g(x, k, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// g(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $g(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is negative and `rm` is `Exact` but `self` is not
            /// divisible by $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round).
            #[inline]
            fn shl_round(mut self, bits: $t, rm: RoundingMode) -> (Integer, Ordering) {
                let o = self.shl_round_assign(bits, rm);
                (self, o)
            }
        }

        impl<'a> ShlRound<$t> for &'a Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies or divides it by a power of 2), taking it by
            /// reference, and rounds according to the specified rounding mode. An [`Ordering`] is
            /// also returned, indicating whether the returned value is less than, equal to, or
            /// greater than the exact value. If `bits` is non-negative, then the returned
            /// [`Ordering`] is always `Equal`, even if the higher bits of the result are lost.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
            /// necessary if `bits` is negative.
            ///
            /// Let $q = x2^k$, and let $g$ be the function that just returns the first element of
            /// the pair, without the [`Ordering`]:
            ///
            /// $g(x, k, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $g(x, k, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// g(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $g(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, k, r) = (g(x, k, r), \operatorname{cmp}(g(x, k, r), q))$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is negative and `rm` is `Exact` but `self` is not
            /// divisible by $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round).
            #[inline]
            fn shl_round(self, bits: $t, rm: RoundingMode) -> (Integer, Ordering) {
                shl_round_signed_ref(self, bits, rm)
            }
        }

        impl ShlRoundAssign<$t> for Integer {
            /// Left-shifts an [`Integer`] (multiplies or divides it by a power of 2) and rounds
            /// according to the specified rounding mode, in place. An [`Ordering`] is returned,
            /// indicating whether the assigned value is less than, equal to, or greater than the
            /// exact value.
            ///
            /// Passing `Floor` is equivalent to using `>>`. To test whether `Exact` can be passed,
            /// use `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
            /// necessary if `bits` is negative.
            ///
            /// See the [`ShlRound`] documentation for details.
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
            /// See [here](super::shl_round#shl_round_assign).
            #[inline]
            fn shl_round_assign(&mut self, bits: $t, rm: RoundingMode) -> Ordering {
                shl_round_assign_i(self, bits, rm)
            }
        }
    };
}
apply_to_signeds!(impl_shl_round_signed);
