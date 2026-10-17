// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, RemPowerOf2, RemPowerOf2Assign};

impl ModPowerOf2 for IntegerMatrix {
    type Output = NaturalMatrix;

    /// Divides every element of an [`IntegerMatrix`] by $2^k$, keeping the remainders as a
    /// [`NaturalMatrix`], taking the matrix by value.
    ///
    /// Each remainder is non-negative, as with [`ModPowerOf2`] for
    /// [`Integer`](crate::integer::Integer): a negative element $x$ becomes $2^k - (-x \bmod 2^k)$
    /// unless it is a multiple of $2^k$. So the result has natural elements, and is reduced modulo
    /// $2^k$, which is to say that [`mod_power_of_2_is_reduced`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced::mod_power_of_2_is_reduced)
    /// returns `true` for it. The height and width are unchanged, and elements that reduce to zero
    /// stay.
    ///
    /// $$
    /// f(A, k) = B, \quad \text{where} \quad B_{ij} = A_{ij} - 2^k \left \lfloor \frac{A_{ij}}{2^k}
    /// \right \rfloor.
    /// $$
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Every element is taken modulo 4, and negative ones become non-negative.
    /// let a = IntegerMatrix::from_str("[1, -3, -2]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2(2).to_string(), "[1, 1, 2]");
    ///
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// let a = IntegerMatrix::from_str("[-4, 3]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2(2).to_string(), "[0, 3]");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> NaturalMatrix {
        NaturalMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements
                .into_iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect(),
        )
    }
}

impl ModPowerOf2 for &IntegerMatrix {
    type Output = NaturalMatrix;

    /// Divides every element of an [`IntegerMatrix`] by $2^k$, keeping the remainders as a
    /// [`NaturalMatrix`], taking the matrix by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`IntegerMatrix`] that takes
    /// the matrix by value for details.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Every element is taken modulo 4, and negative ones become non-negative.
    /// let a = IntegerMatrix::from_str("[1, -3, -2]").unwrap();
    /// assert_eq!((&a).mod_power_of_2(2).to_string(), "[1, 1, 2]");
    ///
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// let a = IntegerMatrix::from_str("[-4, 3]").unwrap();
    /// assert_eq!((&a).mod_power_of_2(2).to_string(), "[0, 3]");
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

impl RemPowerOf2 for IntegerMatrix {
    type Output = Self;

    /// Divides every element of an [`IntegerMatrix`] by $2^k$, keeping the remainders, taking the
    /// matrix by value.
    ///
    /// Each remainder has the sign of its element and a smaller absolute value than $2^k$, as with
    /// [`RemPowerOf2`] for [`Integer`](crate::integer::Integer)s. This is the remainder of
    /// truncating division, and the result stays an [`IntegerMatrix`]; for a remainder that is
    /// always non-negative, and a [`NaturalMatrix`] result, use [`ModPowerOf2`]. The height and
    /// width are unchanged.
    ///
    /// $$
    /// f(A, k) = B, \quad \text{where} \quad B_{ij} = A_{ij} - 2^k \operatorname{sgn}(A_{ij})
    ///     \left \lfloor \frac{|A_{ij}|}{2^k} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Every element is taken modulo 4, keeping its sign.
    /// let a = IntegerMatrix::from_str("[1, -7, -2]").unwrap();
    /// assert_eq!(a.clone().rem_power_of_2(2).to_string(), "[1, -3, -2]");
    ///
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// let a = IntegerMatrix::from_str("[-4, -3]").unwrap();
    /// assert_eq!(a.clone().rem_power_of_2(2).to_string(), "[0, -3]");
    /// ```
    #[inline]
    fn rem_power_of_2(mut self, pow: u64) -> Self {
        self.rem_power_of_2_assign(pow);
        self
    }
}

impl RemPowerOf2 for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Divides every element of an [`IntegerMatrix`] by $2^k$, keeping the remainders, taking the
    /// matrix by reference.
    ///
    /// See the documentation for the [`RemPowerOf2`] implementation on [`IntegerMatrix`] that takes
    /// the matrix by value for details.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Every element is taken modulo 4, keeping its sign.
    /// let a = IntegerMatrix::from_str("[1, -7, -2]").unwrap();
    /// assert_eq!((&a).rem_power_of_2(2).to_string(), "[1, -3, -2]");
    ///
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// let a = IntegerMatrix::from_str("[-4, -3]").unwrap();
    /// assert_eq!((&a).rem_power_of_2(2).to_string(), "[0, -3]");
    /// ```
    #[inline]
    fn rem_power_of_2(self, pow: u64) -> IntegerMatrix {
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements
                .iter()
                .map(|x| x.rem_power_of_2(pow))
                .collect(),
        )
    }
}

impl RemPowerOf2Assign for IntegerMatrix {
    /// Divides every element of an [`IntegerMatrix`] by $2^k$, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`RemPowerOf2`] implementation on [`IntegerMatrix`] that takes
    /// the matrix by value for details.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -7, -2]").unwrap();
    /// a.rem_power_of_2_assign(2);
    /// assert_eq!(a.to_string(), "[1, -3, -2]");
    /// ```
    fn rem_power_of_2_assign(&mut self, pow: u64) {
        for x in &mut self.elements {
            x.rem_power_of_2_assign(pow);
        }
    }
}
