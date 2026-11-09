// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{ShrRoundWithOrdering, ShrRoundWithOrderingAssign};
use malachite_base::rounding_modes::RoundingMode;

// Whether the exact value is non-negative. Zero takes the sign of the side the exact value lies on,
// so that a zero lying just above a negative value is shifted as a negative number.
fn exact_sign(x: &Integer, o: Ordering) -> bool {
    if x.abs == 0u32 { o != Greater } else { x.sign }
}

fn shr_round_with_ordering_ref(
    x: &Integer,
    bits: u64,
    rm: RoundingMode,
    o: Ordering,
) -> (Integer, Ordering) {
    if exact_sign(x, o) {
        let (abs, o) = (&x.abs).shr_round_with_ordering(bits, rm, o);
        (Integer::from(abs), o)
    } else {
        let (abs, o) = (&x.abs).shr_round_with_ordering(bits, -rm, o.reverse());
        (-abs, o.reverse())
    }
}

fn shr_round_with_ordering_assign(
    x: &mut Integer,
    bits: u64,
    rm: RoundingMode,
    o: Ordering,
) -> Ordering {
    let sign = exact_sign(x, o);
    let o = if sign {
        x.abs.shr_round_with_ordering_assign(bits, rm, o)
    } else {
        x.abs
            .shr_round_with_ordering_assign(bits, -rm, o.reverse())
            .reverse()
    };
    x.sign = sign || x.abs == 0u32;
    o
}

impl ShrRoundWithOrdering for Integer {
    type Output = Self;

    /// Right-shifts an [`Integer`] (divides it by a power of 2) that is itself an approximation of
    /// an exact value, rounding according to a specified rounding mode, taking the [`Integer`] by
    /// value. The [`Integer`]'s [`Ordering`] against the exact value is passed in, and an
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the shifted exact value.
    ///
    /// The exact value is taken to lie just above $x$ if the passed-in [`Ordering`] is `Less`, just
    /// below it if `Greater`, and to equal it if `Equal`, in which case this function is the same
    /// as [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound). If $x$ is the result of
    /// rounding some value $t$ to an integer, with any rounding mode, and the passed-in
    /// [`Ordering`] compares $x$ with $t$, then for $k > 0$ the result is exactly what rounding
    /// $t/2^k$ once would have produced, with no double-rounding error.
    ///
    /// When the passed-in [`Ordering`] is `Less`, $f(x, k, \mathrm{Floor}) = \lfloor x/2^k
    /// \rfloor$ and $f(x, k, \mathrm{Ceiling}) = \lfloor x/2^k \rfloor + 1$. When it is `Greater`,
    /// $f(x, k, \mathrm{Floor}) = \lceil x/2^k \rceil - 1$ and $f(x, k, \mathrm{Ceiling}) = \lceil
    /// x/2^k \rceil$. `Down` and `Up` round toward and away from zero, according to the sign of the
    /// exact value, and $\mathrm{Nearest}$ rounds $x/2^k$ to the closer integer, an exact half
    /// rounding up if the passed-in [`Ordering`] is `Less` and down if it is `Greater`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the passed-in [`Ordering`] is not `Equal` or the shift is not
    /// exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ShrRoundWithOrdering;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// // -0x180 >> 8 is an exact half, -1.5; the exact value lies just above it
    /// assert_eq!(
    ///     Integer::from(-0x180)
    ///         .shr_round_with_ordering(8, Nearest, Less)
    ///         .to_debug_string(),
    ///     "(-1, Greater)"
    /// );
    /// // and here just below it
    /// assert_eq!(
    ///     Integer::from(-0x180)
    ///         .shr_round_with_ordering(8, Nearest, Greater)
    ///         .to_debug_string(),
    ///     "(-2, Less)"
    /// );
    /// // the exact value lies just below 0
    /// assert_eq!(
    ///     Integer::ZERO
    ///         .shr_round_with_ordering(8, Floor, Greater)
    ///         .to_debug_string(),
    ///     "(-1, Less)"
    /// );
    /// assert_eq!(
    ///     Integer::ZERO
    ///         .shr_round_with_ordering(8, Down, Greater)
    ///         .to_debug_string(),
    ///     "(0, Greater)"
    /// );
    /// ```
    #[inline]
    fn shr_round_with_ordering(
        mut self,
        bits: u64,
        rm: RoundingMode,
        o: Ordering,
    ) -> (Self, Ordering) {
        let o = shr_round_with_ordering_assign(&mut self, bits, rm, o);
        (self, o)
    }
}

impl ShrRoundWithOrdering for &Integer {
    type Output = Integer;

    /// Right-shifts an [`Integer`] (divides it by a power of 2) that is itself an approximation of
    /// an exact value, rounding according to a specified rounding mode, taking the [`Integer`] by
    /// reference. The [`Integer`]'s [`Ordering`] against the exact value is passed in, and an
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the shifted exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the passed-in [`Ordering`] is not `Equal` or the shift is not
    /// exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ShrRoundWithOrdering;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-0x200))
    ///         .shr_round_with_ordering(8, Up, Greater)
    ///         .to_debug_string(),
    ///     "(-3, Less)"
    /// );
    /// ```
    #[inline]
    fn shr_round_with_ordering(
        self,
        bits: u64,
        rm: RoundingMode,
        o: Ordering,
    ) -> (Integer, Ordering) {
        shr_round_with_ordering_ref(self, bits, rm, o)
    }
}

impl ShrRoundWithOrderingAssign for Integer {
    /// Right-shifts an [`Integer`] (divides it by a power of 2) in place, where the [`Integer`] is
    /// itself an approximation of an exact value, rounding according to a specified rounding mode.
    /// The [`Integer`]'s [`Ordering`] against the exact value is passed in, and an [`Ordering`] is
    /// returned, indicating whether the assigned value is less than, equal to, or greater than the
    /// shifted exact value.
    ///
    /// See the [`ShrRoundWithOrdering`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the passed-in [`Ordering`] is not `Equal` or the shift is not
    /// exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ShrRoundWithOrderingAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Integer::from(-0x180);
    /// assert_eq!(x.shr_round_with_ordering_assign(8, Nearest, Less), Greater);
    /// assert_eq!(x.to_string(), "-1");
    /// ```
    #[inline]
    fn shr_round_with_ordering_assign(
        &mut self,
        bits: u64,
        rm: RoundingMode,
        o: Ordering,
    ) -> Ordering {
        shr_round_with_ordering_assign(self, bits, rm, o)
    }
}
