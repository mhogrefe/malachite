// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModPowerOf2ElementProduct, ModPowerOf2Product};

impl ModPowerOf2ElementProduct for NaturalVector {
    type Output = Natural;

    /// Returns the product of the elements of a [`NaturalVector`] modulo $2^k$, taking the vector
    /// by reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $2^k = 1$.
    ///
    /// $$
    /// f(v, k) = \prod_i v_i \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element is greater than or equal to $2^k$. As soon as the partial product is
    /// zero, the remaining elements are not examined, so an unreduced element after that point is
    /// not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ElementProduct;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.to_mod_power_of_2_element_product(4), 9);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_power_of_2_element_product(3), 1);
    /// ```
    #[inline]
    fn to_mod_power_of_2_element_product(&self, pow: u64) -> Natural {
        Natural::mod_power_of_2_product(self.elements.iter(), pow)
    }

    /// Returns the product of the elements of a [`NaturalVector`] modulo $2^k$, taking the vector
    /// by value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $2^k = 1$.
    ///
    /// $$
    /// f(v, k) = \prod_i v_i \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element is greater than or equal to $2^k$. As soon as the partial product is
    /// zero, the remaining elements are not examined, so an unreduced element after that point is
    /// not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2ElementProduct;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.into_mod_power_of_2_element_product(4), 9);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_power_of_2_element_product(3), 1);
    /// ```
    #[inline]
    fn into_mod_power_of_2_element_product(self, pow: u64) -> Natural {
        Natural::mod_power_of_2_product(self.elements.into_iter(), pow)
    }
}
