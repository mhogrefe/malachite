// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::{ComparableGaussianInteger, ComparableGaussianIntegerRef};
use core::cmp::Ordering;

impl Ord for ComparableGaussianIntegerRef<'_> {
    /// Compares two [`ComparableGaussianIntegerRef`]s.
    ///
    /// The order is lexicographic: real parts are compared first, and imaginary parts break ties.
    /// This is a total order, and its equality agrees with
    /// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) equality, but it is not
    /// compatible with arithmetic: no total order on the complex numbers is. It is intended for
    /// canonically sorting Gaussian integers and for using them as keys in ordered collections.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeI, One, Zero};
    /// use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
    ///
    /// let zero = GaussianInteger::ZERO;
    /// let one = GaussianInteger::ONE;
    /// let i = GaussianInteger::I;
    /// let negative_i = GaussianInteger::NEGATIVE_I;
    ///
    /// // 0 < i, since the real parts are equal and 0 < 1
    /// assert!(ComparableGaussianIntegerRef(&zero) < ComparableGaussianIntegerRef(&i));
    /// // -i < i
    /// assert!(ComparableGaussianIntegerRef(&negative_i) < ComparableGaussianIntegerRef(&i));
    /// // i < 1, since 0 < 1 and the real parts are compared first
    /// assert!(ComparableGaussianIntegerRef(&i) < ComparableGaussianIntegerRef(&one));
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .real
            .cmp(&other.0.real)
            .then_with(|| self.0.imaginary.cmp(&other.0.imaginary))
    }
}

impl PartialOrd for ComparableGaussianIntegerRef<'_> {
    /// Compares two [`ComparableGaussianIntegerRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &ComparableGaussianIntegerRef) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableGaussianInteger {
    /// Compares two [`ComparableGaussianInteger`]s.
    ///
    /// The order is lexicographic: real parts are compared first, and imaginary parts break ties.
    /// This is a total order, and its equality agrees with
    /// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) equality, but it is not
    /// compatible with arithmetic: no total order on the complex numbers is. It is intended for
    /// canonically sorting Gaussian integers and for using them as keys in ordered collections.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeI, One, Zero};
    /// use malachite_nz::gaussian_integer::{ComparableGaussianInteger, GaussianInteger};
    ///
    /// // 0 < i, since the real parts are equal and 0 < 1
    /// assert!(
    ///     ComparableGaussianInteger(GaussianInteger::ZERO)
    ///         < ComparableGaussianInteger(GaussianInteger::I)
    /// );
    /// // -i < i
    /// assert!(
    ///     ComparableGaussianInteger(GaussianInteger::NEGATIVE_I)
    ///         < ComparableGaussianInteger(GaussianInteger::I)
    /// );
    /// // i < 1, since 0 < 1 and the real parts are compared first
    /// assert!(
    ///     ComparableGaussianInteger(GaussianInteger::I)
    ///         < ComparableGaussianInteger(GaussianInteger::ONE)
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for ComparableGaussianInteger {
    /// Compares two [`ComparableGaussianInteger`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_ref().cmp(&other.as_ref()))
    }
}
