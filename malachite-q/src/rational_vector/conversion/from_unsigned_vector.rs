// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> From<UnsignedVector<T>> for RationalVector
where
    Rational: From<T>,
{
    /// Converts an [`UnsignedVector`] to a [`RationalVector`].
    ///
    /// Every unsigned primitive is a [`Rational`], so nothing is lost and nothing can fail. The
    /// elements are converted one by one, each with denominator 1, so the dimension is unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(RationalVector::from(v).to_string(), "(1, 2, 3)");
    ///
    /// assert_eq!(
    ///     RationalVector::from(UnsignedVector::<u8>::from_str("()").unwrap()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    fn from(v: UnsignedVector<T>) -> Self {
        Self {
            elements: v.elements.into_iter().map(Rational::from).collect(),
        }
    }
}
