// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::{ComparableGaussianRational, ComparableGaussianRationalRef};
use core::cmp::Ordering;

impl Ord for ComparableGaussianRationalRef<'_> {
    /// Compares two [`ComparableGaussianRationalRef`]s.
    ///
    /// The order is lexicographic: real parts are compared first, and imaginary parts break ties.
    /// This is a total order, and its equality agrees with
    /// [`GaussianRational`](crate::gaussian_rational::GaussianRational) equality, but it is not
    /// compatible with arithmetic: no total order on the complex numbers is. It is intended for
    /// canonically sorting Gaussian rationals and for using them as keys in ordered collections.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, One, OneHalf, Zero};
    /// use malachite_q::gaussian_rational::{ComparableGaussianRationalRef, GaussianRational};
    ///
    /// let zero = GaussianRational::ZERO;
    /// let one = GaussianRational::ONE;
    /// let one_half = GaussianRational::ONE_HALF;
    /// let i = GaussianRational::I;
    ///
    /// // 0 < i, since the real parts are equal and 0 < 1
    /// assert!(ComparableGaussianRationalRef(&zero) < ComparableGaussianRationalRef(&i));
    /// // i < 1/2, since 0 < 1/2 and the real parts are compared first
    /// assert!(ComparableGaussianRationalRef(&i) < ComparableGaussianRationalRef(&one_half));
    /// // 1/2 < 1
    /// assert!(ComparableGaussianRationalRef(&one_half) < ComparableGaussianRationalRef(&one));
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .real
            .cmp(&other.0.real)
            .then_with(|| self.0.imaginary.cmp(&other.0.imaginary))
    }
}

impl PartialOrd for ComparableGaussianRationalRef<'_> {
    /// Compares two [`ComparableGaussianRationalRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &ComparableGaussianRationalRef) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableGaussianRational {
    /// Compares two [`ComparableGaussianRational`]s.
    ///
    /// The order is lexicographic: real parts are compared first, and imaginary parts break ties.
    /// This is a total order, and its equality agrees with
    /// [`GaussianRational`](crate::gaussian_rational::GaussianRational) equality, but it is not
    /// compatible with arithmetic: no total order on the complex numbers is. It is intended for
    /// canonically sorting Gaussian rationals and for using them as keys in ordered collections.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, One, OneHalf, Zero};
    /// use malachite_q::gaussian_rational::{ComparableGaussianRational, GaussianRational};
    ///
    /// // 0 < i, since the real parts are equal and 0 < 1
    /// assert!(
    ///     ComparableGaussianRational(GaussianRational::ZERO)
    ///         < ComparableGaussianRational(GaussianRational::I)
    /// );
    /// // i < 1/2, since 0 < 1/2 and the real parts are compared first
    /// assert!(
    ///     ComparableGaussianRational(GaussianRational::I)
    ///         < ComparableGaussianRational(GaussianRational::ONE_HALF)
    /// );
    /// // 1/2 < 1
    /// assert!(
    ///     ComparableGaussianRational(GaussianRational::ONE_HALF)
    ///         < ComparableGaussianRational(GaussianRational::ONE)
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for ComparableGaussianRational {
    /// Compares two [`ComparableGaussianRational`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_ref().cmp(&other.as_ref()))
    }
}
