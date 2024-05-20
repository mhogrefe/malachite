// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem,
};

impl DivMod<Integer> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// has the same sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-3, -7)"
    /// );
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-3, 7)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    #[inline]
    fn div_mod(mut self, other: Integer) -> (Integer, Integer) {
        let r = self.div_assign_mod(other);
        (self, r)
    }
}

impl<'a> DivMod<&'a Integer> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning the quotient and remainder. The quotient is rounded towards negative
    /// infinity, and the remainder has the same sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-3, -7)"
    /// );
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-3, 7)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    #[inline]
    fn div_mod(mut self, other: &'a Integer) -> (Integer, Integer) {
        let r = self.div_assign_mod(other);
        (self, r)
    }
}

impl<'a> DivMod<Integer> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The quotient is rounded towards negative
    /// infinity, and the remainder has the same sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-3, -7)"
    /// );
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-3, 7)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    fn div_mod(self, other: Integer) -> (Integer, Integer) {
        let q_sign = self.sign == other.sign;
        let (q, r) = if q_sign {
            (&self.abs).div_mod(other.abs)
        } else {
            (&self.abs).ceiling_div_neg_mod(other.abs)
        };
        (
            Integer::from_sign_and_abs(q_sign, q),
            Integer::from_sign_and_abs(other.sign, r),
        )
    }
}

impl<'a, 'b> DivMod<&'b Integer> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning the
    /// quotient and remainder. The quotient is rounded towards negative infinity, and the remainder
    /// has the same sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-3, -7)"
    /// );
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-3, 7)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    fn div_mod(self, other: &'b Integer) -> (Integer, Integer) {
        let q_sign = self.sign == other.sign;
        let (q, r) = if q_sign {
            (&self.abs).div_mod(&other.abs)
        } else {
            (&self.abs).ceiling_div_neg_mod(&other.abs)
        };
        (
            Integer::from_sign_and_abs(q_sign, q),
            Integer::from_sign_and_abs(other.sign, r),
        )
    }
}

impl DivAssignMod<Integer> for Integer {
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded towards
    /// negative infinity, and the remainder has the same sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_mod(Integer::from(10)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_mod(Integer::from(-10)), -7);
    /// assert_eq!(x, -3);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_mod(Integer::from(10)), 7);
    /// assert_eq!(x, -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_mod(Integer::from(-10)), -3);
    /// assert_eq!(x, 2);
    /// ```
    fn div_assign_mod(&mut self, other: Integer) -> Integer {
        let r = if self.sign == other.sign {
            self.sign = true;
            self.abs.div_assign_mod(other.abs)
        } else {
            let r = self.abs.ceiling_div_assign_neg_mod(other.abs);
            if self.abs != 0 {
                self.sign = false;
            }
            r
        };
        Integer::from_sign_and_abs(other.sign, r)
    }
}

impl<'a> DivAssignMod<&'a Integer> for Integer {
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and returning the remainder. The quotient is rounded towards
    /// negative infinity, and the remainder has the same sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_mod(&Integer::from(10)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_mod(&Integer::from(-10)), -7);
    /// assert_eq!(x, -3);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_mod(&Integer::from(10)), 7);
    /// assert_eq!(x, -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_mod(&Integer::from(-10)), -3);
    /// assert_eq!(x, 2);
    /// ```
    fn div_assign_mod(&mut self, other: &'a Integer) -> Integer {
        let r = if self.sign == other.sign {
            self.sign = true;
            self.abs.div_assign_mod(&other.abs)
        } else {
            let r = self.abs.ceiling_div_assign_neg_mod(&other.abs);
            if self.abs != 0 {
                self.sign = false;
            }
            r
        };
        Integer::from_sign_and_abs(other.sign, r)
    }
}

