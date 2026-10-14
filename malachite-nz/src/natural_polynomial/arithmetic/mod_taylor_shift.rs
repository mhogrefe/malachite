// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::taylor_shift::taylor_shift_horner;
use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_mul::{
    mod_mul_full, mod_mul_word, mul_word_preferred,
};
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    DivExactAssign, DivModPrecomputed, ModAddAssign, ModAssign, ModIsReduced, ModMulPrecomputed,
    ModMulPrecomputedAssign, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{ModTaylorShift, ModTaylorShiftAssign};

// The length from which splitting beats Horner's rule, as measured on this machine.
//
// - For c = 1, Horner's rule is the exact shift by 1, which is cheap while its partial sums fit in
//   a word or two, so the split starts at about 100 for a modulus smaller than a limb, and moves
//   out to a few hundred for larger moduli.
// - For any other c and a modulus that fits in a limb, Horner's rule uses word arithmetic with
//   precomputed modular multiplication, and the split wins from about 50 to 160, later for a
//   modulus that fills the limb.
// - For any other c and a larger modulus, Horner's rule multiplies `Natural`s modulo m at every
//   step, and the split wins almost at once.
fn split_cutoff(c: &Natural, m: &Natural) -> usize {
    let bits = m.significant_bits();
    if *c == 1u32 {
        if bits < Limb::WIDTH {
            96
        } else if bits == Limb::WIDTH {
            160
        } else if bits <= 128 {
            256
        } else {
            384
        }
    } else if bits <= 32 {
        48
    } else if bits < Limb::WIDTH {
        80
    } else if bits == Limb::WIDTH {
        160
    } else if bits <= 1024 {
        16
    } else {
        48
    }
}

// Shifts the polynomial with coefficients `xs`, all reduced modulo `m`, by `c`, reduced modulo `m`,
// with Horner's rule, keeping the coefficients reduced.
crate_test_fn! {mod_taylor_shift_horner(xs: &mut [Natural], c: &Natural, m: &Natural) {
    let n = xs.len();
    if n <= 1 || *c == 0u32 {
        return;
    }
    if *c == 1u32 {
        // The partial sums grow by at most n bits, so the exact shift by 1, with its word, double
        // word, and limb-array paths, is cheap; the result is reduced once at the end.
        let mut ys: Vec<Integer> = xs.iter_mut().map(|x| Integer::from(take(x))).collect();
        taylor_shift_horner(&mut ys, &Integer::ONE);
        for (x, y) in xs.iter_mut().zip(ys) {
            *x = y.unsigned_abs();
            x.mod_assign(m);
        }
    } else if m.significant_bits() <= Limb::WIDTH {
        // Every step multiplies by the same c modulo the same m, so the data for that
        // multiplication is computed once.
        let m = Limb::exact_from(m);
        let c = Limb::exact_from(c);
        let data = Limb::precompute_mod_mul_data(&m);
        let mut ws: Vec<Limb> = xs.iter().map(Limb::exact_from).collect();
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let product = c.mod_mul_precomputed(ws[j + 1], m, &data);
                ws[j].mod_add_assign(product, m);
            }
        }
        for (x, w) in xs.iter_mut().zip(ws) {
            *x = Natural::from(w);
        }
    } else {
        let data: ModMulData =
            <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let (lo, hi) = xs.split_at_mut(j + 1);
                let product = (&hi[0]).mod_mul_precomputed(c, m, &data);
                lo[j].mod_add_assign(product, m);
            }
        }
    }
}}

// Shifts the polynomial with coefficients `xs`, all reduced modulo `m`, by `c`, reduced modulo `m`,
// by splitting it as $p_0 + x^k p_1$ and returning $p_0(x + c) + (x + c)^k p_1(x + c)$ modulo $m$,
// with both halves shifted recursively.
crate_test_fn! {mod_taylor_shift_divide_and_conquer(xs: &mut [Natural], c: &Natural, m: &Natural) {
    if *c == 0u32 || xs.len() < split_cutoff(c, m) {
        mod_taylor_shift_horner(xs, c, m);
    } else {
        mod_taylor_shift_divide_and_conquer_split(xs, c, m);
    }
}}

