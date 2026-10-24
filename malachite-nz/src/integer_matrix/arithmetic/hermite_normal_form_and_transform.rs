// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 Alex J. Best
//
//      Copyright © 2017 Tommy Hofmann
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::hermite_normal_form::hnf_minors_in_place;
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    HermiteNormalForm, HermiteNormalFormAndTransform, Rank,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

// The Hermite normal form H of `a`, which is m by n, with a unimodular m by m matrix U such that UA
// = H, both read off from the Hermite normal form of [A | I].
//
// This is equivalent to `_fmpz_mat_hnf_transform_naive` from `fmpz_mat/hnf_transform.c`, FLINT
// 3.6.0.
crate_test_fn! {
hnf_transform_naive(a: &IntegerMatrix) -> (IntegerMatrix, IntegerMatrix) {
    let (m, n) = (a.height, a.width);
    if m == 0 {
        return (a.clone(), IntegerMatrix::identity(0));
    }
    let w = n + m;
    // Append an identity matrix to the right of A.
    let mut xs = vec![Integer::ZERO; m * w];
    for i in 0..m {
        xs[i * w..i * w + n].clone_from_slice(&a.elements[i * n..(i + 1) * n]);
        xs[i * w + n + i] = Integer::ONE;
    }
    let h2 = IntegerMatrix {
        elements: xs,
        height: m,
        width: w,
    }
    .hermite_normal_form();
    // Recover H and U.
    let mut h = Vec::with_capacity(m * n);
    let mut u = Vec::with_capacity(m * m);
    for row in h2.elements.chunks_exact(w) {
        h.extend_from_slice(&row[..n]);
        u.extend_from_slice(&row[n..]);
    }
    (
        IntegerMatrix {
            elements: h,
            height: m,
            width: n,
        },
        IntegerMatrix {
            elements: u,
            height: m,
            width: m,
        },
    )
}}

// The Hermite normal form H of `a`, which is m by n with rank n, with a unimodular m by m matrix U
// such that UA = H, by the algorithm of Kannan and Bachem.
//
// This is equivalent to `fmpz_mat_hnf_minors_transform` from `fmpz_mat/hnf_minors_transform.c`,
// FLINT 3.6.0.
crate_test_fn! {
hnf_minors_transform(a: &IntegerMatrix) -> (IntegerMatrix, IntegerMatrix) {
    let mut h = a.clone();
    let mut u = IntegerMatrix::identity(u64::exact_from(a.height));
    hnf_minors_in_place(&mut h, Some(&mut u));
    (h, u)
}}

// The Hermite normal form H of `a`, which is m by n, with a unimodular m by m matrix U such that UA
// = H: by the algorithm of Kannan and Bachem when `a` has full column rank, and from the Hermite
// normal form of [A | I] otherwise.
//
// This is equivalent to `fmpz_mat_hnf_transform` from `fmpz_mat/hnf_transform.c`, FLINT 3.6.0,
// except that the prime modulo which the rank is first found is the first one above $2^{W-5}$,
// where $W$ is the limb width, rather than a random one, and that a rank deficiency modulo the
// prime is confirmed over the integers.
fn hnf_transform_helper(a: &IntegerMatrix) -> (IntegerMatrix, IntegerMatrix) {
    let (m, n) = (a.height, a.width);
    if n > m {
        return hnf_transform_naive(a);
    }
    let full_rank = m == 0
        || n == 0
        || WordLu::new(a, next_prime(OPTIMAL_MODULUS_MIN)).rank == n
        || usize::exact_from(a.rank()) == n;
    if full_rank {
        hnf_minors_transform(a)
    } else {
        hnf_transform_naive(a)
    }
}

impl HermiteNormalFormAndTransform for IntegerMatrix {
    type Output = (Self, Self);

