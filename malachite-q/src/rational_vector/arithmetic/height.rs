// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::{RationalVector, ZERO};
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;

impl Height for RationalVector {
    type Output = Natural;

    /// Returns the height of a [`RationalVector`]: the largest of the heights of its elements,
    /// taking the vector by reference and cloning.
    ///
    /// The 0-dimensional vector has no elements, and its height is 0. Every other vector has height
    /// at least 1, since every denominator is at least 1; in particular, a nonzero-dimensional
    /// vector of zeros has height 1, the height of $0 = 0/1$.
    ///
    /// $$
    /// f(v) = H(v) = \max_i H(v_i), \quad H(p/q) = \max(|p|, q).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the number of bits of the height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -3, 1/4)")
    ///         .unwrap()
    ///         .to_height(),
    ///     4
    /// );
    /// assert_eq!(RationalVector::from_str("()").unwrap().to_height(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_max_height` from `fmpq_vec/max_height.c`, FLINT 3.6.0.
    #[inline]
    fn to_height(&self) -> Natural {
        self.height_ref().clone()
    }

    /// Returns the height of a [`RationalVector`]: the largest of the heights of its elements,
    /// taking the vector by value.
    ///
    /// The element of largest height, or the part of it that is the height, is moved out of the
    /// vector rather than cloned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements' numerators and denominators.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -3, 1/4)")
    ///         .unwrap()
    ///         .into_height(),
    ///     4
    /// );
    /// assert_eq!(RationalVector::from_str("()").unwrap().into_height(), 0);
    /// ```
    #[inline]
    fn into_height(self) -> Natural {
        self.elements
            .into_iter()
            .map(Rational::into_height)
            .max()
            .unwrap_or(Natural::ZERO)
    }

    /// Returns the number of significant bits of the height of a [`RationalVector`].
    ///
    /// Since bit length is monotone, this is the largest of the elements' heights' bit lengths, and
    /// 0 for the 0-dimensional vector, without materializing the height.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements' numerators and denominators.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -3, 1/4)")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     3
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_max_height_bits` from `fmpq_vec/max_height_bits.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        self.elements
            .iter()
            .map(Rational::height_significant_bits)
            .max()
            .unwrap_or(0)
    }
}

impl HeightRef for RationalVector {
    /// Returns a reference to the height of a [`RationalVector`]: the largest of the heights of its
    /// elements.
    ///
    /// The height is one of the magnitudes the vector already holds, so it is lent rather than
    /// built. For the 0-dimensional vector, a reference to 0 is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements' numerators and denominators.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HeightRef;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     *RationalVector::from_str("(1/2, -3, 1/4)")
    ///         .unwrap()
    ///         .height_ref(),
    ///     4
    /// );
    /// assert_eq!(*RationalVector::from_str("()").unwrap().height_ref(), 0);
    /// ```
    #[inline]
    fn height_ref(&self) -> &Natural {
        self.elements
            .iter()
            .map(Rational::height_ref)
            .max()
            .unwrap_or(&ZERO)
    }
}
