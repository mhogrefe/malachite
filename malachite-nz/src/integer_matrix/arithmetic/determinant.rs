// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018, 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::certify_lu_mod_p::rank_certify_lu_mod_p;
use crate::integer_matrix::arithmetic::determinant_bound::determinant_bound_helper;
use crate::integer_matrix::arithmetic::determinant_divisor::determinant_divisor;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::fflu_determinant;
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, Determinant, DivRound, ModInverse, ModMul, ModNeg, ModSub, MulSubMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;

// The determinant of a square matrix of dimension at most 4, by cofactor expansion.
//
// This is equivalent to `fmpz_mat_det_cofactor` from `fmpz_mat/det_cofactor.c`, FLINT 3.6.0.
crate_test_fn! {
determinant_cofactor(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let e = |i: usize, j: usize| &a.elements[i * a.width + j];
    let fmms = |a: &Integer, b: &Integer, c: &Integer, d: &Integer| a.mul_sub_mul(b, c, d);
    match a.height {
        0 => Integer::ONE,
        1 => e(0, 0).clone(),
        2 => fmms(e(0, 0), e(1, 1), e(0, 1), e(1, 0)),
        3 => {
            (fmms(e(1, 0), e(2, 1), e(1, 1), e(2, 0)) * e(0, 2))
                .add_mul(fmms(e(1, 2), e(2, 0), e(1, 0), e(2, 2)), e(0, 1))
                .add_mul(fmms(e(1, 1), e(2, 2), e(1, 2), e(2, 1)), e(0, 0))
        }
        4 => {
            let mut det =
                fmms(e(0, 3), e(1, 2), e(0, 2), e(1, 3)) * fmms(e(2, 1), e(3, 0), e(2, 0), e(3, 1));
            for (a, b) in [
                (
                    fmms(e(0, 1), e(1, 3), e(0, 3), e(1, 1)),
                    fmms(e(2, 2), e(3, 0), e(2, 0), e(3, 2)),
                ),
                (
                    fmms(e(0, 2), e(1, 1), e(0, 1), e(1, 2)),
                    fmms(e(2, 3), e(3, 0), e(2, 0), e(3, 3)),
                ),
                (
                    fmms(e(0, 3), e(1, 0), e(0, 0), e(1, 3)),
                    fmms(e(2, 2), e(3, 1), e(2, 1), e(3, 2)),
                ),
                (
                    fmms(e(0, 0), e(1, 2), e(0, 2), e(1, 0)),
                    fmms(e(2, 3), e(3, 1), e(2, 1), e(3, 3)),
                ),
                (
                    fmms(e(0, 1), e(1, 0), e(0, 0), e(1, 1)),
                    fmms(e(2, 3), e(3, 2), e(2, 2), e(3, 3)),
                ),
            ] {
                det.add_mul_assign(a, b);
            }
            det
        }
        _ => panic!("cofactor expansion is only implemented for dimensions up to 4"),
    }
}}

// The determinant of a square matrix, by Bareiss's fraction-free elimination: the last pivot of the
// fraction-free LU decomposition, negated if the row permutation is odd.
//
// This is equivalent to `fmpz_mat_det_bareiss` from `fmpz_mat/det_bareiss.c`, FLINT 3.6.0.
crate_test_fn! {
determinant_bareiss(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    if n == 0 {
        return Integer::ONE;
    }
    fflu_determinant(&a.elements, n, true)
}}

// The largest dimension for which the determinant is found by cofactor expansion.
const COFACTOR_MAX_DIMENSION: usize = 4;

// The largest dimension for which the determinant is found by Bareiss's elimination.
const BAREISS_MAX_DIMENSION: usize = 24;

// The largest dimension for which the determinant is always found by the multimodular algorithm
// without a divisor.
const MODULAR_MAX_DIMENSION: usize = 59;

