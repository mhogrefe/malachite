// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::determinant::determinant_helper;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DeterminantSign, Parity, PowerOf2, Sign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::OrdProducts;
use malachite_base::num::conversion::traits::{
    ExactFrom, IntegerMantissaAndExponent, SciMantissaAndExponent,
};
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};

// The largest mean number of significant bits of the elements for which computing the determinant
// of a matrix of dimension `n`, at least 3, is usually faster than certifying its sign by
// preconditioning. Found by benchmarking matrices with pseudorandom elements, both of uniform size
// and of widely varying sizes.
const fn exact_max_bits(n: usize) -> u64 {
    match n {
        3 => 1024,
        4 => 512,
        5 => 192,
        6 => 48,
        7 => 24,
        8 | 9 => 16,
        10 => 12,
        11..=14 => 8,
        15 | 16 => 6,
        17..=22 => 4,
        23 | 24 => 3,
        25..=32 => 2,
        33..=48 => 1,
        _ => 0,
    }
}

// The number of bits after the binary point kept in the fixed-point preconditioner.
const PRECONDITIONER_BITS: i64 = 53;

// An approximation to `x / 2^bits` as an `f64`, where `bits` is at least the number of significant
// bits of `x`; parts too small for the exponent range of an `f64` become 0.
fn scaled_to_f64(x: &Integer, bits: u64) -> f64 {
    if *x == 0u32 {
        return 0.0;
    }
    let (mantissa, exponent): (f64, u64) = (&x.abs).sci_mantissa_and_exponent();
    let k = i64::exact_from(exponent) - i64::exact_from(bits);
    let y = if k < f64::MIN_EXPONENT {
        0.0
    } else {
        mantissa * f64::power_of_2(k)
    };
    if x.sign { y } else { -y }
}

// An integer approximation to `x * 2^shift`, where `x` is finite.
fn fixed_point(x: f64, shift: i64) -> Integer {
    if x == 0.0 {
        return Integer::ZERO;
    }
    let (mantissa, exponent) = x.abs().integer_mantissa_and_exponent();
    let y = Integer::from(mantissa) << (exponent + shift);
    if x > 0.0 { y } else { -y }
}

// The LU decomposition with partial pivoting of the `n` by `n` row-major matrix `b`, in place and
// in floating point: afterwards the strictly lower part of `b` holds L, whose diagonal is all ones,
// and the rest holds U. Returns the row permutation and whether it is odd, or `None` if a pivot is
// zero or not finite.
fn approximate_lu(b: &mut [f64], n: usize) -> Option<(Vec<usize>, bool)> {
    let mut perm: Vec<usize> = (0..n).collect();
    let mut odd = false;
    for k in 0..n {
        let mut p = k;
        let mut max = b[k * n + k].abs();
        for i in k + 1..n {
            let y = b[i * n + k].abs();
            if y > max {
                max = y;
                p = i;
            }
        }
        if max == 0.0 || !max.is_finite() {
            return None;
        }
        if p != k {
            for j in 0..n {
                b.swap(k * n + j, p * n + j);
            }
            perm.swap(k, p);
            odd.not_assign();
        }
        let pivot = b[k * n + k];
        for i in k + 1..n {
            let f = b[i * n + k] / pivot;
            b[i * n + k] = f;
            for j in k + 1..n {
                b[i * n + j] -= f * b[k * n + j];
            }
        }
    }
    Some((perm, odd))
}

