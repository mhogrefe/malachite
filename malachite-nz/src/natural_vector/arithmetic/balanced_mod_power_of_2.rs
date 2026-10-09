// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;

impl BalancedModPowerOf2 for NaturalVector {
    type Output = IntegerVector;

    /// Reduces every element of a [`NaturalVector`] modulo $2^k$ to the representative closest to
    /// zero, taking the vector by value.
    ///
    /// Each element $r_i$ of the result satisfies $-2^{k-1} < r_i \leq 2^{k-1}$ and is congruent to
    /// the original element modulo $2^k$, which determine it uniquely, as with
    /// [`BalancedModPowerOf2`] for [`Natural`](crate::natural::Natural)s; for $k = 0$ every element
    /// is 0. A remainder of exactly $2^{k-1}$ is positive. Remainders above $2^{k-1}$ become
    /// negative, which is why the result is an
    /// [`IntegerVector`](crate::integer_vector::IntegerVector).
    ///
    /// The dimension is unchanged: unlike a polynomial, a vector keeps the elements that reduce to
    /// zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Each element goes to its balanced remainder modulo 8, and half the modulus
    /// // stays positive.
    /// let v = NaturalVector::from_str("(19, 23, 20)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod_power_of_2(3).to_string(),
    ///     "(3, -1, 4)"
    /// );
    /// let v = NaturalVector::from_str("(8, 7)").unwrap();
    /// assert_eq!(v.clone().balanced_mod_power_of_2(3).to_string(), "(0, -1)");
    /// ```
    fn balanced_mod_power_of_2(self, pow: u64) -> IntegerVector {
        IntegerVector {
            elements: self
                .elements
                .into_iter()
                .map(|x| x.balanced_mod_power_of_2(pow))
                .collect(),
        }
    }
}

impl BalancedModPowerOf2 for &NaturalVector {
    type Output = IntegerVector;

    /// Reduces every element of a [`NaturalVector`] modulo $2^k$ to the representative closest to
    /// zero, taking the vector by reference.
    ///
    /// See the documentation for the [`BalancedModPowerOf2`] implementation on [`NaturalVector`]
    /// that takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Each element goes to its balanced remainder modulo 8, and half the modulus
    /// // stays positive.
    /// let v = NaturalVector::from_str("(19, 23, 20)").unwrap();
    /// assert_eq!((&v).balanced_mod_power_of_2(3).to_string(), "(3, -1, 4)");
    /// let v = NaturalVector::from_str("(8, 7)").unwrap();
    /// assert_eq!((&v).balanced_mod_power_of_2(3).to_string(), "(0, -1)");
    /// ```
    fn balanced_mod_power_of_2(self, pow: u64) -> IntegerVector {
        IntegerVector {
            elements: self
                .elements
                .iter()
                .map(|x| x.balanced_mod_power_of_2(pow))
                .collect(),
        }
    }
}
