// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;

impl RationalVector {
    /// Converts a slice of [`Rational`]s to a [`RationalVector`], cloning them.
    ///
    /// The vector's dimension is the length of the slice. Every slice is a valid vector, so this
    /// cannot fail; the empty slice gives the 0-dimensional vector.
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
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_elements(&[Rational::ONE, Rational::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(RationalVector::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    pub fn from_elements(xs: &[Rational]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }

    /// Converts a [`Vec`] of [`Rational`]s to a [`RationalVector`], taking ownership of it.
    ///
    /// The vector's dimension is the length of the [`Vec`]. Every [`Vec`] is a valid vector, so
    /// this cannot fail, and nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_owned_elements(vec![Rational::ONE, Rational::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     RationalVector::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    pub const fn from_owned_elements(xs: Vec<Rational>) -> Self {
        Self { elements: xs }
    }
}
