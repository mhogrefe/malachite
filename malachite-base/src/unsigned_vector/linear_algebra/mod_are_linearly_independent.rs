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
use crate::num::linear_algebra::traits::{ModAreLinearlyIndependent, ModHowellForm};
use crate::num::modular::traits::ModIsReduced;
use crate::unsigned_matrix::{UnsignedMatrix, common_dimension};
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;

impl<T: PrimitiveUnsigned> ModAreLinearlyIndependent<T> for UnsignedVector<T>
where
    UnsignedMatrix<T>: ModHowellForm<T, Output = UnsignedMatrix<T>>,
{
    /// Determines whether [`UnsignedVector`]s are linearly independent modulo $m$: whether no
    /// nontrivial linear combination of them, with coefficients in $\mathbb{Z}/m\mathbb{Z}$, is
    /// zero modulo $m$.
    ///
    /// The vectors must all have the same dimension and must already be reduced modulo $m$. The
    /// modulus need not be prime: modulo 4, for example, $(2)$ is not linearly independent, since
    /// $2 \cdot (2) = (0)$. Vectors are linearly independent modulo $m$ exactly when they are
    /// linearly independent modulo every prime factor of $m$. An empty slice is linearly
    /// independent, and so is every slice modulo 1, where the only coefficient is 0.
    ///
    /// The vectors $v_1, \ldots, v_r$ of dimension $d$ are made the rows of an $r \times (d + r)$
    /// matrix, each followed by the corresponding row of the $r \times r$ identity. The rows of
    /// this matrix span the vectors $(\sum_i c_iv_i, c)$, so the combinations of the $v_i$ that
    /// vanish are the vectors in the span whose first $d$ elements are zero. By the defining
    /// property of the Howell form, as computed by [`ModHowellForm`], these are combinations of the
    /// Howell form's rows whose pivots are in the last $r$ columns; so the vectors are linearly
    /// independent exactly when no row of the Howell form has its pivot there.
    ///
    /// # Worst-case complexity
    /// $T(r, d) = O((r + d)^3)$
    ///
    /// $M(r, d) = O((r + d)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, and $d$ is the dimension of
    /// the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors do not all have the same dimension, or if any element
    /// of any vector is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAreLinearlyIndependent;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = |s| UnsignedVector::<u8>::from_str(s).unwrap();
    /// // 2 * (2, 0) = (0, 0) modulo 4
    /// assert!(!UnsignedVector::mod_are_linearly_independent(
    ///     &[v("(2, 0)")],
    ///     4
    /// ));
    /// // k * (2, 1) = (0, 0) modulo 4 only when k = 0
    /// assert!(UnsignedVector::mod_are_linearly_independent(
    ///     &[v("(2, 1)")],
    ///     4
    /// ));
    /// assert!(UnsignedVector::mod_are_linearly_independent(
    ///     &[v("(1, 2)"), v("(0, 1)")],
    ///     7
    /// ));
    /// assert!(!UnsignedVector::mod_are_linearly_independent(
    ///     &[v("(1, 2)"), v("(2, 4)")],
    ///     7
    /// ));
    /// assert!(UnsignedVector::<u8>::mod_are_linearly_independent(&[], 7));
    /// ```
    fn mod_are_linearly_independent(xs: &[Self], m: T) -> bool {
        assert_ne!(m, T::ZERO, "the modulus must be nonzero");
        let d = common_dimension(xs, "vectors");
        assert!(
            xs.iter().all(|x| x.mod_is_reduced(&m)),
            "each vector must be reduced modulo {m}"
        );
        let r = xs.len();
        if r == 0 || m == T::ONE {
            return true;
        }
        // more vectors than their dimension are dependent, since m > 1
        if r > d {
            return false;
        }
        let w = d + r;
        let mut elements = vec![T::ZERO; r * w];
        for (i, (row, x)) in elements.chunks_exact_mut(w).zip(xs).enumerate() {
            row[..d].copy_from_slice(&x.elements);
            row[d + i] = T::ONE;
        }
        let howell =
            UnsignedMatrix::from_elements(u64::exact_from(r), u64::exact_from(w), &elements)
                .mod_howell_form(m);
        howell.elements.chunks_exact(w).all(|row| {
            row[..d].iter().any(|&x| x != T::ZERO) || row[d..].iter().all(|&x| x == T::ZERO)
        })
    }
}