// The unique y with y ≡ x (mod `product`), y ≡ `r` (mod `p`), and -Mp/2 < y <= Mp/2, where M is
// `product`, given x with -M/2 < x <= M/2 and the prime `p`, which does not divide M.
//
// This is equivalent to `fmpz_CRT_ui` from `fmpz/CRT.c`, FLINT 3.6.0, with `sign = 1`, computed
// with word-sized residues.
pub(crate) fn balanced_crt_word(x: &Integer, product: &Natural, r: Limb, p: Limb) -> Integer {
    let x_mod = x.abs.rem_limb_ref(p);
    let x_mod = if x.sign { x_mod } else { x_mod.mod_neg(p) };
    let s = r
        .mod_sub(x_mod, p)
        .mod_mul(product.rem_limb_ref(p).mod_inverse(p).unwrap(), p);
    let y = x + Integer::from(product * Natural::from(s));
    let new_product = product * Natural::from(p);
    // y > M/2 exactly when 2y > M.
    if y > 0u32 && new_product.cmp_double(y.unsigned_abs_ref()) == Less {
        y - Integer::from(new_product)
    } else {
        y
    }
}

// Returns the determinant of the square matrix `a`, given `d`, a divisor of it: the quotient
// det(A)/d is found by the Chinese remainder theorem from its values modulo primes that do not
// divide `d`, until their product exceeds twice a bound for its absolute value, unless a zero
// determinant is certified first. If `proved` is false, the computation stops once the quotient has
// been the same for primes whose product has more than 100 bits, so the result is only correct with
// high probability.
//
// This is equivalent to `fmpz_mat_det_modular_given_divisor` from
// `fmpz_mat/det_modular_given_divisor.c`, FLINT 3.6.0, except that `d` is nonnegative.
crate_test_fn! {
determinant_modular_given_divisor(a: &IntegerMatrix, d: &Natural, proved: bool) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    if n == 0 {
        return Integer::ONE;
    }
    if *d == 0u32 {
        return Integer::ZERO;
    }
    // A bound for |det(A)/d|, doubled to accommodate the sign.
    let bound = (determinant_bound_helper(a, true) << 1u32).div_round(d, Ceiling).0;
    let mut x = Integer::ZERO;
    let mut product = Natural::ONE;
    let mut stable_product = Natural::ZERO;
    let mut p = OPTIMAL_MODULUS_MIN;
    while product <= bound {
        // The next prime that does not divide d.
        let mut d_mod;
        loop {
            p = next_prime(p);
            d_mod = d.rem_limb_ref(p);
            if d_mod != 0 {
                break;
            }
        }
        let lu = WordLu::new(a, p);
        // Attempt to certify a zero determinant.
        if lu.rank != n && rank_certify_lu_mod_p(a, lu.rank, &lu.perm, &lu.pivots_nonpivots) {
            x = Integer::ZERO;
            break;
        }
        let x_mod = lu.determinant().mod_mul(d_mod.mod_inverse(p).unwrap(), p);
        let x_new = balanced_crt_word(&x, &product, x_mod, p);
        if x_new == x {
            stable_product *= Natural::from(p);
            if !proved && stable_product.significant_bits() > 100 {
                break;
            }
        } else {
            stable_product = Natural::from(p);
        }
        product *= Natural::from(p);
        x = x_new;
    }
    x * Integer::from(d)
}}

// Returns the determinant of the square matrix `a` with the multimodular algorithm, as
// `determinant_modular_given_divisor` does with the divisor 1.
//
// This is equivalent to `fmpz_mat_det_modular` from `fmpz_mat/det_modular.c`, FLINT 3.6.0.
crate_test_fn! {
determinant_modular(a: &IntegerMatrix, proved: bool) -> Integer {
    determinant_modular_given_divisor(a, &Natural::ONE, proved)
}}

// Returns the determinant of the square matrix `a` with the multimodular algorithm, given a large
// divisor of it found by `determinant_divisor`, so that fewer primes are needed.
//
// This is equivalent to `fmpz_mat_det_modular_accelerated` from
// `fmpz_mat/det_modular_accelerated.c`, FLINT 3.6.0.
crate_test_fn! {
determinant_modular_accelerated(a: &IntegerMatrix, proved: bool) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    determinant_modular_given_divisor(a, &determinant_divisor(a), proved)
}}

