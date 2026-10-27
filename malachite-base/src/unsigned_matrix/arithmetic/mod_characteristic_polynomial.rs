// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
//      Copyright © 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModCharacteristicPolynomial;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::TrailingZeros;
use crate::polynomial::Polynomial;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::berkowitz_over_ring::UnsignedModBerkowitzRing;
use crate::unsigned_matrix::arithmetic::charpoly_over_ring::{
    charpoly_danilevsky_over_ring, charpoly_hessenberg_over_ring,
};
use crate::unsigned_matrix::arithmetic::mod_determinant::charpoly_berkowitz;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_characteristic_polynomial::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;
use alloc::vec::Vec;

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`,
// reduced modulo `m`, from the constant coefficient up, by Danilevsky's method; or `None` if a
// pivot is not invertible modulo `m`, which cannot happen if `m` is prime.
//
// This is equivalent to `_nmod_mat_charpoly_danilevsky` from `nmod_mat/charpoly.c`, FLINT 3.6.0.
crate_test_fn! {
charpoly_danilevsky<T: PrimitiveUnsigned>(xs: &[T], n: usize, m: T) -> Option<Vec<T>> {
    charpoly_danilevsky_over_ring(&UnsignedModBerkowitzRing::new(m, n), xs, n)
}}

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`,
// reduced modulo `m`, from the constant coefficient up, by reduction to Hessenberg form, which
// works for any modulus.
crate_test_fn! {
charpoly_hessenberg<T: PrimitiveUnsigned>(xs: &[T], n: usize, m: T) -> Vec<T> {
    charpoly_hessenberg_over_ring(&UnsignedModBerkowitzRing::new(m, n), xs, n)
}}

// The dimension from which Danilevsky's method is faster than Berkowitz's algorithm, measured on
// random matrices modulo primes. It depends on the width of `T` and on the size of `m`, which
// determine how cheap modular products are and how many words Berkowitz's dot products are
// accumulated in.
fn danilevsky_min_dimension<T: PrimitiveUnsigned>(m: T) -> usize {
    let bits = m.significant_bits();
    if T::WIDTH == u64::WIDTH {
        match bits {
            0..=30 => 15,
            31..=32 => 8,
            33..=35 => 12,
            36..=51 => 20,
            52..=61 => 30,
            62..=63 => 17,
            _ => 24,
        }
    } else if T::WIDTH == u128::WIDTH {
        match bits {
            0..=96 => 20,
            97..=125 => 31,
            126 => 18,
            _ => 14,
        }
    } else {
        match bits {
            0..=15 => 10,
            16..=30 => 8,
            _ => 6,
        }
    }
}