    /// Computes the Hermite normal form $H$ of an [`IntegerMatrix`] $A$, together with a unimodular
    /// matrix $U$ such that $UA = H$, taking $A$ by value.
    ///
    /// The Hermite normal form $H$ of an $h \times w$ matrix $A$ is described in
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm). The
    /// transformation $U$ is an $h \times h$ matrix with determinant $\pm 1$ and $UA = H$. It is
    /// unique when the rank of $A$ is $h$, but not otherwise: the rows of $U$ that give the zero
    /// rows of $H$ may be any basis of the left nullspace of $A$, and they may be added to the
    /// others.
    ///
    /// When $A$ has full column rank, $H$ and $U$ are found together by the algorithm of Kannan and
    /// Bachem, which puts the leading principal minors into Hermite normal form in turn. Otherwise
    /// they are read off from the Hermite normal form $[H \mid U]$ of $[A \mid I]$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^5 (b + \log n) \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: $[A \mid I]$
    /// has $O(n)$ columns, and its Hermite normal form is found within the bounds of
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm), which
    /// hold for the classical algorithm only because it is used for a bounded number of rows.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HermiteNormalFormAndTransform;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (h, u) = a.hermite_normal_form_and_transform();
    /// assert_eq!(h.to_string(), "[1, 0; 0, 2]");
    /// assert_eq!(u.to_string(), "[-2, 1; 3, -1]");
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// let (h, u) = a.hermite_normal_form_and_transform();
    /// assert_eq!(h.to_string(), "[1, 1; 0, 2; 0, 0]");
    /// assert_eq!(u.to_string(), "[607, -260, -289; -34, 15, 16; -21, 9, 10]");
    ///
    /// // A matrix of rank 1
    /// let a = IntegerMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let (h, u) = a.hermite_normal_form_and_transform();
    /// assert_eq!(h.to_string(), "[1, 2; 0, 0]");
    /// assert_eq!(u.to_string(), "[0, 1; 1, -2]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_hnf_transform` from `fmpz_mat/hnf_transform.c`, FLINT 3.6.0,
    /// except that a rank deficiency found modulo a prime is confirmed over the integers before $[A
    /// \mid I]$ is used, so that $U$ is the one FLINT gives for the same $A$ except when FLINT's
    /// random prime is unlucky.
    #[inline]
    fn hermite_normal_form_and_transform(self) -> (Self, Self) {
        hnf_transform_helper(&self)
    }
}

impl HermiteNormalFormAndTransform for &IntegerMatrix {
    type Output = (IntegerMatrix, IntegerMatrix);

    /// Computes the Hermite normal form $H$ of an [`IntegerMatrix`] $A$, together with a unimodular
    /// matrix $U$ such that $UA = H$, taking $A$ by reference.
    ///
    /// The Hermite normal form $H$ of an $h \times w$ matrix $A$ is described in
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm). The
    /// transformation $U$ is an $h \times h$ matrix with determinant $\pm 1$ and $UA = H$. It is
    /// unique when the rank of $A$ is $h$, but not otherwise: the rows of $U$ that give the zero
    /// rows of $H$ may be any basis of the left nullspace of $A$, and they may be added to the
    /// others.
    ///
    /// When $A$ has full column rank, $H$ and $U$ are found together by the algorithm of Kannan and
    /// Bachem, which puts the leading principal minors into Hermite normal form in turn. Otherwise
    /// they are read off from the Hermite normal form $[H \mid U]$ of $[A \mid I]$.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^5 (b + \log n) \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: $[A \mid I]$
    /// has $O(n)$ columns, and its Hermite normal form is found within the bounds of
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm), which
    /// hold for the classical algorithm only because it is used for a bounded number of rows.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HermiteNormalFormAndTransform;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (h, u) = (&a).hermite_normal_form_and_transform();
    /// assert_eq!(h.to_string(), "[1, 0; 0, 2]");
    /// assert_eq!(u.to_string(), "[-2, 1; 3, -1]");
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// let (h, u) = (&a).hermite_normal_form_and_transform();
    /// assert_eq!(h.to_string(), "[1, 1; 0, 2; 0, 0]");
    /// assert_eq!(u.to_string(), "[607, -260, -289; -34, 15, 16; -21, 9, 10]");
    ///
    /// // A matrix of rank 1
    /// let a = IntegerMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let (h, u) = (&a).hermite_normal_form_and_transform();
    /// assert_eq!(h.to_string(), "[1, 2; 0, 0]");
    /// assert_eq!(u.to_string(), "[0, 1; 1, -2]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_hnf_transform` from `fmpz_mat/hnf_transform.c`, FLINT 3.6.0,
    /// except that a rank deficiency found modulo a prime is confirmed over the integers before $[A
    /// \mid I]$ is used, so that $U$ is the one FLINT gives for the same $A$ except when FLINT's
    /// random prime is unlucky.
    #[inline]
    fn hermite_normal_form_and_transform(self) -> (IntegerMatrix, IntegerMatrix) {
        hnf_transform_helper(self)
    }
}