// Returns the determinant of the square matrix `a`: by cofactor expansion for dimensions up to 4,
// by Bareiss's elimination up to 24, by the multimodular algorithm up to 59, and above that by the
// multimodular algorithm with a divisor, unless the dimension is less than the maximum number of
// bits of the elements.
//
// This is equivalent to `fmpz_mat_det` from `fmpz_mat/det.c`, FLINT 3.6.0.
fn determinant_helper(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    if n <= COFACTOR_MAX_DIMENSION {
        determinant_cofactor(a)
    } else if n <= BAREISS_MAX_DIMENSION {
        determinant_bareiss(a)
    } else if n <= MODULAR_MAX_DIMENSION || u64::wrapping_from(n) < vec_max_bits(&a.elements).0 {
        determinant_modular(a, true)
    } else {
        determinant_modular_accelerated(a, true)
    }
}

impl Determinant for IntegerMatrix {
    type Output = Integer;

    /// Computes the determinant of a square [`IntegerMatrix`], taking it by value.
    ///
    /// Matrices of dimension at most 4 use cofactor expansion, and those up to 24 Bareiss's
    /// fraction-free elimination. Larger ones use a multimodular algorithm, which finds the
    /// determinant modulo word-sized primes and combines the results by the Chinese remainder
    /// theorem until the product of the primes exceeds twice a Hadamard bound; a zero determinant
    /// is recognized early by certifying a rank deficiency found modulo a prime. Above dimension
    /// 59, unless the dimension is less than the largest number of bits of an element, a large
    /// divisor of the determinant is first found by solving a linear system with Dixon's $p$-adic
    /// lifting, so that fewer primes are needed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n) \log (nb) \log\log (nb) + n^3 b (b + \log n))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of its elements: the determinant and the intermediate
    /// values of the elimination have $O(n(b + \log n))$ bits, and the multimodular algorithm uses
    /// $O(n(b + \log n))$ word-sized primes, each costing $O(n^3)$ word operations and $O(n^2)$
    /// reductions of elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Determinant;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "-2");
    ///
    /// let a =
    ///     IntegerMatrix::from_str("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "72");
    ///
    /// // A singular matrix
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_det` from `fmpz_mat/det.c`, FLINT 3.6.0.
    #[inline]
    fn determinant(self) -> Integer {
        determinant_helper(&self)
    }
}

impl Determinant for &IntegerMatrix {
    type Output = Integer;

    /// Computes the determinant of a square [`IntegerMatrix`], taking it by reference.
    ///
    /// Matrices of dimension at most 4 use cofactor expansion, and those up to 24 Bareiss's
    /// fraction-free elimination. Larger ones use a multimodular algorithm, which finds the
    /// determinant modulo word-sized primes and combines the results by the Chinese remainder
    /// theorem until the product of the primes exceeds twice a Hadamard bound; a zero determinant
    /// is recognized early by certifying a rank deficiency found modulo a prime. Above dimension
    /// 59, unless the dimension is less than the largest number of bits of an element, a large
    /// divisor of the determinant is first found by solving a linear system with Dixon's $p$-adic
    /// lifting, so that fewer primes are needed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n) \log (nb) \log\log (nb) + n^3 b (b + \log n))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $b$ is
    /// the maximum number of significant bits of its elements: the determinant and the intermediate
    /// values of the elimination have $O(n(b + \log n))$ bits, and the multimodular algorithm uses
    /// $O(n(b + \log n))$ word-sized primes, each costing $O(n^3)$ word operations and $O(n^2)$
    /// reductions of elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Determinant;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "-2");
    ///
    /// let a =
    ///     IntegerMatrix::from_str("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "72");
    ///
    /// // A singular matrix
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_det` from `fmpz_mat/det.c`, FLINT 3.6.0.
    #[inline]
    fn determinant(self) -> Integer {
        determinant_helper(self)
    }
}
