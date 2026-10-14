// Copyright © 2026 Mikhail Hogrefe
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
    ModPowerOf2EntrywiseInverse, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2MulAssign, Parity,
};

// Replaces every element of `xs` by its inverse modulo `2^pow`, returning `false`, and leaving `xs`
// unchanged, if some element is not invertible. The elements must already be reduced modulo
// `2^pow`.
//
// As in `mod_entrywise_inverse`, the product of all the elements is inverted once and each inverse
// is recovered from prefix products. The product is odd, and so invertible, exactly when every
// element is.
fn mod_power_of_2_entrywise_inverse_in_place(xs: &mut [Natural], pow: u64) -> bool {
    if xs.is_empty() || pow == 0 {
        // Modulo 1, every element is 0, which is its own inverse.
        return true;
    }
    // products[k] = xs[0] * ... * xs[k]
    let mut products: Vec<Natural> = Vec::with_capacity(xs.len());
    products.push(xs[0].clone());
    for x in &xs[1..] {
        let product = x.mod_power_of_2_mul(products.last().unwrap(), pow);
        products.push(product);
    }
    let product = products.pop().unwrap();
    if product.even() {
        return false;
    }
    let mut inverse = product.mod_power_of_2_inverse(pow).unwrap();
    // At the end of iteration k, xs[k] = (xs[0] * ... * xs[k - 1]) * (xs[0] * ... * xs[k])^{-1} =
    // xs[k]^{-1}, and inverse = (xs[0] * ... * xs[k - 1])^{-1}.
    for k in (1..xs.len()).rev() {
        let x = replace(
            &mut xs[k],
            (&inverse).mod_power_of_2_mul(&products[k - 1], pow),
        );
        inverse.mod_power_of_2_mul_assign(&x, pow);
    }
    xs[0] = inverse;
    true
}

fn mod_power_of_2_entrywise_inverse_helper(
    mut v: NaturalVector,
    pow: u64,
) -> Option<NaturalVector> {
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    if mod_power_of_2_entrywise_inverse_in_place(&mut v.elements, pow) {
        Some(v)
    } else {
        None
    }
}

impl ModPowerOf2EntrywiseInverse for NaturalVector {
    type Output = Self;

    /// Finds the multiplicative inverse of every element of a [`NaturalVector`] modulo $2^k$,
    /// taking the vector by value. The elements must already be reduced modulo $2^k$.
    ///
    /// Returns `None` if $k > 0$ and some element is even, and so has no inverse. Rather than
    /// inverting each element, this inverts the product of all the elements once and recovers each
    /// inverse from partial products, so it computes one inverse and about three multiplications
    /// per element.
    ///
    /// $$
    /// f(v, k) = \\begin{cases}
    ///     \operatorname{Some}((v_0^{-1} \bmod 2^k, v_1^{-1} \bmod 2^k, \ldots,
    ///         v_{n-1}^{-1} \bmod 2^k))
    ///         & \text{if} \\quad k = 0 \text{ or every } v_i \text{ is odd}, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseInverse;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 3, 5, 7)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_entrywise_inverse(4).unwrap().to_string(),
    ///     "(1, 11, 13, 7)"
    /// );
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.mod_power_of_2_entrywise_inverse(4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_inverse(self, pow: u64) -> Option<Self> {
        mod_power_of_2_entrywise_inverse_helper(self, pow)
    }
}

impl ModPowerOf2EntrywiseInverse for &NaturalVector {
    type Output = NaturalVector;

    /// Finds the multiplicative inverse of every element of a [`NaturalVector`] modulo $2^k$,
    /// taking the vector by reference. The elements must already be reduced modulo $2^k$.
    ///
    /// Returns `None` if $k > 0$ and some element is even, and so has no inverse. Rather than
    /// inverting each element, this inverts the product of all the elements once and recovers each
    /// inverse from partial products, so it computes one inverse and about three multiplications
    /// per element.
    ///
    /// $$
    /// f(v, k) = \\begin{cases}
    ///     \operatorname{Some}((v_0^{-1} \bmod 2^k, v_1^{-1} \bmod 2^k, \ldots,
    ///         v_{n-1}^{-1} \bmod 2^k))
    ///         & \text{if} \\quad k = 0 \text{ or every } v_i \text{ is odd}, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseInverse;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 3, 5, 7)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_entrywise_inverse(4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 11, 13, 7)"
    /// );
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_entrywise_inverse(4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_inverse(self, pow: u64) -> Option<NaturalVector> {
        mod_power_of_2_entrywise_inverse_helper(self.clone(), pow)
    }
}