impl DivRem<Integer> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the first [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
    /// \right \rfloor, \space
    /// x - y \operatorname{sgn}(xy)
    /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_rem(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_rem(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_rem(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_rem(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    #[inline]
    fn div_rem(mut self, other: Integer) -> (Integer, Integer) {
        let r = self.div_assign_rem(other);
        (self, r)
    }
}

impl<'a> DivRem<&'a Integer> for Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning the quotient and remainder. The quotient is rounded towards zero and
    /// the remainder has the same sign as the first [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
    /// \right \rfloor, \space
    /// x - y \operatorname{sgn}(xy)
    /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_rem(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .div_rem(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_rem(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .div_rem(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    #[inline]
    fn div_rem(mut self, other: &'a Integer) -> (Integer, Integer) {
        let r = self.div_assign_rem(other);
        (self, r)
    }
}

impl<'a> DivRem<Integer> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The quotient is rounded towards zero and
    /// the remainder has the same sign as the first [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
    /// \right \rfloor, \space
    /// x - y \operatorname{sgn}(xy)
    /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_rem(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_rem(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_rem(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_rem(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    #[inline]
    fn div_rem(self, other: Integer) -> (Integer, Integer) {
        let (q, r) = (&self.abs).div_mod(other.abs);
        (
            Integer::from_sign_and_abs(self.sign == other.sign, q),
            Integer::from_sign_and_abs(self.sign, r),
        )
    }
}

impl<'a, 'b> DivRem<&'b Integer> for &'a Integer {
    type DivOutput = Integer;
    type RemOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning the
    /// quotient and remainder. The quotient is rounded towards zero and the remainder has the same
    /// sign as the first [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
    /// \right \rfloor, \space
    /// x - y \operatorname{sgn}(xy)
    /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor \right ).
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_rem(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .div_rem(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_rem(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .div_rem(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(2, -3)"
    /// );
    /// ```
    #[inline]
    fn div_rem(self, other: &'b Integer) -> (Integer, Integer) {
        let (q, r) = (&self.abs).div_mod(&other.abs);
        (
            Integer::from_sign_and_abs(self.sign == other.sign, q),
            Integer::from_sign_and_abs(self.sign, r),
        )
    }
}

impl DivAssignRem<Integer> for Integer {
    type RemOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded towards zero
    /// and the remainder has the same sign as the first [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor,
    /// $$
    /// $$
    /// x \gets \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
    /// \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_rem(Integer::from(10)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_rem(Integer::from(-10)), 3);
    /// assert_eq!(x, -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_rem(Integer::from(10)), -3);
    /// assert_eq!(x, -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_rem(Integer::from(-10)), -3);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: Integer) -> Integer {
        let r = Integer::from_sign_and_abs(self.sign, self.abs.div_assign_mod(other.abs));
        self.sign = self.sign == other.sign || self.abs == 0;
        r
    }
}

impl<'a> DivAssignRem<&'a Integer> for Integer {
    type RemOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and returning the remainder. The quotient is rounded towards
    /// zero and the remainder has the same sign as the first [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor,
    /// $$
    /// $$
    /// x \gets \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
    /// \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_rem(&Integer::from(10)), 3);
    /// assert_eq!(x, 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.div_assign_rem(&Integer::from(-10)), 3);
    /// assert_eq!(x, -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_rem(&Integer::from(10)), -3);
    /// assert_eq!(x, -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.div_assign_rem(&Integer::from(-10)), -3);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_assign_rem(&mut self, other: &'a Integer) -> Integer {
        let r = Integer::from_sign_and_abs(self.sign, self.abs.div_assign_mod(&other.abs));
        self.sign = self.sign == other.sign || self.abs == 0;
        r
    }
}

impl CeilingDivMod<Integer> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning the
    /// quotient and remainder. The quotient is rounded towards positive infinity and the remainder
    /// has the opposite sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// x - y\left \lceil \frac{x}{y} \right \rceil \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * 10 + -7 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .ceiling_div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(3, -7)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .ceiling_div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .ceiling_div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .ceiling_div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    /// ```
    #[inline]
    fn ceiling_div_mod(mut self, other: Integer) -> (Integer, Integer) {
        let r = self.ceiling_div_assign_mod(other);
        (self, r)
    }
}

impl<'a> CeilingDivMod<&'a Integer> for Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both the first by value and the second
    /// by reference and returning the quotient and remainder. The quotient is rounded towards
    /// positive infinity and the remainder has the opposite sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// x - y\left \lceil \frac{x}{y} \right \rceil \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * 10 + -7 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .ceiling_div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(3, -7)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .ceiling_div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .ceiling_div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(
    ///     Integer::from(-23)
    ///         .ceiling_div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    /// ```
    #[inline]
    fn ceiling_div_mod(mut self, other: &'a Integer) -> (Integer, Integer) {
        let r = self.ceiling_div_assign_mod(other);
        (self, r)
    }
}

impl<'a> CeilingDivMod<Integer> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The quotient is rounded towards positive
    /// infinity and the remainder has the opposite sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// x - y\left \lceil \frac{x}{y} \right \rceil \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * 10 + -7 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .ceiling_div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(3, -7)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .ceiling_div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .ceiling_div_mod(Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .ceiling_div_mod(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    /// ```
    fn ceiling_div_mod(self, other: Integer) -> (Integer, Integer) {
        let q_sign = self.sign == other.sign;
        let (q, r) = if q_sign {
            (&self.abs).ceiling_div_neg_mod(other.abs)
        } else {
            (&self.abs).div_mod(other.abs)
        };
        (
            Integer::from_sign_and_abs(q_sign, q),
            Integer::from_sign_and_abs(!other.sign, r),
        )
    }
}

impl<'a, 'b> CeilingDivMod<&'b Integer> for &'a Integer {
    type DivOutput = Integer;
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning the
    /// quotient and remainder. The quotient is rounded towards positive infinity and the remainder
    /// has the opposite sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
    /// x - y\left \lceil \frac{x}{y} \right \rceil \right ).
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivMod;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * 10 + -7 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .ceiling_div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(3, -7)"
    /// );
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .ceiling_div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(-2, 3)"
    /// );
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .ceiling_div_mod(&Integer::from(10))
    ///         .to_debug_string(),
    ///     "(-2, -3)"
    /// );
    ///
    /// // 3 * -10 + 7 = -23
    /// assert_eq!(
    ///     (&Integer::from(-23))
    ///         .ceiling_div_mod(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "(3, 7)"
    /// );
    /// ```
    fn ceiling_div_mod(self, other: &'b Integer) -> (Integer, Integer) {
        let q_sign = self.sign == other.sign;
        let (q, r) = if q_sign {
            (&self.abs).ceiling_div_neg_mod(&other.abs)
        } else {
            (&self.abs).div_mod(&other.abs)
        };
        (
            Integer::from_sign_and_abs(q_sign, q),
            Integer::from_sign_and_abs(!other.sign, r),
        )
    }
}

impl CeilingDivAssignMod<Integer> for Integer {
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value and returning the remainder. The quotient is rounded towards
    /// positive infinity and the remainder has the opposite sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lceil\frac{x}{y} \right \rceil,
    /// $$
    /// $$
    /// x \gets \left \lceil \frac{x}{y} \right \rceil.
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * 10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.ceiling_div_assign_mod(Integer::from(10)), -7);
    /// assert_eq!(x, 3);
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.ceiling_div_assign_mod(Integer::from(-10)), 3);
    /// assert_eq!(x, -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.ceiling_div_assign_mod(Integer::from(10)), -3);
    /// assert_eq!(x, -2);
    ///
    /// // 3 * -10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.ceiling_div_assign_mod(Integer::from(-10)), 7);
    /// assert_eq!(x, 3);
    /// ```
    fn ceiling_div_assign_mod(&mut self, other: Integer) -> Integer {
        let r = if self.sign == other.sign {
            self.sign = true;
            self.abs.ceiling_div_assign_neg_mod(other.abs)
        } else {
            let r = self.abs.div_assign_mod(other.abs);
            self.sign = self.abs == 0;
            r
        };
        Integer::from_sign_and_abs(!other.sign, r)
    }
}

impl<'a> CeilingDivAssignMod<&'a Integer> for Integer {
    type ModOutput = Integer;

    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference and returning the remainder. The quotient is rounded towards
    /// positive infinity and the remainder has the opposite sign as the second [`Integer`].
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lceil\frac{x}{y} \right \rceil,
    /// $$
    /// $$
    /// x \gets \left \lceil \frac{x}{y} \right \rceil.
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
    /// use malachite_base::num::arithmetic::traits::CeilingDivAssignMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 3 * 10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.ceiling_div_assign_mod(&Integer::from(10)), -7);
    /// assert_eq!(x, 3);
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// assert_eq!(x.ceiling_div_assign_mod(&Integer::from(-10)), 3);
    /// assert_eq!(x, -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.ceiling_div_assign_mod(&Integer::from(10)), -3);
    /// assert_eq!(x, -2);
    ///
    /// // 3 * -10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// assert_eq!(x.ceiling_div_assign_mod(&Integer::from(-10)), 7);
    /// assert_eq!(x, 3);
    /// ```
    fn ceiling_div_assign_mod(&mut self, other: &'a Integer) -> Integer {
        let r = if self.sign == other.sign {
            self.sign = true;
            self.abs.ceiling_div_assign_neg_mod(&other.abs)
        } else {
            let r = self.abs.div_assign_mod(&other.abs);
            self.sign = self.abs == 0;
            r
        };
        Integer::from_sign_and_abs(!other.sign, r)
    }
}
