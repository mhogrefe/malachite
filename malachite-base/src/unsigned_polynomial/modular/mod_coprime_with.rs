// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::fail_on_untested_path;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::modular::traits::ModCoprimeWith;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_gcd::{
    GcdCutoffs, GcdModulus, assert_reduced, checked_monic_gcd, normalize,
};
use crate::unsigned_polynomial::modular::subproduct_tree::{Modulus, TreeModulus};
use alloc::vec::Vec;
use core::cell::RefCell;

// Whether the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long as `ys`
// and neither ends with a zero, are coprime, by Euclid's algorithm; or `Err(c)`, where `c` is a
// nonzero leading coefficient met along the way that has no inverse.
fn coprime_euclidean<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Result<bool, C> {
    let mut xs = xs.to_vec();
    let mut ys = ys.to_vec();
    loop {
        let Some(y) = ys.last() else {
            // The GCD is `xs`, which generates the whole ring exactly when it is an invertible
            // constant; when its leading coefficient has no inverse, nothing is known yet.
            return match xs.last() {
                None => Ok(false),
                Some(x) if m.inverse(x).is_some() => Ok(xs.len() == 1),
                Some(x) => Err(x.clone()),
            };
        };
        let Some(inverse) = m.inverse(y) else {
            return Err(y.clone());
        };
        if ys.len() == 1 {
            return Ok(true);
        }
        let mut rs = m.rem(&xs, &ys, &inverse);
        normalize(&mut rs, m);
        xs = core::mem::replace(&mut ys, rs);
    }
}

// The arithmetic of `inner`, recording the first nonzero element whose inverse was asked for and
// does not exist, so that a GCD algorithm that gives up on meeting it can say where it gave up.
struct RecordingModulus<'a, C, M> {
    inner: &'a M,
    non_invertible: RefCell<Option<C>>,
}

impl<C: Clone, M: GcdModulus<C>> TreeModulus<C> for RecordingModulus<'_, C, M> {
    #[inline]
    fn zero(&self) -> C {
        self.inner.zero()
    }

    #[inline]
    fn one(&self) -> C {
        self.inner.one()
    }

    #[inline]
    fn neg(&self, x: &C) -> C {
        self.inner.neg(x)
    }

    #[inline]
    fn add(&self, x: &C, y: &C) -> C {
        self.inner.add(x, y)
    }

    #[inline]
    fn sub(&self, x: &C, y: &C) -> C {
        self.inner.sub(x, y)
    }

    #[inline]
    fn mul(&self, x: &C, y: &C) -> C {
        self.inner.mul(x, y)
    }

    #[inline]
    fn mul_to_out(&self, out: &mut [C], xs: &[C], ys: &[C]) {
        self.inner.mul_to_out(out, xs, ys);
    }

    #[inline]
    fn rem_monic(&self, xs: &[C], ys: &[C]) -> Vec<C> {
        self.inner.rem_monic(xs, ys)
    }

    #[inline]
    fn reversed_inverse(&self, ys: &[C]) -> Vec<C> {
        self.inner.reversed_inverse(ys)
    }

    #[inline]
    fn rem_monic_preinv(&self, xs: &[C], ys: &[C], inverse: &[C]) -> Vec<C> {
        self.inner.rem_monic_preinv(xs, ys, inverse)
    }

    #[inline]
    fn evaluate(&self, poly: &[C], x: &C) -> C {
        self.inner.evaluate(poly, x)
    }
}

impl<C: Clone, M: GcdModulus<C>> GcdModulus<C> for RecordingModulus<'_, C, M> {
    #[inline]
    fn is_zero(&self, x: &C) -> bool {
        self.inner.is_zero(x)
    }

    fn inverse(&self, x: &C) -> Option<C> {
        let inverse = self.inner.inverse(x);
        if inverse.is_none() && !self.inner.is_zero(x) {
            self.non_invertible
                .borrow_mut()
                .get_or_insert_with(|| x.clone());
        }
        inverse
    }

    #[inline]
    fn rem(&self, xs: &[C], ys: &[C], inverse: &C) -> Vec<C> {
        self.inner.rem(xs, ys, inverse)
    }

    #[inline]
    fn div_rem(&self, xs: &[C], ys: &[C], inverse: &C) -> (Vec<C>, Vec<C>) {
        self.inner.div_rem(xs, ys, inverse)
    }

    #[inline]
    fn gcd_cutoffs(&self) -> GcdCutoffs {
        self.inner.gcd_cutoffs()
    }

    #[inline]
    fn mul_polynomials(&self, xs: &[C], ys: &[C]) -> Vec<C> {
        self.inner.mul_polynomials(xs, ys)
    }
}

