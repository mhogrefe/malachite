// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModPowerOf2ElementSum, ModPowerOf2Sum};

impl ModPowerOf2ElementSum for NaturalVector {
    type Output = Natural;

    /// Returns the sum of the elements of a [`NaturalVector`] modulo $2^k$, taking the vector by
    /// reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v, k) = \sum_i v_i \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements, and $m$ is the number of bits of the modulus.
    ///
    /// # Panics
    /// Panics if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ElementSum;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 6, 7)").unwrap();
    /// assert_eq!(v.to_mod_power_of_2_element_sum(3), 2);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_power_of_2_element_sum(3), 0);
    /// ```
    #[inline]
    fn to_mod_power_of_2_element_sum(&self, pow: u64) -> Natural {
        Natural::mod_power_of_2_sum(self.elements.iter(), pow)
    }

    /// Returns the sum of the elements of a [`NaturalVector`] modulo $2^k$, taking the vector by
    /// value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v, k) = \sum_i v_i \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements, and $m$ is the number of bits of the modulus.
    ///
    /// # Panics
    /// Panics if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ElementSum;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 6, 7)").unwrap();
    /// assert_eq!(v.into_mod_power_of_2_element_sum(3), 2);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_power_of_2_element_sum(3), 0);
    /// ```
    #[inline]
    fn into_mod_power_of_2_element_sum(self, pow: u64) -> Natural {
        Natural::mod_power_of_2_sum(self.elements.into_iter(), pow)
    }
}
