// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};

impl ModPowerOf2 for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by $2^k$, keeping the remainders, taking the
    /// vector by value.
    ///
    /// The result is reduced modulo $2^k$, which is to say that [`mod_power_of_2_is_reduced`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced::mod_power_of_2_is_reduced)
    /// returns `true` for it. The dimension is unchanged: unlike a polynomial, a vector keeps the
    /// elements that reduce to zero, so $(4, 3)$ modulo 4 is $(0, 3)$.
    ///
    /// A [`Natural`](crate::natural::Natural) element can be larger than any $2^k$, so a large $k$
    /// is as meaningful as a small one.
    ///
    /// $$
    /// f(v, k) = w, \\quad \text{where} \\quad w_i = v_i - 2^k \left \lfloor \frac{v_i}{2^k}
    /// \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Every element is taken modulo 2.
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .mod_power_of_2(1)
    ///         .to_string(),
    ///     "(1, 1, 0)"
    /// );
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// assert_eq!(
    ///     NaturalVector::from_str("(4, 3)")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "(0, 3)"
    /// );
    ///
    /// // A power too wide for a `u64` still does something here.
    /// assert_eq!(
    ///     NaturalVector::from_str("(340282366920938463463374607431768211457)")
    ///         .unwrap()
    ///         .mod_power_of_2(64)
    ///         .to_string(),
    ///     "(1)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Self {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl ModPowerOf2 for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by $2^k$, keeping the remainders, taking the
    /// vector by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`NaturalVector`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(4, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2(2).to_string(), "(0, 3)");
    /// // The vector is left alone.
    /// assert_eq!(v.to_string(), "(4, 3)");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> NaturalVector {
        NaturalVector {
            elements: self
                .elements
                .iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect(),
        }
    }
}

impl ModPowerOf2Assign for NaturalVector {
    /// Divides every element of a [`NaturalVector`] by $2^k$, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`NaturalVector`] for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 3, 2)").unwrap();
    /// v.mod_power_of_2_assign(1);
    /// assert_eq!(v.to_string(), "(1, 1, 0)");
    ///
    /// let mut v = NaturalVector::from_str("(4, 3)").unwrap();
    /// v.mod_power_of_2_assign(2);
    /// assert_eq!(v.to_string(), "(0, 3)");
    /// ```
    #[inline]
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        for x in &mut self.elements {
            x.mod_power_of_2_assign(pow);
        }
    }
}
