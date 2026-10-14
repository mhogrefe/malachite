// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::taylor_shift::taylor_shift_horner;
use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::mod_power_of_2_mul_full;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::max;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    DivExactAssign, ModPowerOf2, ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2MulAssign, UnsignedAbs, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::polynomial::{ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign};

// The length from which splitting beats Horner's rule, as measured on this machine.
//
// - For c = 1, Horner's rule is the exact shift by 1, which slows down several-fold once its
//   partial sums no longer fit in two words, so for a word-sized modulus the split starts there;
//   for larger moduli the crossover moves out slowly.
// - For any other c and a word-sized modulus, Horner's rule is wrapping word arithmetic, so cheap
//   that it is faster than splitting up to lengths of several hundred to over a thousand, growing
//   with the modulus.
// - For any other c and a larger modulus, Horner's rule does a modular multiplication of `Natural`s
//   per step, and splitting pays almost at once.
fn split_cutoff(c: &Natural, pow: u64) -> usize {
    let cutoff = if *c == 1u32 {
        if pow <= Limb::WIDTH {
            max(64, TWICE_WIDTH - pow)
        } else if pow <= 256 {
            192
        } else {
            320
        }
    } else if pow <= Limb::WIDTH {
        448 + 13 * pow
    } else if pow <= 1024 {
        32
    } else {
        64
    };
    usize::wrapping_from(cutoff)
}

// Shifts the polynomial with coefficients `xs`, all reduced modulo `2^pow`, by `c`, reduced modulo
// `2^pow`, with Horner's rule, keeping the coefficients reduced.
crate_test_fn! {mod_power_of_2_taylor_shift_horner(xs: &mut [Natural], c: &Natural, pow: u64) {
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
            *x = y.unsigned_abs().mod_power_of_2(pow);
        }
    } else if pow <= Limb::WIDTH {
        // Arithmetic modulo 2^Limb::WIDTH is wrapping arithmetic, and 2^pow divides 2^Limb::WIDTH,
        // so reducing at the end gives the result modulo 2^pow.
        let c = Limb::wrapping_from(c);
        let mut ws: Vec<Limb> = xs.iter().map(Limb::wrapping_from).collect();
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let product = c.wrapping_mul(ws[j + 1]);
                ws[j].wrapping_add_assign(product);
            }
        }
        for (x, w) in xs.iter_mut().zip(ws) {
            *x = Natural::from(w.mod_power_of_2(pow));
        }
    } else {
        for i in (0..n - 1).rev() {
            for j in i..n - 1 {
                let (lo, hi) = xs.split_at_mut(j + 1);
                let product = (&hi[0]).mod_power_of_2_mul(c, pow);
                lo[j].mod_power_of_2_add_assign(product, pow);
            }
        }
    }
}}

// Shifts the polynomial with coefficients `xs`, all reduced modulo `2^pow`, by `c`, reduced modulo
// `2^pow`, by splitting it as $p_0 + x^m p_1$ and returning $p_0(x + c) + (x + c)^m p_1(x + c)$
// modulo $2^k$, with both halves shifted recursively.
crate_test_fn! {mod_power_of_2_taylor_shift_divide_and_conquer(
    xs: &mut [Natural],
    c: &Natural,
    pow: u64,
) {
    let len = xs.len();
    if *c == 0u32 || len < split_cutoff(c, pow) {
        mod_power_of_2_taylor_shift_horner(xs, c, pow);
        return;
    }
    mod_power_of_2_taylor_shift_divide_and_conquer_split(xs, c, pow);
}}

