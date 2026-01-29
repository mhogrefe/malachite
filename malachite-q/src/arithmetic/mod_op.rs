// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::mem::swap;
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, DivRound, Mod, ModAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;

impl Mod<Self> for Rational {
    type Output = Self;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking both
    /// by reference. The remainder has the same sign as the second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .mod_op(Rational::ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -3 * 1 + 9/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .mod_op(Rational::ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    ///
    /// // 3 * -1 - 9/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .mod_op(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .mod_op(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, other: Self) -> Self {
        let x_sign = self >= 0u32;
        let (n1, d1) = self.into_numerator_and_denominator();
        let y_sign = other >= 0u32;
        let (n2, d2) = other.into_numerator_and_denominator();
        let n1d2 = Integer::from_sign_and_abs(x_sign, n1 * &d2);
        let n2d1 = Integer::from_sign_and_abs(y_sign, n2 * &d1);
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n = n1d2 - n2d1 * q;
        Self::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl Mod<&Self> for Rational {
    type Output = Self;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking the
    /// first by value and the second by reference. The remainder has the same sign as the second
    /// [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .mod_op(&Rational::ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -3 * 1 + 9/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .mod_op(&Rational::ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    ///
    /// // 3 * -1 - 9/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .mod_op(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .mod_op(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    /// ```
    #[inline]
    fn mod_op(self, other: &Self) -> Self {
        let x_sign = self >= 0u32;
        let (n1, d1) = self.into_numerator_and_denominator();
        let (n2, d2) = other.numerator_and_denominator_ref();
        let n1d2 = Integer::from_sign_and_abs(x_sign, n1 * d2);
        let n2d1 = Integer::from_sign_and_abs(*other >= 0u32, n2 * &d1);
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n = n1d2 - n2d1 * q;
        Self::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl Mod<Rational> for &Rational {
    type Output = Rational;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking the
    /// first by reference and the second by value. The remainder has the same sign as the second
    /// [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ``````
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .mod_op(Rational::ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -3 * 1 + 9/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .mod_op(Rational::ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    ///
    /// // 3 * -1 - 9/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .mod_op(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .mod_op(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    /// ```
    fn mod_op(self, other: Rational) -> Rational {
        let (n1, d1) = self.numerator_and_denominator_ref();
        let y_sign = other >= 0u32;
        let (n2, d2) = other.into_numerator_and_denominator();
        let n1d2 = Integer::from_sign_and_abs(*self >= 0u32, n1 * &d2);
        let n2d1 = Integer::from_sign_and_abs(y_sign, n2 * d1);
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n = n1d2 - n2d1 * q;
        Rational::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl Mod<&Rational> for &Rational {
    type Output = Rational;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking both
    /// by reference. The remainder has the same sign as the second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .mod_op(&Rational::ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -3 * 1 + 9/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .mod_op(&Rational::ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    ///
    /// // 3 * -1 - 9/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .mod_op(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .mod_op(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    /// ```
    fn mod_op(self, other: &Rational) -> Rational {
        let (n1, d1) = self.numerator_and_denominator_ref();
        let (n2, d2) = other.numerator_and_denominator_ref();
        let n1d2 = Integer::from_sign_and_abs(*self >= 0u32, n1 * d2);
        let n2d1 = Integer::from_sign_and_abs(*other >= 0u32, n2 * d1);
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n = n1d2 - n2d1 * q;
        Rational::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl ModAssign<Self> for Rational {
    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], mutating the
    /// first in place and taking the second by value. The remainder has the same sign as the second
    /// [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.mod_assign(Rational::ONE);
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // -3 * 1 + 9/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.mod_assign(Rational::ONE);
    /// assert_eq!(x.to_string(), "9/10");
    ///
    /// // 3 * -1 - 9/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.mod_assign(Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "-9/10");
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.mod_assign(Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "-1/10");
    /// ```
    fn mod_assign(&mut self, other: Self) {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        *self = x.mod_op(other);
    }
}

impl ModAssign<&Self> for Rational {
    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], mutating the
    /// first in place and taking the second by reference. The remainder has the same sign as the
    /// second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.mod_assign(&Rational::ONE);
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // -3 * 1 + 9/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.mod_assign(&Rational::ONE);
    /// assert_eq!(x.to_string(), "9/10");
    ///
    /// // 3 * -1 - 9/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.mod_assign(&Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "-9/10");
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.mod_assign(&Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "-1/10");
    /// ```
    fn mod_assign(&mut self, other: &Self) {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        *self = x.mod_op(other);
    }
}

impl Rem<Self> for Rational {
    type Output = Self;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking both
    /// by value. The remainder has the same sign as the first [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (Rational::from_unsigneds(21u8, 10) % Rational::ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -1/10
    /// assert_eq!(
    ///     (Rational::from_signeds(-21, 10) % Rational::ONE).to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (Rational::from_unsigneds(21u8, 10) % Rational::NEGATIVE_ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (Rational::from_signeds(-21, 10) % Rational::NEGATIVE_ONE).to_string(),
    ///     "-1/10"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: Self) -> Self {
        let x_sign = self >= 0;
        let (n1, d1) = self.into_numerator_and_denominator();
        let (n2, d2) = other.into_numerator_and_denominator();
        let n1d2 = n1 * &d2;
        let n2d1 = n2 * &d1;
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n2d1q = Integer::from_sign_and_abs(x_sign, n2d1 * q);
        let n = Integer::from_sign_and_abs(x_sign, n1d2) - n2d1q;
        Self::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl Rem<&Self> for Rational {
    type Output = Self;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking the
    /// first by value and the second by reference. The remainder has the same sign as the first
    /// [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (Rational::from_unsigneds(21u8, 10) % &Rational::ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -1/10
    /// assert_eq!(
    ///     (Rational::from_signeds(-21, 10) % &Rational::ONE).to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (Rational::from_unsigneds(21u8, 10) % &Rational::NEGATIVE_ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (Rational::from_signeds(-21, 10) % &Rational::NEGATIVE_ONE).to_string(),
    ///     "-1/10"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: &Self) -> Self {
        let x_sign = self >= 0;
        let (n1, d1) = self.into_numerator_and_denominator();
        let (n2, d2) = other.numerator_and_denominator_ref();
        let n1d2 = n1 * d2;
        let n2d1 = n2 * &d1;
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n2d1q = Integer::from_sign_and_abs(x_sign, n2d1 * q);
        let n = Integer::from_sign_and_abs(x_sign, n1d2) - n2d1q;
        Self::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl Rem<Rational> for &Rational {
    type Output = Rational;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking the
    /// first by reference and the second by value. The remainder has the same sign as the first
    /// [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10) % Rational::ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -1/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10) % Rational::ONE).to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10) % Rational::NEGATIVE_ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10) % Rational::NEGATIVE_ONE).to_string(),
    ///     "-1/10"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: Rational) -> Rational {
        let x_sign = *self >= 0;
        let (n1, d1) = self.numerator_and_denominator_ref();
        let (n2, d2) = other.into_numerator_and_denominator();
        let n1d2 = n1 * &d2;
        let n2d1 = n2 * d1;
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n2d1q = Integer::from_sign_and_abs(x_sign, n2d1 * q);
        let n = Integer::from_sign_and_abs(x_sign, n1d2) - n2d1q;
        Rational::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl Rem<&Rational> for &Rational {
    type Output = Rational;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking both
    /// by reference. The remainder has the same sign as the first [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10) % &Rational::ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -1/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10) % &Rational::ONE).to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10) % &Rational::NEGATIVE_ONE).to_string(),
    ///     "1/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10) % &Rational::NEGATIVE_ONE).to_string(),
    ///     "-1/10"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: &Rational) -> Rational {
        let x_sign = *self >= 0;
        let (n1, d1) = self.numerator_and_denominator_ref();
        let (n2, d2) = other.numerator_and_denominator_ref();
        let n1d2 = n1 * d2;
        let n2d1 = n2 * d1;
        let q = (&n1d2).div_round(&n2d1, Floor).0;
        let n2d1q = Integer::from_sign_and_abs(x_sign, n2d1 * q);
        let n = Integer::from_sign_and_abs(x_sign, n1d2) - n2d1q;
        Rational::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl RemAssign<Self> for Rational {
    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], mutating the
    /// first in place and taking the second by value. The remainder has the same sign as the first
    /// [`Rational`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x %= Rational::ONE;
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // -2 * 1 - 1/10 = -1/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x %= Rational::ONE;
    /// assert_eq!(x.to_string(), "-1/10");
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x %= Rational::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x %= Rational::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "-1/10");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        *self = x % other;
    }
}

impl RemAssign<&Self> for Rational {
    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], mutating the
    /// first in place and taking the second by reference. The remainder has the same sign as the
    /// first [`Rational`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 2 * 1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x %= &Rational::ONE;
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // -2 * 1 - 1/10 = -1/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x %= &Rational::ONE;
    /// assert_eq!(x.to_string(), "-1/10");
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x %= &Rational::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x %= &Rational::NEGATIVE_ONE;
    /// assert_eq!(x.to_string(), "-1/10");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Self) {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        *self = x % other;
    }
}

impl CeilingMod<Self> for Rational {
    type Output = Self;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking both
    /// by value. The remainder has the opposite sign as the second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 3 * 1 - 9/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .ceiling_mod(Rational::ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .ceiling_mod(Rational::ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .ceiling_mod(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // 3 * -1 + 9/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .ceiling_mod(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    /// ```
    #[inline]
    fn ceiling_mod(self, other: Self) -> Self {
        let x_sign = self >= 0u32;
        let (n1, d1) = self.into_numerator_and_denominator();
        let y_sign = other >= 0u32;
        let (n2, d2) = other.into_numerator_and_denominator();
        let n1d2 = Integer::from_sign_and_abs(x_sign, n1 * &d2);
        let n2d1 = Integer::from_sign_and_abs(y_sign, n2 * &d1);
        let q = (&n1d2).div_round(&n2d1, Ceiling).0;
        let n = n1d2 - n2d1 * q;
        Self::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl CeilingMod<&Self> for Rational {
    type Output = Self;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking the
    /// first by value and the second by reference. The remainder has the opposite sign as the
    /// second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 3 * 1 - 9/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .ceiling_mod(&Rational::ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .ceiling_mod(&Rational::ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     Rational::from_unsigneds(21u8, 10)
    ///         .ceiling_mod(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // 3 * -1 + 9/10 = -21/10
    /// assert_eq!(
    ///     Rational::from_signeds(-21, 10)
    ///         .ceiling_mod(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    /// ```
    #[inline]
    fn ceiling_mod(self, other: &Self) -> Self {
        let x_sign = self >= 0u32;
        let (n1, d1) = self.into_numerator_and_denominator();
        let (n2, d2) = other.numerator_and_denominator_ref();
        let n1d2 = Integer::from_sign_and_abs(x_sign, n1 * d2);
        let n2d1 = Integer::from_sign_and_abs(*other > 0u32, n2 * &d1);
        let q = (&n1d2).div_round(&n2d1, Ceiling).0;
        let n = n1d2 - n2d1 * q;
        Self::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl CeilingMod<Rational> for &Rational {
    type Output = Rational;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking the
    /// first by reference and the second by value. The remainder has the opposite sign as the
    /// second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 3 * 1 - 9/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .ceiling_mod(Rational::ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .ceiling_mod(Rational::ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .ceiling_mod(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // 3 * -1 + 9/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .ceiling_mod(Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    /// ```
    fn ceiling_mod(self, other: Rational) -> Rational {
        let (n1, d1) = self.numerator_and_denominator_ref();
        let y_sign = other >= 0u32;
        let (n2, d2) = other.into_numerator_and_denominator();
        let n1d2 = Integer::from_sign_and_abs(*self >= 0u32, n1 * &d2);
        let n2d1 = Integer::from_sign_and_abs(y_sign, n2 * d1);
        let q = (&n1d2).div_round(&n2d1, Ceiling).0;
        let n = n1d2 - n2d1 * q;
        Rational::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl CeilingMod<&Rational> for &Rational {
    type Output = Rational;

    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], taking both
    /// by reference. The remainder has the opposite sign as the second [`Rational`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 3 * 1 - 9/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .ceiling_mod(&Rational::ONE)
    ///         .to_string(),
    ///     "-9/10"
    /// );
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .ceiling_mod(&Rational::ONE)
    ///         .to_string(),
    ///     "-1/10"
    /// );
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// assert_eq!(
    ///     (&Rational::from_unsigneds(21u8, 10))
    ///         .ceiling_mod(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "1/10"
    /// );
    ///
    /// // 3 * -1 + 9/10 = -21/10
    /// assert_eq!(
    ///     (&Rational::from_signeds(-21, 10))
    ///         .ceiling_mod(&Rational::NEGATIVE_ONE)
    ///         .to_string(),
    ///     "9/10"
    /// );
    /// ```
    fn ceiling_mod(self, other: &Rational) -> Rational {
        let (n1, d1) = self.numerator_and_denominator_ref();
        let (n2, d2) = other.numerator_and_denominator_ref();
        let n1d2 = Integer::from_sign_and_abs(*self >= 0u32, n1 * d2);
        let n2d1 = Integer::from_sign_and_abs(*other >= 0u32, n2 * d1);
        let q = (&n1d2).div_round(&n2d1, Ceiling).0;
        let n = n1d2 - n2d1 * q;
        Rational::from_sign_and_naturals(n >= 0, n.unsigned_abs(), d1 * d2)
    }
}

impl CeilingModAssign<Self> for Rational {
    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], mutating the
    /// first in place and taking the second by value. The remainder has the opposite sign as the
    /// second number.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lceil\frac{x}{y} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 3 * 1 - 9/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.ceiling_mod_assign(Rational::ONE);
    /// assert_eq!(x.to_string(), "-9/10");
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.ceiling_mod_assign(Rational::ONE);
    /// assert_eq!(x.to_string(), "-1/10");
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.ceiling_mod_assign(Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // 3 * -1 + 9/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.ceiling_mod_assign(Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "9/10");
    /// ```
    fn ceiling_mod_assign(&mut self, other: Self) {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        *self = x.ceiling_mod(other);
    }
}

impl CeilingModAssign<&Self> for Rational {
    /// Computes the remainder when a [`Rational`] is divided by another [`Rational`], mutating the
    /// first in place and taking the second by reference. The remainder has the opposite sign as
    /// the second number.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lceil\frac{x}{y} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
    /// use malachite_base::num::basic::traits::{NegativeOne, One};
    /// use malachite_q::Rational;
    ///
    /// // 3 * 1 - 9/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.ceiling_mod_assign(&Rational::ONE);
    /// assert_eq!(x.to_string(), "-9/10");
    ///
    /// // -2 * 1 - 1/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.ceiling_mod_assign(&Rational::ONE);
    /// assert_eq!(x.to_string(), "-1/10");
    ///
    /// // -2 * -1 + 1/10 = 21/10
    /// let mut x = Rational::from_unsigneds(21u8, 10);
    /// x.ceiling_mod_assign(&Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "1/10");
    ///
    /// // 3 * -1 + 9/10 = -21/10
    /// let mut x = Rational::from_signeds(-21, 10);
    /// x.ceiling_mod_assign(&Rational::NEGATIVE_ONE);
    /// assert_eq!(x.to_string(), "9/10");
    /// ```
    fn ceiling_mod_assign(&mut self, other: &Self) {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        *self = x.ceiling_mod(other);
    }
}
