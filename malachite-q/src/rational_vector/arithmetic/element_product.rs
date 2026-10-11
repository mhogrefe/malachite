// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::ElementProduct;

impl ElementProduct for RationalVector {
    type Output = Rational;

    /// Returns the product of the elements of a [`RationalVector`], taking the vector by reference.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1. The elements are
    /// multiplied in a balanced tree, so that the operands of each multiplication have similar
    /// sizes, and multiplication stops as soon as a zero element is found.
    ///
    /// $$
    /// f(v) = \prod_i v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementProduct;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -2/3)")
    ///         .unwrap()
    ///         .to_element_product()
    ///         .to_string(),
    ///     "-1/3"
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("()").unwrap().to_element_product(),
    ///     1
    /// );
    /// ```
    #[inline]
    fn to_element_product(&self) -> Rational {
        self.elements.iter().product()
    }

    /// Returns the product of the elements of a [`RationalVector`], taking the vector by value.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1. The elements are
    /// multiplied in a balanced tree, so that the operands of each multiplication have similar
    /// sizes, and multiplication stops as soon as a zero element is found.
    ///
    /// $$
    /// f(v) = \prod_i v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementProduct;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -2/3)")
    ///         .unwrap()
    ///         .into_element_product()
    ///         .to_string(),
    ///     "-1/3"
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .into_element_product(),
    ///     1
    /// );
    /// ```
    #[inline]
    fn into_element_product(self) -> Rational {
        self.elements.into_iter().product()
    }
}
