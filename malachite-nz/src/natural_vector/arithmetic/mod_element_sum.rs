// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModElementSum, ModSum};

impl ModElementSum<Natural> for NaturalVector {
    type Output = Natural;

    /// Returns the sum of the elements of a [`NaturalVector`] modulo another number $m$, taking the
    /// vector by reference and the modulus by value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v, m) = \sum_i v_i \bmod m.
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
    /// Panics if `m` is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModElementSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 6, 7)").unwrap();
    /// assert_eq!(v.to_mod_element_sum(Natural::from(10u32)), 8);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_element_sum(Natural::from(10u32)), 0);
    /// ```
    #[inline]
    fn to_mod_element_sum(&self, m: Natural) -> Natural {
        Natural::mod_sum(self.elements.iter(), m)
    }

    /// Returns the sum of the elements of a [`NaturalVector`] modulo another number $m$, taking the
    /// vector by value and the modulus by value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v, m) = \sum_i v_i \bmod m.
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
    /// Panics if `m` is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModElementSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 6, 7)").unwrap();
    /// assert_eq!(v.into_mod_element_sum(Natural::from(10u32)), 8);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_element_sum(Natural::from(10u32)), 0);
    /// ```
    #[inline]
    fn into_mod_element_sum(self, m: Natural) -> Natural {
        Natural::mod_sum(self.elements.into_iter(), m)
    }
}

impl ModElementSum<&Natural> for NaturalVector {
    type Output = Natural;

    /// Returns the sum of the elements of a [`NaturalVector`] modulo another number $m$, taking the
    /// vector by reference and the modulus by reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v, m) = \sum_i v_i \bmod m.
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
    /// Panics if `m` is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModElementSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 6, 7)").unwrap();
    /// assert_eq!(v.to_mod_element_sum(&Natural::from(10u32)), 8);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_element_sum(&Natural::from(10u32)), 0);
    /// ```
    #[inline]
    fn to_mod_element_sum(&self, m: &Natural) -> Natural {
        Natural::mod_sum(self.elements.iter(), m)
    }

    /// Returns the sum of the elements of a [`NaturalVector`] modulo another number $m$, taking the
    /// vector by value and the modulus by reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v, m) = \sum_i v_i \bmod m.
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
    /// Panics if `m` is zero, or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModElementSum;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 6, 7)").unwrap();
    /// assert_eq!(v.into_mod_element_sum(&Natural::from(10u32)), 8);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_element_sum(&Natural::from(10u32)), 0);
    /// ```
    #[inline]
    fn into_mod_element_sum(self, m: &Natural) -> Natural {
        Natural::mod_sum(self.elements.into_iter(), m)
    }
}
