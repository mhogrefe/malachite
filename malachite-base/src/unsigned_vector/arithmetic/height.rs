// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Height;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> Height for UnsignedVector<T> {
    type Output = T;

    /// Returns the height of an [`UnsignedVector`]: the largest of its elements, taking the vector
    /// by reference.
    ///
    /// The 0-dimensional vector has no elements, and its height is 0.
    ///
    /// $$
    /// f(v) = H(v) = \max_i v_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .to_height(),
    ///     3
    /// );
    /// assert_eq!(UnsignedVector::<u8>::from_str("()").unwrap().to_height(), 0);
    /// ```
    #[inline]
    fn to_height(&self) -> T {
        self.elements.iter().copied().max().unwrap_or(T::ZERO)
    }

    /// Returns the height of an [`UnsignedVector`]: the largest of its elements, taking the vector
    /// by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .into_height(),
    ///     3
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("()").unwrap().into_height(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn into_height(self) -> T {
        self.to_height()
    }

    /// Returns the number of significant bits of the height of an [`UnsignedVector`].
    ///
    /// Since bit length is monotone, this is the largest of the elements' heights' bit lengths, and
    /// 0 for the 0-dimensional vector, without materializing the height.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     2
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("()")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_max_bits` from `nmod_vec/max_bits.c`, FLINT 3.6.0.
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        self.to_height().significant_bits()
    }
}
