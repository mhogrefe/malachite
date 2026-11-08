// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_vector::IntegerVector;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, GeneralizedCross};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{Cofactor, Nullspace};

// Panics unless the `n - 1` vectors all have dimension `n`, and returns `n`.
pub(crate) fn generalized_cross_dimension(xs: &[IntegerVector]) -> usize {
    let n = xs.len() + 1;
    assert!(
        xs.iter().all(|x| x.elements.len() == n),
        "the generalized cross product of {} vectors is only defined for vectors of dimension {n}",
        xs.len()
    );
    n
}

// The `n`-by-`n` matrix whose first `n - 1` rows are the vectors and whose last row is zero.
// Element `i` of the generalized cross product of the vectors is its cofactor in row `n - 1` and
// column `i`.
pub(crate) fn square_with_zero_last_row(xs: &[IntegerVector], n: usize) -> IntegerMatrix {
    let mut elements = Vec::with_capacity(n * n);
    for x in xs {
        elements.extend_from_slice(&x.elements);
    }
    elements.resize(n * n, Integer::ZERO);
    let n = u64::exact_from(n);
    IntegerMatrix::from_owned_elements(n, n, elements)
}

// The generalized cross product w of the vectors. w is in the nullspace of the matrix whose rows
// are the vectors. If they are linearly independent, the nullspace is spanned by a single vector x,
// and w is the multiple of x whose element k, for any k with x_k nonzero, is the cofactor that
// defines it; otherwise, w is zero.
crate_test_fn! {
generalized_cross_by_nullspace(xs: &[IntegerVector]) -> Vec<Integer> {
    let n = generalized_cross_dimension(xs);
    let a = square_with_zero_last_row(xs, n);
    let last = u64::exact_from(n - 1);
    let nullspace = a.window(0..last, 0..last + 1).nullspace();
    if nullspace.width() != 1 {
        return vec![Integer::ZERO; n];
    }
    let x = nullspace.into_elements();
    let k = x.iter().position(|e| *e != 0u32).unwrap();
    let wk = a.cofactor(last, u64::exact_from(k));
    let xk = &x[k];
    // w_i = (w_k / x_k) x_i
    if wk == *xk {
        x
    } else if -&wk == *xk {
        x.into_iter().map(|e| -e).collect()
    } else {
        x.iter().map(|e| (e * &wk).div_exact(xk)).collect()
    }
}}

impl GeneralizedCross for IntegerVector {
    /// Computes the generalized cross product of $n - 1$ [`IntegerVector`]s of dimension $n$.
    ///
    /// $$
    /// f(v_0, \ldots, v_{n - 2})_i = (-1)^{n - 1 + i} \det A_{(i)},
    /// $$
    ///
    /// where $A$ is the matrix whose rows are the vectors and $A_{(i)}$ is $A$ without column $i$.
    /// The result $w$ satisfies $w \cdot x = \det \begin{pmatrix} A \\ x \end{pmatrix}$ for every
    /// $x$: it is perpendicular to each vector, its length is the $(n - 1)$-dimensional volume of
    /// the parallelotope they span, and the vectors followed by $w$ are positively oriented, since
    /// $\det \begin{pmatrix} A \\ w \end{pmatrix} = |w|^2 \geq 0$. It is zero exactly when the
    /// vectors are linearly dependent. For $n = 2$ it rotates the vector a quarter turn
    /// counterclockwise, $(a, b) \mapsto (-b, a)$; for $n = 3$ it is the cross product; and no
    /// vectors give $(1)$.
    ///
    /// The result is found from the [nullspace](Nullspace::nullspace) of $A$ and a single
    /// [cofactor](Cofactor::cofactor), rather than as $n$ separate cofactors.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 b \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension, and $b$ is the maximum
    /// number of significant bits of the elements.
    ///
    /// # Panics
    /// Panics if the vectors do not all have dimension one more than their number.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::GeneralizedCross;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = |s| IntegerVector::from_str(s).unwrap();
    /// // a quarter turn counterclockwise
    /// assert_eq!(
    ///     IntegerVector::generalized_cross(&[v("(3, 4)")]).to_string(),
    ///     "(-4, 3)"
    /// );
    /// // the ordinary cross product
    /// assert_eq!(
    ///     IntegerVector::generalized_cross(&[v("(1, 2, 3)"), v("(4, 5, 6)")]).to_string(),
    ///     "(-3, 6, -3)"
    /// );
    /// assert_eq!(
    ///     IntegerVector::generalized_cross(&[
    ///         v("(1, 2, 3, 4)"),
    ///         v("(0, 1, -1, 2)"),
    ///         v("(2, 0, 1, 1)")
    ///     ])
    ///     .to_string(),
    ///     "(5, 17, -1, -9)"
    /// );
    /// assert_eq!(
    ///     IntegerVector::generalized_cross(&[
    ///         v("(1, 0, 0, 0)"),
    ///         v("(0, 1, 0, 0)"),
    ///         v("(0, 0, 1, 0)")
    ///     ])
    ///     .to_string(),
    ///     "(0, 0, 0, 1)"
    /// );
    /// assert_eq!(IntegerVector::generalized_cross(&[]).to_string(), "(1)");
    /// ```
    #[inline]
    fn generalized_cross(xs: &[Self]) -> Self {
        Self {
            elements: generalized_cross_by_nullspace(xs),
        }
    }
}
