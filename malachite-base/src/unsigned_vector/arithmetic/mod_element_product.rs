// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModElementProduct;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> ModElementProduct<T> for UnsignedVector<T> {
    type Output = T;

    /// Returns the product of the elements of an [`UnsignedVector`] modulo another number $m$,
    /// taking the vector by reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $m = 1$.
    ///
    /// $$
    /// f(v, m) = \prod_i v_i \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element is greater than or equal to `m`. As soon as the
    /// partial product is zero, the remaining elements are not examined, so an unreduced element
    /// after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModElementProduct;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.to_mod_element_product(10), 5);
    /// let v = UnsignedVector::<u8>::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_element_product(10), 1);
    /// ```
    #[inline]
    fn to_mod_element_product(&self, m: T) -> T {
        T::mod_product(self.elements.iter(), m)
    }

    /// Returns the product of the elements of an [`UnsignedVector`] modulo another number $m$,
    /// taking the vector by value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $m = 1$.
    ///
    /// $$
    /// f(v, m) = \prod_i v_i \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element is greater than or equal to `m`. As soon as the
    /// partial product is zero, the remaining elements are not examined, so an unreduced element
    /// after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModElementProduct;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.into_mod_element_product(10), 5);
    /// let v = UnsignedVector::<u8>::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_element_product(10), 1);
    /// ```
    #[inline]
    fn into_mod_element_product(self, m: T) -> T {
        T::mod_product(self.elements.into_iter(), m)
    }
}
