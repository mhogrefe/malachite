// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    ShrRound, ShrRoundAssign, ShrRoundWithOrdering, ShrRoundWithOrderingAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode::{self, *};

fn shr_round_with_ordering_ref(
    x: &Natural,
    bits: u64,
    rm: RoundingMode,
    o: Ordering,
) -> (Natural, Ordering) {
    if o == Equal {
        return x.shr_round(bits, rm);
    }
    assert_ne!(
        rm, Exact,
        "Cannot shift an inexact value with Exact rounding"
    );
    // The exact value lies just above `x` (`o == Less`) or just below it (`o == Greater`).
    let zeros = x.trailing_zeros();
    if zeros.is_none_or(|z| z >= bits) {
        // `x` is divisible by `2^bits`, so the shifted exact value lies just beside the integer `x
        // >> bits`, and the result is that integer, one more, or one less.
        let q = x >> bits;
        match (o, rm) {
            (Greater, Floor | Down) => {
                assert_ne!(q, 0u32, "Cannot round a negative value to a Natural");
                (q - Natural::ONE, Less)
            }
            (Less, Ceiling | Up) => (q + Natural::ONE, Greater),
            _ => (q, o),
        }
    } else if rm == Nearest && zeros == Some(bits - 1) {
        // `x >> bits` is an exact half, which the side of the exact value resolves.
        let q = x >> bits;
        if o == Greater {
            (q, Less)
        } else {
            (q + Natural::ONE, Greater)
        }
    } else {
        // The shifted exact value lies strictly between the same two integers as `x >> bits`, and
        // on the same side of their midpoint, so it rounds as `x >> bits` does.
        x.shr_round(bits, rm)
    }
}

fn shr_round_with_ordering_assign(
    x: &mut Natural,
    bits: u64,
    rm: RoundingMode,
    o: Ordering,
) -> Ordering {
    if o == Equal {
        return x.shr_round_assign(bits, rm);
    }
    assert_ne!(
        rm, Exact,
        "Cannot shift an inexact value with Exact rounding"
    );
    // See `shr_round_with_ordering_ref` for the three cases.
    let zeros = x.trailing_zeros();
    if zeros.is_none_or(|z| z >= bits) {
        *x >>= bits;
        match (o, rm) {
            (Greater, Floor | Down) => {
                assert_ne!(*x, 0u32, "Cannot round a negative value to a Natural");
                *x -= Natural::ONE;
                Less
            }
            (Less, Ceiling | Up) => {
                *x += Natural::ONE;
                Greater
            }
            _ => o,
        }
    } else if rm == Nearest && zeros == Some(bits - 1) {
        *x >>= bits;
        if o == Greater {
            Less
        } else {
            *x += Natural::ONE;
            Greater
        }
    } else {
        x.shr_round_assign(bits, rm)
    }
}

impl ShrRoundWithOrdering for Natural {
    type Output = Self;

