// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    Abs, Average, AverageAssign, AverageRound, AverageRoundAssign,
};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::rounding_modes::RoundingMode::{self, *};
use core::cmp::Ordering::{self, *};

// Since x + y == 2(x & y) + (x ^ y), the floor of the average is (x & y) + ((x ^ y) >> 1), with an
// arithmetic shift for signed types, and neither the shift nor the addition can overflow. The
// average is either exact or a half more than the floor, so the ceiling, when it differs from the
// floor, is one more and cannot overflow either.
fn average_round_primitive<T: PrimitiveInt>(x: T, y: T, rm: RoundingMode) -> (T, Ordering) {
    let floor = (x & y) + ((x ^ y) >> 1);
    if (x ^ y).even() {
        return (floor, Equal);
    }
    match rm {
        Floor => (floor, Less),
        Ceiling => (floor + T::ONE, Greater),
        Down => {
            if floor < T::ZERO {
                (floor + T::ONE, Greater)
            } else {
                (floor, Less)
            }
        }
        Up => {
            if floor < T::ZERO {
                (floor, Less)
            } else {
                (floor + T::ONE, Greater)
            }
        }
        Nearest => {
            if floor.even() {
                (floor, Less)
            } else {
                (floor + T::ONE, Greater)
            }
        }
        Exact => {
            panic!("Average is not exact: ({x} + {y}) / 2");
        }
    }
}

macro_rules! impl_average {
    ($t:ident) => {
        impl AverageRound<$t> for $t {
            type Output = $t;

            /// Computes the average (arithmetic mean) of two numbers and rounds according to a
            /// specified rounding mode. An [`Ordering`] is also returned, indicating whether the
            /// returned value is less than, equal to, or greater than the exact value.
            ///
            /// The average is computed without overflow; the result always fits in the same type as
            /// the inputs.
            ///
            /// Let $a = \frac{x + y}{2}$, and let $g$ be the function that just returns the first
            /// element of the pair, without the [`Ordering`]. Since $a$ is either an integer or a
            /// half more than an integer,
            ///
            /// $$
            /// g(x, y, \mathrm{Floor}) = \lfloor a \rfloor,
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Ceiling}) = \lceil a \rceil,
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Down}) = \operatorname{sgn}(a) \lfloor |a| \rfloor,
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Up}) = \operatorname{sgn}(a) \lceil |a| \rceil,
            /// $$
            ///
            /// $$
            /// g(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     a & \text{if} \\quad a \in \Z, \\\\
            ///     \lfloor a \rfloor & \text{if} \\quad a \notin \Z
            ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is even}, \\\\
            ///     \lceil a \rceil & \text{if} \\quad a \notin \Z
            ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is odd,}
            /// \end{cases}
            /// $$
            ///
            /// and $g(x, y, \mathrm{Exact}) = a$, but panics if $a \notin \Z$.
            ///
            /// Then
            ///
            /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), a))$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
            ///
            /// # Examples
            /// See [here](super::average#average_round).
            #[inline]
            fn average_round(self, other: $t, rm: RoundingMode) -> ($t, Ordering) {
                average_round_primitive(self, other, rm)
            }
        }

        impl AverageRoundAssign<$t> for $t {
            /// Computes the average (arithmetic mean) of two numbers, rounding according to a
            /// specified rounding mode and replacing the first number with it. An [`Ordering`] is
            /// returned, indicating whether the assigned value is less than, equal to, or greater
            /// than the exact value.
            ///
            /// The average is computed without overflow; the result always fits in the same type as
            /// the inputs.
            ///
            /// See the [`AverageRound`](super::traits::AverageRound) documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `rm` is `Exact` but the average of `self` and `other` is not an integer.
            ///
            /// # Examples
            /// See [here](super::average#average_round_assign).
            #[inline]
            fn average_round_assign(&mut self, other: $t, rm: RoundingMode) -> Ordering {
                let o;
                (*self, o) = average_round_primitive(*self, other, rm);
                o
            }
        }

        impl Average<$t> for $t {
            type Output = $t;

            /// Computes the average (arithmetic mean) of two numbers, rounding to the nearest
            /// integer. Two-way ties are broken by rounding to the even integer.
            ///
            /// The average is computed without overflow; the result always fits in the same type as
            /// the inputs. This is equivalent to
            /// [`average_round`](super::traits::AverageRound::average_round) with
            /// [`Nearest`](crate::rounding_modes::RoundingMode::Nearest).
            ///
            /// $$
            /// f(x, y) = \begin{cases}
            ///     a & \text{if} \\quad a \in \Z, \\\\
            ///     \lfloor a \rfloor & \text{if} \\quad a \notin \Z
            ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is even}, \\\\
            ///     \lceil a \rceil & \text{if} \\quad a \notin \Z
            ///     \\ \text{and} \\ \lfloor a \rfloor \\ \text{is odd,}
            /// \end{cases}
            /// $$
            ///
            /// where $a = \frac{x + y}{2}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::average#average).
            #[inline]
            fn average(self, other: $t) -> $t {
                average_round_primitive(self, other, Nearest).0
            }
        }

        impl AverageAssign<$t> for $t {
            /// Computes the average (arithmetic mean) of two numbers, rounding to the nearest
            /// integer and replacing the first number with it. Two-way ties are broken by rounding
            /// to the even integer.
            ///
            /// The average is computed without overflow; the result always fits in the same type as
            /// the inputs.
            ///
            /// See the [`Average`](super::traits::Average) documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::average#average_assign).
            #[inline]
            fn average_assign(&mut self, other: $t) {
                *self = average_round_primitive(*self, other, Nearest).0;
            }
        }
    };
}
apply_to_primitive_ints!(impl_average);

