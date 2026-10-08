// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, m: T) {
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
}

// Negates every element modulo m.
fn negate<T: PrimitiveUnsigned>(elements: &mut [T], m: T) {
    for x in elements {
        if *x != T::ZERO {
            *x = m - *x;
        }
    }
}

impl<T: PrimitiveUnsigned> ModNeg<T> for UnsignedVector<T> {
    type Output = Self;

    /// Negates an [`UnsignedVector`] modulo $m$, taking the vector by value. The elements must
    /// already be reduced modulo $m$.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 0)").unwrap();
    /// assert_eq!(v.mod_neg(7).to_string(), "(2, 6, 0)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_neg` from `nmod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(mut self, m: T) -> Self {
        self.mod_neg_assign(m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModNeg<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Negates an [`UnsignedVector`] modulo $m$, taking the vector by reference. The elements must
    /// already be reduced modulo $m$.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNeg;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 0)").unwrap();
    /// assert_eq!((&v).mod_neg(7).to_string(), "(2, 6, 0)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_neg` from `nmod_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn mod_neg(self, m: T) -> UnsignedVector<T> {
        assert_reduced(self, m);
        let mut elements = self.elements.clone();
        negate(&mut elements, m);
        UnsignedVector { elements }
    }
}

impl<T: PrimitiveUnsigned> ModNegAssign<T> for UnsignedVector<T> {
    /// Negates an [`UnsignedVector`] modulo $m$, in place. The elements must already be reduced
    /// modulo $m$.
    ///
    /// See [`mod_neg`](ModNeg::mod_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModNegAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 0)").unwrap();
    /// v.mod_neg_assign(7);
    /// assert_eq!(v.to_string(), "(2, 6, 0)");
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: T) {
        assert_reduced(self, m);
        negate(&mut self.elements, m);
    }
}
