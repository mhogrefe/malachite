// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs,
};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::rounding_modes::RoundingMode;
use core::cmp::Ordering::{self, *};
use core::ops::{Shl, ShlAssign};

fn shl_round<
    T: PrimitiveInt + Shl<U, Output = T> + ShrRound<U, Output = T>,
    U,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
    rm: RoundingMode,
) -> (T, Ordering) {
    if bits >= S::ZERO {
        let width = S::wrapping_from(T::WIDTH);
        (
            if width >= S::ZERO && bits >= width {
                T::ZERO
            } else {
                x << bits.unsigned_abs()
            },
            Equal,
        )
    } else {
        x.shr_round(bits.unsigned_abs(), rm)
    }
}

fn shl_round_assign<
    T: PrimitiveInt + ShlAssign<U> + ShrRoundAssign<U>,
    U,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: &mut T,
    bits: S,
    rm: RoundingMode,
) -> Ordering {
    if bits >= S::ZERO {
        let width = S::wrapping_from(T::WIDTH);
        if width >= S::ZERO && bits >= width {
            *x = T::ZERO;
        } else {
            *x <<= bits.unsigned_abs();
        }
        Equal
    } else {
        x.shr_round_assign(bits.unsigned_abs(), rm)
    }
}

macro_rules! impl_shl_round {
    ($t:ident) => {
        macro_rules! impl_shl_round_inner {
            ($u:ident) => {
                impl ShlRound<$u> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2 or divides it by a power
                    /// of 2 and takes the floor) and rounds according to the specified rounding
                    /// mode. An [`Ordering`] is also returned, indicating whether the returned
                    /// value is less than, equal to, or greater than the exact value. If `bits` is
                    /// non-negative, then the returned [`Ordering`] is always `Equal`, even if the
                    /// higher bits of the result are lost.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `bits > 0 || self.divisible_by_power_of_2(bits)`.
                    /// Rounding might only be necessary if `bits` is negative.
                    ///
                    /// Let $q = x2^k$, and let $g$ be the function that just returns the first
                    /// element of the pair, without the [`Ordering`]:
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
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `Exact` but `self` is not divisible
                    /// by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shl_round#shl_round).
                    #[inline]
                    fn shl_round(self, bits: $u, rm: RoundingMode) -> ($t, Ordering) {
                        shl_round(self, bits, rm)
                    }
                }

                impl ShlRoundAssign<$u> for $t {
                    /// Left-shifts a number (multiplies it by a power of 2 or divides it by a power
                    /// of 2 and takes the floor) and rounds according to the specified rounding
                    /// mode, in place. An [`Ordering`] is returned, indicating whether the assigned
                    /// value is less than, equal to, or greater than the exact value. If `bits` is
                    /// non-negative, then the returned [`Ordering`] is always `Equal`, even if the
                    /// higher bits of the result are lost.
                    ///
                    /// Passing `Floor` or `Down` is equivalent to using `>>`. To test whether
                    /// `Exact` can be passed, use `bits > 0 || self.divisible_by_power_of_2(bits)`.
                    /// Rounding might only be necessary if `bits` is negative.
                    ///
                    /// See the [`ShlRound`] documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `Exact` but `self` is not divisible
                    /// by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shl_round#shl_round_assign).
                    #[inline]
                    fn shl_round_assign(&mut self, bits: $u, rm: RoundingMode) -> Ordering {
                        shl_round_assign(self, bits, rm)
                    }
                }
            };
        }
        apply_to_signeds!(impl_shl_round_inner);
    };
}
apply_to_primitive_ints!(impl_shl_round);
