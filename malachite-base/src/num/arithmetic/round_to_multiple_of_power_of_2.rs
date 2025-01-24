// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign};
use crate::num::basic::integers::PrimitiveInt;
use crate::rounding_modes::RoundingMode;
use core::cmp::Ordering;

fn round_to_multiple_of_power_of_2<T: PrimitiveInt>(
    x: T,
    pow: u64,
    rm: RoundingMode,
) -> (T, Ordering) {
    let (s, o) = x.shr_round(pow, rm);
    (s.arithmetic_checked_shl(pow).unwrap(), o)
}

macro_rules! impl_round_to_multiple_of_power_of_2 {
    ($t:ident) => {
        impl RoundToMultipleOfPowerOf2<u64> for $t {
            type Output = $t;

            /// Rounds a number to a multiple of $2^k$ according to a specified rounding mode. An
            /// [`Ordering`] is also returned, indicating whether the returned value is less than,
            /// equal to, or greater than the original value.
            ///
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// Let $q = \frac{x}{2^k}$:
            ///
            /// $f(x, k, \mathrm{Down}) = 2^k \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = 2^k \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     2^k \lfloor q \rfloor & \text{if} \\quad
            ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
            ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
            ///     2^k \lfloor q \rfloor &
            ///     \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even} \\\\
            ///     2^k \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
            ///
            /// The following two expressions are equivalent:
            /// - `x.round_to_multiple_of_power_of_2(pow, Exact)`
            /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of the power of 2.
            /// - If `rm` is `Floor`, but `self` is negative with a too-large absolute value to
            ///   round to the next lowest multiple.
            /// - If `rm` is `Ceiling`, but `self` is too large to round to the next highest
            ///   multiple.
            /// - If `rm` is `Up`, but `self` has too large an absolute value to round to the next
            ///   multiple with a greater absolute value.
            /// - If `rm` is `Nearest`, but the nearest multiple is outside the representable range.
            ///
            /// # Examples
            /// See [here](super::round_to_multiple_of_power_of_2#round_to_multiple_of_power_of_2).
            #[inline]
            fn round_to_multiple_of_power_of_2(self, pow: u64, rm: RoundingMode) -> ($t, Ordering) {
                round_to_multiple_of_power_of_2(self, pow, rm)
            }
        }

        impl RoundToMultipleOfPowerOf2Assign<u64> for $t {
            /// Rounds a number to a multiple of $2^k$ in place, according to a specified rounding
            /// mode. An [`Ordering`] is returned, indicating whether the returned value is less
            /// than, equal to, or greater than the original value.
            ///
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// See the [`RoundToMultipleOfPowerOf2`] documentation for details.
            ///
            /// The following two expressions are equivalent:
            /// - `x.round_to_multiple_of_power_of_2_assign(pow, Exact);`
            /// - `assert!(x.divisible_by_power_of_2(pow));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of the power of 2.
            /// - If `rm` is `Floor`, but `self` is negative with a too-large absolute value to
            ///   round to the next lowest multiple.
            /// - If `rm` is `Ceiling`, but `self` is too large to round to the next highest
            ///   multiple.
            /// - If `rm` is `Up`, but `self` has too large an absolute value to round to the next
            ///   multiple with a greater absolute value.
            /// - If `rm` is `Nearest`, but the nearest multiple is outside the representable range.
            ///
            /// # Examples
            /// See
            /// [here](super::round_to_multiple_of_power_of_2#round_to_multiple_of_power_of_2_assign).
            #[inline]
            fn round_to_multiple_of_power_of_2_assign(
                &mut self,
                pow: u64,
                rm: RoundingMode,
            ) -> Ordering {
                let o;
                (*self, o) = self.round_to_multiple_of_power_of_2(pow, rm);
                o
            }
        }
    };
}
apply_to_primitive_ints!(impl_round_to_multiple_of_power_of_2);
