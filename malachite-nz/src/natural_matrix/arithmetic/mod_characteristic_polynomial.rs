// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
//      Copyright © 2021 Daniel Schultz
//
//      Copyright © 2022, 2023, 2025, 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::InnerNatural::Small;
use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_determinant::{
    NaturalModBerkowitzRing, charpoly_berkowitz,
};
use crate::natural_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use crate::natural_polynomial::NaturalPolynomial;
use crate::platform::DoubleLimb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExact, ExtendedGcd, IsPowerOf2, Mod, ModAddAssign,
    ModCharacteristicPolynomial, ModInverse,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_matrix::arithmetic::charpoly_over_ring::{
    CharpolyRing, charpoly_danilevsky_over_ring, charpoly_hessenberg_over_ring,
};
use malachite_base::unsigned_matrix::arithmetic::mod_characteristic_polynomial::mod_charpoly;
use malachite_base::unsigned_matrix::arithmetic::mod_power_of_2_characteristic_polynomial::*;

impl CharpolyRing for NaturalModBerkowitzRing<'_> {
    #[inline]
    fn is_zero(&self, x: &Natural) -> bool {
        *x == 0u32
    }

    #[inline]
    fn add_assign(&self, x: &mut Natural, y: &Natural) {
        x.mod_add_assign(y, self.m);
    }

    #[inline]
    fn add_mul_assign(&self, x: &mut Natural, y: &Natural, z: &Natural) {
        x.add_mul_assign(y, z);
        *x %= self.m;
    }

    #[inline]
    fn inverse(&self, x: &Natural) -> Option<Natural> {
        x.mod_inverse(self.m)
    }

    fn dot_strided(&self, xs: &[Natural], ys: &[Natural], stride: usize) -> Natural {
        let mut s = Natural::ZERO;
        for (x, y) in xs.iter().zip(ys.iter().step_by(stride)) {
            s.add_mul_assign(x, y);
        }
        s % self.m
    }

    fn xgcd(&self, a: &Natural, b: &Natural) -> (Natural, Natural, Natural, Natural) {
        let (g, s, t) = a.extended_gcd(b);
        let m = Integer::from(self.m);
        (
            Natural::exact_from(s.mod_op(&m)),
            Natural::exact_from(t.mod_op(&m)),
            a.div_exact(&g),
            b.div_exact(&g),
        )
    }
}

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`,
// reduced modulo `m`, from the constant coefficient up, by Danilevsky's method; or `None` if a
// pivot is not invertible modulo `m`, which cannot happen if `m` is prime.
//
// This is equivalent to `_gr_mat_charpoly_danilevsky` from `gr_mat/charpoly_danilevsky.c`, FLINT
// 3.6.0, for the integers modulo `m`.
crate_test_fn! {
charpoly_danilevsky(xs: &[Natural], n: usize, m: &Natural) -> Option<Vec<Natural>> {
    charpoly_danilevsky_over_ring(&NaturalModBerkowitzRing { m }, xs, n)
}}

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`,
// reduced modulo `m`, from the constant coefficient up, by reduction to Hessenberg form, which
// works for any modulus.
crate_test_fn! {
charpoly_hessenberg(xs: &[Natural], n: usize, m: &Natural) -> Vec<Natural> {
    charpoly_hessenberg_over_ring(&NaturalModBerkowitzRing { m }, xs, n)
}}

// The dimension from which Danilevsky's method is faster than Berkowitz's algorithm, measured on
// random matrices modulo primes of more than two limbs, as a function of the modulus's size.
const fn danilevsky_min_dimension(bits: u64) -> usize {
    match bits {
        0..=192 => 16,
        193..=384 => 14,
        _ => 12,
    }
}

// If Danilevsky's method fails, the reduction to Hessenberg form is used from this dimension on,
// and Berkowitz's algorithm below it.
const HESSENBERG_MIN_DIMENSION: usize = 20;

