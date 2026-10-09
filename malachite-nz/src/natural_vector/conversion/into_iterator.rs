// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec;
use core::slice;

impl IntoIterator for NaturalVector {
    type Item = Natural;
    type IntoIter = vec::IntoIter<Natural>;

    /// Converts a [`NaturalVector`] into an iterator over its elements, taking the vector by value.
    ///
    /// The elements are produced in order, starting with the element at index 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    ///     ["1", "2", "3"]
    /// );
    /// ```
    #[inline]
    fn into_iter(self) -> vec::IntoIter<Natural> {
        self.elements.into_iter()
    }
}

impl<'a> IntoIterator for &'a NaturalVector {
    type Item = &'a Natural;
    type IntoIter = slice::Iter<'a, Natural>;

    /// Converts a reference to a [`NaturalVector`] into an iterator over references to its
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let mut strings = Vec::new();
    /// for x in &v {
    ///     strings.push(x.to_string());
    /// }
    /// assert_eq!(strings, ["1", "2", "3"]);
    /// ```
    #[inline]
    fn into_iter(self) -> slice::Iter<'a, Natural> {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a mut NaturalVector {
    type Item = &'a mut Natural;
    type IntoIter = slice::IterMut<'a, Natural>;

    /// Converts a mutable reference to a [`NaturalVector`] into an iterator over mutable references
    /// to its elements.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// for x in &mut v {
    ///     *x += Natural::ONE;
    /// }
    /// assert_eq!(v.to_string(), "(2, 3, 4)");
    /// ```
    #[inline]
    fn into_iter(self) -> slice::IterMut<'a, Natural> {
        self.elements.iter_mut()
    }
}
