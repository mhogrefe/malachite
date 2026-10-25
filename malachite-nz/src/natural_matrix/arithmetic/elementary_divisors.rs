// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::ElementaryDivisors;

impl ElementaryDivisors for NaturalMatrix {
    type Output = Vec<Natural>;

    /// Computes the elementary divisors of a [`NaturalMatrix`], taking it by value.
    ///
    /// The elementary divisors are described in
    /// [`elementary_divisors`](malachite_base::num::arithmetic::traits::ElementaryDivisors) for an
    /// [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `elementary_divisors` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementaryDivisors;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!(a.elementary_divisors().to_debug_string(), "[1, 6]");
    ///
    /// // A matrix of rank 2
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.elementary_divisors().to_debug_string(), "[1, 3]");
    /// ```
    #[inline]
    fn elementary_divisors(self) -> Vec<Natural> {
        IntegerMatrix::from(self).elementary_divisors()
    }
}

impl ElementaryDivisors for &NaturalMatrix {
    type Output = Vec<Natural>;

    /// Computes the elementary divisors of a [`NaturalMatrix`], taking it by reference.
    ///
    /// The elementary divisors are described in
    /// [`elementary_divisors`](malachite_base::num::arithmetic::traits::ElementaryDivisors) for an
    /// [`IntegerMatrix`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `elementary_divisors` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ElementaryDivisors;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!((&a).elementary_divisors().to_debug_string(), "[1, 6]");
    ///
    /// // A matrix of rank 2
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).elementary_divisors().to_debug_string(), "[1, 3]");
    /// ```
    #[inline]
    fn elementary_divisors(self) -> Vec<Natural> {
        IntegerMatrix::from(self).elementary_divisors()
    }
}