// The divide-and-conquer step of `mod_power_of_2_taylor_shift_divide_and_conquer`, applied to `xs`
// whatever its length, with the halves shifted by `mod_power_of_2_taylor_shift_divide_and_conquer`.
// `xs` must have length at least 2.
crate_test_fn! {mod_power_of_2_taylor_shift_divide_and_conquer_split(
    xs: &mut [Natural],
    c: &Natural,
    pow: u64,
) {
    let len = xs.len();
    let len_lo = len >> 1;
    let (xs_lo, xs_hi) = xs.split_at_mut(len_lo);
    mod_power_of_2_taylor_shift_divide_and_conquer(xs_lo, c, pow);
    mod_power_of_2_taylor_shift_divide_and_conquer(xs_hi, c, pow);
    // The coefficients of (x + c)^len_lo modulo 2^pow: the binomial coefficients are computed
    // exactly, one from the last, and reduced as they are stored.
    let mut binomials = Vec::with_capacity(len_lo + 1);
    let mut binomial = Natural::ONE;
    binomials.push(Natural::ONE.mod_power_of_2(pow));
    for k in 1..=len_lo {
        binomial *= Natural::from(len_lo + 1 - k);
        binomial.div_exact_assign(Natural::from(k));
        binomials.push((&binomial).mod_power_of_2(pow));
    }
    if *c != 1u32 {
        // power is c^(len_lo - k) modulo 2^pow when it multiplies binomial k.
        let mut power = c.clone();
        for b in binomials[..len_lo].iter_mut().rev() {
            b.mod_power_of_2_mul_assign(&power, pow);
            power.mod_power_of_2_mul_assign(c, pow);
        }
    }
    let mut product = mod_power_of_2_mul_full(&binomials, xs_hi, pow);
    let (product_lo, product_hi) = product.split_at_mut(len_lo);
    for (x, y) in xs_lo.iter_mut().zip(product_lo) {
        x.mod_power_of_2_add_assign(take(y), pow);
    }
    xs_hi.swap_with_slice(product_hi);
}}

fn assert_reduced(p: &NaturalPolynomial, c: &Natural, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

impl ModPowerOf2TaylorShift<&Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo $2^k$, replacing $x$ with $x +
    /// c$, taking both by reference. The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond $k$ bits, whatever
    /// the length of the polynomial and the size of $c$. Long polynomials are split in two, both
    /// halves are shifted recursively, and the high half is multiplied by $(x + c)^m$ modulo $2^k$;
    /// short ones are shifted with Horner's rule, which for $c = 1$ needs only additions and for a
    /// word-sized modulus uses wrapping word arithmetic. The leading coefficient does not change,
    /// so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `pow`, the size
    /// to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `c` or any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_taylor_shift(&Natural::from(10u32), 4)
    ///         .to_string(),
    ///     "x^2+7*x+4"
    /// );
    ///
    /// // Modulo 2, the binomial coefficients reduce to the parity of Pascal's triangle.
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_taylor_shift(&Natural::ONE, 1)
    ///         .to_string(),
    ///     "x^3+x^2+x+1"
    /// );
    /// ```
    fn mod_power_of_2_taylor_shift(self, c: &Natural, pow: u64) -> NaturalPolynomial {
        let mut p = self.clone();
        p.mod_power_of_2_taylor_shift_assign(c, pow);
        p
    }
}

impl ModPowerOf2TaylorShift<Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo $2^k$, replacing $x$ with $x +
    /// c$, taking the polynomial by reference and $c$ by value. The coefficients and $c$ must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond $k$ bits, whatever
    /// the length of the polynomial and the size of $c$. Long polynomials are split in two, both
    /// halves are shifted recursively, and the high half is multiplied by $(x + c)^m$ modulo $2^k$;
    /// short ones are shifted with Horner's rule, which for $c = 1$ needs only additions and for a
    /// word-sized modulus uses wrapping word arithmetic. The leading coefficient does not change,
    /// so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `pow`, the size
    /// to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `c` or any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_taylor_shift(Natural::from(10u32), 4)
    ///         .to_string(),
    ///     "x^2+7*x+4"
    /// );
    ///
    /// // Modulo 2, the binomial coefficients reduce to the parity of Pascal's triangle.
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     (&p).mod_power_of_2_taylor_shift(Natural::ONE, 1)
    ///         .to_string(),
    ///     "x^3+x^2+x+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_taylor_shift(self, c: Natural, pow: u64) -> NaturalPolynomial {
        self.mod_power_of_2_taylor_shift(&c, pow)
    }
}

impl ModPowerOf2TaylorShift<&Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo $2^k$, replacing $x$ with $x +
    /// c$, taking the polynomial by value and $c$ by reference. The coefficients and $c$ must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond $k$ bits, whatever
    /// the length of the polynomial and the size of $c$. Long polynomials are split in two, both
    /// halves are shifted recursively, and the high half is multiplied by $(x + c)^m$ modulo $2^k$;
    /// short ones are shifted with Horner's rule, which for $c = 1$ needs only additions and for a
    /// word-sized modulus uses wrapping word arithmetic. The leading coefficient does not change,
    /// so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `pow`, the size
    /// to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `c` or any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_power_of_2_taylor_shift(&Natural::from(10u32), 4)
    ///         .to_string(),
    ///     "x^2+7*x+4"
    /// );
    ///
    /// // Modulo 2, the binomial coefficients reduce to the parity of Pascal's triangle.
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_power_of_2_taylor_shift(&Natural::ONE, 1)
    ///         .to_string(),
    ///     "x^3+x^2+x+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_taylor_shift(mut self, c: &Natural, pow: u64) -> Self {
        self.mod_power_of_2_taylor_shift_assign(c, pow);
        self
    }
}

