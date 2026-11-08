// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_vector::RationalVector;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::GeneralizedCross;
use malachite_base::num::basic::traits::One;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

// Panics unless the `n - 1` vectors all have dimension `n`, and returns `n`.
pub(crate) fn generalized_cross_dimension(xs: &[RationalVector]) -> usize {
    let n = xs.len() + 1;
    assert!(
        xs.iter().all(|x| x.elements.len() == n),
        "the generalized cross product of {} vectors is only defined for vectors of dimension {n}",
        xs.len()
    );
    n
}

impl GeneralizedCross for RationalVector {
    /// Computes the generalized cross product of $n - 1$ [`RationalVector`]s of dimension $n$.
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
    /// Each vector is multiplied by the least common multiple of its denominators, the generalized
    /// cross product of the resulting integer vectors is found, and it is divided by the product of
    /// those multipliers.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of
    /// [`IntegerVector::generalized_cross`](malachite_nz::integer_vector::IntegerVector) for
    /// vectors whose elements have $O(nb)$ bits, where $n$ is the dimension and $b$ is the maximum
    /// number of significant bits of the numerators and denominators of the elements.
    ///
    /// # Panics
    /// Panics if the vectors do not all have dimension one more than their number.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::GeneralizedCross;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = |s| RationalVector::from_str(s).unwrap();
    /// assert_eq!(
    ///     RationalVector::generalized_cross(&[v("(1/2, 3)")]).to_string(),
    ///     "(-3, 1/2)"
    /// );
    /// assert_eq!(
    ///     RationalVector::generalized_cross(&[v("(1/2, 0, 0)"), v("(0, 1/3, 0)")]).to_string(),
    ///     "(0, 0, 1/6)"
    /// );
    /// assert_eq!(RationalVector::generalized_cross(&[]).to_string(), "(1)");
    /// ```
    fn generalized_cross(xs: &[Self]) -> Self {
        generalized_cross_dimension(xs);
        if xs.is_empty() {
            return Self {
                elements: vec![Rational::ONE],
            };
        }
        let (numerators, denominators) =
            RationalMatrix::from_rows(xs).to_numerators_and_row_denominators();
        let rows: Vec<IntegerVector> = numerators
            .rows()
            .map(|row| IntegerVector::from_elements(row.as_slice()))
            .collect();
        let d = Integer::from(denominators.elements.into_iter().product::<Natural>());
        Self {
            elements: IntegerVector::generalized_cross(&rows)
                .elements
                .iter()
                .map(|e| Rational::from_integers_ref(e, &d))
                .collect(),
        }
    }
}