// The divide-and-conquer step of `mod_taylor_shift_divide_and_conquer`, applied to `xs` whatever
// its length, with the halves shifted by `mod_taylor_shift_divide_and_conquer`. `xs` must have
// length at least 2.
crate_test_fn! {mod_taylor_shift_divide_and_conquer_split(
    xs: &mut [Natural],
    c: &Natural,
    m: &Natural,
) {
    let len = xs.len();
    let len_lo = len >> 1;
    let (xs_lo, xs_hi) = xs.split_at_mut(len_lo);
    mod_taylor_shift_divide_and_conquer(xs_lo, c, m);
    mod_taylor_shift_divide_and_conquer(xs_hi, c, m);
    // The coefficients of (x + c)^len_lo modulo m: the binomial coefficients are computed exactly,
    // one from the last, and reduced as they are stored, since dividing modulo m would need k to be
    // invertible.
    let mut binomials = Vec::with_capacity(len_lo + 1);
    let mut binomial = Natural::ONE;
    binomials.push(Natural::ONE);
    let data = Natural::precompute_div_mod_data(m);
    for k in 1..=len_lo {
        binomial *= Natural::from(len_lo + 1 - k);
        binomial.div_exact_assign(Natural::from(k));
        binomials.push((&binomial).div_mod_precomputed(m, &data).1);
    }
    if *c != 1u32 {
        // power is c^(len_lo - k) modulo m when it multiplies binomial k.
        let data: ModMulData =
            <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
        let mut power = c.clone();
        for b in binomials[..len_lo].iter_mut().rev() {
            b.mod_mul_precomputed_assign(&power, m, &data);
            power.mod_mul_precomputed_assign(c, m, &data);
        }
    }
    // The product has exactly len coefficients, since the factors have len_lo + 1 and len - len_lo.
    let mut product: Vec<Natural> = if mul_word_preferred(binomials.len(), xs_hi.len(), m) {
        mod_mul_word(&binomials, xs_hi, m)
            .into_iter()
            .map(Natural::from)
            .collect()
    } else {
        mod_mul_full(&binomials, xs_hi, m)
    };
    let (product_lo, product_hi) = product.split_at_mut(len_lo);
    for (x, y) in xs_lo.iter_mut().zip(product_lo) {
        x.mod_add_assign(take(y), m);
    }
    xs_hi.swap_with_slice(product_hi);
}}

fn assert_reduced(p: &NaturalPolynomial, c: &Natural, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(*c < *m, "c must be reduced mod m, but {c} >= {m}");
}

impl ModTaylorShift<&Natural, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking all three by reference. The coefficients and $c$ must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(&Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(&Natural::ONE, &Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    fn mod_taylor_shift(self, c: &Natural, m: &Natural) -> NaturalPolynomial {
        let mut p = self.clone();
        p.mod_taylor_shift_assign(c, m);
        p
    }
}

impl ModTaylorShift<&Natural, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking the polynomial by reference, $c$ by reference, and $m$ by value. The
    /// coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(&Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(&Natural::ONE, Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(self, c: &Natural, m: Natural) -> NaturalPolynomial {
        self.mod_taylor_shift(c, &m)
    }
}

impl ModTaylorShift<Natural, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking the polynomial by reference, $c$ by value, and $m$ by reference. The
    /// coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(Natural::ONE, &Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(self, c: Natural, m: &Natural) -> NaturalPolynomial {
        self.mod_taylor_shift(&c, m)
    }
}

impl ModTaylorShift<Natural, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking the polynomial by reference, $c$ by value, and $m$ by value. The
    /// coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     (&p).mod_taylor_shift(Natural::ONE, Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(self, c: Natural, m: Natural) -> NaturalPolynomial {
        self.mod_taylor_shift(&c, &m)
    }
}

impl ModTaylorShift<&Natural, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking the polynomial by value, $c$ by reference, and $m$ by reference. The
    /// coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(&Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(&Natural::ONE, &Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(mut self, c: &Natural, m: &Natural) -> Self {
        self.mod_taylor_shift_assign(c, m);
        self
    }
}

impl ModTaylorShift<&Natural, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking the polynomial by value, $c$ by reference, and $m$ by value. The
    /// coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(&Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(&Natural::ONE, Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(mut self, c: &Natural, m: Natural) -> Self {
        self.mod_taylor_shift_assign(c, &m);
        self
    }
}

impl ModTaylorShift<Natural, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking the polynomial by value, $c$ by value, and $m$ by reference. The
    /// coefficients and $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(Natural::from(3u32), &Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(Natural::ONE, &Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(mut self, c: Natural, m: &Natural) -> Self {
        self.mod_taylor_shift_assign(&c, m);
        self
    }
}

