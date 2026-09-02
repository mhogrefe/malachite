// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::logic::traits::SignificantBits;

impl GaussianInteger {
    /// Returns the larger of the numbers of significant bits of the real and imaginary parts of a
    /// [`GaussianInteger`], each taken in absolute value.
    ///
    /// This is the size measure that FLINT's `fmpzi_bits` computes, and the one that the sizes of
    /// the parts are compared against when an algorithm is chosen; the
    /// [`SignificantBits`](malachite_base::num::logic::traits::SignificantBits) implementation sums
    /// the two counts instead.
    ///
    /// $$
    /// f(a + bi) = \max(\operatorname{bits}(a), \operatorname{bits}(b)),
    /// $$
    /// where $\operatorname{bits}(n)$ is the number of significant bits of $|n|$, with
    /// $\operatorname{bits}(0) = 0$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::ZERO.max_significant_bits(), 0);
    /// assert_eq!(
    ///     GaussianInteger::from_str("3+4i")
    ///         .unwrap()
    ///         .max_significant_bits(),
    ///     3
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("1000000000000+i")
    ///         .unwrap()
    ///         .max_significant_bits(),
    ///     40
    /// );
    /// ```
    #[inline]
    pub fn max_significant_bits(&self) -> u64 {
        self.real
            .significant_bits()
            .max(self.imaginary.significant_bits())
    }
}

impl SignificantBits for &GaussianInteger {
    /// Returns the sum of the numbers of significant bits of the real and imaginary parts of a
    /// [`GaussianInteger`], each taken in absolute value.
    ///
    /// $$
    /// f(a + bi) = \operatorname{bits}(a) + \operatorname{bits}(b),
    /// $$
    /// where $\operatorname{bits}(n)$ is the number of significant bits of $|n|$, with
    /// $\operatorname{bits}(0) = 0$. The larger of the two counts alone is available as
    /// [`max_significant_bits`](GaussianInteger::max_significant_bits).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::ZERO.significant_bits(), 0);
    /// assert_eq!(GaussianInteger::from(100).significant_bits(), 7);
    /// assert_eq!(
    ///     GaussianInteger::from_str("3+4i")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     5
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("1000000000000+i")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     41
    /// );
    /// ```
    #[inline]
    fn significant_bits(self) -> u64 {
        self.real.significant_bits() + self.imaginary.significant_bits()
    }
}
