// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
}

// Negates every element modulo 2^pow.
fn negate<T: PrimitiveUnsigned>(elements: &mut [T], pow: u64) {
    for x in elements {
        *x = x.wrapping_neg().mod_power_of_2(pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Neg for UnsignedVector<T> {
    type Output = Self;

    /// Negates an [`UnsignedVector`] modulo $2^k$, taking the vector by value. The elements must
    /// already be reduced modulo $2^k$.
    ///
    /// Each element $x$ becomes $-x \bmod 2^k$: $2^k - x$ if $x$ is nonzero, and 0 if it is zero.
    /// The dimension is unchanged.
    ///
    /// $$
    /// f(v, k) = -v \bmod 2^k.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self` is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(v.mod_power_of_2_neg(3).to_string(), "(3, 7, 5)");
    ///
    /// // A zero element stays zero.
    /// let v = UnsignedVector::<u8>::from_str("(0, 1)").unwrap();
    /// assert_eq!(v.mod_power_of_2_neg(8).to_string(), "(0, 255)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_neg` from `nmod_vec/neg.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_neg(mut self, pow: u64) -> Self {
        self.mod_power_of_2_neg_assign(pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Neg for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Negates an [`UnsignedVector`] modulo $2^k$, taking the vector by reference. The elements
    /// must already be reduced modulo $2^k$.
    ///
    /// Each element $x$ becomes $-x \bmod 2^k$: $2^k - x$ if $x$ is nonzero, and 0 if it is zero.
    /// The dimension is unchanged.
    ///
    /// $$
    /// f(v, k) = -v \bmod 2^k.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self` is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Neg;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_neg(3).to_string(), "(3, 7, 5)");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_neg` from `nmod_vec/neg.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_neg(self, pow: u64) -> UnsignedVector<T> {
        assert_reduced(self, pow);
        let mut elements = self.elements.clone();
        negate(&mut elements, pow);
        UnsignedVector { elements }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2NegAssign for UnsignedVector<T> {
    /// Negates an [`UnsignedVector`] modulo $2^k$, in place. The elements must already be reduced
    /// modulo $2^k$.
    ///
    /// See [`mod_power_of_2_neg`](ModPowerOf2Neg::mod_power_of_2_neg).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self` is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// v.mod_power_of_2_neg_assign(3);
    /// assert_eq!(v.to_string(), "(3, 7, 5)");
    /// ```
    #[inline]
    fn mod_power_of_2_neg_assign(&mut self, pow: u64) {
        assert_reduced(self, pow);
        negate(&mut self.elements, pow);
    }
}