// The characteristic polynomial modulo `m` of the square matrix `a`, whose elements are reduced
// modulo `m`: on single limbs if `m` fits in one and on double limbs if it fits in two, as for
// `UnsignedMatrix`s. Otherwise, if `m` is a power of 2, the matrix is reduced to Hessenberg form
// with pivots of smallest 2-adic valuation; small matrices are handled by Berkowitz's algorithm and
// larger ones by Danilevsky's method, which succeeds if `m` is prime; and if it fails, Berkowitz's
// algorithm or, for large matrices, the reduction to Hessenberg form with extended-GCD row
// operations is used instead.
//
// This is equivalent to `fmpz_mod_mat_charpoly` from `fmpz_mod_mat/charpoly.c`, FLINT 3.6.0, which
// calls `_gr_fmpz_mod_mat_charpoly` from `gr/fmpz_mod.c`, except for the choice of algorithm: FLINT
// uses Danilevsky's method from dimension 16 on and Berkowitz's algorithm otherwise and when
// Danilevsky's method fails.
fn mod_charpoly_helper(a: &NaturalMatrix, m: &Natural) -> NaturalPolynomial {
    assert_reduced(a, m);
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let berkowitz = || {
        let mut cp = vec![Natural::ZERO; n + 1];
        charpoly_berkowitz(&mut cp, &a.elements, n, m);
        cp
    };
    let coefficients = if let Natural(Small(small_m)) = *m {
        mod_charpoly(to_limb_matrix(a).elements_ref(), n, small_m)
            .into_iter()
            .map(Natural::from)
            .collect()
    } else if m.significant_bits() <= TWICE_WIDTH {
        let elements: Vec<DoubleLimb> = a.elements.iter().map(DoubleLimb::exact_from).collect();
        mod_charpoly(&elements, n, DoubleLimb::exact_from(m))
            .into_iter()
            .map(Natural::from)
            .collect()
    } else if m.is_power_of_2() {
        charpoly_mod_power_of_2(a.elements.clone(), n, m.trailing_zeros().unwrap())
    } else if n < danilevsky_min_dimension(m.significant_bits()) {
        berkowitz()
    } else if let Some(p) = charpoly_danilevsky(&a.elements, n, m) {
        p
    } else if n < HESSENBERG_MIN_DIMENSION {
        berkowitz()
    } else {
        charpoly_hessenberg(&a.elements, n, m)
    };
    NaturalPolynomial::from_coefficients_asc(coefficients)
}

impl ModCharacteristicPolynomial<Natural> for NaturalMatrix {
    type Output = NaturalPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`] modulo $m$, taking the
    /// matrix by value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The characteristic polynomial
    /// of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$ whose
    /// coefficients are reduced modulo $m$, and that of the $0 \times 0$ matrix is 1; modulo 1,
    /// every polynomial is 0. Since reduction modulo $m$ is a ring homomorphism, the result is the
    /// integer characteristic polynomial with its coefficients reduced modulo $m$. The modulus may
    /// be composite.
    ///
    /// When $m$ fits in one or two limbs, the computation is done on limbs, as for an
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix). Otherwise, if $m$ is a
    /// power of 2, the matrix is reduced to a similar upper Hessenberg matrix, whose elements more
    /// than one below the diagonal are 0, choosing pivots of smallest 2-adic valuation, which
    /// divide the other elements of their columns. Otherwise, small matrices are handled by
    /// Berkowitz's division-free algorithm and larger ones, from dimension 12, 14, or 16 on
    /// depending on the size of $m$, by Danilevsky's method, which reduces the matrix to Frobenius
    /// normal form with similarity transforms that divide by pivots and always succeeds when $m$ is
    /// prime. If a pivot is not invertible, Berkowitz's algorithm is used instead, or, from
    /// dimension 20 on, a reduction to Hessenberg form that clears each element below a
    /// non-invertible pivot with a row operation of determinant 1 built from the extended GCD of
    /// the two elements, followed by the inverse column operation. The characteristic polynomial of
    /// a Hessenberg matrix follows from a division-free recurrence.
    ///
    /// $$
    /// f(A, m) = \det(xI - A) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^3 k \log k \log\log k + n^2 k^2)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if $m$ is 0, or if any element is not reduced modulo
    /// $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_characteristic_polynomial(Natural::from(6u32))
    ///         .to_string(),
    ///     "x^2+x+4"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let m = Natural::from_str("1361129467683753853853498429727072845819").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_characteristic_polynomial(m).to_string(),
    ///     "x^2+1361129467683753853853498429727072845814*x+\
    ///     1361129467683753853853498429727072845817"
    /// );
    /// ```
    #[inline]
    fn mod_characteristic_polynomial(self, m: Natural) -> NaturalPolynomial {
        mod_charpoly_helper(&self, &m)
    }
}

