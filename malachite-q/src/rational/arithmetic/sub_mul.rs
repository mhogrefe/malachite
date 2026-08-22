// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};

// Like `fmpq_submul`, this forms the product and adds it, rather than fusing the two steps. Both
// halves put their result in lowest terms, and the intermediate product is the smaller thing to
// reduce, so there is nothing to be saved by deferring that to a single reduction at the end.
//
// This is equivalent to `_fmpq_submul` from `fmpq/submul.c`, FLINT 3.6.0.

impl SubMul<Self, Self> for Rational {
    type Output = Self;

    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s, taking all three by
    /// value.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul(Rational::from_signeds(2, 3), Rational::from_signeds(3, 4))
    ///         .to_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .sub_mul(Rational::from_signeds(-1, 2), Rational::from_signeds(1, 3))
    ///         .to_string(),
    ///     "139/42"
    /// );
    /// ```
    #[inline]
    fn sub_mul(self, y: Self, z: Self) -> Self {
        self - y * z
    }
}

impl<'a> SubMul<Self, &'a Self> for Rational {
    type Output = Self;

    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s, taking the first two by
    /// value and the third by reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul(Rational::from_signeds(2, 3), &Rational::from_signeds(3, 4))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn sub_mul(self, y: Self, z: &'a Self) -> Self {
        self - y * z
    }
}

impl<'a> SubMul<&'a Self, Self> for Rational {
    type Output = Self;

    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s, taking the first and
    /// third by value and the second by reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul(&Rational::from_signeds(2, 3), Rational::from_signeds(3, 4))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn sub_mul(self, y: &'a Self, z: Self) -> Self {
        self - y * z
    }
}

impl<'a, 'b> SubMul<&'a Self, &'b Self> for Rational {
    type Output = Self;

    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s, taking the first by
    /// value and the second and third by reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul(&Rational::from_signeds(2, 3), &Rational::from_signeds(3, 4))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn sub_mul(self, y: &'a Self, z: &'b Self) -> Self {
        self - y * z
    }
}

impl SubMul<&Rational, &Rational> for &Rational {
    type Output = Rational;

    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s, taking all three by
    /// reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::ONE_HALF)
    ///         .sub_mul(&Rational::from_signeds(2, 3), &Rational::from_signeds(3, 4))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn sub_mul(self, y: &Rational, z: &Rational) -> Rational {
        self - y * z
    }
}

impl SubMulAssign<Self, Self> for Rational {
    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s in place, taking both
    /// [`Rational`]s on the right-hand side by value.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_assign(Rational::from_signeds(2, 3), Rational::from_signeds(3, 4));
    /// assert_eq!(x.to_string(), "0");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: Self, z: Self) {
        *self -= y * z;
    }
}

impl<'a> SubMulAssign<Self, &'a Self> for Rational {
    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s in place, taking the
    /// first [`Rational`] on the right-hand side by value and the second by reference.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_assign(Rational::from_signeds(2, 3), &Rational::from_signeds(3, 4));
    /// assert_eq!(x.to_string(), "0");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: Self, z: &'a Self) {
        *self -= y * z;
    }
}

impl<'a> SubMulAssign<&'a Self, Self> for Rational {
    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s in place, taking the
    /// first [`Rational`] on the right-hand side by reference and the second by value.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_assign(&Rational::from_signeds(2, 3), Rational::from_signeds(3, 4));
    /// assert_eq!(x.to_string(), "0");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: &'a Self, z: Self) {
        *self -= y * z;
    }
}

impl<'a, 'b> SubMulAssign<&'a Self, &'b Self> for Rational {
    /// Subtracts a [`Rational`] by the product of two other [`Rational`]s in place, taking both
    /// [`Rational`]s on the right-hand side by reference.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_assign(&Rational::from_signeds(2, 3), &Rational::from_signeds(3, 4));
    /// assert_eq!(x.to_string(), "0");
    /// ```
    #[inline]
    fn sub_mul_assign(&mut self, y: &'a Self, z: &'b Self) {
        *self -= y * z;
    }
}
