// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> ModPowerOf2IsReduced for UnsignedMatrix<T> {
    /// Returns whether an [`UnsignedMatrix`] is reduced modulo $2^k$; in other words, whether every
    /// one of its elements has no more than $k$ significant bits.
    ///
    /// A matrix with no elements, of any shape, is reduced modulo every power of 2, including
    /// $2^0$. A matrix of zeros is too, and every matrix is reduced modulo $2^k$ for $k \geq W$,
    /// where $W$ is `T::WIDTH`.
    ///
    /// $f(A, k) = (\max_{i,j} A_{ij} < 2^k)$, with the maximum of no elements taken to be 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// // The largest element is 3, which needs two bits.
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 3; 2, 0]").unwrap();
    /// assert_eq!(m.mod_power_of_2_is_reduced(2), true);
    /// assert_eq!(m.mod_power_of_2_is_reduced(1), false);
    ///
    /// // A matrix with no elements and a matrix of zeros are reduced modulo every power of 2.
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{3,0}")
    ///         .unwrap()
    ///         .mod_power_of_2_is_reduced(0),
    ///     true
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]")
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
