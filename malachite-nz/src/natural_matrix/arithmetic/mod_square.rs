// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::square::matrix_square;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModAssign, ModIsReduced, ModSquare, ModSquareAssign,
};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of a square modulo `m`: `m` is positive, the matrix is square, and its
// elements are reduced.
fn assert_arguments(a: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// The elements of the square of `a` modulo `m`, after checking the arguments. When `m` fits in a
// limb, so does every element, and the matrix is squared as an `UnsignedMatrix` of limbs; otherwise
// the exact square is reduced once, as by `fmpz_mod_mat_sqr` from `fmpz_mod_mat/sqr.c`, FLINT
// 3.6.0.
fn square_elements(a: &NaturalMatrix, m: &Natural) -> Vec<Natural> {
    assert_arguments(a, m);
    if let Natural(Small(small_m)) = *m {
        let n = u64::exact_from(a.height);
        UnsignedMatrix::from_owned_elements(
            n,
            n,
            a.elements.iter().map(Limb::wrapping_from).collect(),
        )
        .mod_square(small_m)
        .elements
        .into_iter()
        .map(Natural::from)
        .collect()
    } else {
        let mut elements = matrix_square(&a.elements, a.height);
        for x in &mut elements {
            x.mod_assign(m);
        }
        elements
    }
}

impl ModSquare<Natural> for NaturalMatrix {
    type Output = Self;

    /// Squares a square [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix by value and
    /// the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The square $A^2 = AA$ of an
    /// $n \times n$ matrix $A$ is the $n \times n$ matrix whose element in row $i$ and column $j$
    /// is the dot product of row $i$ of $A$ with column $j$ of $A$, reduced modulo $m$:
    ///
    /// $$
    /// f(A, m)_{ij} = \left ( \sum_{l=0}^{n-1} A_{il}A_{lj} \right ) \bmod m.
    /// $$
    ///
    /// When $m$ fits in a limb, the matrix is squared as for [`UnsignedMatrix`]es; otherwise the
    /// exact square is computed, as by [`Square`](malachite_base::num::arithmetic::traits::Square),
    /// and each of its elements reduced once.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_square(Natural::from(9u32)).to_string(),
    ///     "[7, 1; 6, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_square(Natural::from(5u32)).to_string(),
    ///     "[4, 1, 3; 0, 4, 1; 1, 4, 1]"
    /// );
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_square(m).to_string(),
    ///     "[21, 60000000000000000003; 40000000000000000007, 20000000000000000022]"
    /// );
    /// ```
    #[inline]
    fn mod_square(self, m: Natural) -> Self {
        (&self).mod_square(&m)
    }
}

impl ModSquare<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Squares a square [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix by value and
    /// the modulus by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_square(&Natural::from(9u32)).to_string(),
    ///     "[7, 1; 6, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_square(&Natural::from(5u32)).to_string(),
    ///     "[4, 1, 3; 0, 4, 1; 1, 4, 1]"
    /// );
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     a.mod_square(&m).to_string(),
    ///     "[21, 60000000000000000003; 40000000000000000007, 20000000000000000022]"
    /// );
    /// ```
    #[inline]
    fn mod_square(self, m: &Natural) -> Self {
        (&self).mod_square(m)
    }
}

impl ModSquare<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Squares a square [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix by reference
    /// and the modulus by value.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(Natural::from(9u32)).to_string(),
    ///     "[7, 1; 6, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(Natural::from(5u32)).to_string(),
    ///     "[4, 1, 3; 0, 4, 1; 1, 4, 1]"
    /// );
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(m).to_string(),
    ///     "[21, 60000000000000000003; 40000000000000000007, 20000000000000000022]"
    /// );
    /// ```
    fn mod_square(self, m: Natural) -> NaturalMatrix {
        NaturalMatrix {
            elements: square_elements(self, &m),
            height: self.height,
            width: self.width,
        }
    }
}

impl ModSquare<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Squares a square [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix by reference
    /// and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(&Natural::from(9u32)).to_string(),
    ///     "[7, 1; 6, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(&Natural::from(5u32)).to_string(),
    ///     "[4, 1, 3; 0, 4, 1; 1, 4, 1]"
    /// );
    ///
    /// let a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(&m).to_string(),
    ///     "[21, 60000000000000000003; 40000000000000000007, 20000000000000000022]"
    /// );
    /// ```
    fn mod_square(self, m: &Natural) -> NaturalMatrix {
        NaturalMatrix {
            elements: square_elements(self, m),
            height: self.height,
            width: self.width,
        }
    }
}

impl ModSquareAssign<Natural> for NaturalMatrix {
    /// Squares a square [`NaturalMatrix`] modulo a [`Natural`] $m$ in place, taking the modulus by
    /// value.
    ///
    /// See the documentation for the [`ModSquare`] implementation that takes both arguments by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_square_assign(Natural::from(9u32));
    /// assert_eq!(a.to_string(), "[7, 1; 6, 4]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_square_assign(Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[4, 1, 3; 0, 4, 1; 1, 4, 1]");
    ///
    /// let mut a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_square_assign(m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[21, 60000000000000000003; 40000000000000000007, 20000000000000000022]"
    /// );
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: Natural) {
        self.elements = square_elements(self, &m);
    }
}

impl ModSquareAssign<&Natural> for NaturalMatrix {
    /// Squares a square [`NaturalMatrix`] modulo a [`Natural`] $m$ in place, taking the modulus by
    /// reference.
    ///
    /// See the documentation for the [`ModSquare`] implementation that takes both arguments by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_square_assign(&Natural::from(9u32));
    /// assert_eq!(a.to_string(), "[7, 1; 6, 4]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_square_assign(&Natural::from(5u32));
    /// assert_eq!(a.to_string(), "[4, 1, 3; 0, 4, 1; 1, 4, 1]");
    ///
    /// let mut a =
    ///     NaturalMatrix::from_str("[10000000000000000000, 3; 7, 10000000000000000001]").unwrap();
    /// let m = Natural::from_str("100000000000000000000").unwrap();
    /// a.mod_square_assign(&m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[21, 60000000000000000003; 40000000000000000007, 20000000000000000022]"
    /// );
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: &Natural) {
        self.elements = square_elements(self, m);
    }
}
