// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::linear_algebra::traits::{ModCofactor, ModDeterminant};
use crate::num::modular::traits::{ModGeneralizedCross, ModIsReduced};
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use crate::vector::Vector;
use alloc::vec::Vec;

// Panics unless the `n - 1` vectors all have dimension `n`, and returns `n`.
pub(crate) fn generalized_cross_dimension<T: PrimitiveUnsigned>(xs: &[UnsignedVector<T>]) -> usize {
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
pub(crate) fn square_with_zero_last_row<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    n: usize,
) -> UnsignedMatrix<T> {
    let mut elements = Vec::with_capacity(n * n);
    for x in xs {
        elements.extend_from_slice(&x.elements);
    }
    elements.resize(n * n, T::ZERO);
    let n = u64::exact_from(n);
    UnsignedMatrix::from_owned_elements(n, n, elements)
}

impl<T: PrimitiveUnsigned> ModGeneralizedCross<T> for UnsignedVector<T>
where
    UnsignedMatrix<T>: ModDeterminant<T, Output = T>,
{
    /// Computes the generalized cross product of $n - 1$ [`UnsignedVector`]s of dimension $n$
    /// modulo $m$.
    ///
    /// $$
    /// f(v_0, \ldots, v_{n - 2}, m)_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod m,
    /// $$
    ///
    /// where $A$ is the matrix whose rows are the vectors and $A_{(i)}$ is $A$ without column $i$.
    /// The result $w$ satisfies $w \cdot x \equiv \det \begin{pmatrix} A \\ x \end{pmatrix} \pmod
    /// m$ for every $x$, so it is perpendicular to each vector modulo $m$. The modulus may be
    /// composite. For $n = 2$ it rotates the vector a quarter turn counterclockwise, $(a, b)
    /// \mapsto (-b, a) \bmod m$; for $n = 3$ it is the cross product; and no vectors give $(1 \bmod
    /// m)$.
    ///
    /// Element $i$ is the [cofactor](ModCofactor::mod_cofactor) in row $n - 1$ and column $i$ of
    /// the matrix whose rows are the vectors followed by a zero row, found modulo $m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^4 \log m)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension, and $m$ is the modulus.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors do not all have dimension one more than their number,
    /// or if any element of any vector is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::modular::traits::ModGeneralizedCross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = |s| UnsignedVector::<u64>::from_str(s).unwrap();
    /// assert_eq!(
    ///     UnsignedVector::mod_generalized_cross(&[v("(1, 2, 3)"), v("(4, 5, 6)")], 7).to_string(),
    ///     "(4, 6, 4)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::mod_generalized_cross(&[v("(3, 4)")], 7).to_string(),
    ///     "(3, 3)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::mod_generalized_cross(
    ///         &[v("(1, 2, 3, 4)"), v("(0, 1, 6, 2)"), v("(2, 0, 1, 1)")],
    ///         7
    ///     )
    ///     .to_string(),
    ///     "(5, 3, 6, 5)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u64>::mod_generalized_cross(&[], 5).to_string(),
    ///     "(1)"
    /// );
    /// ```
    fn mod_generalized_cross(xs: &[Self], m: T) -> Self {
        assert_ne!(m, T::ZERO, "the modulus must be nonzero");
        let n = generalized_cross_dimension(xs);
        assert!(
            xs.iter().all(|x| x.mod_is_reduced(&m)),
            "each vector must be reduced modulo {m}"
        );
        let a = square_with_zero_last_row(xs, n);
        let last = u64::exact_from(n - 1);
        Self::from_owned_elements((0..=last).map(|i| (&a).mod_cofactor(last, i, m)).collect())
    }
}
