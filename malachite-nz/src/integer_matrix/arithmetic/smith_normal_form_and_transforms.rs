// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2023 Jean Kieffer
//
//      Copyright © 2026 Edgar Costa
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::smith_normal_form::{
    SmithTransforms, snf_iterative_in_place,
};
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::SmithNormalFormAndTransforms;
use malachite_base::num::conversion::traits::ExactFrom;

// The Smith normal form S of `a`, which is m by n, with unimodular m by m and n by n matrices U and
// V such that UAV = S.
//
// This is equivalent to `fmpz_mat_snf_transform` from `fmpz_mat/snf_transform.c`, FLINT 3.6.0, with
// neither U nor V `NULL`.
fn snf_transforms_helper(a: &IntegerMatrix) -> (IntegerMatrix, IntegerMatrix, IntegerMatrix) {
    let mut transforms = SmithTransforms {
        u: IntegerMatrix::identity(u64::exact_from(a.height)),
        v: IntegerMatrix::identity(u64::exact_from(a.width)),
    };
    let mut s = a.clone();
    snf_iterative_in_place(&mut s, Some(&mut transforms));
    (s, transforms.u, transforms.v)
}

impl SmithNormalFormAndTransforms for IntegerMatrix {
    type Output = (Self, Self, Self);

    /// Computes the Smith normal form $S$ of an [`IntegerMatrix`] $A$, together with unimodular
    /// matrices $U$ and $V$ such that $UAV = S$, taking $A$ by value.
    ///
    /// The Smith normal form $S$ of an $h \times w$ matrix $A$ is described in
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm). The
    /// transforms $U$ and $V$ are $h \times h$ and $w \times w$ matrices with determinants $\pm 1$
    /// and $UAV = S$. They are not unique.
    ///
    /// Row and column Hermite normal forms, together with their transforms (see
    /// [`hermite_normal_form_and_transform`][hnft]), alternate until the matrix is diagonal. Then
    /// each pair of diagonal elements $a$ and $b$ that violates the divisibility condition is
    /// replaced with $\gcd(a, b)$ and $\operatorname{lcm}(a, b)$ by row and column operations built
    /// from Bézout's identity, and the rows of $U$ whose diagonal elements are negative are
    /// negated.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^5 (b + \log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: there are
    /// $O(n^2 (b + \log n))$ alternations, as for
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm), and each
    /// multiplies the transforms by a matrix whose elements have $O(n(b + \log n))$ bits, so that
    /// their elements have $O(n^3 (b + \log n)^2)$ bits.
    ///
    /// [hnft]: malachite_base::num::arithmetic::traits::HermiteNormalFormAndTransform
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalFormAndTransforms;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (s, u, v) = a.smith_normal_form_and_transforms();
    /// assert_eq!(s.to_string(), "[1, 0; 0, 2]");
    /// assert_eq!(u.to_string(), "[-2, 1; 3, -1]");
    /// assert_eq!(v.to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// let (s, u, v) = a.smith_normal_form_and_transforms();
    /// assert_eq!(s.to_string(), "[2, 0, 0; 0, 6, 0; 0, 0, 12]");
    /// assert_eq!(u.to_string(), "[1, 0, 0; -1, 3, 2; 3, -4, -3]");
    /// assert_eq!(v.to_string(), "[1, -2, -2; 0, 1, 0; 0, 0, 1]");
    ///
    /// // A matrix of rank 1
    /// let a = IntegerMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let (s, u, v) = a.smith_normal_form_and_transforms();
    /// assert_eq!(s.to_string(), "[1, 0; 0, 0]");
    /// assert_eq!(u.to_string(), "[0, 1; 1, -2]");
    /// assert_eq!(v.to_string(), "[1, 2; 0, -1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_snf_transform` from `fmpz_mat/snf_transform.c`, FLINT 3.6.0,
    /// with neither `U` nor `V` `NULL`, and gives FLINT's $U$ and $V$ except when FLINT's random
    /// prime in `fmpz_mat_hnf_transform` is unlucky.
    #[inline]
    fn smith_normal_form_and_transforms(self) -> (Self, Self, Self) {
        snf_transforms_helper(&self)
    }
}

impl SmithNormalFormAndTransforms for &IntegerMatrix {
    type Output = (IntegerMatrix, IntegerMatrix, IntegerMatrix);

    /// Computes the Smith normal form $S$ of an [`IntegerMatrix`] $A$, together with unimodular
    /// matrices $U$ and $V$ such that $UAV = S$, taking $A$ by reference.
    ///
    /// The Smith normal form $S$ of an $h \times w$ matrix $A$ is described in
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm). The
    /// transforms $U$ and $V$ are $h \times h$ and $w \times w$ matrices with determinants $\pm 1$
    /// and $UAV = S$. They are not unique.
    ///
    /// Row and column Hermite normal forms, together with their transforms (see
    /// [`hermite_normal_form_and_transform`][hnft]), alternate until the matrix is diagonal. Then
    /// each pair of diagonal elements $a$ and $b$ that violates the divisibility condition is
    /// replaced with $\gcd(a, b)$ and $\operatorname{lcm}(a, b)$ by row and column operations built
    /// from Bézout's identity, and the rows of $U$ whose diagonal elements are negative are
    /// negated.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^5 (b + \log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: there are
    /// $O(n^2 (b + \log n))$ alternations, as for
    /// [`smith_normal_form`](malachite_base::num::arithmetic::traits::SmithNormalForm), and each
    /// multiplies the transforms by a matrix whose elements have $O(n(b + \log n))$ bits, so that
    /// their elements have $O(n^3 (b + \log n)^2)$ bits.
    ///
    /// [hnft]: malachite_base::num::arithmetic::traits::HermiteNormalFormAndTransform
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalFormAndTransforms;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (s, u, v) = (&a).smith_normal_form_and_transforms();
    /// assert_eq!(s.to_string(), "[1, 0; 0, 2]");
    /// assert_eq!(u.to_string(), "[-2, 1; 3, -1]");
    /// assert_eq!(v.to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// let (s, u, v) = (&a).smith_normal_form_and_transforms();
    /// assert_eq!(s.to_string(), "[2, 0, 0; 0, 6, 0; 0, 0, 12]");
    /// assert_eq!(u.to_string(), "[1, 0, 0; -1, 3, 2; 3, -4, -3]");
    /// assert_eq!(v.to_string(), "[1, -2, -2; 0, 1, 0; 0, 0, 1]");
    ///
    /// // A matrix of rank 1
    /// let a = IntegerMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// let (s, u, v) = (&a).smith_normal_form_and_transforms();
    /// assert_eq!(s.to_string(), "[1, 0; 0, 0]");
    /// assert_eq!(u.to_string(), "[0, 1; 1, -2]");
    /// assert_eq!(v.to_string(), "[1, 2; 0, -1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_snf_transform` from `fmpz_mat/snf_transform.c`, FLINT 3.6.0,
    /// with neither `U` nor `V` `NULL`, and gives FLINT's $U$ and $V$ except when FLINT's random
    /// prime in `fmpz_mat_hnf_transform` is unlucky.
    #[inline]
    fn smith_normal_form_and_transforms(self) -> (IntegerMatrix, IntegerMatrix, IntegerMatrix) {
        snf_transforms_helper(self)
    }
}
