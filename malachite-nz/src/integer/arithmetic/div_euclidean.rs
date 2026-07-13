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
    DivAssignEuclidean, DivEuclidean, DivMod, UnsignedAbs,
};
use malachite_base::num::basic::traits::One;

// Adjusts the `(quotient, remainder)` returned by `div_mod` (whose remainder has the sign of the
// divisor) into the Euclidean `(quotient, remainder)`, whose remainder is always nonnegative and is
// therefore returned as a [`Natural`]. A negative remainder occurs only for a negative divisor;
// adding the divisor's absolute value to the remainder and incrementing the quotient preserves $x =
// qy + r$ while making $r$ nonnegative.
fn make_remainder_nonnegative(q: Integer, r: Integer, other: Integer) -> (Integer, Natural) {
    if r < 0u32 {
        (q + Integer::ONE, (r - other).unsigned_abs())
    } else {
        (q, r.unsigned_abs())
    }
}

impl DivEuclidean<Self> for Integer {
    type DivOutput = Self;
    type ModOutput = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning the
    /// quotient and remainder. The quotient is rounded so that the remainder is nonnegative; the
    /// remainder is returned as a [`Natural`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor, \space
    /// x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_euclidean(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_euclidean(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: Self) -> (Self, Natural) {
        let (q, r) = self.div_mod(&other);
        make_remainder_nonnegative(q, r, other)
    }
}

impl DivEuclidean<&Self> for Integer {
    type DivOutput = Self;
    type ModOutput = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning the quotient and remainder. The quotient is rounded so that the
    /// remainder is nonnegative; the remainder is returned as a [`Natural`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor, \space
    /// x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_euclidean(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-3, 7)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Self) -> (Self, Natural) {
        let (q, r) = self.div_mod(other);
        if r < 0u32 {
            (q + Self::ONE, (r - other).unsigned_abs())
        } else {
            (q, r.unsigned_abs())
        }
    }
}

impl DivEuclidean<Integer> for &Integer {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The quotient is rounded so that the
    /// remainder is nonnegative; the remainder is returned as a [`Natural`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor, \space
    /// x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_euclidean(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: Integer) -> (Integer, Natural) {
        let (q, r) = self.div_mod(&other);
        make_remainder_nonnegative(q, r, other)
    }
}

impl DivEuclidean<&Integer> for &Integer {
    type DivOutput = Integer;
    type ModOutput = Natural;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning the
    /// quotient and remainder. The quotient is rounded so that the remainder is nonnegative; the
    /// remainder is returned as a [`Natural`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor, \space
    /// x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_euclidean(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Integer) -> (Integer, Natural) {
        let (q, r) = self.div_mod(other);
        if r < 0u32 {
            (q + Integer::ONE, (r - other).unsigned_abs())
        } else {
            (q, r.unsigned_abs())
        }
    }
}

impl DivAssignEuclidean<Self> for Integer {
    type ModOutput = Natural;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded so that the
    /// remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor,
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::DivAssignEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_euclidean(Integer::from(10)), 7);
    /// assert_eq!(x, -3);
    /// ```
    #[inline]
    fn div_assign_euclidean(&mut self, other: Self) -> Natural {
        let (q, r) = (&*self).div_euclidean(other);
        *self = q;
        r
    }
}

impl DivAssignEuclidean<&Self> for Integer {
    type ModOutput = Natural;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and returning the remainder. The quotient is rounded so that
    /// the remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(y) \left \lfloor \frac{x}{|y|} \right \rfloor,
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::DivAssignEuclidean;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * -10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_euclidean(&Integer::from(-10)), 7);
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn div_assign_euclidean(&mut self, other: &Self) -> Natural {
        let (q, r) = (&*self).div_euclidean(other);
        *self = q;
        r
    }
}
