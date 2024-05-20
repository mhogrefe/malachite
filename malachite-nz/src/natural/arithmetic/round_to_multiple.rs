// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl RoundToMultiple<Natural> for Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified rounding
    /// mode. Both [`Natural`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(5u32)
    ///         .round_to_multiple(Natural::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(Natural::from(4u32), Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(Natural::from(4u32), Up)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(Natural::from(5u32), Exact)
    ///         .to_debug_string(),
    ///     "(10, Equal)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(9, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .round_to_multiple(Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(21, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32)
    ///         .round_to_multiple(Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(16, Greater)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let o = self.round_to_multiple_assign(other, rm);
        (self, o)
    }
}

impl<'a> RoundToMultiple<&'a Natural> for Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified rounding
    /// mode. The first [`Natural`] is taken by value and the second by reference. An [`Ordering`]
    /// is also returned, indicating whether the returned value is less than, equal to, or greater
    /// than the original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(5u32)
    ///         .round_to_multiple(&Natural::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(&Natural::from(4u32), Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(&Natural::from(4u32), Up)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(&Natural::from(5u32), Exact)
    ///         .to_debug_string(),
    ///     "(10, Equal)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(&Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(9, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .round_to_multiple(&Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(21, Greater)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .round_to_multiple(&Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32)
    ///         .round_to_multiple(&Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(16, Greater)"
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: &'a Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let o = self.round_to_multiple_assign(other, rm);
        (self, o)
    }
}

impl<'a> RoundToMultiple<Natural> for &'a Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified rounding
    /// mode. The first [`Natural`] is taken by reference and the second by value. An [`Ordering`]
    /// is also returned, indicating whether the returned value is less than, equal to, or greater
    /// than the original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(5u32))
    ///         .round_to_multiple(Natural::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(Natural::from(4u32), Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(Natural::from(4u32), Up)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(Natural::from(5u32), Exact)
    ///         .to_debug_string(),
    ///     "(10, Equal)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(9, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .round_to_multiple(Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(21, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32))
    ///         .round_to_multiple(Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(16, Greater)"
    /// );
    /// ```
    fn round_to_multiple(self, other: Natural, rm: RoundingMode) -> (Natural, Ordering) {
        match (self, other) {
            (x, y) if *x == y => (y, Equal),
            (x, Natural::ZERO) => match rm {
                Down | Floor | Nearest => (Natural::ZERO, Less),
                _ => panic!("Cannot round {x} to zero using RoundingMode {rm}"),
            },
            (x, y) => {
                let r = x % &y;
                if r == 0 {
                    (x.clone(), Equal)
                } else {
                    let floor = x - &r;
                    match rm {
                        Down | Floor => (floor, Less),
                        Up | Ceiling => (floor + y, Greater),
                        Nearest => {
                            match (r << 1u64).cmp(&y) {
                                Less => (floor, Less),
                                Greater => (floor + y, Greater),
                                Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if floor == 0 {
                                        (floor, Less)
                                    } else {
                                        let ceiling = &floor + y;
                                        if floor.trailing_zeros() > ceiling.trailing_zeros() {
                                            (floor, Less)
                                        } else {
                                            (ceiling, Greater)
                                        }
                                    }
                                }
                            }
                        }
                        Exact => {
                            panic!("Cannot round {x} to {y} using RoundingMode {rm}")
                        }
                    }
                }
            }
        }
    }
}

impl<'a, 'b> RoundToMultiple<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified rounding
    /// mode. Both [`Natural`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(5u32))
    ///         .round_to_multiple(&Natural::ZERO, Down)
    ///         .to_debug_string(),
    ///     "(0, Less)"
    /// );
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(&Natural::from(4u32), Down)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(&Natural::from(4u32), Up)
    ///         .to_debug_string(),
    ///     "(12, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(&Natural::from(5u32), Exact)
    ///         .to_debug_string(),
    ///     "(10, Equal)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(&Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(9, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .round_to_multiple(&Natural::from(3u32), Nearest)
    ///         .to_debug_string(),
    ///     "(21, Greater)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .round_to_multiple(&Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(8, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32))
    ///         .round_to_multiple(&Natural::from(4u32), Nearest)
    ///         .to_debug_string(),
    ///     "(16, Greater)"
    /// );
    /// ```
    fn round_to_multiple(self, other: &'b Natural, rm: RoundingMode) -> (Natural, Ordering) {
        match (self, other) {
            (x, y) if x == y => (x.clone(), Equal),
            (x, &Natural::ZERO) => match rm {
                Down | Floor | Nearest => (Natural::ZERO, Less),
                _ => panic!("Cannot round {x} to zero using RoundingMode {rm}"),
            },
            (x, y) => {
                let r = x % y;
                if r == 0 {
                    (x.clone(), Equal)
                } else {
                    let floor = x - &r;
                    match rm {
                        Down | Floor => (floor, Less),
                        Up | Ceiling => (floor + y, Greater),
                        Nearest => {
                            match (r << 1u64).cmp(y) {
                                Less => (floor, Less),
                                Greater => (floor + y, Greater),
                                Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if floor == 0 {
                                        (floor, Less)
                                    } else {
                                        let ceiling = &floor + y;
                                        if floor.trailing_zeros() > ceiling.trailing_zeros() {
                                            (floor, Less)
                                        } else {
                                            (ceiling, Greater)
                                        }
                                    }
                                }
                            }
                        }
                        Exact => {
                            panic!("Cannot round {x} to {y} using RoundingMode {rm}")
                        }
                    }
                }
            }
        }
    }
}

