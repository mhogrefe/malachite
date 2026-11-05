// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::linear_algebra::similarity_transform::RationalRing;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExact, DivExactAssign, MulSubMulAssign, Reciprocal, SubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::linear_algebra::traits::{GramSchmidt, GramSchmidtAssign};
use malachite_base::num::number_theory::traits::LcmAssign;
use malachite_base::unsigned_matrix::linear_algebra::gram_schmidt_over_ring::GramSchmidtRing;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl GramSchmidtRing for RationalRing {
    type Element = Rational;

    #[inline]
    fn is_zero(&self, x: &Rational) -> bool {
        *x == 0u32
    }

    #[inline]
    fn mul(&self, x: &Rational, y: &Rational) -> Rational {
        x * y
    }

    #[inline]
    fn inverse(&self, x: &Rational) -> Option<Rational> {
        Some(x.reciprocal())
    }

    fn dot(&self, xs: &[Rational], ys: &[Rational]) -> Rational {
        let mut sum = Rational::ZERO;
        for (x, y) in xs.iter().zip(ys) {
            sum.add_mul_assign(x, y);
        }
        sum
    }

    fn sub_mul_assign(&self, xs: &mut [Rational], c: &Rational, ys: &[Rational]) {
        for (x, y) in xs.iter_mut().zip(ys) {
            x.sub_mul_assign(c, y);
        }
    }
}

fn integer_dot(xs: &[Integer], ys: &[Integer]) -> Integer {
    let mut sum = Integer::ZERO;
    for (x, y) in xs.iter().zip(ys) {
        sum.add_mul_assign(x, y);
    }
    sum
}

// The Gram-Schmidt process applied to the columns of the `height` by `width` matrix with elements
// `xs`, in row-major order, without normalizing them.
//
// Scaling a column by a nonzero number scales the same column of the result, and changes no other,
// so each column is first multiplied by the LCM of its elements' denominators, making it integral.
// The integral columns $a_1, a_2, \ldots$ are then orthogonalized without fractions: if $D_s$ is
// the determinant of the Gram matrix of the first $s$ nonzero columns of the result, and $b_s$ is
// the $s$th of them, then $c_s = D_{s - 1}b_s$ is integral, and so is $D_s$ times the residual of
// any integral vector after projecting out $b_1, \ldots, b_s$. A column $v = a_i$ is reduced
// against each earlier $c_s$ by $v \gets (D_s v - \lambda c_s) / D_{s - 1}$, where $\lambda =
// \langle v, c_s \rangle / D_{s - 1}$, both divisions being exact; it ends as $c = D_k b_i$, and if
// it is nonzero, $D_{k + 1} = \langle c, c \rangle / D_k$. Finally $b_i = c / D_k$ is divided by
// the scale.
//
// The result is that of `fmpq_mat_gso` from `fmpq_mat/gso.c`, FLINT 3.6.0, which works with
// fractions throughout.
fn gram_schmidt_elements(xs: Vec<Rational>, height: usize, width: usize) -> Vec<Rational> {
    if height == 0 || width == 0 {
        return xs;
    }
    let mut scales = vec![Natural::ONE; width];
    for (k, x) in xs.iter().enumerate() {
        scales[k % width].lcm_assign(&x.denominator);
    }
    // The integral columns, in column-major order.
    let mut cs = vec![Integer::ZERO; height * width];
    for (k, x) in xs.into_iter().enumerate() {
        let (i, j) = (k / width, k % width);
        let multiplier = (&scales[j]).div_exact(x.denominator);
        cs[j * height + i] = Integer::from_sign_and_abs(x.sign, x.numerator * multiplier);
    }
    // For each nonzero column of the result so far: its index s and D_{s - 1} and D_s.
    let mut independent: Vec<(usize, Integer, Integer)> = Vec::new();
    // For each column i: D_k, by which c is divided to get b_i.
    let mut divisors = Vec::with_capacity(width);
    for i in 0..width {
        let (done, rest) = cs.split_at_mut(i * height);
        let v = &mut rest[..height];
        for (s, previous, current) in &independent {
            let c_s = &done[s * height..(s + 1) * height];
            let lambda = integer_dot(v, c_s).div_exact(previous);
            for (x, c) in v.iter_mut().zip(c_s) {
                x.mul_sub_mul_assign(current, &lambda, c);
                x.div_exact_assign(previous);
            }
        }
        let d_k = independent.last().map_or(Integer::ONE, |t| t.2.clone());
        if i + 1 < width && v.iter().any(|x| *x != 0u32) {
            let d_next = integer_dot(v, v).div_exact(&d_k);
            independent.push((i, d_k.clone(), d_next));
        }
        divisors.push(d_k);
    }
    let mut out = Vec::with_capacity(height * width);
    for i in 0..height {
        for j in 0..width {
            out.push(Rational::from_integers(
                take(&mut cs[j * height + i]),
                &divisors[j] * Integer::from(&scales[j]),
            ));
        }
    }
    out
}

fn gram_schmidt_owned(a: RationalMatrix) -> RationalMatrix {
    let (height, width) = (a.height, a.width);
    RationalMatrix {
        elements: gram_schmidt_elements(a.elements, height, width),
        height,
        width,
    }
}

impl GramSchmidt for RationalMatrix {
    type Output = Self;

