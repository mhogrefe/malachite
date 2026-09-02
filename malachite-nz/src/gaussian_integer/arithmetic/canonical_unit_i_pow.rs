// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
use malachite_base::num::comparison::traits::PartialOrdAbs;

impl CanonicalUnitIPow for GaussianInteger {
    /// Finds the power of $i$ that brings a [`GaussianInteger`] into canonical unit form.
    ///
    /// A nonzero value has four associates, $x$, $ix$, $-x$, and $-ix$; the canonical one is the
    /// associate whose argument lies in $(-\pi/4, \pi/4]$, that is, whose real part $a$ is positive
    /// and whose imaginary part $b$ satisfies $-a < b \leq a$. The result is the $k \in \\{0, 1, 2,
    /// 3\\}$ such that $x i^k$ is canonical, and 0 for zero. The choice of associate, including the
    /// tie on the diagonals, matches FLINT's `fmpzi_canonical_unit_i_pow`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianInteger::from_str("2+i")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     0
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("-1+2i")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     3
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("-2-i")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     2
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("1-2i")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     1
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("1+i")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     0
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("1-i")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     1
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("0")
    ///         .unwrap()
    ///         .canonical_unit_i_pow(),
    ///     0
    /// );
    /// ```
    fn canonical_unit_i_pow(&self) -> u64 {
        match self.real.cmp(&self.imaginary) {
            Equal => u64::from(self.real < 0u32) << 1,
            Greater => u64::from(self.real.le_abs(&self.imaginary)),
            Less => {
                if self.real.le_abs(&self.imaginary) {
                    3
                } else {
                    2
                }
            }
        }
    }
}
