// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use core::slice;

impl<T: PrimitiveUnsigned> IntoIterator for UnsignedVector<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    /// Converts an [`UnsignedVector`] into an iterator over its elements, taking the vector by
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
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
    ///     ["1", "2", "3"]
    /// );
    /// ```
    #[inline]
    fn into_iter(self) -> vec::IntoIter<T> {
        self.elements.into_iter()
    }
}

impl<'a, T: PrimitiveUnsigned> IntoIterator for &'a UnsignedVector<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    /// Converts a reference to an [`UnsignedVector`] into an iterator over references to its
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
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let mut strings = Vec::new();
    /// for x in &v {
    ///     strings.push(x.to_string());
    /// }
    /// assert_eq!(strings, ["1", "2", "3"]);
    /// ```
    #[inline]
    fn into_iter(self) -> slice::Iter<'a, T> {
        self.elements.iter()
    }
}

impl<'a, T: PrimitiveUnsigned> IntoIterator for &'a mut UnsignedVector<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    /// Converts a mutable reference to an [`UnsignedVector`] into an iterator over mutable
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
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// for x in &mut v {
    ///     *x += 1;
    /// }
    /// assert_eq!(v.to_string(), "(2, 3, 4)");
    /// ```
    #[inline]
    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.elements.iter_mut()
    }
}