// The three-case midpoint algorithm used by C++'s `std::midpoint`. When both inputs are at most
// half the maximum, the sum cannot overflow, and (x + y) / 2 is correctly rounded: an addition
// whose result lands in the subnormal range is exact, so the sum and the halving never both round.
// Otherwise at least one input is huge. An input too small to halve exactly is added whole; its
// halving error is far below the rounding quantum of the huge input's half, so the result is
// unaffected. Everything else is halved exactly first.
fn average_primitive_float<T: PrimitiveFloat>(x: T, y: T) -> T {
    if !x.is_finite() || !y.is_finite() {
        // for infinities and NaNs, behave exactly like the naive expression
        return (x + y) / T::TWO;
    }
    let half_max = T::MAX_FINITE / T::TWO;
    let double_min = T::MIN_POSITIVE_NORMAL * T::TWO;
    let abs_x = x.abs();
    let abs_y = y.abs();
    if abs_x <= half_max && abs_y <= half_max {
        (x + y) / T::TWO
    } else if abs_x < double_min {
        x + y / T::TWO
    } else if abs_y < double_min {
        x / T::TWO + y
    } else {
        x / T::TWO + y / T::TWO
    }
}

macro_rules! impl_average_primitive_float {
    ($t:ident) => {
        impl Average<$t> for $t {
            type Output = $t;

            /// Computes the average (arithmetic mean) of two floating-point numbers.
            ///
            /// For finite values the result is the correctly rounded average: the nearest
            /// representable value, with ties going to the value with the even mantissa. The
            /// computation avoids intermediate overflow and underflow, so extreme values average
            /// correctly. If either value is infinite or `NaN`, the result is whatever `(x + y) /
            /// 2.0` produces.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::average#average).
            #[inline]
            fn average(self, other: $t) -> $t {
                average_primitive_float(self, other)
            }
        }

        impl AverageAssign<$t> for $t {
            /// Computes the average (arithmetic mean) of two floating-point numbers, replacing the
            /// first number with it.
            ///
            /// For finite values the result is the correctly rounded average: the nearest
            /// representable value, with ties going to the value with the even mantissa. The
            /// computation avoids intermediate overflow and underflow, so extreme values average
            /// correctly. If either value is infinite or `NaN`, the result is whatever `(x + y) /
            /// 2.0` produces.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::average#average_assign).
            #[inline]
            fn average_assign(&mut self, other: $t) {
                *self = average_primitive_float(*self, other);
            }
        }
    };
}
apply_to_primitive_floats!(impl_average_primitive_float);
