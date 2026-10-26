// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2012, 2013 Sebastian Pancratz
//
//      Copyright © 2023, 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::characteristic_polynomial_bound::*;
use crate::integer_matrix::arithmetic::mul::multi_mod::next_prime;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_vector::IntegerVector;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::{LIMB_HIGH_BIT, WIDTH_MINUS_1};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, CharacteristicPolynomial, SubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_matrix::arithmetic::berkowitz_over_ring::{
    BerkowitzRing, charpoly_berkowitz_over_ring,
};
use malachite_base::unsigned_matrix::arithmetic::mod_characteristic_polynomial::mod_charpoly;
use malachite_base::unsigned_vector::UnsignedVector;

// The characteristic polynomial of a 2 by 2 matrix.
//
// This is equivalent to `_fmpz_mat_charpoly_small_2x2` from `fmpz_mat/charpoly.c`, FLINT 3.6.0.
fn charpoly_2x2(cp: &mut [Integer], x: &[Integer]) {
    cp[1] = -(&x[0] + &x[3]);
    cp[0] = &x[0] * &x[3];
    cp[0].sub_mul_assign(&x[1], &x[2]);
}

// The characteristic polynomial of a 3 by 3 matrix.
//
// This is equivalent to `_fmpz_mat_charpoly_small_3x3` from `fmpz_mat/charpoly.c`, FLINT 3.6.0.
fn charpoly_3x3(cp: &mut [Integer], x: &[Integer]) {
    let e = |i: usize, j: usize| &x[i * 3 + j];
    let mut a_0 = e(1, 0) * e(2, 1);
    a_0.sub_mul_assign(e(1, 1), e(2, 0));
    cp[0] = -(&a_0 * e(0, 2));
    cp[1] = -(e(2, 0) * e(0, 2));
    a_0 = e(1, 2) * e(2, 0);
    a_0.sub_mul_assign(e(1, 0), e(2, 2));
    cp[0].sub_mul_assign(&a_0, e(0, 1));
    cp[1].sub_mul_assign(e(1, 0), e(0, 1));
    a_0 = e(1, 1) * e(2, 2);
    let a_1 = -(e(1, 1) + e(2, 2));
    a_0.sub_mul_assign(e(1, 2), e(2, 1));
    cp[0].sub_mul_assign(&a_0, e(0, 0));
    cp[1].sub_mul_assign(&a_1, e(0, 0));
    cp[1] += &a_0;
    cp[2] = a_1 - e(0, 0);
}

// The integers, for `charpoly_berkowitz_over_ring`.
struct IntegerBerkowitzRing;

impl BerkowitzRing for IntegerBerkowitzRing {
    type Element = Integer;

    #[inline]
    fn zero(&self) -> Integer {
        Integer::ZERO
    }

    #[inline]
    fn one(&self) -> Integer {
        Integer::ONE
    }

    #[inline]
    fn neg(&self, x: &Integer) -> Integer {
        -x
    }

    #[inline]
    fn add(&self, x: &Integer, y: &Integer) -> Integer {
        x + y
    }

    #[inline]
    fn sub_assign(&self, x: &mut Integer, y: &Integer) {
        *x -= y;
    }

    #[inline]
    fn mul(&self, x: &Integer, y: &Integer) -> Integer {
        x * y
    }

    fn dot(&self, xs: &[Integer], ys: &[Integer]) -> Integer {
        let mut s = Integer::ZERO;
        for (x, y) in xs.iter().zip(ys) {
            s.add_mul_assign(x, y);
        }
        s
    }

    fn dot_rev(&self, xs: &[Integer], ys: &[Integer]) -> Integer {
        let mut s = Integer::ZERO;
        for (x, y) in xs.iter().zip(ys.iter().rev()) {
            s.add_mul_assign(x, y);
        }
        s
    }
}

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, from
// the constant coefficient up, with Berkowitz's division-free algorithm.
//
// This is equivalent to `_fmpz_mat_charpoly_berkowitz` from `fmpz_mat/charpoly.c`, FLINT 3.6.0,
// which calls `_gr_mat_charpoly_berkowitz` from `gr_mat/charpoly_berkowitz.c`.
crate_test_fn! {
charpoly_berkowitz(xs: &[Integer], n: usize) -> Vec<Integer> {
    charpoly_berkowitz_over_ring(&IntegerBerkowitzRing, xs, n)
}}

