// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::vector::Vector;
use alloc::vec::Vec;

impl<T: Clone> Vector<T> {
    /// Converts a slice to a [`Vector`], cloning the elements.
    ///
    /// The vector's dimension is the length of the slice. Every slice is a valid vector, so this
    /// cannot fail; the empty slice gives the 0-dimensional vector.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(Vector::from_elements(&[1u32, 2]).to_string(), "(1, 2)");
    /// assert_eq!(Vector::<u32>::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    pub fn from_elements(xs: &[T]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }
}

impl<T> Vector<T> {
    /// Converts a [`Vec`] to a [`Vector`], taking ownership of it.
    ///
    /// The vector's dimension is the length of the [`Vec`]. Every [`Vec`] is a valid vector, so
    /// this cannot fail, and nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     Vector::from_owned_elements(vec![1u32, 2]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     Vector::<u32>::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    pub const fn from_owned_elements(xs: Vec<T>) -> Self {
        Self { elements: xs }
    }
}
