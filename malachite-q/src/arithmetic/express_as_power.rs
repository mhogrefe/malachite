// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{CheckedRoot, Gcd};
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_nz::natural::Natural;

// The largest `e` such that `n` is a perfect `e`th power (1 if `n` is not a perfect power), or
// `None` if `n` is 1 (a unit imposes no constraint on the exponent).
fn unit_or_perfect_power_exponent(n: &Natural) -> Option<u64> {
    if *n == 1u32 {
        None
    } else {
        Some(n.express_as_power().map_or(1, |(_, e)| e))
    }
}

impl ExpressAsPower for Rational {
    /// Expresses a [`Rational`] as a perfect power if possible.
    ///
    /// Returns `Some((root, exponent))`, where `root.pow(exponent) == self`, `exponent` is greater
    /// than 1 and as large as possible, and `root` is therefore not itself a perfect power. Returns
    /// `None` when no such `exponent` exists.
    ///
    /// The exponent is always positive; the `root` absorbs the "direction" of the power. For
    /// example, $1/9 = (1/3)^2$ is returned as `(1/3, 2)` rather than `(3, -2)`. A negative
    /// [`Rational`] only has a real `root` when `exponent` is odd, so for example $-1/4$ is not a
    /// perfect power, but $-1/8 = (-1/2)^3$ is.
    ///
    /// Following the convention of the integer implementations, 0 and 1 are expressed as squares.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}((r, e)) & \text{if} \\quad x = r^e,\\ e > 1\\ \text{maximal},\\
    ///         r \in \mathbb{Q} \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::ExpressAsPower;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from(8u32).express_as_power().to_debug_string(),
    ///     "Some((2, 3))"
    /// );
    /// assert_eq!(
    ///     Rational::from_unsigneds(9u32, 4).express_as_power().to_debug_string(),
    ///     "Some((3/2, 2))"
    /// );
    /// assert_eq!(
    ///     Rational::from_unsigneds(1u32, 9).express_as_power().to_debug_string(),
    ///     "Some((1/3, 2))"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-1, 8).express_as_power().to_debug_string(),
    ///     "Some((-1/2, 3))"
    /// );
    /// assert_eq!(
    ///     Rational::from_unsigneds(3u32, 2).express_as_power().to_debug_string(),
    ///     "None"
    /// );
    /// ```
    fn express_as_power(&self) -> Option<(Self, u64)> {
        // 0 and 1 are expressed as squares, matching the integer implementations.
        if *self == 0u32 || *self == 1u32 {
            return Some((self.clone(), 2));
        }
        let positive = *self > 0u32;
        let (numerator, denominator) = self.numerator_and_denominator_ref();
        // `self` is a perfect `e`th power iff both its numerator and denominator are, so the largest
        // valid `e` is the gcd of their individual perfect-power exponents. A unit (1) imposes no
        // constraint, so it is skipped.
        let e = match (
            unit_or_perfect_power_exponent(numerator),
            unit_or_perfect_power_exponent(denominator),
        ) {
            // Both are 1, so `self` is -1 (1 was handled above): no maximal exponent exists.
            (None, None) => return None,
            (Some(e), None) | (None, Some(e)) => e,
            (Some(e1), Some(e2)) => e1.gcd(e2),
        };
        // A negative base requires an odd exponent: (-r)^e is positive when `e` is even. Reduce `e`
        // to its odd part. (The even part is absorbed into the root.)
        let e = if positive { e } else { e >> e.trailing_zeros() };
        if e <= 1 {
            return None;
        }
        // `e` divides both the numerator's and denominator's perfect-power exponents, so the root is
        // exact (and correctly signed for a negative `self` with odd `e`).
        Some((self.checked_root(e).unwrap(), e))
    }
}
