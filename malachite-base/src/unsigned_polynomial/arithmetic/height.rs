// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Height;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> Height for UnsignedPolynomial<T> {
    type Output = T;

    /// Returns the height of a [`UnsignedPolynomial`]: the largest of its coefficients.
    ///
    /// The zero polynomial has no coefficients, and its height is 0.
    ///
    /// $$
    /// f(p) = H(p) = \max_i |p_i|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_height(),
    ///     3
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^100")
    ///         .unwrap()
    ///         .to_height(),
    ///     1
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("0")
    ///         .unwrap()
    ///         .to_height(),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_height` from `fmpz_poly/norms.c`, FLINT 3.6.0, for a
    /// polynomial whose coefficients are all nonnegative.
    #[inline]
    fn to_height(&self) -> T {
        self.coefficients_asc()
            .iter()
            .copied()
            .max()
            .unwrap_or(T::ZERO)
    }

    /// Returns the height of a [`UnsignedPolynomial`], taking it by value.
    ///
    /// A [`u64`] is [`Copy`], so this is the same work as [`to_height`](Height::to_height); it is
    /// here so that the two spellings agree across the types that implement [`Height`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .into_height(),
    ///     3
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("0")
    ///         .unwrap()
    ///         .into_height(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn into_height(self) -> T {
        self.to_height()
    }

    /// Returns the number of significant bits of the height of a [`UnsignedPolynomial`].
    ///
    /// Since bit length is monotone, this is the largest of the coefficients' bit lengths, which is
    /// the bit length of the largest coefficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     2
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("0")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        self.to_height().significant_bits()
    }
}
