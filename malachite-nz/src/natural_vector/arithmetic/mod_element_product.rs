// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModElementProduct, ModProduct};

impl ModElementProduct<Natural> for NaturalVector {
    type Output = Natural;

    /// Returns the product of the elements of a [`NaturalVector`] modulo another number $m$, taking
    /// the vector by reference and the modulus by value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $m = 1$.
    ///
    /// $$
    /// f(v, m) = \prod_i v_i \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.to_mod_element_product(Natural::from(10u32)), 5);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_element_product(Natural::from(10u32)), 1);
    /// ```
    #[inline]
    fn to_mod_element_product(&self, m: Natural) -> Natural {
        Natural::mod_product(self.elements.iter(), m)
    }

    /// Returns the product of the elements of a [`NaturalVector`] modulo another number $m$, taking
    /// the vector by value and the modulus by value. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $m = 1$.
    ///
    /// $$
    /// f(v, m) = \prod_i v_i \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.into_mod_element_product(Natural::from(10u32)), 5);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_element_product(Natural::from(10u32)), 1);
    /// ```
    #[inline]
    fn into_mod_element_product(self, m: Natural) -> Natural {
        Natural::mod_product(self.elements.into_iter(), m)
    }
}

impl ModElementProduct<&Natural> for NaturalVector {
    type Output = Natural;

    /// Returns the product of the elements of a [`NaturalVector`] modulo another number $m$, taking
    /// the vector by reference and the modulus by reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $m = 1$.
    ///
    /// $$
    /// f(v, m) = \prod_i v_i \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.to_mod_element_product(&Natural::from(10u32)), 5);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.to_mod_element_product(&Natural::from(10u32)), 1);
    /// ```
    #[inline]
    fn to_mod_element_product(&self, m: &Natural) -> Natural {
        Natural::mod_product(self.elements.iter(), m)
    }

    /// Returns the product of the elements of a [`NaturalVector`] modulo another number $m$, taking
    /// the vector by value and the modulus by reference. The elements must already be reduced.
    ///
    /// The 0-dimensional vector has no elements, and their product is 1, reduced: 0 when $m = 1$.
    ///
    /// $$
    /// f(v, m) = \prod_i v_i \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 5, 7)").unwrap();
    /// assert_eq!(v.into_mod_element_product(&Natural::from(10u32)), 5);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.into_mod_element_product(&Natural::from(10u32)), 1);
    /// ```
    #[inline]
    fn into_mod_element_product(self, m: &Natural) -> Natural {
        Natural::mod_product(self.elements.into_iter(), m)
    }
}
