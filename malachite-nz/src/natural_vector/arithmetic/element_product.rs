// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::ElementProduct;

impl ElementProduct for NaturalVector {
    type Output = Natural;

    /// Returns the product of the elements of a [`NaturalVector`], taking the vector by reference.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(2, 3, 7)")
    ///         .unwrap()
    ///         .to_element_product(),
    ///     42
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("()").unwrap().to_element_product(),
    ///     1
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_prod` from `fmpz_vec/prod.c`, FLINT 3.6.0.
    #[inline]
    fn to_element_product(&self) -> Natural {
        self.elements.iter().product()
    }

    /// Returns the product of the elements of a [`NaturalVector`], taking the vector by value.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(2, 3, 7)")
    ///         .unwrap()
    ///         .into_element_product(),
    ///     42
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .into_element_product(),
    ///     1
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_prod` from `fmpz_vec/prod.c`, FLINT 3.6.0.
    #[inline]
    fn into_element_product(self) -> Natural {
        self.elements.into_iter().product()
    }
}
