// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use alloc::vec;
use core::slice;

impl IntoIterator for IntegerVector {
    type Item = Integer;
    type IntoIter = vec::IntoIter<Integer>;

    /// Converts an [`IntegerVector`] into an iterator over its elements, taking the vector by
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// assert_eq!(
    ///     v.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    ///     ["1", "-2", "3"]
    /// );
    /// ```
    #[inline]
    fn into_iter(self) -> vec::IntoIter<Integer> {
        self.elements.into_iter()
    }
}

impl<'a> IntoIterator for &'a IntegerVector {
    type Item = &'a Integer;
    type IntoIter = slice::Iter<'a, Integer>;

    /// Converts a reference to an [`IntegerVector`] into an iterator over references to its
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// let mut strings = Vec::new();
    /// for x in &v {
    ///     strings.push(x.to_string());
    /// }
    /// assert_eq!(strings, ["1", "-2", "3"]);
    /// ```
    #[inline]
    fn into_iter(self) -> slice::Iter<'a, Integer> {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a mut IntegerVector {
    type Item = &'a mut Integer;
    type IntoIter = slice::IterMut<'a, Integer>;

    /// Converts a mutable reference to an [`IntegerVector`] into an iterator over mutable
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// for x in &mut v {
    ///     *x += Integer::ONE;
    /// }
    /// assert_eq!(v.to_string(), "(2, -1, 4)");
    /// ```
    #[inline]
    fn into_iter(self) -> slice::IterMut<'a, Integer> {
        self.elements.iter_mut()
    }
}
