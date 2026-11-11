// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;

impl BigExponentFloat {
    /// Determines a [`BigExponentFloat`]'s complexity. The complexity is defined as follows:
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(0) = 1,
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = \max(|\lfloor \log_2 |x|\rfloor|, p),
    /// $$
    ///
    /// where $p$ is the precision of $x$. The first term saturates at `u64::MAX`.
    ///
    /// Informally, the complexity is proportional to the number of characters you would need to
    /// write the [`BigExponentFloat`] out without using exponents.
    ///
    /// See also the [`BigExponentFloat`] implementation of [`SignificantBits`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One, Two};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.complexity(), 1);
    /// assert_eq!(BigExponentFloat::ONE.complexity(), 1);
    /// assert_eq!(BigExponentFloat::TWO.complexity(), 1);
    /// ```
    pub fn complexity(&self) -> u64 {
        match self {
            Self(Finite { exponent, .. }) => max(
                u64::saturating_from(&(exponent - Integer::ONE).unsigned_abs()),
                self.get_prec().unwrap(),
            ),
            _ => 1,
        }
    }
}

impl SignificantBits for &BigExponentFloat {
    /// Returns the number of significant bits of a [`BigExponentFloat`]. This is defined as
    /// follows:
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(0) = 1,
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = p,
    /// $$
    ///
    /// where $p$ is the precision of $x$.
    ///
    /// See also the [`BigExponentFloat::complexity`] function.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.significant_bits(), 1);
    /// assert_eq!(BigExponentFloat::ONE.significant_bits(), 1);
    /// ```
    #[inline]
    fn significant_bits(self) -> u64 {
        self.get_prec().unwrap_or(1)
    }
}
