// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Div, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul,
};

fn assert_valid(v: &NaturalVector, c: &Natural, pow: u64) {
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

// Divides every element of `v` by `c` modulo `2^pow`, as `mod_power_of_2_div` does element by
// element, or returns `None` if some element has no quotient. With `s` the number of trailing zeros
// of `c`, every quotient is `(x >> s) * (c >> s)^{-1} mod 2^(pow - s)`, so the inverse is computed
// only once.
fn mod_power_of_2_div_helper(v: &NaturalVector, c: &Natural, pow: u64) -> Option<NaturalVector> {
    assert_valid(v, c, pow);
    let Some(s) = c.trailing_zeros() else {
        // Only 0 has a quotient, and it is 0.
        return if v.elements.iter().all(|x| *x == 0u32) {
            Some(v.clone())
        } else {
            None
        };
    };
    if v.elements
        .iter()
        .any(|x| x.trailing_zeros().is_some_and(|t| t < s))
    {
        return None;
    }
    // c is nonzero and less than 2^pow, so s < pow.
    let k = pow - s;
    let inverse = (c >> s).mod_power_of_2_inverse(k).unwrap();
    Some(NaturalVector {
        elements: v
            .elements
            .iter()
            .map(|x| (x >> s).mod_power_of_2_mul(&inverse, k))
            .collect(),
    })
}

impl ModPowerOf2Div<Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo $2^k$, taking the
    /// vector by value and $c$ by value, and returning `None` if some element has no quotient. The
    /// elements and $c$ must already be reduced modulo $2^k$.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `c` or any element of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_power_of_2_div(Natural::from(3u32), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(11, 6, 1)"
    /// );
    /// // 2 is not invertible modulo 16, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_power_of_2_div(Natural::TWO, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_div(Natural::TWO, 4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, c: Natural, pow: u64) -> Option<Self> {
        mod_power_of_2_div_helper(&self, &c, pow)
    }
}

impl ModPowerOf2Div<&Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo $2^k$, taking the
    /// vector by value and $c$ by reference, and returning `None` if some element has no quotient.
    /// The elements and $c$ must already be reduced modulo $2^k$.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `c` or any element of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_power_of_2_div(&Natural::from(3u32), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(11, 6, 1)"
    /// );
    /// // 2 is not invertible modulo 16, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_power_of_2_div(&Natural::TWO, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_div(&Natural::TWO, 4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, c: &Natural, pow: u64) -> Option<Self> {
        mod_power_of_2_div_helper(&self, c, pow)
    }
}

impl ModPowerOf2Div<Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo $2^k$, taking the
    /// vector by reference and $c$ by value, and returning `None` if some element has no quotient.
    /// The elements and $c$ must already be reduced modulo $2^k$.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `c` or any element of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_div(Natural::from(3u32), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(11, 6, 1)"
    /// );
    /// // 2 is not invertible modulo 16, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_div(Natural::TWO, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_div(Natural::TWO, 4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, c: Natural, pow: u64) -> Option<NaturalVector> {
        mod_power_of_2_div_helper(self, &c, pow)
    }
}

impl ModPowerOf2Div<&Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo $2^k$, taking the
    /// vector by reference and $c$ by reference, and returning `None` if some element has no
    /// quotient. The elements and $c$ must already be reduced modulo $2^k$.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `c` or any element of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Div;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_div(&Natural::from(3u32), 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(11, 6, 1)"
    /// );
    /// // 2 is not invertible modulo 16, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_div(&Natural::TWO, 4)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_div(&Natural::TWO, 4), None);
    /// ```
    #[inline]
    fn mod_power_of_2_div(self, c: &Natural, pow: u64) -> Option<NaturalVector> {
        mod_power_of_2_div_helper(self, c, pow)
    }
}
