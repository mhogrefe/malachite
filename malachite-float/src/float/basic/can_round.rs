// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::rounding_modes::RoundingMode::{self, Exact};
use malachite_nz::natural::arithmetic::float::round::float_can_round_raw;

impl Float {
    // This is mpfr_can_round from round_prec.c, MPFR 4.2.2, without the faithful-rounding cases,
    // which have no counterpart among Malachite's rounding modes.
    /// Determines whether an approximation is accurate enough to commit to a correctly rounded
    /// result.
    ///
    /// `self` should be an approximation of some unknown real number $x$, obtained by rounding in
    /// the direction `rnd1` with error at most $2^{e-\\text{{err}}}$, where $e$ is the raw exponent
    /// of `self` (so the error is at most one ulp of `self` when `err` equals the precision of
    /// `self`). This function returns whether that information suffices to round $x$ correctly to
    /// precision `prec` in the direction `rnd2` — that is, whether every real number consistent
    /// with the approximation rounds to the same value. If it returns true, rounding `self` to
    /// precision `prec` with `rnd2` gives that value.
    ///
    /// This is the test at the heart of Ziv's strategy for computing correctly rounded functions:
    /// compute an approximation with a known error bound, and retry with more precision until this
    /// function accepts it.
    ///
    /// If `self` is `NaN`, infinite, or zero, the result is `false`: no error bound of this form
    /// conveys enough information to round those.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rnd1` or `rnd2` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// // A 100-bit approximation of sqrt(2), accurate to about 90 bits, is more than enough
    /// // to round to double precision...
    /// let x = Float::from(2u32).sqrt_prec(100).0;
    /// assert!(x.can_round(90, Nearest, Nearest, 53));
    ///
    /// // ...but knowing only 53 of its bits is not.
    /// assert!(!x.can_round(53, Nearest, Nearest, 53));
    /// ```
    pub fn can_round(&self, err: i64, rnd1: RoundingMode, rnd2: RoundingMode, prec: u64) -> bool {
        assert_ne!(prec, 0);
        assert_ne!(rnd1, Exact);
        assert_ne!(rnd2, Exact);
        match self {
            Self(Finite {
                sign, significand, ..
            }) => float_can_round_raw(significand, !sign, err, rnd1, rnd2, prec),
            _ => false,
        }
    }
}
