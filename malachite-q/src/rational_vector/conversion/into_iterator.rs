// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use alloc::vec;
use core::slice;

impl IntoIterator for RationalVector {
    type Item = Rational;
    type IntoIter = vec::IntoIter<Rational>;

    /// Converts a [`RationalVector`] into an iterator over its elements, taking the vector by
    /// value.
    ///
    /// The elements are produced in order, starting with the element at index 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// assert_eq!(
    ///     v.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    ///     ["1/2", "-2", "3"]
    /// );
    /// ```
    #[inline]
    fn into_iter(self) -> vec::IntoIter<Rational> {
        self.elements.into_iter()
    }
}

impl<'a> IntoIterator for &'a RationalVector {
    type Item = &'a Rational;
    type IntoIter = slice::Iter<'a, Rational>;

    /// Converts a reference to a [`RationalVector`] into an iterator over references to its
    /// elements.
    ///
    /// The elements are produced in order, starting with the element at index 0. This is what `for
    /// x in &v` uses.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// let mut strings = Vec::new();
    /// for x in &v {
    ///     strings.push(x.to_string());
    /// }
    /// assert_eq!(strings, ["1/2", "-2", "3"]);
    /// ```
    #[inline]
    fn into_iter(self) -> slice::Iter<'a, Rational> {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a mut RationalVector {
    type Item = &'a mut Rational;
    type IntoIter = slice::IterMut<'a, Rational>;

    /// Converts a mutable reference to a [`RationalVector`] into an iterator over mutable
    /// references to its elements.
    ///
    /// The elements are produced in order, starting with the element at index 0. This is what `for
    /// x in &mut v` uses; since every list of elements is a valid vector, any element may be
    /// changed.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    /// for x in &mut v {
    ///     *x += Rational::ONE;
    /// }
    /// assert_eq!(v.to_string(), "(3/2, -1, 4)");
    /// ```
    #[inline]
    fn into_iter(self) -> slice::IterMut<'a, Rational> {
        self.elements.iter_mut()
    }
}