// Approximate inverses of the unit lower triangular L and the upper triangular U stored together in
// `lu`, as row-major matrices in floating point. Returns `None` if any element is not finite.
fn approximate_triangular_inverses(lu: &[f64], n: usize) -> Option<(Vec<f64>, Vec<f64>)> {
    let mut l_inverse = vec![0.0; n * n];
    for j in 0..n {
        l_inverse[j * n + j] = 1.0;
        for i in j + 1..n {
            let mut s = 0.0;
            for k in j..i {
                s -= lu[i * n + k] * l_inverse[k * n + j];
            }
            l_inverse[i * n + j] = s;
        }
    }
    let mut u_inverse = vec![0.0; n * n];
    for j in (0..n).rev() {
        u_inverse[j * n + j] = 1.0 / lu[j * n + j];
        for i in (0..j).rev() {
            let mut s = 0.0;
            for k in i + 1..=j {
                s -= lu[i * n + k] * u_inverse[k * n + j];
            }
            u_inverse[i * n + j] = s / lu[i * n + i];
        }
    }
    if l_inverse.iter().chain(&u_inverse).all(|x| x.is_finite()) {
        Some((l_inverse, u_inverse))
    } else {
        None
    }
}

// Whether the `n` by `n` row-major matrix `c` looks strictly diagonally dominant with a positive
// diagonal, in floating point and with a margin of 2: a cheap prediction of whether the exact check
// will succeed.
fn approximately_dominant(c: &[f64], n: usize) -> bool {
    c.chunks_exact(n).enumerate().all(|(i, row)| {
        let diagonal = row[i];
        let off_diagonal: f64 = row
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, z)| z.abs())
            .sum();
        diagonal > 0.0 && off_diagonal < 0.5 * diagonal
    })
}

// The product of two `n` by `n` row-major matrices in floating point.
fn f64_matrix_mul(xs: &[f64], ys: &[f64], n: usize) -> Vec<f64> {
    let mut zs = vec![0.0; n * n];
    for i in 0..n {
        for k in 0..n {
            let x = xs[i * n + k];
            if x != 0.0 {
                for j in 0..n {
                    zs[i * n + j] += x * ys[k * n + j];
                }
            }
        }
    }
    zs
}