// The coefficients of the characteristic polynomial of the n by n matrix `a`, from the constant
// coefficient up, computed modulo enough word-sized primes for their product to exceed twice the
// bound on the coefficients, and recovered from the residues by the Chinese remainder theorem.
//
// This is equivalent to `_fmpz_mat_charpoly_modular` from `fmpz_mat/charpoly_modular.c`, FLINT
// 3.6.0, without the threads.
crate_test_fn! {
charpoly_modular(a: &IntegerMatrix) -> Vec<Integer> {
    let n = a.height;
    if n == 0 {
        return vec![Integer::ONE];
    }
    // Account for signs.
    let bound = characteristic_polynomial_bound_helper(a).significant_bits() + 1;
    let prime_count = bound.div_ceil(WIDTH_MINUS_1);
    let mut primes = Vec::new();
    let mut p = LIMB_HIGH_BIT;
    for _ in 0..prime_count {
        p = next_prime(p);
        primes.push(p);
    }
    let residues: Vec<UnsignedVector<Limb>> = a
        .multi_mod(&primes)
        .iter()
        .zip(&primes)
        .map(|(a_mod, &p)| UnsignedVector {
            elements: mod_charpoly(a_mod.elements_ref(), n, p),
        })
        .collect();
    IntegerVector::multi_balanced_crt(&primes, &residues)
        .unwrap()
        .elements
}}

// The largest dimension at which Berkowitz's algorithm is faster than the multimodular algorithm,
// for matrices whose largest element has `bits` bits. Berkowitz's algorithm does $O(n^4)$
// operations on numbers that grow with the dimension, and the multimodular algorithm does $O(n^3)$
// word operations per prime, with a number of primes that grows with the dimension and the size of
// the elements; the cutoffs were measured on random matrices.
const fn berkowitz_max_dimension(bits: u64) -> usize {
    match bits {
        0..=16 => 8,
        33..=128 => 22,
        129..=256 => 20,
        257..=512 => 16,
        // 17 to 32 bits, and more than 512
        _ => 10,
    }
}

// Whether Berkowitz's algorithm is preferable to the multimodular algorithm for the n by n matrix
// `a`.
//
// This replaces `want_berkowitz` from `fmpz_mat/charpoly.c`, FLINT 3.6.0, which chooses Berkowitz's
// algorithm for dimensions up to 8, and above that for dimensions below the number of bits of the
// number of bits of the largest element. Both choices give the same polynomial.
fn want_berkowitz(n: usize, a: &IntegerMatrix) -> bool {
    n <= berkowitz_max_dimension(vec_max_bits(&a.elements).0)
}

// The coefficients of the characteristic polynomial of the square matrix `a`, from the constant
// coefficient up.
//
// This is equivalent to `_fmpz_mat_charpoly` from `fmpz_mat/charpoly.c`, FLINT 3.6.0.
fn charpoly_helper(a: &IntegerMatrix) -> Vec<Integer> {
    let n = a.height;
    if n <= 3 {
        let mut cp = vec![Integer::ZERO; n + 1];
        cp[n] = Integer::ONE;
        match n {
            1 => cp[0] = -&a.elements[0],
            2 => charpoly_2x2(&mut cp, &a.elements),
            3 => charpoly_3x3(&mut cp, &a.elements),
            _ => {}
        }
        return cp;
    }
    if a.is_zero() {
        let mut cp = vec![Integer::ZERO; n + 1];
        cp[n] = Integer::ONE;
        return cp;
    }
    if want_berkowitz(n, a) {
        charpoly_berkowitz(&a.elements, n)
    } else {
        charpoly_modular(a)
    }
}

impl CharacteristicPolynomial for IntegerMatrix {
    type Output = IntegerPolynomial;

