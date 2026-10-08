// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> ModPowerOf2IsReduced for UnsignedVector<T> {
    /// Returns whether an [`UnsignedVector`] is reduced modulo $2^k$; in other words, whether every
    /// one of its elements has no more than $k$ significant bits.
    ///
    /// The 0-dimensional vector has no elements and is reduced modulo every power of 2, including
    /// $2^0$. A vector of zeros is too, and every vector is reduced modulo $2^k$ for $k \geq W$,
    /// where $W$ is `T::WIDTH`.
    ///
    /// $f(v, k) = (\max_i v_i < 2^k)$, with the maximum of no elements taken to be 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// // The largest element is 3, which needs two bits.
    /// let v = UnsignedVector::<u64>::from_str("(1, 3, 2)").unwrap();
    /// assert_eq!(v.mod_power_of_2_is_reduced(2), true);
    /// assert_eq!(v.mod_power_of_2_is_reduced(1), false);
    ///
    /// // The 0-dimensional vector and a vector of zeros are reduced modulo every power of 2.
    /// assert_eq!(
    ///     UnsignedVector::<u64>::from_str("()")
    ///         .unwrap()
    ///         .mod_power_of_2_is_reduced(0),
    ///     true
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u64>::from_str("(0, 0)")
    ///         .unwrap()
    ///         .mod_power_of_2_is_reduced(0),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool {
        self.elements
            .iter()
            .all(|x| x.mod_power_of_2_is_reduced(pow))
    }
}
