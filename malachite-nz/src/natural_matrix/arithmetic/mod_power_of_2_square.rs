// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::square::matrix_square;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a square modulo $2^k$: the matrix is square, and its elements are
// reduced.
fn assert_arguments(a: &NaturalMatrix, pow: u64) {
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// The elements of the square of `a` modulo $2^k$, after checking the arguments. When $k$ is at most
// the width of a limb, every element fits in a limb, and the matrix is squared as an
// `UnsignedMatrix` of limbs, in wrapping limb arithmetic; otherwise the exact square is reduced
// once.
fn square_elements(a: &NaturalMatrix, pow: u64) -> Vec<Natural> {
    assert_arguments(a, pow);
    if pow <= Limb::WIDTH {
        let n = u64::exact_from(a.height);
        UnsignedMatrix::<Limb>::from_owned_elements(
            n,
            n,
            a.elements.iter().map(Limb::wrapping_from).collect(),
        )
        .mod_power_of_2_square(pow)
        .into_elements()
        .into_iter()
        .map(Natural::from)
        .collect()
    } else {
        let mut elements = matrix_square(&a.elements, a.height);
        for x in &mut elements {
            x.mod_power_of_2_assign(pow);
        }
        elements
    }
}

impl ModPowerOf2Square for NaturalMatrix {
    type Output = Self;

    /// Squares a square [`NaturalMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The square $A^2 = AA$ of an
    /// $n \times n$ matrix $A$ is the $n \times n$ matrix whose element in row $i$ and column $j$
    /// is the dot product of row $i$ of $A$ with column $j$ of $A$, reduced modulo $2^k$:
    ///
    /// $$
    /// f(A, k)_{ij} = \left ( \sum_{l=0}^{n-1} A_{il}A_{lj} \right ) \bmod 2^k.
    /// $$
    ///
    /// When $k$ is at most the width of a limb, the matrix is squared in wrapping limb arithmetic,
    /// as for [`UnsignedMatrix`]es, since $2^k$ divides the limb modulus; otherwise the exact
    /// square is computed, as by [`Square`](malachite_base::num::arithmetic::traits::Square), and
    /// each of its elements reduced once.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any of its elements is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_square(4).to_string(), "[7, 10; 15, 6]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_square(100).to_string(), "[7, 10; 15, 22]");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_square(3).to_string(),
    ///     "[4, 1, 3; 5, 1, 1; 1, 4, 1]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_square(self, pow: u64) -> Self {
        (&self).mod_power_of_2_square(pow)
    }
}

impl ModPowerOf2Square for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Squares a square [`NaturalMatrix`] modulo $2^k$, taking it by reference.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any of its elements is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_square(4).to_string(), "[7, 10; 15, 6]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_square(100).to_string(),
    ///     "[7, 10; 15, 22]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_square(3).to_string(),
    ///     "[4, 1, 3; 5, 1, 1; 1, 4, 1]"
    /// );
    /// ```
    fn mod_power_of_2_square(self, pow: u64) -> NaturalMatrix {
        NaturalMatrix {
            elements: square_elements(self, pow),
            height: self.height,
            width: self.width,
        }
    }
}

impl ModPowerOf2SquareAssign for NaturalMatrix {
    /// Squares a square [`NaturalMatrix`] modulo $2^k$ in place.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any of its elements is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_square_assign(4);
    /// assert_eq!(a.to_string(), "[7, 10; 15, 6]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_square_assign(100);
    /// assert_eq!(a.to_string(), "[7, 10; 15, 22]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_power_of_2_square_assign(3);
    /// assert_eq!(a.to_string(), "[4, 1, 3; 5, 1, 1; 1, 4, 1]");
    /// ```
    fn mod_power_of_2_square_assign(&mut self, pow: u64) {
        self.elements = square_elements(self, pow);
    }
}
