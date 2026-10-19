// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSum, ModTrace};

fn check_args(a: &NaturalMatrix, m: &Natural) {
    assert!(
        a.is_square(),
        "the matrix must be square, but it is {} by {}",
        a.height,
        a.width
    );
    assert!(
        a.mod_is_reduced(m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
}

impl ModTrace<Natural> for NaturalMatrix {
    type Output = Natural;

    /// Returns the trace of a square [`NaturalMatrix`] modulo $m$, the sum of its diagonal elements
    /// modulo $m$, taking the matrix by reference, and the modulus by value. The elements must
    /// already be reduced.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0. Every element is
    /// checked to be reduced, not only the diagonal ones.
    ///
    /// $$
    /// f(A, m) = \sum_{i=0}^{n-1} A_{ii} \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `m` is zero, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModTrace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // 5 + 7 = 12, which is 3 mod 9.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .to_mod_trace(Natural::from(9u32))
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .to_mod_trace(Natural::from(9u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn to_mod_trace(&self, m: Natural) -> Natural {
        check_args(self, &m);
        // The diagonal elements are `width + 1` apart in row-major order.
        Natural::mod_sum(self.elements.iter().step_by(self.width + 1), m)
    }

    /// Returns the trace of a square [`NaturalMatrix`] modulo $m$, the sum of its diagonal elements
    /// modulo $m$, taking the matrix by value, and the modulus by value. The elements must already
    /// be reduced.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0. Every element is
    /// checked to be reduced, not only the diagonal ones.
    ///
    /// $$
    /// f(A, m) = \sum_{i=0}^{n-1} A_{ii} \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `m` is zero, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModTrace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // 5 + 7 = 12, which is 3 mod 9.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .into_mod_trace(Natural::from(9u32))
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .into_mod_trace(Natural::from(9u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn into_mod_trace(self, m: Natural) -> Natural {
        check_args(&self, &m);
        let step = self.width + 1;
        Natural::mod_sum(self.elements.into_iter().step_by(step), m)
    }
}

impl ModTrace<&Natural> for NaturalMatrix {
    type Output = Natural;

    /// Returns the trace of a square [`NaturalMatrix`] modulo $m$, the sum of its diagonal elements
    /// modulo $m$, taking the matrix by reference, and the modulus by reference. The elements must
    /// already be reduced.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0. Every element is
    /// checked to be reduced, not only the diagonal ones.
    ///
    /// $$
    /// f(A, m) = \sum_{i=0}^{n-1} A_{ii} \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `m` is zero, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModTrace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // 5 + 7 = 12, which is 3 mod 9.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .to_mod_trace(&Natural::from(9u32))
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .to_mod_trace(&Natural::from(9u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn to_mod_trace(&self, m: &Natural) -> Natural {
        check_args(self, m);
        // The diagonal elements are `width + 1` apart in row-major order.
        Natural::mod_sum(self.elements.iter().step_by(self.width + 1), m)
    }

    /// Returns the trace of a square [`NaturalMatrix`] modulo $m$, the sum of its diagonal elements
    /// modulo $m$, taking the matrix by value, and the modulus by reference. The elements must
    /// already be reduced.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0. Every element is
    /// checked to be reduced, not only the diagonal ones.
    ///
    /// $$
    /// f(A, m) = \sum_{i=0}^{n-1} A_{ii} \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `m` is zero, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModTrace;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // 5 + 7 = 12, which is 3 mod 9.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .into_mod_trace(&Natural::from(9u32))
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .into_mod_trace(&Natural::from(9u32))
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn into_mod_trace(self, m: &Natural) -> Natural {
        check_args(&self, m);
        let step = self.width + 1;
        Natural::mod_sum(self.elements.into_iter().step_by(step), m)
    }
}