    /// Applies the Gram-Schmidt process to the columns of a [`RationalMatrix`], taking it by value,
    /// without normalizing them.
    ///
    /// Column $i$ of the result $B$ is column $i$ of the matrix $A$ minus its projections onto the
    /// earlier columns of $B$:
    /// $$
    /// b_i = a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j,
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal, but
    /// not normalized, since that would need square roots. A column that depends on the earlier
    /// ones becomes 0, and the later columns skip it, so the result has the shape of the matrix and
    /// its nonzero columns span the same space as the columns of the matrix. The process works on
    /// columns; the LLL algorithm usually works on rows.
    ///
    /// Scaling a column by a nonzero number scales the same column of the result and changes no
    /// other, so each column is first multiplied by the LCM of its denominators. The integral
    /// columns are then orthogonalized without fractions, by exact divisions by determinants of
    /// Gram matrices, and each column of the result is formed as a single fraction at the end.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(h^2 w^2 b (w + \log (hwb)) \log (hwb) \log\log (hwb))$
    ///
    /// $M(h, w, b) = O(h^2 w^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is the maximum number of significant bits of the numerators and denominators
    /// of its elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::GramSchmidt;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.gram_schmidt().to_string(), "[1, 1/2; 1, -1/2]");
    ///
    /// // The second column is twice the first, so it becomes 0
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 1; 0, 0, 5]").unwrap();
    /// assert_eq!(a.gram_schmidt().to_string(), "[1, 0, 2; 2, 0, -1; 0, 0, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_gso` from `fmpq_mat/gso.c`, FLINT 3.6.0, except that the
    /// result is returned rather than written to a matrix that must have the same shape, and is
    /// computed without fractions.
    #[inline]
    fn gram_schmidt(self) -> Self {
        gram_schmidt_owned(self)
    }
}

impl GramSchmidt for &RationalMatrix {
    type Output = RationalMatrix;

    /// Applies the Gram-Schmidt process to the columns of a [`RationalMatrix`], taking it by
    /// reference, without normalizing them.
    ///
    /// Column $i$ of the result $B$ is column $i$ of the matrix $A$ minus its projections onto the
    /// earlier columns of $B$:
    /// $$
    /// b_i = a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j,
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal, but
    /// not normalized, since that would need square roots. A column that depends on the earlier
    /// ones becomes 0, and the later columns skip it, so the result has the shape of the matrix and
    /// its nonzero columns span the same space as the columns of the matrix. The process works on
    /// columns; the LLL algorithm usually works on rows.
    ///
    /// Scaling a column by a nonzero number scales the same column of the result and changes no
    /// other, so each column is first multiplied by the LCM of its denominators. The integral
    /// columns are then orthogonalized without fractions, by exact divisions by determinants of
    /// Gram matrices, and each column of the result is formed as a single fraction at the end.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(h^2 w^2 b (w + \log (hwb)) \log (hwb) \log\log (hwb))$
    ///
    /// $M(h, w, b) = O(h^2 w^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is the maximum number of significant bits of the numerators and denominators
    /// of its elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::GramSchmidt;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).gram_schmidt().to_string(), "[1, 1/2; 1, -1/2]");
    ///
    /// // The second column is twice the first, so it becomes 0
    /// let a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 1; 0, 0, 5]").unwrap();
    /// assert_eq!(
    ///     (&a).gram_schmidt().to_string(),
    ///     "[1, 0, 2; 2, 0, -1; 0, 0, 5]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_gso` from `fmpq_mat/gso.c`, FLINT 3.6.0, except that the
    /// result is returned rather than written to a matrix that must have the same shape, and is
    /// computed without fractions.
    #[inline]
    fn gram_schmidt(self) -> RationalMatrix {
        gram_schmidt_owned(self.clone())
    }
}

impl GramSchmidtAssign for RationalMatrix {
    /// Replaces a [`RationalMatrix`] with the result of applying the Gram-Schmidt process to its
    /// columns, without normalizing them.
    ///
    /// Column $i$ of the result $B$ is column $i$ of the matrix $A$ minus its projections onto the
    /// earlier columns of $B$:
    /// $$
    /// b_i = a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j,
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal, but
    /// not normalized, since that would need square roots. A column that depends on the earlier
    /// ones becomes 0, and the later columns skip it, so the result has the shape of the matrix and
    /// its nonzero columns span the same space as the columns of the matrix. The process works on
    /// columns; the LLL algorithm usually works on rows.
    ///
    /// Scaling a column by a nonzero number scales the same column of the result and changes no
    /// other, so each column is first multiplied by the LCM of its denominators. The integral
    /// columns are then orthogonalized without fractions, by exact divisions by determinants of
    /// Gram matrices, and each column of the result is formed as a single fraction at the end.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(h^2 w^2 b (w + \log (hwb)) \log (hwb) \log\log (hwb))$
    ///
    /// $M(h, w, b) = O(h^2 w^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is the maximum number of significant bits of the numerators and denominators
    /// of its elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::GramSchmidtAssign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.gram_schmidt_assign();
    /// assert_eq!(a.to_string(), "[1, 1/2; 1, -1/2]");
    ///
    /// // The second column is twice the first, so it becomes 0
    /// let mut a = RationalMatrix::from_str("[1, 2, 3; 2, 4, 1; 0, 0, 5]").unwrap();
    /// a.gram_schmidt_assign();
    /// assert_eq!(a.to_string(), "[1, 0, 2; 2, 0, -1; 0, 0, 5]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_gso` from `fmpq_mat/gso.c`, FLINT 3.6.0, except that the
    /// result is returned rather than written to a matrix that must have the same shape, and is
    /// computed without fractions.
    #[inline]
    fn gram_schmidt_assign(&mut self) {
        let elements = take(&mut self.elements);
        self.elements = gram_schmidt_elements(elements, self.height, self.width);
    }
}
