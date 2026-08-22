// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{AddMul, MulAddMul, MulAddMulAssign};

// `mul_add_mul` has no `fmpq` counterpart; it is the rational lift of the integer `mul_add_mul`,
// which descends from FLINT's `fmpz_fmma`. Like [`add_mul`](super::add_mul) it forms each product
// and combines them rather than fusing, and for the same reason: the six gcds involved -- two
// cross-cancellations per product, then two for the sum -- each remove a different class of common
// factor, so there is no shorter route to the reduced result.

impl MulAddMul<Self, Self, Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking all four by value.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a> MulAddMul<Self, Self, &'a Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first, the second and the third
    /// by value and the fourth by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: &'a Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a> MulAddMul<Self, &'a Self, Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first, the second and the fourth
    /// by value and the third by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &'a Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a, 'b> MulAddMul<Self, &'a Self, &'b Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first and the second by value
    /// and the third and the fourth by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &'a Self, w: &'b Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a> MulAddMul<&'a Self, Self, Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first, the third and the fourth
    /// by value and the second by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &'a Self, z: Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a, 'b> MulAddMul<&'a Self, Self, &'b Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first and the third by value and
    /// the second and the fourth by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &'a Self, z: Self, w: &'b Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a, 'b> MulAddMul<&'a Self, &'b Self, Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first and the fourth by value
    /// and the second and the third by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &'a Self, z: &'b Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl<'a, 'b, 'c> MulAddMul<&'a Self, &'b Self, &'c Self> for Rational {
    type Output = Self;

    /// Adds the products of two pairs of [`Rational`]s, taking the first by value and the rest by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .mul_add_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &'a Self, z: &'b Self, w: &'c Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<&Rational, &Rational, &Rational> for &Rational {
    type Output = Rational;

    /// Adds the products of two pairs of [`Rational`]s, taking all four by reference.
    ///
    /// $$
    /// f(x, y, z, w) = xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::ONE_HALF)
    ///         .mul_add_mul(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             &Rational::from_signeds(4, 5)
    ///         )
    ///         .to_string(),
    ///     "14/15"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Rational, z: &Rational, w: &Rational) -> Rational {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMulAssign<Self, Self, Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking all four by value.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a> MulAddMulAssign<Self, Self, &'a Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first, the second and
    /// the third by value and the fourth by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: &'a Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a> MulAddMulAssign<Self, &'a Self, Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first, the second and
    /// the fourth by value and the third by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &'a Self, w: Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a, 'b> MulAddMulAssign<Self, &'a Self, &'b Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first and the second
    /// by value and the third and the fourth by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &'a Self, w: &'b Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a> MulAddMulAssign<&'a Self, Self, Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first, the third and
    /// the fourth by value and the second by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &'a Self, z: Self, w: Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a, 'b> MulAddMulAssign<&'a Self, Self, &'b Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first and the third by
    /// value and the second and the fourth by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &'a Self, z: Self, w: &'b Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a, 'b> MulAddMulAssign<&'a Self, &'b Self, Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first and the fourth
    /// by value and the second and the third by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &'a Self, z: &'b Self, w: Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}

impl<'a, 'b, 'c> MulAddMulAssign<&'a Self, &'b Self, &'c Self> for Rational {
    /// Adds the products of two pairs of [`Rational`]s, in place, taking the first by value and the
    /// rest by reference.
    ///
    /// $$
    /// x \gets xy + zw.
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
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.mul_add_mul_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     &Rational::from_signeds(4, 5),
    /// );
    /// assert_eq!(x.to_string(), "14/15");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &'a Self, z: &'b Self, w: &'c Self) {
        *self = (&*self * y).add_mul(z, w);
    }
}
