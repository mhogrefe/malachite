// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_reduced(a: &NaturalMatrix, c: &Natural, m: &Natural) {
    assert!(
        a.mod_is_reduced(m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// Multiplies every element of `a` by `c` modulo `m`, in place. The data for multiplying modulo `m`
// is computed once and shared by all the elements.
fn mod_mul_assign_helper(a: &mut NaturalMatrix, c: &Natural, m: &Natural) {
    assert_reduced(a, c, m);
    mod_mul_assign_unchecked(a, c, m);
}

// As `mod_mul_assign_helper`, for elements and `c` already known to be reduced modulo `m`.
pub(crate) fn mod_mul_assign_unchecked(a: &mut NaturalMatrix, c: &Natural, m: &Natural) {
    match *c {
        Natural::ZERO => a.elements.fill(Natural::ZERO),
        Natural::ONE => {}
        _ => {
            let data =
                <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
            for x in &mut a.elements {
                x.mod_mul_precomputed_assign(c, m, &data);
            }
        }
    }
}

fn mod_mul_ref_helper(a: &NaturalMatrix, c: &Natural, m: &Natural) -> NaturalMatrix {
    assert_reduced(a, c, m);
    mod_mul_unchecked(a, c, m)
}

// As `mod_mul_ref_helper`, for elements and `c` already known to be reduced modulo `m`.
pub(crate) fn mod_mul_unchecked(a: &NaturalMatrix, c: &Natural, m: &Natural) -> NaturalMatrix {
    NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        match *c {
            Natural::ZERO => vec![Natural::ZERO; a.elements.len()],
            Natural::ONE => a.elements.clone(),
            _ => {
                let data =
                    <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
                a.elements
                    .iter()
                    .map(|x| x.mod_mul_precomputed(c, m, &data))
                    .collect()
            }
        },
    )
}

impl ModMul<Natural, Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by value, the scalar by value, and the modulus by value. The elements
    /// and the scalar must already be reduced modulo $m$.
    ///
    /// Each product is reduced modulo $m$, and the result has the same shape as the matrix. The
    /// data for multiplying modulo $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(A, c, m) = cA \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_mul(Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: Natural, m: Natural) -> Self {
        mod_mul_assign_helper(&mut self, &c, &m);
        self
    }
}

impl ModMul<Natural, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by value, the scalar by value, and the modulus by reference. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_mul(Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: Natural, m: &Natural) -> Self {
        mod_mul_assign_helper(&mut self, &c, m);
        self
    }
}

impl ModMul<&Natural, Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by value, the scalar by reference, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_mul(&Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: &Natural, m: Natural) -> Self {
        mod_mul_assign_helper(&mut self, c, &m);
        self
    }
}

impl ModMul<&Natural, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by value, the scalar by reference, and the modulus by reference. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_mul(&Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(mut self, c: &Natural, m: &Natural) -> Self {
        mod_mul_assign_helper(&mut self, c, m);
        self
    }
}

impl ModMul<Natural, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by reference, the scalar by value, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: Natural, m: Natural) -> NaturalMatrix {
        mod_mul_ref_helper(self, &c, &m)
    }
}

impl ModMul<Natural, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by reference, the scalar by value, and the modulus by reference. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: Natural, m: &Natural) -> NaturalMatrix {
        mod_mul_ref_helper(self, &c, m)
    }
}

impl ModMul<&Natural, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by reference, the scalar by reference, and the modulus by value. The
    /// elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(&Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: &Natural, m: Natural) -> NaturalMatrix {
        mod_mul_ref_helper(self, c, &m)
    }
}

impl ModMul<&Natural, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, taking the matrix by reference, the scalar by reference, and the modulus by reference.
    /// The elements and the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_mul(&Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "[1, 3; 2, 6]"
    /// );
    /// ```
    #[inline]
    fn mod_mul(self, c: &Natural, m: &Natural) -> NaturalMatrix {
        mod_mul_ref_helper(self, c, m)
    }
}

impl ModMulAssign<Natural, Natural> for NaturalMatrix {
    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by value and the modulus by value. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_mul_assign(Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[1, 3; 2, 6]");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: Natural, m: Natural) {
        mod_mul_assign_helper(self, &c, &m);
    }
}

impl ModMulAssign<Natural, &Natural> for NaturalMatrix {
    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by value and the modulus by reference. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_mul_assign(Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[1, 3; 2, 6]");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: Natural, m: &Natural) {
        mod_mul_assign_helper(self, &c, m);
    }
}

impl ModMulAssign<&Natural, Natural> for NaturalMatrix {
    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by reference and the modulus by value. The elements and the
    /// scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_mul_assign(&Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[1, 3; 2, 6]");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: &Natural, m: Natural) {
        mod_mul_assign_helper(self, c, &m);
    }
}

impl ModMulAssign<&Natural, &Natural> for NaturalMatrix {
    /// Multiplies every element of a [`NaturalMatrix`] by a [`Natural`] modulo another [`Natural`]
    /// $m$, in place, taking the scalar by reference and the modulus by reference. The elements and
    /// the scalar must already be reduced modulo $m$.
    ///
    /// See the documentation for the [`ModMul`] implementation that takes everything by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self`, or `c`, is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_mul_assign(&Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[1, 3; 2, 6]");
    /// ```
    #[inline]
    fn mod_mul_assign(&mut self, c: &Natural, m: &Natural) {
        mod_mul_assign_helper(self, c, m);
    }
}
