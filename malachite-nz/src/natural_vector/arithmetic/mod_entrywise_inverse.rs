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

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use core::mem::replace;
use malachite_base::num::arithmetic::traits::{
    ModEntrywiseInverse, ModInverse, ModIsReduced, ModMulPrecomputed, ModMulPrecomputedAssign,
};

// Replaces every element of `xs` by its inverse modulo `m`, returning `false`, and leaving `xs`
// unchanged, if some element is not invertible. The elements must already be reduced modulo `m`.
//
// Rather than inverting each element, this inverts the product of all the elements once and
// recovers each inverse from prefix products, at a cost of about three multiplications per element.
// The product is invertible exactly when every element is.
//
// This is equivalent to `_nmod_vec_invert_generic` from `nmod_vec/invert.c`, FLINT 3.6.0, with
// `Natural`s in place of words, except that it detects a non-invertible element instead of assuming
// that there is none.
fn mod_entrywise_inverse_in_place(xs: &mut [Natural], m: &Natural) -> bool {
    if xs.is_empty() || *m == 1u32 {
        // Modulo 1, every element is 0, which is its own inverse.
        return true;
    }
    let data = <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    // products[k] = xs[0] * ... * xs[k]
    let mut products: Vec<Natural> = Vec::with_capacity(xs.len());
    products.push(xs[0].clone());
    for x in &xs[1..] {
        let product = x.mod_mul_precomputed(products.last().unwrap(), m, &data);
        products.push(product);
    }
    let product = products.pop().unwrap();
    if product == 0u32 {
        return false;
    }
    let Some(mut inverse) = product.mod_inverse(m) else {
        return false;
    };
    // At the end of iteration k, xs[k] = (xs[0] * ... * xs[k - 1]) * (xs[0] * ... * xs[k])^{-1} =
    // xs[k]^{-1}, and inverse = (xs[0] * ... * xs[k - 1])^{-1}.
    for k in (1..xs.len()).rev() {
        let x = replace(
            &mut xs[k],
            (&inverse).mod_mul_precomputed(&products[k - 1], m, &data),
        );
        inverse.mod_mul_precomputed_assign(&x, m, &data);
    }
    xs[0] = inverse;
    true
}

fn assert_reduced(v: &NaturalVector, m: &Natural) {
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
}

fn mod_entrywise_inverse_helper(mut v: NaturalVector, m: &Natural) -> Option<NaturalVector> {
    assert_reduced(&v, m);
    if mod_entrywise_inverse_in_place(&mut v.elements, m) {
        Some(v)
    } else {
        None
    }
}

impl ModEntrywiseInverse<Natural> for NaturalVector {
    type Output = Self;

    /// Finds the multiplicative inverse of every element of a [`NaturalVector`] modulo a
    /// [`Natural`] $m$, taking the vector by value and $m$ by value. The elements must already be
    /// reduced modulo $m$.
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
    /// $T(n, m) = O(nm \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_inverse(Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 4, 5, 2)"
    /// );
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.mod_entrywise_inverse(Natural::from(6u32)), None);
    /// ```
    #[inline]
    fn mod_entrywise_inverse(self, m: Natural) -> Option<Self> {
        mod_entrywise_inverse_helper(self, &m)
    }
}

impl ModEntrywiseInverse<&Natural> for NaturalVector {
    type Output = Self;

    /// Finds the multiplicative inverse of every element of a [`NaturalVector`] modulo a
    /// [`Natural`] $m$, taking the vector by value and $m$ by reference. The elements must already
    /// be reduced modulo $m$.
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
    /// $T(n, m) = O(nm \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3, 4)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_inverse(&Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 4, 5, 2)"
    /// );
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.mod_entrywise_inverse(&Natural::from(6u32)), None);
    /// ```
    #[inline]
    fn mod_entrywise_inverse(self, m: &Natural) -> Option<Self> {
        mod_entrywise_inverse_helper(self, m)
    }
}

impl ModEntrywiseInverse<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Finds the multiplicative inverse of every element of a [`NaturalVector`] modulo a
    /// [`Natural`] $m$, taking the vector by reference and $m$ by value. The elements must already
    /// be reduced modulo $m$.
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
    /// $T(n, m) = O(nm \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_inverse(Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 4, 5, 2)"
    /// );
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_entrywise_inverse(Natural::from(6u32)), None);
    /// ```
    #[inline]
    fn mod_entrywise_inverse(self, m: Natural) -> Option<NaturalVector> {
        mod_entrywise_inverse_helper(self.clone(), &m)
    }
}

impl ModEntrywiseInverse<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Finds the multiplicative inverse of every element of a [`NaturalVector`] modulo a
    /// [`Natural`] $m$, taking the vector by reference and $m$ by reference. The elements must
    /// already be reduced modulo $m$.
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
    /// $T(n, m) = O(nm \log m \log\log m + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3, 4)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_inverse(&Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 4, 5, 2)"
    /// );
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_entrywise_inverse(&Natural::from(6u32)), None);
    /// ```
    #[inline]
    fn mod_entrywise_inverse(self, m: &Natural) -> Option<NaturalVector> {
        mod_entrywise_inverse_helper(self.clone(), m)
    }
}