impl RoundToMultipleAssign<Natural> for Natural {
    /// Rounds a [`Natural`] to a multiple of another [`Natural`] in place, according to a specified
    /// rounding mode. The [`Natural`] on the right-hand side is taken by value. An [`Ordering`] is
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// original value.
    ///
    /// See the [`RoundToMultiple`] documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(5u32);
    /// assert_eq!(x.round_to_multiple_assign(Natural::ZERO, Down), Less);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(x.round_to_multiple_assign(Natural::from(4u32), Down), Less);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(x.round_to_multiple_assign(Natural::from(4u32), Up), Greater);
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Natural::from(5u32), Exact),
    ///     Equal
    /// );
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Natural::from(3u32), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(20u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Natural::from(3u32), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, 21);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Natural::from(4u32), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(14u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(Natural::from(4u32), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, 16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: Natural, rm: RoundingMode) -> Ordering {
        match (&mut *self, other) {
            (x, y) if *x == y => Equal,
            (x, Natural::ZERO) => match rm {
                Down | Floor | Nearest => {
                    *self = Natural::ZERO;
                    Less
                }
                _ => panic!("Cannot round {x} to zero using RoundingMode {rm}"),
            },
            (x, y) => {
                let r = &*x % &y;
                if r == 0 {
                    Equal
                } else {
                    *x -= &r;
                    match rm {
                        Down | Floor => Less,
                        Up | Ceiling => {
                            *x += y;
                            Greater
                        }
                        Nearest => {
                            match (r << 1u64).cmp(&y) {
                                Less => Less,
                                Greater => {
                                    *x += y;
                                    Greater
                                }
                                Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if *x == 0 {
                                        Less
                                    } else {
                                        let ceiling = &*x + y;
                                        if x.trailing_zeros() < ceiling.trailing_zeros() {
                                            *x = ceiling;
                                            Greater
                                        } else {
                                            Less
                                        }
                                    }
                                }
                            }
                        }
                        Exact => {
                            panic!("Cannot round {x} to {y} using RoundingMode {rm}")
                        }
                    }
                }
            }
        }
    }
}

impl<'a> RoundToMultipleAssign<&'a Natural> for Natural {
    /// Rounds a [`Natural`] to a multiple of another [`Natural`] in place, according to a specified
    /// rounding mode. The [`Natural`] on the right-hand side is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the returned value is less than, equal to, or greater
    /// than the original value.
    ///
    /// See the [`RoundToMultiple`] documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(5u32);
    /// assert_eq!(x.round_to_multiple_assign(&Natural::ZERO, Down), Less);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(x.round_to_multiple_assign(&Natural::from(4u32), Down), Less);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Natural::from(4u32), Up),
    ///     Greater
    /// );
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Natural::from(5u32), Exact),
    ///     Equal
    /// );
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Natural::from(3u32), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(20u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Natural::from(3u32), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, 21);
    ///
    /// let mut x = Natural::from(10u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Natural::from(4u32), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(14u32);
    /// assert_eq!(
    ///     x.round_to_multiple_assign(&Natural::from(4u32), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x, 16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: &'a Natural, rm: RoundingMode) -> Ordering {
        match (&mut *self, other) {
            (x, y) if *x == *y => Equal,
            (x, &Natural::ZERO) => match rm {
                Down | Floor | Nearest => {
                    *self = Natural::ZERO;
                    Less
                }
                _ => panic!("Cannot round {x} to zero using RoundingMode {rm}"),
            },
            (x, y) => {
                let r = &*x % y;
                if r == 0 {
                    Equal
                } else {
                    *x -= &r;
                    match rm {
                        Down | Floor => Less,
                        Up | Ceiling => {
                            *x += y;
                            Greater
                        }
                        Nearest => {
                            match (r << 1u64).cmp(y) {
                                Less => Less,
                                Greater => {
                                    *x += y;
                                    Greater
                                }
                                Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if *x == 0 {
                                        Less
                                    } else {
                                        let ceiling = &*x + y;
                                        if x.trailing_zeros() < ceiling.trailing_zeros() {
                                            *x = ceiling;
                                            Greater
                                        } else {
                                            Less
                                        }
                                    }
                                }
                            }
                        }
                        Exact => {
                            panic!("Cannot round {x} to {y} using RoundingMode {rm}")
                        }
                    }
                }
            }
        }
    }
}
