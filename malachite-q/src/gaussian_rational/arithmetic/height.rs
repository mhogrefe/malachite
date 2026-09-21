// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_nz::natural::Natural;

impl Height for GaussianRational {
    type Output = Natural;

    /// Returns the height of a [`GaussianRational`]: the larger of the heights of its real and
    /// imaginary parts, taking the [`GaussianRational`] by reference and cloning.
    ///
    /// A [`Rational`](crate::Rational)'s height is the larger of the absolute value of its
    /// numerator and its denominator, so this is the largest of four magnitudes. The height of zero
    /// is 1, as it is for a [`Rational`](crate::Rational), since zero is $0/1$ and its denominator
    /// is 1.
    ///
    /// $$
    /// f(a + bi) = H(a + bi) = \max(H(a), H(b)).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("1/3+i/2").unwrap().to_height(),
    ///     3
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("22/7").unwrap().to_height(),
    ///     22
    /// );
    /// assert_eq!(GaussianRational::from_str("0").unwrap().to_height(), 1);
    /// ```
    #[inline]
    fn to_height(&self) -> Natural {
        self.height_ref().clone()
    }

    /// Returns the height of a [`GaussianRational`]: the larger of the heights of its real and
    /// imaginary parts, taking the [`GaussianRational`] by value.
    ///
    /// The largest of the four magnitudes is moved out rather than cloned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("1/3+i/2")
    ///         .unwrap()
    ///         .into_height(),
    ///     3
    /// );
    /// assert_eq!(GaussianRational::from_str("0").unwrap().into_height(), 1);
    /// ```
    #[inline]
    fn into_height(self) -> Natural {
        max(self.real.into_height(), self.imaginary.into_height())
    }

    /// Returns the number of significant bits of the height of a [`GaussianRational`].
    ///
    /// Since bit length is monotone, this is the larger of the two parts' height bit lengths,
    /// without materializing the height.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("1/3+i/2")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     2
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("0")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     1
    /// );
    /// ```
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        max(
            self.real.height_significant_bits(),
            self.imaginary.height_significant_bits(),
        )
    }
}

impl HeightRef for GaussianRational {
    /// Returns a reference to the height of a [`GaussianRational`]: the larger of the heights of
    /// its real and imaginary parts.
    ///
    /// Each part holds its numerator's magnitude and its denominator as
    /// [`Natural`](malachite_nz::natural::Natural)s, so the height is one of those four and is
    /// already there to be lent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HeightRef;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     *GaussianRational::from_str("1/3+i/2")
    ///         .unwrap()
    ///         .height_ref(),
    ///     3
    /// );
    /// assert_eq!(*GaussianRational::from_str("0").unwrap().height_ref(), 1);
    /// ```
    #[inline]
    fn height_ref(&self) -> &Natural {
        max(self.real.height_ref(), self.imaginary.height_ref())
    }
}