// The sign of the determinant of the square matrix `a`, of dimension at least 1 and with no zero
// row or column, certified by preconditioning, or `None` if the certificate fails.
//
// Let R and S be diagonal matrices of powers of 2 that equilibrate `a`: R brings the largest
// element of each row near 1, and then S does the same for each column of RA. A floating-point LU
// decomposition with partial pivoting, P(RAS) ≈ LU, gives approximate inverses X ≈ L⁻¹ and Y
// ≈ U⁻¹. If, in floating point, YXP(RAS) does not look strictly diagonally dominant, the
// certificate is abandoned before any exact arithmetic. Otherwise X and Y are rounded to integer
// matrices X' and Y': X' with every diagonal element 2^53, and Y' with each row scaled by a power
// of 2 so that its diagonal element is near ±2^53. Then C = Y'X'P(2^e R)A, where 2^e R is a
// diagonal matrix of positive powers of 2, is computed exactly, and CS is close to a diagonal
// matrix with positive diagonal unless `a` is ill-conditioned. If CS is strictly diagonally
// dominant with a positive diagonal, which is checked exactly by weighting the columns of C, then
// by Gershgorin's theorem every real eigenvalue of CS is positive, and the others come in conjugate
// pairs, so det(CS) > 0. Since det(X'), det(2^e R), and det(S) are positive, the sign of det(A) is
// then the sign of the permutation P times the signs of the diagonal elements of Y'. Only that
// exact check decides; the floating-point steps merely choose X' and Y' and predict the outcome, so
// their rounding errors cannot make the result wrong, only make the certificate fail.
crate_test_fn! {
determinant_sign_preconditioned(a: &IntegerMatrix) -> Option<Ordering> {
    let n = a.height;
    let row_bits: Vec<u64> = a
        .elements
        .chunks_exact(n)
        .map(|row| vec_max_bits(row).0)
        .collect();
    let mut b: Vec<f64> = a
        .elements
        .chunks_exact(n)
        .zip(&row_bits)
        .flat_map(|(row, &bits)| row.iter().map(move |x| scaled_to_f64(x, bits)))
        .collect();
    // The column weights: column j of the row-scaled matrix is multiplied by 2^weights[j].
    let weights: Vec<i64> = (0..n)
        .map(|j| {
            (0..n)
                .map(|i| b[i * n + j])
                .filter(|&x| x != 0.0)
                .map(|x| -x.abs().sci_exponent())
                .min()
                .map_or(0, |w| w.min(1000))
        })
        .collect();
    for row in b.chunks_exact_mut(n) {
        for (x, &w) in row.iter_mut().zip(&weights) {
            *x *= f64::power_of_2(w);
        }
    }
    let original = b.clone();
    let (perm, odd) = approximate_lu(&mut b, n)?;
    let (l_inverse, u_inverse) = approximate_triangular_inverses(&b, n)?;
    let permuted: Vec<f64> = perm
        .iter()
        .flat_map(|&i| original[i * n..(i + 1) * n].iter().copied())
        .collect();
    let product = f64_matrix_mul(&u_inverse, &f64_matrix_mul(&l_inverse, &permuted, n), n);
    if !approximately_dominant(&product, n) {
        return None;
    }
    let mut x = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            x.push(match i.cmp(&j) {
                Less => Integer::ZERO,
                Equal => Integer::power_of_2(const { PRECONDITIONER_BITS as u64 }),
                Greater => fixed_point(l_inverse[i * n + j], PRECONDITIONER_BITS),
            });
        }
    }
    let mut y = Vec::with_capacity(n * n);
    let mut negative_diagonal_count = 0u64;
    for i in 0..n {
        let diagonal = u_inverse[i * n + i];
        if diagonal < 0.0 {
            negative_diagonal_count += 1;
        }
        let shift = PRECONDITIONER_BITS - diagonal.abs().sci_exponent();
        y.extend((0..i).map(|_| Integer::ZERO));
        y.extend((i..n).map(|j| fixed_point(u_inverse[i * n + j], shift)));
    }
    let max_row_bits = *row_bits.iter().max().unwrap();
    let mut scaled = Vec::with_capacity(n * n);
    for &i in &perm {
        let shift = max_row_bits - row_bits[i];
        scaled.extend(a.elements[i * n..(i + 1) * n].iter().map(|x| x << shift));
    }
    let size = u64::exact_from(n);
    let x = IntegerMatrix::from_owned_elements(size, size, x);
    let y = IntegerMatrix::from_owned_elements(size, size, y);
    let scaled = IntegerMatrix::from_owned_elements(size, size, scaled);
    let c = &y * &(&x * &scaled);
    // Row i of CS is strictly dominated by its diagonal element exactly when the sum over j ≠ i
    // of |c_ij| 2^(w_j - w) is less than c_ii 2^(w_i - w), where w is the smallest weight.
    let min_weight = *weights.iter().min().unwrap();
    let shifts: Vec<u64> = weights
        .iter()
        .map(|&w| u64::exact_from(w - min_weight))
        .collect();
    for (i, row) in c.elements.chunks_exact(n).enumerate() {
        let diagonal = &row[i];
        if *diagonal <= 0u32 {
            return None;
        }
        let off_diagonal: Natural = row
            .iter()
            .zip(&shifts)
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, (z, &shift))| &z.abs << shift)
            .sum();
        if off_diagonal >= &diagonal.abs << shifts[i] {
            return None;
        }
    }
    Some(if odd == negative_diagonal_count.odd() {
        Greater
    } else {
        Less
    })
}}

