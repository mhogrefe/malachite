// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{
    DivEuclidean, DivEuclideanAssign, DivRound, DivRoundAssign,
};
use malachite_base::rounding_modes::RoundingMode::{Ceiling, Floor};

impl DivEuclidean<Self> for Integer {
    type Output = Self;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning just the
    /// quotient. The quotient is rounded so that the remainder would be nonnegative.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23).div_euclidean(Integer::from(10)), 2);
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(Integer::from(-23).div_euclidean(Integer::from(-10)), 3);
    /// ```
    #[inline]
    fn div_euclidean(self, other: Self) -> Self {
        let rm = if other > 0 { Floor } else { Ceiling };
        self.div_round(other, rm).0
    }
}

impl DivEuclidean<&Self> for Integer {
    type Output = Self;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning just the quotient. The quotient is rounded so that the remainder
    /// would be nonnegative.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23).div_euclidean(&Integer::from(10)), -3);
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Self) -> Self {
        let rm = if *other > 0 { Floor } else { Ceiling };
        self.div_round(other, rm).0
    }
}

impl DivEuclidean<Integer> for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning just the quotient. The quotient is rounded so that the remainder
    /// would be nonnegative.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!((&Integer::from(23)).div_euclidean(Integer::from(-10)), -2);
    /// ```
    #[inline]
    fn div_euclidean(self, other: Integer) -> Integer {
        let rm = if other > 0 { Floor } else { Ceiling };
        self.div_round(other, rm).0
    }
}

impl DivEuclidean<&Integer> for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning just
    /// the quotient. The quotient is rounded so that the remainder would be nonnegative.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!((&Integer::from(-23)).div_euclidean(&Integer::from(-10)), 3);
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Integer) -> Integer {
        let rm = if *other > 0 { Floor } else { Ceiling };
        self.div_round(other, rm).0
    }
}

impl DivEuclideanAssign<Self> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and keeping just the quotient. The quotient is rounded so that the
    /// remainder would be nonnegative.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// x \gets \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivEuclideanAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.div_euclidean_assign(Integer::from(10));
    /// assert_eq!(x, -3);
    /// ```
    #[inline]
    fn div_euclidean_assign(&mut self, other: Self) {
        let rm = if other > 0 { Floor } else { Ceiling };
        self.div_round_assign(other, rm);
    }
}

impl DivEuclideanAssign<&Self> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and keeping just the quotient. The quotient is rounded so that
    /// the remainder would be nonnegative.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// x \gets \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivEuclideanAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * -10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.div_euclidean_assign(&Integer::from(-10));
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn div_euclidean_assign(&mut self, other: &Self) {
        let rm = if *other > 0 { Floor } else { Ceiling };
        self.div_round_assign(other, rm);
    }
}