/// Whether the polynomials with coefficients `xs` and `ys`, neither ending with a zero, are coprime
/// modulo `m`, or `Err(c)`, where `c` is a nonzero element met along the way that has no inverse.
/// Long polynomials use the half-GCD algorithm, and short ones Euclid's algorithm; Euclid's
/// algorithm is also used if the half-GCD algorithm gives up without meeting such an element.
///
/// This is not part of the public API; it is public so that `malachite-nz` can test coprimality of
/// `NaturalPolynomial`s.
#[doc(hidden)]
pub fn coprime_or_non_invertible<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    m: &M,
) -> Result<bool, C> {
    let (xs, ys) = if xs.len() < ys.len() {
        (ys, xs)
    } else {
        (xs, ys)
    };
    if ys.len() >= m.gcd_cutoffs().gcd_hgcd {
        let recording = RecordingModulus {
            inner: m,
            non_invertible: RefCell::new(None),
        };
        if let Some(g) = checked_monic_gcd(xs, ys, &recording) {
            return Ok(g.len() == 1);
        }
        if let Some(c) = recording.non_invertible.into_inner() {
            return Err(c);
        }
        // The GCD algorithms give up only on meeting a nonzero element with no inverse.
        fail_on_untested_path("coprime_or_non_invertible, no non-invertible element recorded");
    }
    coprime_euclidean(xs, ys, m)
}

// The polynomial with coefficients `xs`, reduced modulo `m`, without zeros at the end.
fn reduce<T: PrimitiveUnsigned>(xs: &[T], m: T) -> Vec<T> {
    let mut ys: Vec<T> = xs.iter().map(|&x| x % m).collect();
    while ys.last() == Some(&T::ZERO) {
        ys.pop();
    }
    ys
}

/// Whether the polynomials with coefficients `xs` and `ys`, reduced modulo `m` and neither ending
/// with a zero, are coprime modulo `m`. When a leading coefficient $c$ met along the way has no
/// inverse, $d = \gcd(c, m)$ is a nontrivial factor of $m$, and the polynomials are coprime modulo
/// $m$ exactly when they are coprime modulo $d$ and modulo $m/d$, since a prime divides $m$ exactly
/// when it divides one of them.
///
/// This is not part of the public API; it is public so that `malachite-nz` can test coprimality of
/// `NaturalPolynomial`s modulo word-sized numbers.
#[doc(hidden)]
pub fn mod_coprime_with_slices<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> bool {
    if m == T::ONE {
        return true;
    }
    match coprime_or_non_invertible(xs, ys, &Modulus(m)) {
        Ok(coprime) => coprime,
        Err(c) => {
            let d = c.gcd(m);
            let e = m / d;
            mod_coprime_with_slices(&reduce(xs, d), &reduce(ys, d), d)
                && mod_coprime_with_slices(&reduce(xs, e), &reduce(ys, e), e)
        }
    }
}

fn mod_coprime_with_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> bool {
    assert_reduced(p, q, m);
    mod_coprime_with_slices(&p.coefficients, &q.coefficients, m)
}

impl<T: PrimitiveUnsigned> ModCoprimeWith<Self, T> for UnsignedPolynomial<T> {
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $m$, taking both by value.
    /// The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found by Euclid's algorithm or, for long polynomials, first by the half-GCD
    /// algorithm. When a leading coefficient $c$ met along the way is not invertible modulo $m$, $d
    /// = \gcd(c, m)$ is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$
    /// exactly when they are coprime modulo $d$ and modulo $m/d$, which are tested in the same way;
    /// $m$ is never factored.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.mod_coprime_with(q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!p.mod_coprime_with(q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    /// assert!(p.mod_coprime_with(q, 4));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: Self, m: T) -> bool {
        mod_coprime_with_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModCoprimeWith<&Self, T> for UnsignedPolynomial<T> {
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $m$, taking the first by
    /// value and the second by reference. The coefficients of both must already be reduced modulo
    /// $m$.
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
    /// The GCD is found by Euclid's algorithm or, for long polynomials, first by the half-GCD
    /// algorithm. When a leading coefficient $c$ met along the way is not invertible modulo $m$, $d
    /// = \gcd(c, m)$ is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$
    /// exactly when they are coprime modulo $d$ and modulo $m/d$, which are tested in the same way;
    /// $m$ is never factored.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.mod_coprime_with(&q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!p.mod_coprime_with(&q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    /// assert!(p.mod_coprime_with(&q, 4));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: &Self, m: T) -> bool {
        mod_coprime_with_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModCoprimeWith<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $m$, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
    /// $m$.
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
    /// The GCD is found by Euclid's algorithm or, for long polynomials, first by the half-GCD
    /// algorithm. When a leading coefficient $c$ met along the way is not invertible modulo $m$, $d
    /// = \gcd(c, m)$ is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$
    /// exactly when they are coprime modulo $d$ and modulo $m/d$, which are tested in the same way;
    /// $m$ is never factored.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_coprime_with(q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_coprime_with(q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    /// assert!((&p).mod_coprime_with(q, 4));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: UnsignedPolynomial<T>, m: T) -> bool {
        mod_coprime_with_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModCoprimeWith<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $m$, taking both by
    /// reference. The coefficients of both must already be reduced modulo $m$.
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
    /// The GCD is found by Euclid's algorithm or, for long polynomials, first by the half-GCD
    /// algorithm. When a leading coefficient $c$ met along the way is not invertible modulo $m$, $d
    /// = \gcd(c, m)$ is a nontrivial factor of $m$, and the polynomials are coprime modulo $m$
    /// exactly when they are coprime modulo $d$ and modulo $m/d$, which are tested in the same way;
    /// $m$ is never factored.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModCoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_coprime_with(&q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_coprime_with(&q, 6));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("0").unwrap();
    /// assert!((&p).mod_coprime_with(&q, 4));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_coprime_with(self, other: &UnsignedPolynomial<T>, m: T) -> bool {
        mod_coprime_with_helper(self, other, m)
    }
}