// Returns the sign of the determinant of the square matrix `a`. Dimensions up to 2 are handled
// directly, dimension 2 by `Integer::cmp_products`. A zero row or column gives 0, and a triangular
// matrix the product of the signs of its diagonal. If the elements are small enough on average for
// the dimension, the determinant is computed. Otherwise the sign is certified by preconditioning,
// and if that fails the determinant is computed.
pub(crate) fn determinant_sign_helper(a: &IntegerMatrix) -> Ordering {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let xs = &a.elements;
    match n {
        0 => return Greater,
        1 => return xs[0].sign(),
        2 => return Integer::cmp_products(&xs[0], &xs[3], &xs[1], &xs[2]),
        _ => {}
    }
    if xs.chunks_exact(n).any(|row| row.iter().all(|x| *x == 0u32))
        || (0..n).any(|j| (0..n).all(|i| xs[i * n + j] == 0u32))
    {
        return Equal;
    }
    if a.is_upper_triangular() || a.is_lower_triangular() {
        let negative_count = (0..n).filter(|&i| xs[i * n + i] < 0u32).count();
        return if (0..n).any(|i| xs[i * n + i] == 0u32) {
            Equal
        } else if negative_count.even() {
            Greater
        } else {
            Less
        };
    }
    let total_bits: u64 = xs.iter().map(SignificantBits::significant_bits).sum();
    if total_bits / u64::exact_from(n * n) <= exact_max_bits(n) {
        return determinant_helper(a).sign();
    }
    determinant_sign_preconditioned(a).unwrap_or_else(|| determinant_helper(a).sign())
}

impl DeterminantSign for IntegerMatrix {
    /// Determines the sign of the determinant of a square [`IntegerMatrix`], taking it by value.
    ///
    /// The result is the ordering of the determinant against 0. For dimension 2 it is
    /// [`Integer::cmp_products`](malachite_base::num::comparison::traits::OrdProducts) on the two
    /// diagonals, so the products are formed only if cheaper checks cannot decide. A zero row or
    /// column gives `Equal`, and a triangular matrix gives the product of the signs of its diagonal
    /// elements. If the elements are small enough on average for the dimension that computing the
    /// determinant is usually faster, it is computed. Otherwise the sign is certified without
    /// computing the determinant: an approximate LU decomposition in floating point yields a
    /// preconditioner, the matrix is multiplied by it exactly, and if the product is strictly
    /// diagonally dominant with a positive diagonal, the sign follows from the preconditioner
    /// alone. This succeeds unless the matrix is singular or ill-conditioned, and then the
    /// determinant is computed.
    ///
    /// $$
    /// f(A) = \operatorname{sgn}(\det A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DeterminantSign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.determinant_sign(), Less);
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; -7, 8, 10]").unwrap();
    /// assert_eq!(a.determinant_sign(), Greater);
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.determinant_sign(), Equal);
    /// ```
    #[inline]
    fn determinant_sign(self) -> Ordering {
        determinant_sign_helper(&self)
    }
}

impl DeterminantSign for &IntegerMatrix {
    /// Determines the sign of the determinant of a square [`IntegerMatrix`], taking it by
    /// reference.
    ///
    /// The result is the ordering of the determinant against 0. For dimension 2 it is
    /// [`Integer::cmp_products`](malachite_base::num::comparison::traits::OrdProducts) on the two
    /// diagonals, so the products are formed only if cheaper checks cannot decide. A zero row or
    /// column gives `Equal`, and a triangular matrix gives the product of the signs of its diagonal
    /// elements. If the elements are small enough on average for the dimension that computing the
    /// determinant is usually faster, it is computed. Otherwise the sign is certified without
    /// computing the determinant: an approximate LU decomposition in floating point yields a
    /// preconditioner, the matrix is multiplied by it exactly, and if the product is strictly
    /// diagonally dominant with a positive diagonal, the sign follows from the preconditioner
    /// alone. This succeeds unless the matrix is singular or ill-conditioned, and then the
    /// determinant is computed.
    ///
    /// $$
    /// f(A) = \operatorname{sgn}(\det A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DeterminantSign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Less);
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; -7, 8, 10]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Greater);
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Equal);
    /// ```
    #[inline]
    fn determinant_sign(self) -> Ordering {
        determinant_sign_helper(self)
    }
}
