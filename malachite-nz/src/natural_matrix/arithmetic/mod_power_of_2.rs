// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};

impl ModPowerOf2 for NaturalMatrix {
    type Output = Self;

    /// Divides every element of a [`NaturalMatrix`] by $2^k$, keeping the remainders, taking the
    /// matrix by value.
    ///
    /// The result is reduced modulo $2^k$, which is to say that [`mod_power_of_2_is_reduced`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced::mod_power_of_2_is_reduced)
    /// returns `true` for it. The height and width are unchanged, and elements that reduce to zero
    /// stay, so $[4, 3]$ modulo 4 is $[0, 3]$.
    ///
    /// A [`Natural`](crate::natural::Natural) element can be larger than any $2^k$, so a large $k$
    /// is as meaningful as a small one.
    ///
    /// $$
    /// f(A, k) = B, \quad \text{where} \quad B_{ij} = A_{ij} - 2^k \left \lfloor
    /// \frac{A_{ij}}{2^k} \right \rfloor.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // Every element is taken modulo 2.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[1, 3; 2, 5]")
    ///         .unwrap()
    ///         .mod_power_of_2(1)
    ///         .to_string(),
    ///     "[1, 1; 0, 1]"
    /// );
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[4, 3]")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "[0, 3]"
    /// );
    /// // A power too wide for a `u64` still does something here.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[340282366920938463463374607431768211457]")
    ///         .unwrap()
    ///         .mod_power_of_2(64)
    ///         .to_string(),
    ///     "[1]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Self {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl ModPowerOf2 for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Divides every element of a [`NaturalMatrix`] by $2^k$, keeping the remainders, taking the
    /// matrix by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`NaturalMatrix`] for
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[4, 3; 5, 6]").unwrap();
    /// assert_eq!((&m).mod_power_of_2(2).to_string(), "[0, 3; 1, 2]");
    /// // The matrix is left alone.
    /// assert_eq!(m.to_string(), "[4, 3; 5, 6]");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> NaturalMatrix {
        NaturalMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements
                .iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect(),
        )
    }
}

impl ModPowerOf2Assign for NaturalMatrix {
    /// Divides every element of a [`NaturalMatrix`] by $2^k$, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`NaturalMatrix`] for
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut m = NaturalMatrix::from_str("[1, 3; 2, 5]").unwrap();
    /// m.mod_power_of_2_assign(1);
    /// assert_eq!(m.to_string(), "[1, 1; 0, 1]");
    ///
    /// let mut m = NaturalMatrix::from_str("[4, 3]").unwrap();
    /// m.mod_power_of_2_assign(2);
    /// assert_eq!(m.to_string(), "[0, 3]");
    /// ```
    #[inline]
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        for x in &mut self.elements {
            x.mod_power_of_2_assign(pow);
        }
    }
}
