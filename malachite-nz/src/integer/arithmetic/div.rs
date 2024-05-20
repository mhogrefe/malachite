// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::CheckedDiv;
use malachite_base::num::basic::traits::Zero;

impl Div<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value. The quotient is rounded
    /// towards zero. The quotient and remainder (which is not computed) satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23) / Integer::from(10), 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(Integer::from(23) / Integer::from(-10), -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(Integer::from(-23) / Integer::from(10), -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23) / Integer::from(-10), 2);
    /// ```
    #[inline]
    fn div(mut self, other: Integer) -> Integer {
        self /= other;
        self
    }
}

impl<'a> Div<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference. The quotient is rounded towards zero. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23) / &Integer::from(10), 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(Integer::from(23) / &Integer::from(-10), -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(Integer::from(-23) / &Integer::from(10), -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23) / &Integer::from(-10), 2);
    /// ```
    #[inline]
    fn div(mut self, other: &'a Integer) -> Integer {
        self /= other;
        self
    }
}

impl<'a> Div<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value. The quotient is rounded towards zero. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(&Integer::from(23) / Integer::from(10), 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(&Integer::from(23) / Integer::from(-10), -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(&Integer::from(-23) / Integer::from(10), -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(&Integer::from(-23) / Integer::from(-10), 2);
    /// ```
    #[inline]
    fn div(self, other: Integer) -> Integer {
        Integer::from_sign_and_abs(self.sign == other.sign, &self.abs / other.abs)
    }
}

impl<'a, 'b> Div<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference. The quotient is
    /// rounded towards zero. The quotient and remainder (which is not computed) satisfy $x = qy +
    /// r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(&Integer::from(23) / &Integer::from(10), 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(&Integer::from(23) / &Integer::from(-10), -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// assert_eq!(&Integer::from(-23) / &Integer::from(10), -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(&Integer::from(-23) / &Integer::from(-10), 2);
    /// ```
    #[inline]
    fn div(self, other: &'b Integer) -> Integer {
        Integer::from_sign_and_abs(self.sign == other.sign, &self.abs / &other.abs)
    }
}

impl DivAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value. The quotient is rounded towards zero. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= Integer::from(10);
    /// assert_eq!(x, 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= Integer::from(-10);
    /// assert_eq!(x, -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= Integer::from(10);
    /// assert_eq!(x, -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= Integer::from(-10);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Integer) {
        self.abs /= other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl<'a> DivAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference. The quotient is rounded towards zero. The quotient and
    /// remainder (which is not computed) satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= &Integer::from(10);
    /// assert_eq!(x, 2);
    ///
    /// // -2 * -10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x /= &Integer::from(-10);
    /// assert_eq!(x, -2);
    ///
    /// // -2 * 10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= &Integer::from(10);
    /// assert_eq!(x, -2);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x /= &Integer::from(-10);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &'a Integer) {
        self.abs /= &other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl CheckedDiv<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value. The quotient is rounded
    /// towards negative infinity. The quotient and remainder (which is not computed) satisfy $x =
    /// qy + r$ and $0 \leq r < y$. Returns `None` when the second [`Integer`] is zero, `Some`
    /// otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    ///         \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .checked_div(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "Some(-2)"
    /// );
    /// assert_eq!(Integer::ONE.checked_div(Integer::ZERO), None);
    /// ```
    #[inline]
    fn checked_div(self, other: Integer) -> Option<Integer> {
        match (self, other) {
            (_, integer_zero!()) => None,
            (x, y) => Some(x / y),
        }
    }
}

impl<'a> CheckedDiv<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference. The quotient is rounded towards negative infinity. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$. Returns `None` when the
    /// second [`Integer`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    ///         \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     Integer::from(23)
    ///         .checked_div(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "Some(-2)"
    /// );
    /// assert_eq!(Integer::ONE.checked_div(&Integer::ZERO), None);
    /// ```
    #[inline]
    fn checked_div(self, other: &'a Integer) -> Option<Integer> {
        match (self, other) {
            (_, &integer_zero!()) => None,
            (x, y) => Some(x / y),
        }
    }
}

impl<'a> CheckedDiv<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value. The quotient is rounded towards negative infinity. The quotient and remainder
    /// (which is not computed) satisfy $x = qy + r$ and $0 \leq r < y$. Returns `None` when the
    /// second [`Integer`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    ///         \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .checked_div(Integer::from(-10))
    ///         .to_debug_string(),
    ///     "Some(-2)"
    /// );
    /// assert_eq!((&Integer::ONE).checked_div(Integer::ZERO), None);
    /// ```
    fn checked_div(self, other: Integer) -> Option<Integer> {
        match (self, other) {
            (_, integer_zero!()) => None,
            (x, y) => Some(x / y),
        }
    }
}

impl<'a, 'b> CheckedDiv<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference. The quotient is
    /// rounded towards negative infinity. The quotient and remainder (which is not computed)
    /// satisfy $x = qy + r$ and $0 \leq r < y$. Returns `None` when the second [`Integer`] is zero,
    /// `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \left \lfloor \frac{x}{y} \right \rfloor \right ) &
    /// \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -2 * -10 + 3 = 23
    /// assert_eq!(
    ///     (&Integer::from(23))
    ///         .checked_div(&Integer::from(-10))
    ///         .to_debug_string(),
    ///     "Some(-2)"
    /// );
    /// assert_eq!((&Integer::ONE).checked_div(&Integer::ZERO), None);
    /// ```
    fn checked_div(self, other: &'b Integer) -> Option<Integer> {
        match (self, other) {
            (_, &integer_zero!()) => None,
            (x, y) => Some(x / y),
        }
    }
}