    /// Computes the characteristic polynomial of a square [`IntegerMatrix`], taking it by value.
    ///
    /// The characteristic polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic
    /// polynomial of degree $n$ with integer coefficients; that of the $0 \times 0$ matrix is 1.
    /// Its constant coefficient is $(-1)^n \det A$, and its coefficient of $x^{n-1}$ is
    /// $-\operatorname{tr} A$.
    ///
    /// $$
    /// f(A) = \det(xI - A).
    /// $$
    ///
    /// Matrices up to $3 \times 3$ use explicit formulas, and the zero matrix gives $x^n$.
    /// Otherwise, if the dimension is small for the size of the elements (at most 8 to 22,
    /// depending on the number of bits of the largest element), Berkowitz's division-free algorithm
    /// is used. For larger dimensions, the polynomial is computed modulo enough word-sized primes
    /// for their product to exceed twice
    /// [`characteristic_polynomial_bound`](IntegerMatrix::characteristic_polynomial_bound), by
    /// Danilevsky's method above dimension 8 (or Berkowitz's algorithm, if Danilevsky's method
    /// meets a pivot that is not invertible) and by Berkowitz's algorithm below, and the
    /// coefficients are recovered with the Chinese remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^5 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of its elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.clone().characteristic_polynomial().to_string(), "1");
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^2-5*x-2"
    /// );
    ///
    /// // A rotation by a right angle
    /// let a = IntegerMatrix::from_str("[0, -1; 1, 0]").unwrap();
    /// assert_eq!(a.clone().characteristic_polynomial().to_string(), "x^2+1");
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^3-16*x^2-12*x+3"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(a.clone().characteristic_polynomial().to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_charpoly` from `fmpz_mat/charpoly.c`, FLINT 3.6.0, without
    /// the threads, and with a different choice of the dimensions at which Berkowitz's algorithm is
    /// used, which gives the same polynomial.
    #[inline]
    fn characteristic_polynomial(self) -> IntegerPolynomial {
        (&self).characteristic_polynomial()
    }
}

impl CharacteristicPolynomial for &IntegerMatrix {
    type Output = IntegerPolynomial;

    /// Computes the characteristic polynomial of a square [`IntegerMatrix`], taking it by
    /// reference.
    ///
    /// The characteristic polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic
    /// polynomial of degree $n$ with integer coefficients; that of the $0 \times 0$ matrix is 1.
    /// Its constant coefficient is $(-1)^n \det A$, and its coefficient of $x^{n-1}$ is
    /// $-\operatorname{tr} A$.
    ///
    /// $$
    /// f(A) = \det(xI - A).
    /// $$
    ///
    /// Matrices up to $3 \times 3$ use explicit formulas, and the zero matrix gives $x^n$.
    /// Otherwise, if the dimension is small for the size of the elements (at most 8 to 22,
    /// depending on the number of bits of the largest element), Berkowitz's division-free algorithm
    /// is used. For larger dimensions, the polynomial is computed modulo enough word-sized primes
    /// for their product to exceed twice
    /// [`characteristic_polynomial_bound`](IntegerMatrix::characteristic_polynomial_bound), by
    /// Danilevsky's method above dimension 8 (or Berkowitz's algorithm, if Danilevsky's method
    /// meets a pivot that is not invertible) and by Berkowitz's algorithm below, and the
    /// coefficients are recovered with the Chinese remainder theorem.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^5 m \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^3 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of its elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "1");
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "x^2-5*x-2");
    ///
    /// // A rotation by a right angle
    /// let a = IntegerMatrix::from_str("[0, -1; 1, 0]").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "x^2+1");
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).characteristic_polynomial().to_string(),
    ///     "x^3-16*x^2-12*x+3"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "x^2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_charpoly` from `fmpz_mat/charpoly.c`, FLINT 3.6.0, without
    /// the threads, and with a different choice of the dimensions at which Berkowitz's algorithm is
    /// used, which gives the same polynomial.
    fn characteristic_polynomial(self) -> IntegerPolynomial {
        assert_eq!(self.height, self.width, "the matrix must be square");
        IntegerPolynomial::from_coefficients_asc(charpoly_helper(self))
    }
}
