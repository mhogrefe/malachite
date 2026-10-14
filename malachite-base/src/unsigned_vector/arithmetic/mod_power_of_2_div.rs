// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Div, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_valid<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, c: T, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

// Divides every element of `xs` by `c` modulo `2^pow`, as `mod_power_of_2_div` does element by
// element, returning `false`, and leaving `xs` unchanged, if some element has no quotient. With `s`
// the number of trailing zeros of `c`, every quotient is `(x >> s) * (c >> s)^{-1} mod 2^(pow -
// s)`, so the inverse is computed only once.
fn mod_power_of_2_div_in_place<T: PrimitiveUnsigned>(xs: &mut [T], c: T, pow: u64) -> bool {
    if c == T::ZERO {
        // Only 0 has a quotient, and it is 0.
        return xs.iter().all(|&x| x == T::ZERO);
    }
    // c is nonzero and less than 2^pow, so s < pow.
    let s = c.trailing_zeros();
    if xs.iter().any(|&x| x != T::ZERO && x.trailing_zeros() < s) {
        return false;
    }
    let k = pow - s;
    let inverse = (c >> s).mod_power_of_2_inverse(k).unwrap();
    for x in xs {
        *x = (*x >> s).mod_power_of_2_mul(inverse, k);
    }
    true
}

impl<T: PrimitiveUnsigned> ModPowerOf2Div<T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a number $c$ modulo $2^k$, taking the
    /// vector by value, and returning `None` if some element has no quotient. The elements and $c$
    /// must already be reduced modulo $2^k$.
    ///
    /// Let $s$ be the number of trailing zeros of $c$. An element $x$ has a quotient if and only if
    /// $x = 0$, or $c \neq 0$ and $2^s$ divides $x$; the result is `None` if some element has none.
    /// Otherwise each element of the result is the smallest quotient, the one that
    /// [`ModPowerOf2Div`] returns for that element alone, $(x/2^s)(c/2^s)^{-1} \bmod 2^{k-s}$. The
    /// inverse is computed once and shared by all the elements.
    ///
    /// $f(v, c, k) = \operatorname{Some}(w)$, where each $w_i$ is the least $w_i < 2^k$ with $cw_i
    /// \equiv v_i \mod 2^k$, if such $w_i$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if `c` or any element of `self` is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone().mod_power_of_2_div(3, 4).unwrap().to_string(),
    ///     "(11, 6, 1)"
    /// );
    /// // 2 is not invertible modulo 16, but every element is even.
    /// let v = UnsignedVector::<u8>::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone().mod_power_of_2_div(2, 4).unwrap().to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = UnsignedVector::<u8>::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_div(2, 4), None);
    /// ```
    fn mod_power_of_2_div(mut self, c: T, pow: u64) -> Option<Self> {
        assert_valid(&self, c, pow);
        if mod_power_of_2_div_in_place(&mut self.elements, c, pow) {
            Some(self)
        } else {
            None
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Div<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a number $c$ modulo $2^k$, taking the
    /// vector by reference, and returning `None` if some element has no quotient. The elements and
    /// $c$ must already be reduced modulo $2^k$.
    ///
    /// Let $s$ be the number of trailing zeros of $c$. An element $x$ has a quotient if and only if
    /// $x = 0$, or $c \neq 0$ and $2^s$ divides $x$; the result is `None` if some element has none.
    /// Otherwise each element of the result is the smallest quotient, the one that
    /// [`ModPowerOf2Div`] returns for that element alone, $(x/2^s)(c/2^s)^{-1} \bmod 2^{k-s}$. The
    /// inverse is computed once and shared by all the elements.
    ///
    /// $f(v, c, k) = \operatorname{Some}(w)$, where each $w_i$ is the least $w_i < 2^k$ with $cw_i
    /// \equiv v_i \mod 2^k$, if such $w_i$ exist.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if `c` or any element of `self` is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_div(3, 4).unwrap().to_string(),
    ///     "(11, 6, 1)"
    /// );
    /// // 2 is not invertible modulo 16, but every element is even.
    /// let v = UnsignedVector::<u8>::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_div(2, 4).unwrap().to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = UnsignedVector::<u8>::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_div(2, 4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, c: T, pow: u64) -> Option<UnsignedVector<T>> {
        self.clone().mod_power_of_2_div(c, pow)
    }
}
