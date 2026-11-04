// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_div_rem::{assert_reduced, to_words};
use crate::natural_polynomial::modular::subproduct_tree::NaturalModulus;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::DivExact;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::Gcd;
use malachite_base::polynomial::modular::traits::ModCoprimeWith;
use malachite_base::unsigned_polynomial::modular::mod_coprime_with::{
    coprime_or_non_invertible, mod_coprime_with_slices,
};

// The polynomial with coefficients `xs`, reduced modulo `m`, without zeros at the end.
fn reduce(xs: &[Natural], m: &Natural) -> Vec<Natural> {
    let mut ys: Vec<Natural> = xs.iter().map(|x| x % m).collect();
    while ys.last().is_some_and(|y| *y == 0u32) {
        ys.pop();
    }
    ys
}

// Whether the polynomials with coefficients `xs` and `ys`, reduced modulo `m` and neither ending
// with a zero, are coprime modulo `m`. When `m` fits in one or two limbs, this is done in word
// arithmetic. When a leading coefficient $c$ met along the way has no inverse, $d = \gcd(c, m)$ is
// a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they are
// coprime modulo $d$ and modulo $m/d$.
fn mod_coprime_with_naturals(xs: &[Natural], ys: &[Natural], m: &Natural) -> bool {
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_coprime_with_slices(
            &to_words::<Limb>(xs),
            &to_words::<Limb>(ys),
            Limb::exact_from(m),
        )
    } else if bits <= DoubleLimb::WIDTH {
        mod_coprime_with_slices(
            &to_words::<DoubleLimb>(xs),
            &to_words::<DoubleLimb>(ys),
            DoubleLimb::exact_from(m),
        )
    } else {
        match coprime_or_non_invertible(xs, ys, &NaturalModulus::new(m)) {
            Ok(coprime) => coprime,
            Err(c) => {
                let d = c.gcd(m);
                let e = m.div_exact(&d);
                mod_coprime_with_naturals(&reduce(xs, &d), &reduce(ys, &d), &d)
                    && mod_coprime_with_naturals(&reduce(xs, &e), &reduce(ys, &e), &e)
            }
        }
    }
}

fn mod_coprime_with_helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> bool {
    assert_reduced(p, q, m);
    mod_coprime_with_naturals(&p.coefficients, &q.coefficients, m)
}

impl ModCoprimeWith<Self, Natural> for NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking both
    /// polynomials by value and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.mod_coprime_with(q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!p.mod_coprime_with(q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!(p.mod_coprime_with(q, Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: Self, m: Natural) -> bool {
        mod_coprime_with_helper(&self, &other, &m)
    }
}

impl ModCoprimeWith<Self, &Natural> for NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking both
    /// polynomials by value and the modulus by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.mod_coprime_with(q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!p.mod_coprime_with(q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!(p.mod_coprime_with(q, &Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: Self, m: &Natural) -> bool {
        mod_coprime_with_helper(&self, &other, m)
    }
}

impl ModCoprimeWith<&Self, Natural> for NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking the first
    /// polynomial by value, the second by reference, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.mod_coprime_with(&q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!p.mod_coprime_with(&q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!(p.mod_coprime_with(&q, Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: &Self, m: Natural) -> bool {
        mod_coprime_with_helper(&self, other, &m)
    }
}

impl ModCoprimeWith<&Self, &Natural> for NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking the first
    /// polynomial by value, the second by reference, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.mod_coprime_with(&q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!p.mod_coprime_with(&q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!(p.mod_coprime_with(&q, &Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: &Self, m: &Natural) -> bool {
        mod_coprime_with_helper(&self, other, m)
    }
}

impl ModCoprimeWith<NaturalPolynomial, Natural> for &NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking the first
    /// polynomial by reference, the second by value, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_coprime_with(q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_coprime_with(q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!((&p).mod_coprime_with(q, Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: NaturalPolynomial, m: Natural) -> bool {
        mod_coprime_with_helper(self, &other, &m)
    }
}

impl ModCoprimeWith<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking the first
    /// polynomial by reference, the second by value, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_coprime_with(q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_coprime_with(q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!((&p).mod_coprime_with(q, &Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: NaturalPolynomial, m: &Natural) -> bool {
        mod_coprime_with_helper(self, &other, m)
    }
}

impl ModCoprimeWith<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking both
    /// polynomials by reference and the modulus by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_coprime_with(&q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_coprime_with(&q, Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!((&p).mod_coprime_with(&q, Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: &NaturalPolynomial, m: Natural) -> bool {
        mod_coprime_with_helper(self, other, &m)
    }
}

impl ModCoprimeWith<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $m$, taking both
    /// polynomials by reference and the modulus by reference. The coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{m} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is prime, this is whether the GCD of $a$ and $b$ is 1. A polynomial is coprime with
    /// 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $m$ is 1, every polynomial is 0,
    /// and the result is `true`.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The GCD is
    /// found by Euclid's algorithm or, for long polynomials, first by the half-GCD algorithm. When
    /// a leading coefficient $c$ met along the way is not invertible modulo $m$, $d = \gcd(c, m)$
    /// is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$ exactly when they
    /// are coprime modulo $d$ and modulo $m/d$, which are tested in the same way; $m$ is never
    /// factored.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (n + \log m) m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n (m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_coprime_with(&q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_coprime_with(&q, &Natural::from(6u32)));
    ///
    /// let p = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("0").unwrap();
    /// assert!((&p).mod_coprime_with(&q, &Natural::from(4u32)));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: &NaturalPolynomial, m: &Natural) -> bool {
        mod_coprime_with_helper(self, other, m)
    }
}
