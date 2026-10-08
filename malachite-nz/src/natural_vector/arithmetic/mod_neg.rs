// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};

fn assert_reduced(v: &NaturalVector, m: &Natural) {
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
}

fn mod_neg_ref(v: &NaturalVector, m: &Natural) -> NaturalVector {
    assert_reduced(v, m);
    NaturalVector {
        elements: v.elements.iter().map(|x| x.mod_neg(m)).collect(),
    }
}

fn mod_neg_assign(v: &mut NaturalVector, m: &Natural) {
    assert_reduced(v, m);
    for x in &mut v.elements {
        x.mod_neg_assign(m);
    }
}

impl ModNeg<Natural> for NaturalVector {
    type Output = Self;

    /// Negates a [`NaturalVector`] modulo $m$, taking the vector by value and the modulus by value.
    /// The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// dimension is unchanged.
    ///
    /// $$
    /// f(v, m) = -v \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 0)").unwrap();
    /// assert_eq!(v.mod_neg(Natural::from(7u32)).to_string(), "(2, 6, 0)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: Natural) -> Self {
        mod_neg_assign(&mut self, &m);
        self
    }
}

impl ModNeg<&Natural> for NaturalVector {
    type Output = Self;

    /// Negates a [`NaturalVector`] modulo $m$, taking the vector by value and the modulus by
    /// reference. The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// dimension is unchanged.
    ///
    /// $$
    /// f(v, m) = -v \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 0)").unwrap();
    /// assert_eq!(v.mod_neg(&Natural::from(7u32)).to_string(), "(2, 6, 0)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: &Natural) -> Self {
        mod_neg_assign(&mut self, m);
        self
    }
}

impl ModNeg<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Negates a [`NaturalVector`] modulo $m$, taking the vector by reference and the modulus by
    /// value. The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// dimension is unchanged.
    ///
    /// $$
    /// f(v, m) = -v \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 0)").unwrap();
    /// assert_eq!((&v).mod_neg(Natural::from(7u32)).to_string(), "(2, 6, 0)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: Natural) -> NaturalVector {
        mod_neg_ref(self, &m)
    }
}

impl ModNeg<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Negates a [`NaturalVector`] modulo $m$, taking the vector by reference and the modulus by
    /// reference. The elements must already be reduced modulo $m$.
    ///
    /// Each element $x$ becomes $-x \bmod m$: $m - x$ if $x$ is nonzero, and 0 if it is zero. The
    /// dimension is unchanged.
    ///
    /// $$
    /// f(v, m) = -v \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 0)").unwrap();
    /// assert_eq!((&v).mod_neg(&Natural::from(7u32)).to_string(), "(2, 6, 0)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_neg` from `fmpz_mod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: &Natural) -> NaturalVector {
        mod_neg_ref(self, m)
    }
}

impl ModNegAssign<Natural> for NaturalVector {
    /// Negates a [`NaturalVector`] modulo $m$, in place, taking the modulus by value. The elements
    /// must already be reduced modulo $m$.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 0)").unwrap();
    /// v.mod_neg_assign(Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(2, 6, 0)");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: Natural) {
        mod_neg_assign(self, &m);
    }
}

impl ModNegAssign<&Natural> for NaturalVector {
    /// Negates a [`NaturalVector`] modulo $m$, in place, taking the modulus by reference. The
    /// elements must already be reduced modulo $m$.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 0)").unwrap();
    /// v.mod_neg_assign(&Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(2, 6, 0)");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: &Natural) {
        mod_neg_assign(self, m);
    }
}
