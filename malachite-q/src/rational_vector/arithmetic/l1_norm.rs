// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{Abs, L1Norm};
use malachite_base::num::logic::traits::SignificantBits;

impl L1Norm for RationalVector {
    type Output = Rational;

    /// Returns the $\ell^1$ norm of a [`RationalVector`]: the sum of the absolute values of its elements, taking the vector by
    /// reference.
    ///
    /// The 0-dimensional vector has no elements, and its norm is 0.
    ///
    /// $$
    /// f(v) = \|v\|_1 = \sum_i |v_i|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::L1Norm;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(RationalVector::from_str("(1/2, -1/3)").unwrap().to_l1_norm().to_string(), "5/6");
    /// assert_eq!(RationalVector::from_str("()").unwrap().to_l1_norm(), 0);
    /// ```
    #[inline]
    fn to_l1_norm(&self) -> Rational {
        self.elements.iter().map(Abs::abs).sum()
    }

    /// Returns the $\ell^1$ norm of a [`RationalVector`]: the sum of the absolute values of its elements, taking the vector by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::L1Norm;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(RationalVector::from_str("(1/2, -1/3)").unwrap().into_l1_norm().to_string(), "5/6");
    /// assert_eq!(RationalVector::from_str("()").unwrap().into_l1_norm(), 0);
    /// ```
    #[inline]
    fn into_l1_norm(self) -> Rational {
        self.elements.into_iter().map(Abs::abs).sum()
    }

    /// Returns the number of significant bits of the $\ell^1$ norm of a [`RationalVector`].
    ///
    /// As for a single [`Rational`], the count is the sum of the numbers of significant bits of the
    /// norm's numerator and denominator.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::L1Norm;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -1/3)")
    ///         .unwrap()
    ///         .l1_norm_significant_bits(),
    ///     6
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .l1_norm_significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn l1_norm_significant_bits(&self) -> u64 {
        self.to_l1_norm().significant_bits()
    }
}
