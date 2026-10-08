// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use malachite_base::num::arithmetic::traits::{
    CanonicalizeSign, CanonicalizeSignAssign, NegAssign,
};
use malachite_base::vector::Vector;

impl CanonicalizeSign for IntegerVector {
    type Output = Self;

    /// Brings an [`IntegerVector`] into canonical sign form, taking the vector by value: negates it
    /// if its pivot, its first nonzero element, is negative.
    ///
    /// A vector and its negation are the same up to multiplication by $\pm 1$, the units of the
    /// integers, and this picks the one whose pivot is positive to represent both. A vector of
    /// zeros has no pivot and is left alone.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeSign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(0, -2, 3)").unwrap();
    /// assert_eq!(v.canonicalize_sign().to_string(), "(0, 2, -3)");
    /// let v = IntegerVector::from_str("(0, 2, -3)").unwrap();
    /// assert_eq!(v.canonicalize_sign().to_string(), "(0, 2, -3)");
    /// ```
    #[inline]
    fn canonicalize_sign(mut self) -> Self {
        self.canonicalize_sign_assign();
        self
    }
}

impl CanonicalizeSign for &IntegerVector {
    type Output = IntegerVector;

    /// Brings an [`IntegerVector`] into canonical sign form, taking the vector by reference:
    /// negates it if its pivot, its first nonzero element, is negative.
    ///
    /// See the documentation for the [`CanonicalizeSign`] implementation on [`IntegerVector`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeSign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(0, -2, 3)").unwrap();
    /// assert_eq!((&v).canonicalize_sign().to_string(), "(0, 2, -3)");
    /// ```
    #[inline]
    fn canonicalize_sign(self) -> IntegerVector {
        if self.pivot().is_some_and(|x| *x < 0u32) {
            -self
        } else {
            self.clone()
        }
    }
}

impl CanonicalizeSignAssign for IntegerVector {
    /// Replaces an [`IntegerVector`] with its canonical sign form: negates it if its pivot, its
    /// first nonzero element, is negative.
    ///
    /// See the documentation for the [`CanonicalizeSign`] implementation on [`IntegerVector`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeSignAssign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(0, -2, 3)").unwrap();
    /// v.canonicalize_sign_assign();
    /// assert_eq!(v.to_string(), "(0, 2, -3)");
    /// ```
    #[inline]
    fn canonicalize_sign_assign(&mut self) {
        if self.pivot().is_some_and(|x| *x < 0u32) {
            self.neg_assign();
        }
    }
}
