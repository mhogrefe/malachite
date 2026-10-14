// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::natural_vector::arithmetic::mod_mul::{mod_mul_assign_unchecked, mod_mul_unchecked};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModDiv, ModInverse, ModIsReduced};

fn assert_reduced(v: &NaturalVector, c: &Natural, m: &Natural) {
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        c.mod_is_reduced(m),
        "c must be reduced mod m, but {c} >= {m}"
    );
}

// The inverse of `c` modulo `m`, or `None` if `c` is not invertible. Modulo 1, 0 is invertible, but
// `mod_inverse` does not accept 0, so 0 always takes the element-by-element path, which is correct
// for every modulus.
fn inverse(c: &Natural, m: &Natural) -> Option<Natural> {
    if *c == 0u32 { None } else { c.mod_inverse(m) }
}

// Each element divided by `c` modulo `m`, one element at a time, or `None` if some element has no
// quotient.
fn mod_div_elementwise(v: &NaturalVector, c: &Natural, m: &Natural) -> Option<Vec<Natural>> {
    v.elements.iter().map(|x| x.mod_div(c, m)).collect()
}

fn mod_div_val(mut v: NaturalVector, c: &Natural, m: &Natural) -> Option<NaturalVector> {
    assert_reduced(&v, c, m);
    if let Some(inverse) = inverse(c, m) {
        mod_mul_assign_unchecked(&mut v, &inverse, m);
    } else {
        v.elements = mod_div_elementwise(&v, c, m)?;
    }
    Some(v)
}

fn mod_div_ref(v: &NaturalVector, c: &Natural, m: &Natural) -> Option<NaturalVector> {
    assert_reduced(v, c, m);
    if let Some(inverse) = inverse(c, m) {
        Some(mod_mul_unchecked(v, &inverse, m))
    } else {
        Some(NaturalVector {
            elements: mod_div_elementwise(v, c, m)?,
        })
    }
}

impl ModDiv<Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by value, $c$ by value, and $m$ by value, and returning `None` if
    /// some element has no quotient. The elements and $c$ must already be reduced modulo $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(Natural::from(3u32), Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(Natural::TWO, Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_div(Natural::TWO, Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: Natural, m: Natural) -> Option<Self> {
        mod_div_val(self, &c, &m)
    }
}

impl ModDiv<Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by value, $c$ by value, and $m$ by reference, and returning `None` if
    /// some element has no quotient. The elements and $c$ must already be reduced modulo $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(Natural::from(3u32), &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(Natural::TWO, &Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_div(Natural::TWO, &Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: Natural, m: &Natural) -> Option<Self> {
        mod_div_val(self, &c, m)
    }
}

impl ModDiv<&Natural, Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by value, $c$ by reference, and $m$ by value, and returning `None` if
    /// some element has no quotient. The elements and $c$ must already be reduced modulo $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(&Natural::from(3u32), Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(&Natural::TWO, Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_div(&Natural::TWO, Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: &Natural, m: Natural) -> Option<Self> {
        mod_div_val(self, c, &m)
    }
}

impl ModDiv<&Natural, &Natural> for NaturalVector {
    type Output = Self;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by value, $c$ by reference, and $m$ by reference, and returning
    /// `None` if some element has no quotient. The elements and $c$ must already be reduced modulo
    /// $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(&Natural::from(3u32), &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     v.clone()
    ///         .mod_div(&Natural::TWO, &Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_div(&Natural::TWO, &Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: &Natural, m: &Natural) -> Option<Self> {
        mod_div_val(self, c, m)
    }
}

impl ModDiv<Natural, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by reference, $c$ by value, and $m$ by value, and returning `None` if
    /// some element has no quotient. The elements and $c$ must already be reduced modulo $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(Natural::from(3u32), Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(Natural::TWO, Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_div(Natural::TWO, Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: Natural, m: Natural) -> Option<NaturalVector> {
        mod_div_ref(self, &c, &m)
    }
}

impl ModDiv<Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by reference, $c$ by value, and $m$ by reference, and returning
    /// `None` if some element has no quotient. The elements and $c$ must already be reduced modulo
    /// $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(Natural::from(3u32), &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(Natural::TWO, &Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_div(Natural::TWO, &Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: Natural, m: &Natural) -> Option<NaturalVector> {
        mod_div_ref(self, &c, m)
    }
}

impl ModDiv<&Natural, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by reference, $c$ by reference, and $m$ by value, and returning
    /// `None` if some element has no quotient. The elements and $c$ must already be reduced modulo
    /// $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(&Natural::from(3u32), Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(&Natural::TWO, Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_div(&Natural::TWO, Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: &Natural, m: Natural) -> Option<NaturalVector> {
        mod_div_ref(self, c, &m)
    }
}

impl ModDiv<&Natural, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Divides every element of a [`NaturalVector`] by a [`Natural`] $c$ modulo another [`Natural`]
    /// $m$, taking the vector by reference, $c$ by reference, and $m$ by reference, and returning
    /// `None` if some element has no quotient. The elements and $c$ must already be reduced modulo
    /// $m$.
    ///
    /// If $c$ is invertible modulo $m$, which is the usual case, every element has exactly one
    /// quotient, and this computes the inverse of $c$ once and multiplies every element by it.
    /// Otherwise an element $x$ has a quotient if and only if $\gcd(c, m)$ divides $x$; the result
    /// is `None` if some element has none, and otherwise each element of the result is the quotient
    /// that [`ModDiv`] returns for that element alone.
    ///
    /// $f(v, c, m) = \operatorname{Some}(w)$, where every $w_i < m$ and $cw_i \equiv v_i \mod m$,
    /// if such $w_i$ exist.
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
    /// Panics if `m` is zero, or if `c` or any element of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDiv;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(&Natural::from(3u32), &Natural::from(7u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(5, 3, 1)"
    /// );
    /// // 2 is not invertible modulo 6, but every element is even.
    /// let v = NaturalVector::from_str("(2, 4, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_div(&Natural::TWO, &Natural::from(6u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "(1, 2, 0)"
    /// );
    /// let v = NaturalVector::from_str("(2, 3)").unwrap();
    /// assert_eq!((&v).mod_div(&Natural::TWO, &Natural::from(6u32)), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_scalar_div_fmpz_mod` from `fmpz_mod_vec/scalar.c`,
    /// FLINT 3.6.0, when $c$ is invertible; FLINT throws an exception otherwise.
    #[inline]
    fn mod_div(self, c: &Natural, m: &Natural) -> Option<NaturalVector> {
        mod_div_ref(self, c, m)
    }
}
