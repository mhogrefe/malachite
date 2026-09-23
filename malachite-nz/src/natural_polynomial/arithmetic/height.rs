// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::{NaturalPolynomial, ZERO};
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;

impl Height for NaturalPolynomial {
    type Output = Natural;

    /// Returns the height of a [`NaturalPolynomial`]: the largest of its coefficients, taking the
    /// polynomial by reference and cloning.
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
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// coefficients, and $m$ is the number of bits of the height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .to_height(),
    ///     3
    /// );
    /// assert_eq!(NaturalPolynomial::from_str("x^100").unwrap().to_height(), 1);
    /// assert_eq!(NaturalPolynomial::from_str("0").unwrap().to_height(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_height` from `fmpz_poly/norms.c`, FLINT 3.6.0, for a
    /// polynomial whose coefficients are all nonnegative.
    #[inline]
    fn to_height(&self) -> Natural {
        self.height_ref().clone()
    }

    /// Returns the height of a [`NaturalPolynomial`]: the largest of its coefficients, taking the
    /// polynomial by value.
    ///
    /// The largest coefficient is moved out of the polynomial rather than cloned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .into_height(),
    ///     3
    /// );
    /// assert_eq!(NaturalPolynomial::from_str("0").unwrap().into_height(), 0);
    /// ```
    #[inline]
    fn into_height(self) -> Natural {
        self.into_coefficients_asc()
            .into_iter()
            .max()
            .unwrap_or(Natural::ZERO)
    }

    /// Returns the number of significant bits of the height of a [`NaturalPolynomial`].
    ///
    /// Since bit length is monotone, this is the largest of the coefficients' bit lengths, without
    /// materializing the height.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     2
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("0")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        self.coefficients_asc()
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0)
    }
}

impl HeightRef for NaturalPolynomial {
    /// Returns a reference to the height of a [`NaturalPolynomial`]: the largest of its
    /// coefficients.
    ///
    /// The height of a polynomial over the naturals is one of its coefficients, so nothing needs to
    /// be built. The zero polynomial has no coefficients, and a reference to zero is returned for
    /// it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HeightRef;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     *NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .height_ref(),
    ///     3
    /// );
    /// assert_eq!(*NaturalPolynomial::from_str("0").unwrap().height_ref(), 0);
    /// ```
    #[inline]
    fn height_ref(&self) -> &Natural {
        self.coefficients_asc().iter().max().unwrap_or(&ZERO)
    }
}