impl ModPowerOf2TaylorShift<Natural> for NaturalPolynomial {
    type Output = Self;

    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo $2^k$, replacing $x$ with $x +
    /// c$, taking both by value. The coefficients and $c$ must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond $k$ bits, whatever
    /// the length of the polynomial and the size of $c$. Long polynomials are split in two, both
    /// halves are shifted recursively, and the high half is multiplied by $(x + c)^m$ modulo $2^k$;
    /// short ones are shifted with Horner's rule, which for $c = 1$ needs only additions and for a
    /// word-sized modulus uses wrapping word arithmetic. The leading coefficient does not change,
    /// so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `pow`, the size
    /// to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `c` or any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShift;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_power_of_2_taylor_shift(Natural::from(10u32), 4)
    ///         .to_string(),
    ///     "x^2+7*x+4"
    /// );
    ///
    /// // Modulo 2, the binomial coefficients reduce to the parity of Pascal's triangle.
    /// let p = NaturalPolynomial::from_str("x^3").unwrap();
    /// assert_eq!(
    ///     p.clone()
    ///         .mod_power_of_2_taylor_shift(Natural::ONE, 1)
    ///         .to_string(),
    ///     "x^3+x^2+x+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_taylor_shift(mut self, c: Natural, pow: u64) -> Self {
        self.mod_power_of_2_taylor_shift_assign(&c, pow);
        self
    }
}

impl ModPowerOf2TaylorShiftAssign<&Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo $2^k$ in place, replacing $x$
    /// with $x + c$, taking $c$ by reference. The coefficients and $c$ must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond $k$ bits, whatever
    /// the length of the polynomial and the size of $c$. Long polynomials are split in two, both
    /// halves are shifted recursively, and the high half is multiplied by $(x + c)^m$ modulo $2^k$;
    /// short ones are shifted with Horner's rule, which for $c = 1$ needs only additions and for a
    /// word-sized modulus uses wrapping word arithmetic. The leading coefficient does not change,
    /// so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `pow`, the size
    /// to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `c` or any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_taylor_shift_assign(&Natural::from(10u32), 4);
    /// assert_eq!(p.to_string(), "x^2+7*x+4");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.mod_power_of_2_taylor_shift_assign(&Natural::ONE, 1);
    /// assert_eq!(p.to_string(), "x^3+x^2+x+1");
    /// ```
    fn mod_power_of_2_taylor_shift_assign(&mut self, c: &Natural, pow: u64) {
        assert_reduced(self, c, pow);
        mod_power_of_2_taylor_shift_divide_and_conquer(&mut self.coefficients, c, pow);
    }
}

impl ModPowerOf2TaylorShiftAssign<Natural> for NaturalPolynomial {
    /// Shifts a [`NaturalPolynomial`] by a [`Natural`] $c$ modulo $2^k$ in place, replacing $x$
    /// with $x + c$, taking $c$ by value. The coefficients and $c$ must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// p \\gets p(x + c) \\bmod 2^k.
    /// $$
    ///
    /// The coefficients are kept reduced throughout, so they never grow beyond $k$ bits, whatever
    /// the length of the polynomial and the size of $c$. Long polynomials are split in two, both
    /// halves are shifted recursively, and the high half is multiplied by $(x + c)^m$ modulo $2^k$;
    /// short ones are shifted with Horner's rule, which for $c = 1$ needs only additions and for a
    /// word-sized modulus uses wrapping word arithmetic. The leading coefficient does not change,
    /// so neither does the degree.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is `pow`, the size
    /// to which every coefficient is reduced throughout.
    ///
    /// # Panics
    /// Panics if `c` or any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::ModPowerOf2TaylorShiftAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_taylor_shift_assign(Natural::from(10u32), 4);
    /// assert_eq!(p.to_string(), "x^2+7*x+4");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3").unwrap();
    /// p.mod_power_of_2_taylor_shift_assign(Natural::ONE, 1);
    /// assert_eq!(p.to_string(), "x^3+x^2+x+1");
    /// ```
    #[inline]
    fn mod_power_of_2_taylor_shift_assign(&mut self, c: Natural, pow: u64) {
        self.mod_power_of_2_taylor_shift_assign(&c, pow);
    }
}
