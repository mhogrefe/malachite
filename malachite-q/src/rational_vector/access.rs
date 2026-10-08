// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use core::ops::{Index, IndexMut};

impl Index<usize> for RationalVector {
    type Output = Rational;

    /// Gets a reference to the element of a [`RationalVector`] at an index.
    ///
    /// Indices start at 0, as they do for a [`Vec`](alloc::vec::Vec).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the dimension of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[2], 3);
    /// ```
    #[inline]
    fn index(&self, i: usize) -> &Rational {
        &self.elements[i]
    }
}

impl IndexMut<usize> for RationalVector {
    /// Gets a mutable reference to the element of a [`RationalVector`] at an index.
    ///
    /// Indices start at 0, as they do for a [`Vec`](alloc::vec::Vec).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `i` is greater than or equal to the dimension of the vector.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// v[1] = Rational::from(10u32);
    /// v[2] += Rational::from(4u32);
    /// assert_eq!(v.to_string(), "(1, 10, 7)");
    /// ```
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Rational {
        &mut self.elements[i]
    }
}
