// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2025 Vincent Neiger
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModEntrywiseInverse, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

// Replaces every element of `xs` by its inverse modulo `m`, returning `false`, and leaving `xs`
// unchanged, if some element is not invertible. The elements must already be reduced modulo `m`.
//
// Rather than inverting each element, this inverts the product of all the elements once and
// recovers each inverse from prefix products, at a cost of about three multiplications per element.
// The product is invertible exactly when every element is.
//
// This is equivalent to `_nmod_vec_invert_generic` from `nmod_vec/invert.c`, FLINT 3.6.0, except
// that it detects a non-invertible element instead of assuming that there is none.
pub(crate) fn mod_entrywise_inverse_in_place<T: PrimitiveUnsigned>(xs: &mut [T], m: T) -> bool {
    if xs.is_empty() || m == T::ONE {
        // Modulo 1, every element is 0, which is its own inverse.
        return true;
    }
    let data = T::precompute_mod_mul_data(&m);
    // products[k] = xs[0] * ... * xs[k]
    let mut products = Vec::with_capacity(xs.len());
    let mut product = xs[0];
    products.push(product);
    for &x in &xs[1..] {
        product.mod_mul_precomputed_assign(x, m, &data);
        products.push(product);
    }
    if product == T::ZERO {
        return false;
    }
    let Some(mut inverse) = product.mod_inverse(m) else {
        return false;
    };
    // At the end of iteration k, xs[k] = (xs[0] * ... * xs[k - 1]) * (xs[0] * ... * xs[k])^{-1} =
    // xs[k]^{-1}, and inverse = (xs[0] * ... * xs[k - 1])^{-1}.
    for k in (1..xs.len()).rev() {
        let x = xs[k];
        xs[k] = inverse.mod_mul_precomputed(products[k - 1], m, &data);
        inverse.mod_mul_precomputed_assign(x, m, &data);
    }
    xs[0] = inverse;
    true
}

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, m: T) {
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
}

impl<T: PrimitiveUnsigned> ModEntrywiseInverse<T> for UnsignedVector<T> {
    type Output = Self;

    /// Finds the multiplicative inverse of every element of an [`UnsignedVector`] modulo $m$,
    /// taking the vector by value. The elements must already be reduced modulo $m$.
    ///
    /// Returns `None` if some element is not coprime to $m$, and so has no inverse. Rather than
    /// inverting each element, this inverts the product of all the elements once and recovers each
    /// inverse from partial products, so it computes one inverse and about three multiplications
    /// per element.
    ///
    /// $$
    /// f(v, m) = \\begin{cases}
    ///     \operatorname{Some}((v_0^{-1} \bmod m, v_1^{-1} \bmod m, \ldots, v_{n-1}^{-1} \bmod m))
    ///         & \text{if} \\quad \gcd(v_i, m) = 1 \text{ for all } i, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
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
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_inverse(7).unwrap().to_string(),
    ///     "(1, 4, 5, 2)"
    /// );
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.mod_entrywise_inverse(6), None);
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_invert` from `nmod_vec/invert.c`, FLINT 3.6.0, except that
    /// it detects a non-invertible element instead of assuming that there is none.
    fn mod_entrywise_inverse(mut self, m: T) -> Option<Self> {
        assert_reduced(&self, m);
        if mod_entrywise_inverse_in_place(&mut self.elements, m) {
            Some(self)
        } else {
            None
        }
    }
}

impl<T: PrimitiveUnsigned> ModEntrywiseInverse<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Finds the multiplicative inverse of every element of an [`UnsignedVector`] modulo $m$,
    /// taking the vector by reference. The elements must already be reduced modulo $m$.
    ///
    /// Returns `None` if some element is not coprime to $m$, and so has no inverse. Rather than
    /// inverting each element, this inverts the product of all the elements once and recovers each
    /// inverse from partial products, so it computes one inverse and about three multiplications
    /// per element.
    ///
    /// $$
    /// f(v, m) = \\begin{cases}
    ///     \operatorname{Some}((v_0^{-1} \bmod m, v_1^{-1} \bmod m, \ldots, v_{n-1}^{-1} \bmod m))
    ///         & \text{if} \\quad \gcd(v_i, m) = 1 \text{ for all } i, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
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
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_inverse(7).unwrap().to_string(),
    ///     "(1, 4, 5, 2)"
    /// );
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_entrywise_inverse(6), None);
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_invert` from `nmod_vec/invert.c`, FLINT 3.6.0, except that
    /// it detects a non-invertible element instead of assuming that there is none.
    #[inline]
    fn mod_entrywise_inverse(self, m: T) -> Option<UnsignedVector<T>> {
        self.clone().mod_entrywise_inverse(m)
    }
}