// If Danilevsky's method fails, the reduction to Hessenberg form is used from this dimension on,
// and Berkowitz's algorithm below it.
const HESSENBERG_MIN_DIMENSION: usize = 32;

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`,
// reduced modulo `m`, from the constant coefficient up. If `m` is a power of 2, the matrix is
// reduced to Hessenberg form with pivots of smallest 2-adic valuation. Otherwise, small matrices
// are handled by Berkowitz's algorithm and larger ones by Danilevsky's method, which succeeds if
// `m` is prime; if it fails, Berkowitz's algorithm or, for large matrices, the reduction to
// Hessenberg form with extended-GCD row operations is used instead.
//
// This is equivalent to `nmod_mat_charpoly` from `nmod_mat/charpoly.c`, FLINT 3.6.0, without the
// path for prime moduli below 256 at dimension 32 and up, which computes the same polynomial, and
// except for the choice of algorithm: FLINT uses Danilevsky's method above dimension 8 and
// Berkowitz's algorithm otherwise and when Danilevsky's method fails.
#[doc(hidden)]
pub fn mod_charpoly<T: PrimitiveUnsigned>(xs: &[T], n: usize, m: T) -> Vec<T> {
    let berkowitz = || {
        let mut cp = vec![T::ZERO; n + 1];
        charpoly_berkowitz(&mut cp, xs, n, m);
        cp
    };
    if m.is_power_of_2() {
        charpoly_mod_power_of_2(xs.to_vec(), n, TrailingZeros::trailing_zeros(m))
    } else if n < danilevsky_min_dimension(m) {
        berkowitz()
    } else if let Some(p) = charpoly_danilevsky(xs, n, m) {
        p
    } else if n < HESSENBERG_MIN_DIMENSION {
        berkowitz()
    } else {
        charpoly_hessenberg(xs, n, m)
    }
}

impl<T: PrimitiveUnsigned> ModCharacteristicPolynomial<T> for UnsignedMatrix<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the characteristic polynomial of a square [`UnsignedMatrix`] modulo $m$, taking it
    /// by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The characteristic polynomial
    /// of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$ whose
    /// coefficients are reduced modulo $m$, and that of the $0 \times 0$ matrix is 1; modulo 1,
    /// every polynomial is 0. Since reduction modulo $m$ is a ring homomorphism, the result is the
    /// integer characteristic polynomial with its coefficients reduced modulo $m$. The modulus may
    /// be composite.
    ///
    /// If $m$ is a power of 2, the matrix is reduced to a similar upper Hessenberg matrix, whose
    /// elements more than one below the diagonal are 0, choosing pivots of smallest 2-adic
    /// valuation, which divide the other elements of their columns. Otherwise, small matrices are
    /// handled by Berkowitz's division-free algorithm and larger ones by Danilevsky's method, which
    /// reduces the matrix to Frobenius normal form with similarity transforms that divide by pivots
    /// and always succeeds when $m$ is prime; the dimension from which it is used, between 6 and
    /// 31, depends on the width of `T` and the size of $m$. If a pivot is not invertible,
    /// Berkowitz's algorithm is used instead, or, from dimension 32 on, a reduction to Hessenberg
    /// form that clears each element below a non-invertible pivot with a row operation of
    /// determinant 1 built from the extended GCD of the two elements, followed by the inverse
    /// column operation. The characteristic polynomial of a Hessenberg matrix follows from a
    /// division-free recurrence.
    ///
    /// $$
    /// f(A, m) = \det(xI - A) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if $m$ is 0, or if any element is not reduced modulo
    /// $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_characteristic_polynomial(7).to_string(),
    ///     "x^2+2*x+5"
    /// );
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_characteristic_polynomial(6).to_string(),
    ///     "x^2+x+4"
    /// );
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[2, 3, 1; 3, 2, 3; 1, 3, 0]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_characteristic_polynomial(4).to_string(),
    ///     "x^3+x+2"
    /// );
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(a.clone().mod_characteristic_polynomial(1).to_string(), "0");
    /// ```
    #[inline]
    fn mod_characteristic_polynomial(self, m: T) -> UnsignedPolynomial<T> {
        (&self).mod_characteristic_polynomial(m)
    }
}

impl<T: PrimitiveUnsigned> ModCharacteristicPolynomial<T> for &UnsignedMatrix<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the characteristic polynomial of a square [`UnsignedMatrix`] modulo $m$, taking it
    /// by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The characteristic polynomial
    /// of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$ whose
    /// coefficients are reduced modulo $m$, and that of the $0 \times 0$ matrix is 1; modulo 1,
    /// every polynomial is 0. Since reduction modulo $m$ is a ring homomorphism, the result is the
    /// integer characteristic polynomial with its coefficients reduced modulo $m$. The modulus may
    /// be composite.
    ///
    /// If $m$ is a power of 2, the matrix is reduced to a similar upper Hessenberg matrix, whose
    /// elements more than one below the diagonal are 0, choosing pivots of smallest 2-adic
    /// valuation, which divide the other elements of their columns. Otherwise, small matrices are
    /// handled by Berkowitz's division-free algorithm and larger ones by Danilevsky's method, which
    /// reduces the matrix to Frobenius normal form with similarity transforms that divide by pivots
    /// and always succeeds when $m$ is prime; the dimension from which it is used, between 6 and
    /// 31, depends on the width of `T` and the size of $m$. If a pivot is not invertible,
    /// Berkowitz's algorithm is used instead, or, from dimension 32 on, a reduction to Hessenberg
    /// form that clears each element below a non-invertible pivot with a row operation of
    /// determinant 1 built from the extended GCD of the two elements, followed by the inverse
    /// column operation. The characteristic polynomial of a Hessenberg matrix follows from a
    /// division-free recurrence.
    ///
    /// $$
    /// f(A, m) = \det(xI - A) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if $m$ is 0, or if any element is not reduced modulo
    /// $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_characteristic_polynomial(7).to_string(),
    ///     "x^2+2*x+5"
    /// );
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_characteristic_polynomial(6).to_string(), "x^2+x+4");
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[2, 3, 1; 3, 2, 3; 1, 3, 0]").unwrap();
    /// assert_eq!((&a).mod_characteristic_polynomial(4).to_string(), "x^3+x+2");
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!((&a).mod_characteristic_polynomial(1).to_string(), "0");
    /// ```
    fn mod_characteristic_polynomial(self, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, m);
        assert_eq!(self.height, self.width, "the matrix must be square");
        UnsignedPolynomial::from_coefficients_asc(mod_charpoly(&self.elements, self.height, m))
    }
}
