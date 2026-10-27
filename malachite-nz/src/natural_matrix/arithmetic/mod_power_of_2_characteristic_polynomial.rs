// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_matrix::NaturalMatrix;
use crate::natural_polynomial::NaturalPolynomial;
use crate::platform::{DoubleLimb, Limb};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2CharacteristicPolynomial, ModPowerOf2IsReduced,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_matrix::arithmetic::mod_power_of_2_characteristic_polynomial::*;

fn assert_arguments(a: &NaturalMatrix, pow: u64) {
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// Returns the characteristic polynomial modulo 2^pow of the n by n matrix with elements `elements`,
// which are reduced modulo 2^pow: on single limbs when 2^pow fits in one, on double limbs when it
// fits in two, and otherwise on `Natural`s, which are copied only if they are borrowed, by the same
// algorithm as for `UnsignedMatrix`s.
fn charpoly_helper(elements: Cow<'_, [Natural]>, n: usize, pow: u64) -> NaturalPolynomial {
    let coefficients: Vec<Natural> = if pow <= Limb::WIDTH {
        let elements: Vec<Limb> = elements.iter().map(Limb::exact_from).collect();
        charpoly_mod_power_of_2(elements, n, pow)
            .into_iter()
            .map(Natural::from)
            .collect()
    } else if pow <= TWICE_WIDTH {
        let elements: Vec<DoubleLimb> = elements.iter().map(DoubleLimb::exact_from).collect();
        charpoly_mod_power_of_2(elements, n, pow)
            .into_iter()
            .map(Natural::from)
            .collect()
    } else {
        charpoly_mod_power_of_2(elements.into_owned(), n, pow)
    };
    NaturalPolynomial::from_coefficients_asc(coefficients)
}

impl ModPowerOf2CharacteristicPolynomial for NaturalMatrix {
    type Output = NaturalPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`] modulo $2^k$, taking it
    /// by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The characteristic
    /// polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$
    /// whose coefficients are reduced modulo $2^k$, and that of the $0 \times 0$ matrix is 1;
    /// modulo 1, every polynomial is 0. Since reduction modulo $2^k$ is a ring homomorphism, the
    /// result is the integer characteristic polynomial with its coefficients reduced modulo $2^k$.
    ///
    /// The matrix is reduced to a similar upper Hessenberg matrix, whose elements more than one
    /// below the diagonal are 0. Although $\mathbb{Z}/2^k\mathbb{Z}$ is not a field, its ideals
    /// form a chain, so the element below the diagonal of smallest 2-adic valuation in each column
    /// divides the others: it is moved just below the diagonal by swapping two rows and the same
    /// two columns, and then multiples of its row clear the elements below it, each followed by the
    /// inverse column operation. The characteristic polynomial of the Hessenberg matrix follows
    /// from a recurrence over its leading principal submatrices that needs no division.
    ///
    /// When $2^k$ fits in one or two limbs, the computation is done on limbs, as for an
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix).
    ///
    /// $$
    /// f(A, k) = \det(xI - A) \bmod 2^k.
    /// $$
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
    /// Panics if the matrix is not square, or if any element is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CharacteristicPolynomial;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(3)
    ///         .to_string(),
    ///     "x^2+3*x+6"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(100)
    ///         .to_string(),
    ///     "x^2+1267650600228229401496703205371*x+1267650600228229401496703205374"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(4)
    ///         .to_string(),
    ///     "x^3+9*x^2+14*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_characteristic_polynomial(self, pow: u64) -> NaturalPolynomial {
        assert_arguments(&self, pow);
        charpoly_helper(Cow::Owned(self.elements), self.height, pow)
    }
}

impl ModPowerOf2CharacteristicPolynomial for &NaturalMatrix {
    type Output = NaturalPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`] modulo $2^k$, taking it
    /// by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The characteristic
    /// polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$
    /// whose coefficients are reduced modulo $2^k$, and that of the $0 \times 0$ matrix is 1;
    /// modulo 1, every polynomial is 0. Since reduction modulo $2^k$ is a ring homomorphism, the
    /// result is the integer characteristic polynomial with its coefficients reduced modulo $2^k$.
    ///
    /// The matrix is reduced to a similar upper Hessenberg matrix, whose elements more than one
    /// below the diagonal are 0. Although $\mathbb{Z}/2^k\mathbb{Z}$ is not a field, its ideals
    /// form a chain, so the element below the diagonal of smallest 2-adic valuation in each column
    /// divides the others: it is moved just below the diagonal by swapping two rows and the same
    /// two columns, and then multiples of its row clear the elements below it, each followed by the
    /// inverse column operation. The characteristic polynomial of the Hessenberg matrix follows
    /// from a recurrence over its leading principal submatrices that needs no division.
    ///
    /// When $2^k$ fits in one or two limbs, the computation is done on limbs, as for an
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix).
    ///
    /// $$
    /// f(A, k) = \det(xI - A) \bmod 2^k.
    /// $$
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
    /// Panics if the matrix is not square, or if any element is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CharacteristicPolynomial;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(3).to_string(),
    ///     "x^2+3*x+6"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(100)
    ///         .to_string(),
    ///     "x^2+1267650600228229401496703205371*x+1267650600228229401496703205374"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(4).to_string(),
    ///     "x^3+9*x^2+14*x+8"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_characteristic_polynomial(self, pow: u64) -> NaturalPolynomial {
        assert_arguments(self, pow);
        charpoly_helper(Cow::Borrowed(&self.elements), self.height, pow)
    }
}
