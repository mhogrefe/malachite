// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::linear_algebra::traits::ModPowerOf2Cofactor;
use crate::num::modular::traits::{ModPowerOf2GeneralizedCross, ModPowerOf2IsReduced};
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::modular::mod_generalized_cross::{
    generalized_cross_dimension, square_with_zero_last_row,
};
use crate::vector::Vector;

impl<T: PrimitiveUnsigned> ModPowerOf2GeneralizedCross for UnsignedVector<T> {
    /// Computes the generalized cross product of $n - 1$ [`UnsignedVector`]s of dimension $n$
    /// modulo $2^k$.
    ///
    /// $$
    /// f(v_0, \ldots, v_{n - 2}, k)_i = (-1)^{n - 1 + i} \det A_{(i)} \bmod 2^k,
    /// $$
    ///
    /// where $A$ is the matrix whose rows are the vectors and $A_{(i)}$ is $A$ without column $i$.
    /// The result $w$ satisfies $w \cdot x \equiv \det \begin{pmatrix} A \\ x \end{pmatrix}
    /// \pmod{2^k}$ for every $x$, so it is perpendicular to each vector modulo $2^k$. For $n = 2$
    /// it rotates the vector a quarter turn counterclockwise, $(a, b) \mapsto (-b, a) \bmod 2^k$;
    /// for $n = 3$ it is the cross product; and no vectors give $(1 \bmod 2^k)$.
    ///
    /// Element $i$ is the [cofactor](ModPowerOf2Cofactor::mod_power_of_2_cofactor) in row $n - 1$
    /// and column $i$ of the matrix whose rows are the vectors followed by a zero row, found modulo
    /// $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^4)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the vectors do not all have dimension one
    /// more than their number, or if any element of any vector is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::modular::traits::ModPowerOf2GeneralizedCross;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = |s| UnsignedVector::<u64>::from_str(s).unwrap();
    /// assert_eq!(
    ///     UnsignedVector::mod_power_of_2_generalized_cross(&[v("(2, 0, 1)"), v("(0, 2, 3)")], 2)
    ///         .to_string(),
    ///     "(2, 2, 0)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::mod_power_of_2_generalized_cross(&[v("(3, 4)")], 3).to_string(),
    ///     "(4, 3)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u64>::mod_power_of_2_generalized_cross(&[], 0).to_string(),
    ///     "(0)"
    /// );
    /// ```
    fn mod_power_of_2_generalized_cross(xs: &[Self], pow: u64) -> Self {
        assert!(pow <= T::WIDTH, "pow must be at most {}", T::WIDTH);
        let n = generalized_cross_dimension(xs);
        assert!(
            xs.iter().all(|x| x.mod_power_of_2_is_reduced(pow)),
            "each vector must be reduced modulo 2^{pow}"
        );
        let a = square_with_zero_last_row(xs, n);
        let last = u64::exact_from(n - 1);
        Self::from_owned_elements(
            (0..=last)
                .map(|i| (&a).mod_power_of_2_cofactor(last, i, pow))
                .collect(),
        )
    }
}