    /// Right-shifts a [`Natural`] (divides it by a power of 2) that is itself an approximation of
    /// an exact value, rounding according to a specified rounding mode, taking the [`Natural`] by
    /// value. The [`Natural`]'s [`Ordering`] against the exact value is passed in, and an
    /// [`Ordering`] is returned, indicating whether the returned value is less than, equal to, or
    /// greater than the shifted exact value.
    ///
    /// The exact value is taken to lie just above $x$ if the passed-in [`Ordering`] is `Less`, just
    /// below it if `Greater`, and to equal it if `Equal`, in which case this function is the same
    /// as [`ShrRound`]. If $x$ is the result of rounding some value $t$ to an integer, with any
    /// rounding mode, and the passed-in [`Ordering`] compares $x$ with $t$, then for $k > 0$ the
    /// result is exactly what rounding $t/2^k$ once would have produced, with no double-rounding
    /// error.
    ///
    /// When the passed-in [`Ordering`] is `Less`, $f(x, k, \mathrm{Down}) = f(x, k,
    /// \mathrm{Floor}) = \lfloor x/2^k \rfloor$ and $f(x, k, \mathrm{Up}) = f(x, k,
    /// \mathrm{Ceiling}) = \lfloor x/2^k \rfloor + 1$. When it is `Greater`, $f(x, k,
    /// \mathrm{Down}) = f(x, k, \mathrm{Floor}) = \lceil x/2^k \rceil - 1$ and $f(x, k,
    /// \mathrm{Up}) = f(x, k, \mathrm{Ceiling}) = \lceil x/2^k \rceil$. In both cases
    /// $\mathrm{Nearest}$ rounds $x/2^k$ to the closer integer, an exact half rounding up if the
    /// passed-in [`Ordering`] is `Less` and down if it is `Greater`.
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
    /// exact, and if the result would be negative: when `self` is zero, the passed-in [`Ordering`]
    /// is `Greater`, and `rm` is `Floor` or `Down`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ShrRoundWithOrdering;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0x180 >> 8 is an exact half, 1.5; the exact value lies just below it
    /// assert_eq!(
    ///     Natural::from(0x180u32)
    ///         .shr_round_with_ordering(8, Nearest, Greater)
    ///         .to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// // and here just above it
    /// assert_eq!(
    ///     Natural::from(0x180u32)
    ///         .shr_round_with_ordering(8, Nearest, Less)
    ///         .to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// // 0x200 >> 8 is exactly 2, but the exact value lies just below it
    /// assert_eq!(
    ///     Natural::from(0x200u32)
    ///         .shr_round_with_ordering(8, Floor, Greater)
    ///         .to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     Natural::from(0x200u32)
    ///         .shr_round_with_ordering(8, Ceiling, Greater)
    ///         .to_debug_string(),
    ///     "(2, Greater)"
    /// );
    /// // with an exact input this is shr_round
    /// assert_eq!(
    ///     Natural::from(0x180u32)
    ///         .shr_round_with_ordering(8, Nearest, Equal)
    ///         .to_debug_string(),
    ///     "(2, Greater)"
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

impl ShrRoundWithOrdering for &Natural {
    type Output = Natural;

    /// Right-shifts a [`Natural`] (divides it by a power of 2) that is itself an approximation of
    /// an exact value, rounding according to a specified rounding mode, taking the [`Natural`] by
    /// reference. The [`Natural`]'s [`Ordering`] against the exact value is passed in, and an
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
    /// exact, and if the result would be negative: when `self` is zero, the passed-in [`Ordering`]
    /// is `Greater`, and `rm` is `Floor` or `Down`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ShrRoundWithOrdering;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     (&Natural::from(0x180u32))
    ///         .shr_round_with_ordering(8, Nearest, Greater)
    ///         .to_debug_string(),
    ///     "(1, Less)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(0x200u32))
    ///         .shr_round_with_ordering(8, Up, Less)
    ///         .to_debug_string(),
    ///     "(3, Greater)"
    /// );
    /// ```
    #[inline]
    fn shr_round_with_ordering(
        self,
        bits: u64,
        rm: RoundingMode,
        o: Ordering,
    ) -> (Natural, Ordering) {
        shr_round_with_ordering_ref(self, bits, rm, o)
    }
}

impl ShrRoundWithOrderingAssign for Natural {
    /// Right-shifts a [`Natural`] (divides it by a power of 2) in place, where the [`Natural`] is
    /// itself an approximation of an exact value, rounding according to a specified rounding mode.
    /// The [`Natural`]'s [`Ordering`] against the exact value is passed in, and an [`Ordering`] is
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
    /// exact, and if the result would be negative: when `self` is zero, the passed-in [`Ordering`]
    /// is `Greater`, and `rm` is `Floor` or `Down`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ShrRoundWithOrderingAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Natural::from(0x180u32);
    /// assert_eq!(x.shr_round_with_ordering_assign(8, Nearest, Less), Greater);
    /// assert_eq!(x.to_string(), "2");
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
