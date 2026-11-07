// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::number_theory::traits::{
    StirlingNumberOfTheFirstKind, UnsignedStirlingNumberOfTheFirstKind,
};

impl StirlingNumberOfTheFirstKind for Integer {
    /// Computes the Stirling number of the first kind $s(n, k) = (-1)^{n + k} \left[{n \atop
    /// k}\right]$, the coefficient of $x^k$ in the falling factorial $x(x - 1) \cdots (x - n + 1)$.
    ///
    /// The magnitude is computed as the unsigned Stirling number of the first kind, by
    /// [`UnsignedStirlingNumberOfTheFirstKind`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::number_theory::traits::StirlingNumberOfTheFirstKind;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::stirling_number_of_the_first_kind(0, 0), 1);
    /// assert_eq!(Integer::stirling_number_of_the_first_kind(5, 0), 0);
    /// assert_eq!(Integer::stirling_number_of_the_first_kind(3, 5), 0);
    /// assert_eq!(Integer::stirling_number_of_the_first_kind(5, 2), -50);
    /// assert_eq!(Integer::stirling_number_of_the_first_kind(10, 5), -269325);
    /// assert_eq!(
    ///     Integer::stirling_number_of_the_first_kind(42, 8).to_string(),
    ///     "34710540274511227194886186414162202385600068321280"
    /// );
    /// assert_eq!(
    ///     Integer::stirling_number_of_the_first_kind(u64::MAX, u64::MAX - 2).to_string(),
    ///     "14474011154664524422192363201981531115617445066701938474182899447357142728705"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_number_1` from `arith/stirling1.c`, FLINT 3.6.0.
    fn stirling_number_of_the_first_kind(n: u64, k: u64) -> Self {
        let s = Self::from(Natural::unsigned_stirling_number_of_the_first_kind(n, k));
        if n.even() == k.even() { s } else { -s }
    }
}
