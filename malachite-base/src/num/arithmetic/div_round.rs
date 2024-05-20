// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivRound, DivRoundAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::rounding_modes::RoundingMode::{self, *};
use core::cmp::Ordering::{self, *};

fn div_round_unsigned<T: PrimitiveUnsigned>(x: T, other: T, rm: RoundingMode) -> (T, Ordering) {
    let quotient = x / other;
    let remainder = x - quotient * other;
    match rm {
        _ if remainder == T::ZERO => (quotient, Equal),
        Down | Floor => (quotient, Less),
        Up | Ceiling => (quotient + T::ONE, Greater),
        Nearest => {
            let shifted_other = other >> 1;
            if remainder > shifted_other
                || remainder == shifted_other && other.even() && quotient.odd()
            {
                (quotient + T::ONE, Greater)
            } else {
                (quotient, Less)
            }
        }
        Exact => {
            panic!("Division is not exact: {x} / {other}");
        }
    }
}

macro_rules! impl_div_round_unsigned {
    ($t:ident) => {
        impl DivRound<$t> for $t {
            type Output = $t;

            /// Divides a value by another value and rounds according to a specified rounding mode.
            /// An [`Ordering`] is also returned, indicating whether the returned value is less
            /// than, equal to, or greater than the exact value.
            ///
            /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $$
            /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor &
            ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
            ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
            ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// Then
            ///
            /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round).
            #[inline]
            fn div_round(self, other: $t, rm: RoundingMode) -> ($t, Ordering) {
                div_round_unsigned(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode. An [`Ordering`] is returned, indicating whether the assigned value is
            /// less than, equal to, or greater than the exact value.
            ///
            /// See the [`DivRound`] documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round_assign).
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) -> Ordering {
                let o;
                (*self, o) = self.div_round(other, rm);
                o
            }
        }
    };
}
apply_to_unsigneds!(impl_div_round_unsigned);

fn div_round_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    other: S,
    rm: RoundingMode,
) -> (S, Ordering) {
    if (x >= S::ZERO) == (other >= S::ZERO) {
        let (q, o) = x.unsigned_abs().div_round(other.unsigned_abs(), rm);
        (S::exact_from(q), o)
    } else {
        // Has to be wrapping so that (self, other) == (T::MIN, 1) works
        let (q, o) = x.unsigned_abs().div_round(other.unsigned_abs(), -rm);
        (S::wrapping_from(q).wrapping_neg(), o.reverse())
    }
}

macro_rules! impl_div_round_signed {
    ($t:ident) => {
        impl DivRound<$t> for $t {
            type Output = $t;

            /// Divides a value by another value and rounds according to a specified rounding mode.
            /// An [`Ordering`] is also returned, indicating whether the returned value is less
            /// than, equal to, or greater than the exact value.
            ///
            /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]:
            ///
            /// $$
            /// g(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///     \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
            ///
            /// Then
            ///
            /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `Self::MIN` and `other` is `-1`, or if `rm`
            /// is `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round).
            fn div_round(self, other: $t, rm: RoundingMode) -> ($t, Ordering) {
                div_round_signed(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode. An [`Ordering`] is returned, indicating whether the assigned value is
            /// less than, equal to, or greater than the exact value.
            ///
            /// See the [`DivRound`] documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `Self::MIN` and `other` is `-1`, or if `rm`
            /// is `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round_assign).
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) -> Ordering {
                let o;
                (*self, o) = self.div_round(other, rm);
                o
            }
        }
    };
}
apply_to_signeds!(impl_div_round_signed);
