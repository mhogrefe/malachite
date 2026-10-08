// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

impl<T: PrimitiveUnsigned> UnsignedVector<T> {
    /// Returns an [`UnsignedVector`]'s elements as a [`Vec`], cloning them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements(), [1, 2, 3]);
    /// assert!(
    ///     UnsignedVector::<u32>::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .is_empty()
    /// );
    /// ```
    #[inline]
    pub fn to_elements(&self) -> Vec<T> {
        self.elements.clone()
    }

    /// Returns an [`UnsignedVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`UnsignedVector`].
    ///
    /// Nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements(), [1, 2, 3]);
    /// assert!(
    ///     UnsignedVector::<u32>::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .is_empty()
    /// );
    /// ```
    #[inline]
    pub fn into_elements(self) -> Vec<T> {
        self.elements
    }

    /// Returns a reference to an [`UnsignedVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref(), [1, 2, 3]);
    /// // A slice of the elements can be taken directly.
    /// assert_eq!(&v.elements_ref()[1..], [2, 3]);
    /// assert!(
    ///     UnsignedVector::<u32>::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .is_empty()
    /// );
    /// ```
    #[inline]
    pub fn elements_ref(&self) -> &[T] {
        &self.elements
    }
}