impl ModTaylorShift<Natural, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$, replacing $x$
    /// with $x + c$, taking all three by value. The coefficients and $c$ must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(Natural::from(3u32), Natural::from(7u32))
    ///         .to_string(),
    ///     "x^2+2*x+6"
    /// );
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let p = NaturalPolynomial::from_str("x^4").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_taylor_shift(Natural::ONE, Natural::from(4u32))
    ///         .to_string(),
    ///     "x^4+2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_taylor_shift(mut self, c: Natural, m: Natural) -> Self {
        self.mod_taylor_shift_assign(&c, &m);
        self
    }
}

impl ModTaylorShiftAssign<&Natural, &Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$ in place,
    /// replacing $x$ with $x + c$, taking $c$ and $m$ by reference. The coefficients and $c$ must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_taylor_shift_assign(&Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^2+2*x+6");
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let mut p = NaturalPolynomial::from_str("x^4").unwrap();
    /// p.mod_taylor_shift_assign(&Natural::ONE, &Natural::from(4u32));
    /// assert_eq!(p.to_string(), "x^4+2*x^2+1");
    /// ```
    fn mod_taylor_shift_assign(&mut self, c: &Natural, m: &Natural) {
        assert_reduced(self, c, m);
        mod_taylor_shift_divide_and_conquer(&mut self.coefficients, c, m);
    }
}

impl ModTaylorShiftAssign<&Natural, Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$ in place,
    /// replacing $x$ with $x + c$, taking $c$ by reference and $m$ by value. The coefficients and
    /// $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_taylor_shift_assign(&Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^2+2*x+6");
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let mut p = NaturalPolynomial::from_str("x^4").unwrap();
    /// p.mod_taylor_shift_assign(&Natural::ONE, Natural::from(4u32));
    /// assert_eq!(p.to_string(), "x^4+2*x^2+1");
    /// ```
    #[inline]
    fn mod_taylor_shift_assign(&mut self, c: &Natural, m: Natural) {
        self.mod_taylor_shift_assign(c, &m);
    }
}

impl ModTaylorShiftAssign<Natural, &Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$ in place,
    /// replacing $x$ with $x + c$, taking $c$ by value and $m$ by reference. The coefficients and
    /// $c$ must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_taylor_shift_assign(Natural::from(3u32), &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^2+2*x+6");
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let mut p = NaturalPolynomial::from_str("x^4").unwrap();
    /// p.mod_taylor_shift_assign(Natural::ONE, &Natural::from(4u32));
    /// assert_eq!(p.to_string(), "x^4+2*x^2+1");
    /// ```
    #[inline]
    fn mod_taylor_shift_assign(&mut self, c: Natural, m: &Natural) {
        self.mod_taylor_shift_assign(&c, m);
    }
}

impl ModTaylorShiftAssign<Natural, Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo a [`Natural`] $m$ in place,
    /// replacing $x$ with $x + c$, taking $c$ and $m$ by value. The coefficients and $c$ must
    /// already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod m.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond the size of $m$,
    /// whatever the length of the polynomial. Long polynomials are split in two, both halves are
    /// shifted recursively, and the high half is multiplied by $(x + c)^k$ modulo $m$, whose
    /// binomial coefficients are computed exactly and then reduced, so nothing is ever divided
    /// modulo $m$ and any modulus works, prime or not. Short polynomials are shifted with Horner's
    /// rule, which for $c = 1$ needs only additions and for a modulus that fits in a limb uses word
    /// arithmetic. The leading coefficient does not change, so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if `c` or any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModTaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_taylor_shift_assign(Natural::from(3u32), Natural::from(7u32));
    /// assert_eq!(p.to_string(), "x^2+2*x+6");
    ///
    /// // Modulo 4, which is not prime, the binomial coefficients 4 vanish.
    /// let mut p = NaturalPolynomial::from_str("x^4").unwrap();
    /// p.mod_taylor_shift_assign(Natural::ONE, Natural::from(4u32));
    /// assert_eq!(p.to_string(), "x^4+2*x^2+1");
    /// ```
    #[inline]
    fn mod_taylor_shift_assign(&mut self, c: Natural, m: Natural) {
        self.mod_taylor_shift_assign(&c, &m);
    }
}
