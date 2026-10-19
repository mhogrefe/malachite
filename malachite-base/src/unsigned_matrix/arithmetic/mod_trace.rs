// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModIsReduced, ModTrace};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

fn check_args<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: &T) {
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

impl<T: PrimitiveUnsigned> ModTrace<T> for UnsignedMatrix<T> {
    type Output = T;

    /// Returns the trace of a square [`UnsignedMatrix`] modulo $m$, the sum of its diagonal
    /// elements modulo $m$, taking the matrix by reference. The elements must already be reduced.
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
    /// $M(n) = O(1)$
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// // 5 + 7 = 12, which is 3 mod 9.
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .to_mod_trace(9),
    ///     3
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .to_mod_trace(9),
    ///     0
    /// );
    /// ```
    fn to_mod_trace(&self, m: T) -> T {
        check_args(self, &m);
        // The diagonal elements are `width + 1` apart in row-major order.
        T::mod_sum(self.elements.iter().step_by(self.width + 1), m)
    }

    /// Returns the trace of a square [`UnsignedMatrix`] modulo $m$, the sum of its diagonal
    /// elements modulo $m$, taking the matrix by value. The elements must already be reduced.
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
    /// $M(n) = O(1)$
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
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// // 5 + 7 = 12, which is 3 mod 9.
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .into_mod_trace(9),
    ///     3
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .into_mod_trace(9),
    ///     0
    /// );
    /// ```
    fn into_mod_trace(self, m: T) -> T {
        check_args(&self, &m);
        let step = self.width + 1;
        T::mod_sum(self.elements.into_iter().step_by(step), m)
    }
}
