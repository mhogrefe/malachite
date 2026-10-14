// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2EntrywiseInverse, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

// Replaces every element of `xs` by its inverse modulo `2^pow`, returning `false`, and leaving `xs`
// unchanged, if some element is not invertible. The elements must already be reduced modulo
// `2^pow`.
//
// As in `mod_entrywise_inverse`, the product of all the elements is inverted once and each inverse
// is recovered from prefix products. The product is odd, and so invertible, exactly when every
// element is.
fn mod_power_of_2_entrywise_inverse_in_place<T: PrimitiveUnsigned>(xs: &mut [T], pow: u64) -> bool {
    if xs.is_empty() || pow == 0 {
        // Modulo 1, every element is 0, which is its own inverse.
        return true;
    }
    // products[k] = xs[0] * ... * xs[k]
    let mut products = Vec::with_capacity(xs.len());
    let mut product = xs[0];
    products.push(product);
    for &x in &xs[1..] {
        product.mod_power_of_2_mul_assign(x, pow);
        products.push(product);
    }
    if product.even() {
        return false;
    }
    let mut inverse = product.mod_power_of_2_inverse(pow).unwrap();
    // At the end of iteration k, xs[k] = (xs[0] * ... * xs[k - 1]) * (xs[0] * ... * xs[k])^{-1} =
    // xs[k]^{-1}, and inverse = (xs[0] * ... * xs[k - 1])^{-1}.
    for k in (1..xs.len()).rev() {
        let x = xs[k];
        xs[k] = inverse.mod_power_of_2_mul(products[k - 1], pow);
        inverse.mod_power_of_2_mul_assign(x, pow);
    }
    xs[0] = inverse;
    true
}

fn assert_valid<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseInverse for UnsignedVector<T> {
    type Output = Self;

    /// Finds the multiplicative inverse of every element of an [`UnsignedVector`] modulo $2^k$,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseInverse;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 3, 5, 7)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_entrywise_inverse(4).unwrap().to_string(),
    ///     "(1, 11, 13, 7)"
    /// );
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.mod_power_of_2_entrywise_inverse(4), None);
    /// ```
    fn mod_power_of_2_entrywise_inverse(mut self, pow: u64) -> Option<Self> {
        assert_valid(&self, pow);
        if mod_power_of_2_entrywise_inverse_in_place(&mut self.elements, pow) {
            Some(self)
        } else {
            None
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseInverse for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Finds the multiplicative inverse of every element of an [`UnsignedVector`] modulo $2^k$,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseInverse;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 3, 5, 7)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_entrywise_inverse(4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 11, 13, 7)"
    /// );
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_entrywise_inverse(4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_inverse(self, pow: u64) -> Option<UnsignedVector<T>> {
        self.clone().mod_power_of_2_entrywise_inverse(pow)
    }
}
