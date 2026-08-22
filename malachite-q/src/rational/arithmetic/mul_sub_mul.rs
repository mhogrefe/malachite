// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign, SubMul};

// `mul_sub_mul` has no `fmpq` counterpart; it is the rational lift of the integer `mul_sub_mul`,
// which descends from FLINT's `fmpz_fmms`. Like [`add_mul`](super::add_mul) it forms each product
// and combines them rather than fusing, and for the same reason: the six gcds involved -- two
// cross-cancellations per product, then two for the sum -- each remove a different class of common
// factor, so there is no shorter route to the reduced result.

impl MulSubMul<Self, Self, Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking all
    /// four by value.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a> MulSubMul<Self, Self, &'a Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first, the second and the third by value and the fourth by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: &'a Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a> MulSubMul<Self, &'a Self, Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first, the second and the fourth by value and the third by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &'a Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a, 'b> MulSubMul<Self, &'a Self, &'b Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first and the second by value and the third and the fourth by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &'a Self, w: &'b Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a> MulSubMul<&'a Self, Self, Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first, the third and the fourth by value and the second by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &'a Self, z: Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a, 'b> MulSubMul<&'a Self, Self, &'b Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first and the third by value and the second and the fourth by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &'a Self, z: Self, w: &'b Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a, 'b> MulSubMul<&'a Self, &'b Self, Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first and the fourth by value and the second and the third by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &'a Self, z: &'b Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl<'a, 'b, 'c> MulSubMul<&'a Self, &'b Self, &'c Self> for Rational {
    type Output = Self;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking the
    /// first by value and the rest by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_sub_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &'a Self, z: &'b Self, w: &'c Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<&Rational, &Rational, &Rational> for &Rational {
    type Output = Rational;

    /// Subtracts the product of one pair of [`Rational`]s from the product of another, taking all
    /// four by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::ONE_HALF)
    ///         .mul_sub_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "-4/15"
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Rational, z: &Rational, w: &Rational) -> Rational {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMulAssign<Self, Self, Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking all four by value.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a> MulSubMulAssign<Self, Self, &'a Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first, the second and the third by value and the fourth by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: &'a Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a> MulSubMulAssign<Self, &'a Self, Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first, the second and the fourth by value and the third by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &'a Self, w: Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a, 'b> MulSubMulAssign<Self, &'a Self, &'b Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first and the second by value and the third and the fourth by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &'a Self, w: &'b Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a> MulSubMulAssign<&'a Self, Self, Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first, the third and the fourth by value and the second by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &'a Self, z: Self, w: Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a, 'b> MulSubMulAssign<&'a Self, Self, &'b Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first and the third by value and the second and the fourth by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &'a Self, z: Self, w: &'b Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a, 'b> MulSubMulAssign<&'a Self, &'b Self, Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first and the fourth by value and the second and the third by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &'a Self, z: &'b Self, w: Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}

impl<'a, 'b, 'c> MulSubMulAssign<&'a Self, &'b Self, &'c Self> for Rational {
    /// Subtracts the product of one pair of [`Rational`]s from the product of another, in place,
    /// taking the first by value and the rest by reference.
    ///
    /// $$
    /// x \gets xy - zw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulSubMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_sub_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "-4/15");
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &'a Self, z: &'b Self, w: &'c Self) {
        *self = (&*self * y).sub_mul(z, w);
    }
}
