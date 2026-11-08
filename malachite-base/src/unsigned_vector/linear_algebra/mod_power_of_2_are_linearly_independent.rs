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
use crate::num::linear_algebra::traits::{ModPowerOf2AreLinearlyIndependent, ModRank};
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::unsigned_matrix::{UnsignedMatrix, common_dimension};
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

impl<T: PrimitiveUnsigned> ModPowerOf2AreLinearlyIndependent for UnsignedVector<T> {
    /// Determines whether [`UnsignedVector`]s are linearly independent modulo $2^k$: whether no
    /// nontrivial linear combination of them, with coefficients in $\mathbb{Z}/2^k\mathbb{Z}$, is
    /// zero modulo $2^k$.
    ///
    /// The vectors must all have the same dimension and must already be reduced modulo $2^k$. Since
    /// 2 is the only prime factor of $2^k$, the vectors are linearly independent modulo $2^k$ for
    /// $k > 0$ exactly when they are linearly independent modulo 2, which is when the matrix whose
    /// rows are the vectors reduced modulo 2 has full rank. An empty slice is linearly independent,
    /// and so is every slice modulo 1, where the only coefficient is 0.
    ///
    /// # Worst-case complexity
    /// $T(r, d) = O(rd\min(r, d))$
    ///
    /// $M(r, d) = O(rd)$
    ///
    /// where $T$ is time, $M$ is additional memory, $r$ is `xs.len()`, and $d$ is the dimension of
    /// the vectors.
    ///
    /// # Panics
    /// Panics if the vectors do not all have the same dimension, or if any element of any vector is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2AreLinearlyIndependent;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = |s| UnsignedVector::<u8>::from_str(s).unwrap();
    /// // 2 * (2, 0) = (0, 0) modulo 4
    /// assert!(!UnsignedVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(2, 0)")],
    ///     2
    /// ));
    /// assert!(UnsignedVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(2, 1)")],
    ///     2
    /// ));
    /// assert!(UnsignedVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(1, 2)"), v("(3, 1)")],
    ///     2
    /// ));
    /// assert!(!UnsignedVector::mod_power_of_2_are_linearly_independent(
    ///     &[v("(1, 2)"), v("(3, 2)")],
    ///     2
    /// ));
    /// ```
    fn mod_power_of_2_are_linearly_independent(xs: &[Self], pow: u64) -> bool {
        let d = common_dimension(xs, "vectors");
        assert!(
            xs.iter().all(|x| x.mod_power_of_2_is_reduced(pow)),
            "each vector must be reduced modulo 2^{pow}"
        );
        let r = xs.len();
        if r == 0 || pow == 0 {
            return true;
        }
        if r > d {
            return false;
        }
        let elements: Vec<T> = xs
            .iter()
            .flat_map(|x| x.elements.iter().map(|&e| T::from(e.odd())))
            .collect();
        UnsignedMatrix::from_elements(u64::exact_from(r), u64::exact_from(d), &elements)
            .mod_rank(T::TWO)
            == u64::exact_from(r)
    }
}