impl ModCharacteristicPolynomial<&Natural> for NaturalMatrix {
    type Output = NaturalPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`] modulo $m$, taking the
    /// matrix by value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The characteristic polynomial
    /// of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$ whose
    /// coefficients are reduced modulo $m$, and that of the $0 \times 0$ matrix is 1; modulo 1,
    /// every polynomial is 0. Since reduction modulo $m$ is a ring homomorphism, the result is the
    /// integer characteristic polynomial with its coefficients reduced modulo $m$. The modulus may
    /// be composite.
    ///
    /// When $m$ fits in one or two limbs, the computation is done on limbs, as for an
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix). Otherwise, if $m$ is a
    /// power of 2, the matrix is reduced to a similar upper Hessenberg matrix, whose elements more
    /// than one below the diagonal are 0, choosing pivots of smallest 2-adic valuation, which
    /// divide the other elements of their columns. Otherwise, small matrices are handled by
    /// Berkowitz's division-free algorithm and larger ones, from dimension 12, 14, or 16 on
    /// depending on the size of $m$, by Danilevsky's method, which reduces the matrix to Frobenius
    /// normal form with similarity transforms that divide by pivots and always succeeds when $m$ is
    /// prime. If a pivot is not invertible, Berkowitz's algorithm is used instead, or, from
    /// dimension 20 on, a reduction to Hessenberg form that clears each element below a
    /// non-invertible pivot with a row operation of determinant 1 built from the extended GCD of
    /// the two elements, followed by the inverse column operation. The characteristic polynomial of
    /// a Hessenberg matrix follows from a division-free recurrence.
    ///
    /// $$
    /// f(A, m) = \det(xI - A) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^3 k \log k \log\log k + n^2 k^2)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if $m$ is 0, or if any element is not reduced modulo
    /// $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_characteristic_polynomial(&Natural::from(6u32))
    ///         .to_string(),
    ///     "x^2+x+4"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let m = Natural::from_str("1361129467683753853853498429727072845819").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_characteristic_polynomial(&m).to_string(),
    ///     "x^2+1361129467683753853853498429727072845814*x+\
    ///     1361129467683753853853498429727072845817"
    /// );
    /// ```
    #[inline]
    fn mod_characteristic_polynomial(self, m: &Natural) -> NaturalPolynomial {
        mod_charpoly_helper(&self, m)
    }
}

