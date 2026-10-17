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
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};

fn assert_reduced(a: &NaturalMatrix, m: &Natural) {
    assert!(
        a.mod_is_reduced(m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
}

fn mod_neg_ref(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    assert_reduced(a, m);
    NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(|x| x.mod_neg(m)).collect(),
    )
}

fn mod_neg_assign(a: &mut NaturalMatrix, m: &Natural) {
    assert_reduced(a, m);
    for x in &mut a.elements {
        x.mod_neg_assign(m);
    }
}

impl ModNeg<Natural> for NaturalMatrix {
    type Output = Self;

    /// Negates a [`NaturalMatrix`] modulo $m$, taking the matrix by value and the modulus by value.
    /// The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// shape is unchanged.
    ///
    /// $$
    /// f(A, m) = -A \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_neg(Natural::from(7u32)).to_string(), "[2, 6; 0, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: Natural) -> Self {
        mod_neg_assign(&mut self, &m);
        self
    }
}

impl ModNeg<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Negates a [`NaturalMatrix`] modulo $m$, taking the matrix by value and the modulus by
    /// reference. The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// shape is unchanged.
    ///
    /// $$
    /// f(A, m) = -A \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 0, 3]").unwrap();
    /// assert_eq!(a.mod_neg(&Natural::from(7u32)).to_string(), "[2, 6; 0, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: &Natural) -> Self {
        mod_neg_assign(&mut self, m);
        self
    }
}

impl ModNeg<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Negates a [`NaturalMatrix`] modulo $m$, taking the matrix by reference and the modulus by
    /// value. The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// shape is unchanged.
    ///
    /// $$
    /// f(A, m) = -A \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 0, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_neg(Natural::from(7u32)).to_string(),
    ///     "[2, 6; 0, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: Natural) -> NaturalMatrix {
        mod_neg_ref(self, &m)
    }
}

impl ModNeg<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Negates a [`NaturalMatrix`] modulo $m$, taking the matrix by reference and the modulus by
    /// reference. The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// shape is unchanged.
    ///
    /// $$
    /// f(A, m) = -A \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 0, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_neg(&Natural::from(7u32)).to_string(),
    ///     "[2, 6; 0, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: &Natural) -> NaturalMatrix {
        mod_neg_ref(self, m)
    }
}

impl ModNegAssign<Natural> for NaturalMatrix {
    /// Negates a [`NaturalMatrix`] modulo $m$, in place, taking the modulus by value. The elements
    /// must already be reduced modulo $m$.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 0, 3]").unwrap();
    /// a.mod_neg_assign(Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[2, 6; 0, 4]");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: Natural) {
        mod_neg_assign(self, &m);
    }
}

impl ModNegAssign<&Natural> for NaturalMatrix {
    /// Negates a [`NaturalMatrix`] modulo $m$, in place, taking the modulus by reference. The
    /// elements must already be reduced modulo $m$.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 0, 3]").unwrap();
    /// a.mod_neg_assign(&Natural::from(7u32));
    /// assert_eq!(a.to_string(), "[2, 6; 0, 4]");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: &Natural) {
        mod_neg_assign(self, m);
    }
}
