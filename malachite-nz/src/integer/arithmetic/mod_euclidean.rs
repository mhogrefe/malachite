// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    Mod, ModAssign, ModEuclidean, ModEuclideanAssign, UnsignedAbs,
};

impl ModEuclidean<Self> for Integer {
    type Output = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning just the
    /// remainder. The remainder is nonnegative and is returned as a [`Natural`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23).mod_euclidean(Integer::from(10)), 3);
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(Integer::from(-23).mod_euclidean(Integer::from(-10)), 7);
    /// ```
    #[inline]
    fn mod_euclidean(self, other: Self) -> Natural {
        let r = self.mod_op(&other);
        if r < 0u32 {
            (r - other).unsigned_abs()
        } else {
            r.unsigned_abs()
        }
    }
}

impl ModEuclidean<&Self> for Integer {
    type Output = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning just the remainder. The remainder is nonnegative and is returned as
    /// a [`Natural`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23).mod_euclidean(&Integer::from(10)), 7);
    /// ```
    #[inline]
    fn mod_euclidean(self, other: &Self) -> Natural {
        let r = self.mod_op(other);
        if r < 0u32 {
            (r - other).unsigned_abs()
        } else {
            r.unsigned_abs()
        }
    }
}

impl ModEuclidean<Integer> for &Integer {
    type Output = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning just the remainder. The remainder is nonnegative and is returned as a
    /// [`Natural`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!((&Integer::from(23)).mod_euclidean(Integer::from(-10)), 3);
    /// ```
    #[inline]
    fn mod_euclidean(self, other: Integer) -> Natural {
        let r = self.mod_op(&other);
        if r < 0u32 {
            (r - other).unsigned_abs()
        } else {
            r.unsigned_abs()
        }
    }
}

impl ModEuclidean<&Integer> for &Integer {
    type Output = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning just
    /// the remainder. The remainder is nonnegative and is returned as a [`Natural`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!((&Integer::from(-23)).mod_euclidean(&Integer::from(-10)), 7);
    /// ```
    #[inline]
    fn mod_euclidean(self, other: &Integer) -> Natural {
        let r = self.mod_op(other);
        if r < 0u32 {
            (r - other).unsigned_abs()
        } else {
            r.unsigned_abs()
        }
    }
}

impl ModEuclideanAssign<Self> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the [`Integer`] on the right-hand side
    /// by value and replacing the first [`Integer`] by the remainder. The remainder is nonnegative.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// x \gets x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::ModEuclideanAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_euclidean_assign(Integer::from(10));
    /// assert_eq!(x, 7);
    ///
    /// // 3 * -10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_euclidean_assign(Integer::from(-10));
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn mod_euclidean_assign(&mut self, other: Self) {
        self.mod_assign(&other);
        if *self < 0u32 {
            *self -= other;
        }
    }
}

impl ModEuclideanAssign<&Self> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the [`Integer`] on the right-hand side
    /// by reference and replacing the first [`Integer`] by the remainder. The remainder is
    /// nonnegative.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < |y|$.
    ///
    /// $$
    /// x \gets x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::ModEuclideanAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * -10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_euclidean_assign(&Integer::from(-10));
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn mod_euclidean_assign(&mut self, other: &Self) {
        self.mod_assign(other);
        if *self < 0u32 {
            *self -= other;
        }
    }
}