impl ModCharacteristicPolynomial<Natural> for &NaturalMatrix {
    type Output = NaturalPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`] modulo $m$, taking the
    /// matrix by reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The characteristic polynomial
    /// of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$ whose
    /// coefficients are reduced modulo $m$, and that of the $0 \times 0$ matrix is 1; modulo 1,
    /// every polynomial is 0. Since reduction modulo $m$ is a ring homomorphism, the result is the
    /// integer characteristic polynomial with its coefficients reduced modulo $m$. The modulus may
    /// be composite.
    ///
    /// When $m$ fits in one or two limbs, the computation is done on limbs, as for an
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix). Otherwise, if $m$ is a
    /// power of 2, the matrix is reduced to a similar upper Hessenberg matrix, whose elements more
    /// than one below the diagonal are 0, choosing pivots of smallest 2-adic valuation, which
    /// divide the other elements of their columns. Otherwise, small matrices are handled by
    /// Berkowitz's division-free algorithm and larger ones, from dimension 12, 14, or 16 on
    /// depending on the size of $m$, by Danilevsky's method, which reduces the matrix to Frobenius
    /// normal form with similarity transforms that divide by pivots and always succeeds when $m$ is
    /// prime. If a pivot is not invertible, Berkowitz's algorithm is used instead, or, from
    /// dimension 20 on, a reduction to Hessenberg form that clears each element below a
    /// non-invertible pivot with a row operation of determinant 1 built from the extended GCD of
    /// the two elements, followed by the inverse column operation. The characteristic polynomial of
    /// a Hessenberg matrix follows from a division-free recurrence.
    ///
    /// $$
    /// f(A, m) = \det(xI - A) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^3 k \log k \log\log k + n^2 k^2)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if $m$ is 0, or if any element is not reduced modulo
    /// $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_characteristic_polynomial(Natural::from(6u32))
    ///         .to_string(),
    ///     "x^2+x+4"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let m = Natural::from_str("1361129467683753853853498429727072845819").unwrap();
    /// assert_eq!(
    ///     (&a).mod_characteristic_polynomial(m).to_string(),
    ///     "x^2+1361129467683753853853498429727072845814*x+\
    ///     1361129467683753853853498429727072845817"
    /// );
    /// ```
    #[inline]
    fn mod_characteristic_polynomial(self, m: Natural) -> NaturalPolynomial {
        mod_charpoly_helper(self, &m)
    }
}

impl ModCharacteristicPolynomial<&Natural> for &NaturalMatrix {
    type Output = NaturalPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`] modulo $m$, taking the
    /// matrix by reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The characteristic polynomial
    /// of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$ whose
    /// coefficients are reduced modulo $m$, and that of the $0 \times 0$ matrix is 1; modulo 1,
    /// every polynomial is 0. Since reduction modulo $m$ is a ring homomorphism, the result is the
    /// integer characteristic polynomial with its coefficients reduced modulo $m$. The modulus may
    /// be composite.
    ///
    /// When $m$ fits in one or two limbs, the computation is done on limbs, as for an
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix). Otherwise, if $m$ is a
    /// power of 2, the matrix is reduced to a similar upper Hessenberg matrix, whose elements more
    /// than one below the diagonal are 0, choosing pivots of smallest 2-adic valuation, which
    /// divide the other elements of their columns. Otherwise, small matrices are handled by
    /// Berkowitz's division-free algorithm and larger ones, from dimension 12, 14, or 16 on
    /// depending on the size of $m$, by Danilevsky's method, which reduces the matrix to Frobenius
    /// normal form with similarity transforms that divide by pivots and always succeeds when $m$ is
    /// prime. If a pivot is not invertible, Berkowitz's algorithm is used instead, or, from
    /// dimension 20 on, a reduction to Hessenberg form that clears each element below a
    /// non-invertible pivot with a row operation of determinant 1 built from the extended GCD of
    /// the two elements, followed by the inverse column operation. The characteristic polynomial of
    /// a Hessenberg matrix follows from a division-free recurrence.
    ///
    /// $$
    /// f(A, m) = \det(xI - A) \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^3 k \log k \log\log k + n^2 k^2)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if $m$ is 0, or if any element is not reduced modulo
    /// $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_characteristic_polynomial(&Natural::from(6u32))
    ///         .to_string(),
    ///     "x^2+x+4"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let m = Natural::from_str("1361129467683753853853498429727072845819").unwrap();
    /// assert_eq!(
    ///     (&a).mod_characteristic_polynomial(&m).to_string(),
    ///     "x^2+1361129467683753853853498429727072845814*x+\
    ///     1361129467683753853853498429727072845817"
    /// );
    /// ```
    #[inline]
    fn mod_characteristic_polynomial(self, m: &Natural) -> NaturalPolynomial {
        mod_charpoly_helper(self, m)
    }
}
