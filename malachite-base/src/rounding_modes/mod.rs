// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::named::Named;
use crate::rounding_modes::RoundingMode::*;

/// An enum that specifies how a value should be rounded.
///
/// A `RoundingMode` can often be specified when a function conceptually returns a value of one
/// type, but must be rounded to another type. The most common case is a conceptually real-valued
/// function whose result must be rounded to an integer, like
/// [`div_round`](crate::num::arithmetic::traits::DivRound::div_round).
///
/// # Examples
/// Here are some examples of how floating-point values would be rounded to integer values using the
/// different `RoundingMode`s.
///
/// | x    | `Floor` | `Ceiling` | `Down` | `Up` | `Nearest` | `Exact`    |
/// |------|---------|-----------|--------|------|-----------|------------|
/// |  3.0 |       3 |         3 |      3 |    3 |         3 |          3 |
/// |  3.2 |       3 |         4 |      3 |    4 |         3 | `panic!()` |
/// |  3.8 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  3.5 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  4.5 |       4 |         5 |      4 |    5 |         4 | `panic!()` |
/// | -3.2 |      -4 |        -3 |     -3 |   -4 |        -3 | `panic!()` |
/// | -3.8 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -3.5 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -4.5 |      -5 |        -4 |     -4 |   -5 |        -4 | `panic!()` |
///
/// Sometimes a `RoundingMode` is used in an unusual context, such as rounding an integer to a
/// floating-point number, in which case further explanation of its behavior is provided at the
/// usage site.
///
/// A `RoundingMode` takes up 1 byte of space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RoundingMode {
    /// Applies the function $x \mapsto \operatorname{sgn}(x) \lfloor |x| \rfloor$. In other words,
    /// the value is rounded towards $0$.
    Down,
    /// Applies the function $x \mapsto \operatorname{sgn}(x) \lceil |x| \rceil$. In other words,
    /// the value is rounded away from $0$.
    Up,
    /// Applies the floor function: $x \mapsto \lfloor x \rfloor$. In other words, the value is
    /// rounded towards $-\infty$.
    Floor,
    /// Applies the ceiling function: $x \mapsto \lceil x \rceil$. In other words, the value is
    /// rounded towards $\infty$.
    Ceiling,
    /// Applies the function
    /// $$
    ///   x \mapsto \\begin{cases}
    ///       \lfloor x \rfloor & x - \lfloor x \rfloor < \frac{1}{2} \\\\
    ///       \lceil x \rceil & x - \lfloor x \rfloor > \frac{1}{2} \\\\
    ///       \lfloor x \rfloor &
    ///  x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and}
    ///         \\ \lfloor x \rfloor \\ \text{is even} \\\\
    ///       \lceil x \rceil &
    ///  x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor x \rfloor \\ \text{is odd.}
    ///   \\end{cases}
    /// $$
    /// In other words, it rounds to the nearest integer, and when there's a tie, it rounds to the
    /// nearest even integer. This is also called _bankers' rounding_ and is often used as a
    /// default.
    Nearest,
    /// Panics if the value is not already rounded.
    Exact,
}

impl_named!(RoundingMode);

/// A list of all six rounding modes.
pub const ROUNDING_MODES: [RoundingMode; 6] = [Down, Up, Floor, Ceiling, Nearest, Exact];

/// Iterators that generate [`RoundingMode`]s without repetition.
pub mod exhaustive;
/// Functions for converting a string to a [`RoundingMode`].
pub mod from_str;
/// Functions for negating a [`RoundingMode`].
pub mod neg;
#[cfg(feature = "random")]
/// Iterators that generate [`RoundingMode`]s randomly.
pub mod random;
/// Functions for displaying a [`RoundingMode`].
pub mod to_string;
