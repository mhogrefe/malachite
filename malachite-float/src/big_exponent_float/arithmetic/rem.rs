// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::round_optional;
use crate::big_exponent_float::arithmetic::add_rational::add_rational_helper;
use crate::big_exponent_float::arithmetic::round_to_integer::finite_odd_part;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{Abs, DivMod, ModPow, ModPowerOf2, Parity};
use malachite_base::num::basic::traits::{NaN as NaNTrait, Two, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// The number of low bits of the quotient that are reported.
const QUOTIENT_BITS: u64 = 63;

// c * 2^e mod m. An exponent of up to 3 times the modulus's length is cheapest to apply
// by shifting, as in MPFR's `mpfr_rem1`; a larger one is reduced by modular exponentiation, so that
// no number with e bits is formed.
fn mul_power_of_2_mod(c: Natural, e: &Natural, m: &Natural) -> Natural {
    if *e <= 3 * m.significant_bits() {
        (c << u64::exact_from(e)) % m
    } else if *m <= 2u32 {
        // m is 1 or 2, which divides 2^e, since e > 3.
        Natural::ZERO
    } else {
        c * Natural::TWO.mod_pow(e, m) % m
    }
}

// For finite nonzero x and y, the exact |x| mod |y| (the remainder of |x| / |y| with the quotient
// rounded toward zero) as a nonnegative `BigExponentFloat`, and that quotient modulo 2^`q_bits`.
//
// Write |x| = a * 2^s and |y| = b * 2^t with a and b odd. If s >= t, then |x| / |y| = a * 2^(s - t)
// / b, so the remainder is (a * 2^(s - t) mod b) * 2^t. To get the quotient q modulo 2^k as well,
// for k = `q_bits`, the reduction is done modulo M = b * 2^k: if a * 2^(s - t) = M * j + r', then
// q = 2^k * j + floor(r' / b), and the remainder is r' mod b. For a large gap s - t, 2^(s - t) mod
// M is computed by modular exponentiation, so that no number with s - t bits is formed. If s < t,
// then |x| >= |y| implies a >= b * 2^(t - s), so that t - s < bits(a), and the division a / (b *
// 2^(t - s)) is done directly; the remainder is then scaled by 2^s.
fn finite_rem_abs(
    x: &BigExponentFloat,
    y: &BigExponentFloat,
    q_bits: u64,
) -> (BigExponentFloat, u64) {
    if x.lt_abs(y) {
        return (x.abs(), 0);
    }
    let (a, s) = finite_odd_part(x);
    let (b, t) = finite_odd_part(y);
    let (r, q, scale) = if s >= t {
        let m = &b << q_bits;
        let r_prime = mul_power_of_2_mod(a, &Natural::exact_from(&(&s - &t)), &m);
        if q_bits == 0 {
            (r_prime, 0, t)
        } else {
            let (q, r) = r_prime.div_mod(&b);
            (r, u64::exact_from(&q), t)
        }
    } else {
        let gap = u64::exact_from(&(&t - &s));
        let b = b << gap;
        if q_bits == 0 {
            (a % b, 0, s)
        } else {
            let (q, r) = a.div_mod(b);
            (r, u64::exact_from(&q.mod_power_of_2(q_bits)), s)
        }
    };
    let r = if r == 0u32 {
        BigExponentFloat::ZERO
    } else {
        let exponent = scale + Integer::from(r.significant_bits());
        BigExponentFloat::from_parts(true, exponent, r)
    };
    (r, q)
}

// x - y * q for q = x / y rounded toward zero, or, if `nearest` is set, to the nearest integer with
// ties to even; rounded with `prec_rm` if it is `Some`, and exact otherwise. Also returns ±(|q|
// mod 2^`q_bits`), with q's sign; `q_bits` is at most 63, and at least 1 if `nearest` is set.
//
// The special cases are those of `Float`'s `rem`, except that there is no negative zero: if x or y
// is `NaN`, x is infinite, or y is zero, the result is `NaN`; if x is finite and y is infinite, the
// result is x (rounded); and if x is zero and y is not, the result is zero. In those cases the
// quotient bits are 0.
fn rem_with_quotient_bits(
    x: Cow<'_, BigExponentFloat>,
    y: &BigExponentFloat,
    nearest: bool,
    prec_rm: Option<(u64, RoundingMode)>,
    q_bits: u64,
) -> (BigExponentFloat, Ordering, i64) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    let (x_sign, y_sign) = match (&*x, y) {
        (BigExponentFloat(NaN | Infinity { .. }), _) | (_, BigExponentFloat(NaN | Zero)) => {
            return (BigExponentFloat::NAN, Equal, 0);
        }
        (BigExponentFloat(Zero), _) => return (BigExponentFloat::ZERO, Equal, 0),
        (_, BigExponentFloat(Infinity { .. })) => {
            let (r, o) = round_optional(x.into_owned(), prec_rm);
            return (r, o, 0);
        }
        (
            BigExponentFloat(Finite { sign: x_sign, .. }),
            BigExponentFloat(Finite { sign: y_sign, .. }),
        ) => (*x_sign, *y_sign),
    };
    let (mut r, mut q) = finite_rem_abs(&x, y, q_bits);
    if nearest && r != 0u32 {
        // The quotient rounds up in magnitude iff 2r > |y|, or 2r = |y| and the truncated quotient
        // is odd. Then the remainder becomes r - |y|, which is negative.
        let twice_r = &r << 1u32;
        let cmp = twice_r.partial_cmp_abs(y).unwrap();
        if cmp == Greater || cmp == Equal && q.odd() {
            r -= y.abs();
            q = (q + 1).mod_power_of_2(q_bits);
        }
    }
    if !x_sign {
        r = r.negate();
    }
    let q = i64::exact_from(q);
    let q = if x_sign == y_sign { q } else { -q };
    let (r, o) = round_optional(r, prec_rm);
    (r, o, q)
}

// x - y * q, as for `rem_with_quotient_bits`, computing only the quotient bits that are needed:
// none for a quotient rounded toward zero, and its parity, to break ties, for a quotient rounded to
// nearest.
pub(crate) fn rem_helper(
    x: Cow<'_, BigExponentFloat>,
    y: &BigExponentFloat,
    nearest: bool,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    let (r, o, _) = rem_with_quotient_bits(x, y, nearest, prec_rm, u64::from(nearest));
    (r, o)
}

// x mod n for a `u64` n, with the quotient rounded toward zero, rounded with `prec_rm` if it is
// `Some`, and exact otherwise.
pub(crate) fn rem_unsigned_helper(
    x: Cow<'_, BigExponentFloat>,
    n: u64,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    rem_helper(x, &BigExponentFloat::from(n), false, prec_rm)
}

// x * 2^k, for an `Integer` k. Values that are not finite and nonzero are returned unchanged.
fn mul_power_of_2_integer(mut x: BigExponentFloat, k: &Integer) -> BigExponentFloat {
    if let BigExponentFloat(Finite { exponent, .. }) = &mut x {
        *exponent += k;
    }
    x
}

// Given the remainder R = N mod A and the quotient's low bits q = (N div A) mod 2^`q_bits` of a
// division of magnitudes N / A, where |y| corresponds to A, returns the remainder for a quotient
// rounded toward zero or, if `nearest` is set, to the nearest integer with ties to even, as a signed
// multiple of the same unit, together with the adjusted quotient bits. Rounding the quotient up
// replaces R with R - A, which is negative.
fn nearest_quotient_adjust(
    r: Natural,
    a: &Natural,
    q: u64,
    nearest: bool,
    q_bits: u64,
) -> (Integer, u64) {
    let mut r = Integer::from(r);
    let mut q = q;
    if nearest && r != 0u32 {
        let cmp = (&r << 1u32).partial_cmp(a).unwrap();
        if cmp == Greater || cmp == Equal && q.odd() {
            r -= Integer::from(a);
            q = (q + 1).mod_power_of_2(q_bits);
        }
    }
    (r, q)
}

// The remainder R / den, with R as returned by `nearest_quotient_adjust`, given the sign of x; and
// the quotient bits, with the quotient's sign.
fn signed_rational_rem(
    x_sign: bool,
    y_sign: bool,
    r: Integer,
    den: Natural,
    q: u64,
) -> (Rational, i64) {
    let r = Rational::from_integers(if x_sign { r } else { -r }, Integer::from(den));
    let q = i64::exact_from(q);
    (r, if x_sign == y_sign { q } else { -q })
}

// x - y * q for a `BigExponentFloat` x, a `Rational` y, and q = x / y rounded toward zero or, if
// `nearest` is set, to the nearest integer with ties to even; rounded to `prec` bits with `rm`.
// Also returns ±(|q| mod 2^`q_bits`), with q's sign; `q_bits` is at most 63, and at least 1 if
// `nearest` is set.
//
// The special cases are those of `Float`'s `rem_rational`, except that there is no negative zero:
// if x is `NaN` or infinite, or y is zero, the result is `NaN`; and if x is zero and y is not, the
// result is zero. In those cases the quotient bits are 0.
//
// Write |x| = m * 2^s with m odd, and |y| = a / b in lowest terms. If |x| < |y|, the truncated
// quotient is 0, and the nearest one is ±1 exactly when 2|x| > |y|. Otherwise, if s >= 0, then |x|
// = m * b * 2^s / b, so the remainder is ((m * b * 2^s) mod a) / b; it is computed modulo a * 2^k,
// for k = `q_bits`, as in `finite_rem_abs`, so that a huge s costs only a modular exponentiation.
// If s < 0, then for t = -s, |x| = m * b / (b * 2^t) and |y| = a * 2^t / (b * 2^t); since |x| >=
// |y| >= 1 / b, 2^t <= m * b, so t is small, and the remainder ((m * b) mod (a * 2^t)) / (b * 2^t)
// is computed directly.
pub(crate) fn rem_rational_with_quotient_bits(
    x: Cow<'_, BigExponentFloat>,
    y: &Rational,
    nearest: bool,
    prec: u64,
    rm: RoundingMode,
    q_bits: u64,
) -> (BigExponentFloat, Ordering, i64) {
    assert_ne!(prec, 0);
    let x_sign = match &*x {
        BigExponentFloat(NaN | Infinity { .. }) => return (BigExponentFloat::NAN, Equal, 0),
        _ if *y == 0u32 => return (BigExponentFloat::NAN, Equal, 0),
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal, 0),
        BigExponentFloat(Finite { sign, .. }) => *sign,
    };
    let y_sign = *y > 0u32;
    if x.lt_abs(y) {
        if nearest && (&*x << 1u32).gt_abs(y) {
            // x - y * (±1), with the sign of the quotient x / y
            let y_abs = y.abs();
            let (r, o) =
                add_rational_helper(x, Cow::Owned(if x_sign { -y_abs } else { y_abs }), prec, rm);
            return (r, o, if x_sign == y_sign { 1 } else { -1 });
        }
        let (r, o) = round_optional(x.into_owned(), Some((prec, rm)));
        return (r, o, 0);
    }
    let (m, s) = finite_odd_part(&x);
    let (a, b) = (y.numerator_ref(), y.denominator_ref());
    let (r, a, den, q) = if s >= 0u32 {
        let modulus = a << q_bits;
        let r_prime = mul_power_of_2_mod(m * b, &Natural::exact_from(&s), &modulus);
        let (q, r) = r_prime.div_mod(a);
        (r, a.clone(), b.clone(), u64::exact_from(&q))
    } else {
        let t = u64::exact_from(&-s);
        let a = a << t;
        let (q, r) = (m * b).div_mod(&a);
        (r, a, b << t, u64::exact_from(&q.mod_power_of_2(q_bits)))
    };
    let (r, q) = nearest_quotient_adjust(r, &a, q, nearest, q_bits);
    let (r, q) = signed_rational_rem(x_sign, y_sign, r, den, q);
    let (r, o) = BigExponentFloat::from_rational_prec_round(r, prec, rm);
    (r, o, q)
}

// x - y * q for a `Rational` x, a `BigExponentFloat` y, and q = x / y rounded toward zero or, if
// `nearest` is set, to the nearest integer with ties to even; rounded to `prec` bits with `rm`.
// Also returns ±(|q| mod 2^`q_bits`), with q's sign; `q_bits` is at most 63, and at least 1 if
// `nearest` is set.
//
// The special cases are those of `Float`'s `rational_rem_float`, except that there is no negative
// zero: if y is `NaN` or zero, the result is `NaN`; if y is infinite, the result is x, rounded; and
// if x is zero and y is not `NaN` or zero, the result is zero. In those cases the quotient bits are
// 0.
//
// Write |x| = a / b in lowest terms, and |y| = m * 2^s with m odd. If |x| < |y|, the truncated
// quotient is 0, and the nearest one is ±1 exactly when 2|x| > |y|. Otherwise, if s >= 0, then |y|
// = m * b * 2^s / b, and m * 2^s <= |x| <= a, so s is small and the remainder (a mod (m * b * 2^s))
// / b is computed directly. If s < 0, then for t = -s, |x| = a * 2^t / (b * 2^t) and |y| = m * b /
// (b * 2^t), so the remainder is ((a * 2^t) mod (m * b)) / (b * 2^t); it is computed modulo m * b *
// 2^k, for k = `q_bits`, as in `finite_rem_abs`, so that a huge t costs only a modular
// exponentiation. Then (that remainder times b) / b is rounded and scaled by 2^-t, which is exact,
// since a `BigExponentFloat` has no exponent range: no number with t bits is formed.
pub(crate) fn rational_rem_with_quotient_bits(
    x: &Rational,
    y: Cow<'_, BigExponentFloat>,
    nearest: bool,
    prec: u64,
    rm: RoundingMode,
    q_bits: u64,
) -> (BigExponentFloat, Ordering, i64) {
    assert_ne!(prec, 0);
    let y_sign = match &*y {
        BigExponentFloat(NaN | Zero) => return (BigExponentFloat::NAN, Equal, 0),
        BigExponentFloat(Infinity { .. }) => {
            let (r, o) = BigExponentFloat::from_rational_prec_round_ref(x, prec, rm);
            return (r, o, 0);
        }
        BigExponentFloat(Finite { sign, .. }) => *sign,
    };
    if *x == 0u32 {
        return (BigExponentFloat::ZERO, Equal, 0);
    }
    let x_sign = *x > 0u32;
    if x.lt_abs(&*y) {
        if nearest && (x << 1u32).gt_abs(&*y) {
            // Here |y| < 2|x|, so y's exponent is small and y converts to a `Rational`.
            let y_abs = Rational::exact_from(&*y).abs();
            let r = if x_sign { x - y_abs } else { x + y_abs };
            let (r, o) = BigExponentFloat::from_rational_prec_round(r, prec, rm);
            return (r, o, if x_sign == y_sign { 1 } else { -1 });
        }
        let (r, o) = BigExponentFloat::from_rational_prec_round_ref(x, prec, rm);
        return (r, o, 0);
    }
    let (m, s) = finite_odd_part(&y);
    let (a, b) = (x.numerator_ref(), x.denominator_ref());
    let (r, m, scale, q) = if s >= 0u32 {
        let m = (m * b) << u64::exact_from(&s);
        let (q, r) = a.div_mod(&m);
        (
            r,
            m,
            Integer::ZERO,
            u64::exact_from(&q.mod_power_of_2(q_bits)),
        )
    } else {
        let m = m * b;
        let modulus = &m << q_bits;
        let r_prime = mul_power_of_2_mod(a.clone(), &Natural::exact_from(&-&s), &modulus);
        let (q, r) = r_prime.div_mod(&m);
        (r, m, s, u64::exact_from(&q))
    };
    let (r, q) = nearest_quotient_adjust(r, &m, q, nearest, q_bits);
    let (r, q) = signed_rational_rem(x_sign, y_sign, r, b.clone(), q);
    let (r, o) = BigExponentFloat::from_rational_prec_round(r, prec, rm);
    (mul_power_of_2_integer(r, &scale), o, q)
}

impl BigExponentFloat {
    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder
    /// is less than, equal to, or greater than the exact remainder. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rem_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), &other, false, Some((prec, rm)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).rem_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), other, false, Some((prec, rm)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), &other, false, Some((prec, rm)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793)).rem_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), other, false, Some((prec, rm)))
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded toward zero, as for the `%=` operator on
    /// primitive floats, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to,
    /// or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.f");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa25e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa26");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa26");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), &other, false, Some((prec, rm)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded toward zero, as for the `%=` operator on
    /// primitive floats, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`BigExponentFloat`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to,
    /// or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.f");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa25e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa26");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa26");
    /// ```
    #[inline]
    pub fn rem_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), other, false, Some((prec, rm)));
        *self = result;
        o
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), &other, false, Some((prec, Nearest)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The first [`BigExponentFloat`] is taken by value
    /// and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), other, false, Some((prec, Nearest)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The first [`BigExponentFloat`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), &other, false, Some((prec, Nearest)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded remainder is less than,
    /// equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is always exactly
    /// representable, and is computed exactly before rounding, even when $x/y$ is enormous: the
    /// quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), other, false, Some((prec, Nearest)))
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded toward zero, as for the `%=` operator on
    /// primitive floats, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(x.rem_prec_assign(BigExponentFloat::from(E), 5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(x.rem_prec_assign(BigExponentFloat::from(E), 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.fa26");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), &other, false, Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded toward zero, as for the `%=` operator on
    /// primitive floats, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_assign_ref(&BigExponentFloat::from(E), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.fa26");
    /// ```
    #[inline]
    pub fn rem_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), other, false, Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the specified precision and with the specified rounding
    /// mode. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ieee_remainder_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), &other, true, Some((prec, rm)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the specified precision and with the specified rounding
    /// mode. The first [`BigExponentFloat`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), other, true, Some((prec, rm)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the specified precision and with the specified rounding
    /// mode. The first [`BigExponentFloat`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_val(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_val(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_val(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_val(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_val(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_val(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), &other, true, Some((prec, rm)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the specified precision and with the specified rounding
    /// mode. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{z:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_ref(&BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_ref(&BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_round_ref_ref(&BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), other, true, Some((prec, rm)))
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded to the nearest integer (with ties to
    /// even), as for the IEEE 754 `remainder` operation, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] on the right-hand
    /// side is taken by value. An [`Ordering`] is returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.b8");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb6");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb5");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb6");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), &other, true, Some((prec, rm)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded to the nearest integer (with ties to
    /// even), as for the IEEE 754 `remainder` operation, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`BigExponentFloat`] on the right-hand
    /// side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.b8");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb6");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb5");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb6");
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), other, true, Some((prec, rm)));
        *self = result;
        o
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the nearest value of the specified precision. Both
    /// [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ieee_remainder_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), &other, true, Some((prec, Nearest)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the nearest value of the specified precision. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Owned(self), other, true, Some((prec, Nearest)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the nearest value of the specified precision. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), &other, true, Some((prec, Nearest)))
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation and C's
    /// `remainder`, rounding the result to the nearest value of the specified precision. Both
    /// [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded, if $x$ is finite
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is always exactly representable, and is
    /// computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    /// This is MPFR's `mpfr_remainder`.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{z:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (z, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{z:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        rem_helper(Cow::Borrowed(self), other, true, Some((prec, Nearest)))
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded to the nearest integer (with ties to
    /// even), as for the IEEE 754 `remainder` operation, rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] on the right-hand side is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than,
    /// equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_assign(BigExponentFloat::from(E), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_assign(BigExponentFloat::from(E), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb6");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), &other, true, Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by another
    /// [`BigExponentFloat`] $y$, with the quotient rounded to the nearest integer (with ties to
    /// even), as for the IEEE 754 `remainder` operation, rounding the result to the nearest value
    /// of the specified precision. The [`BigExponentFloat`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_assign_ref(&BigExponentFloat::from(E), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_assign_ref(&BigExponentFloat::from(E), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x0.bdbb6");
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = rem_helper(Cow::Owned(x), other, true, Some((prec, Nearest)));
        *self = result;
        o
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation. The
    /// remainder is exact. Both [`BigExponentFloat`]s are taken by value.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{round}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(10.5).ieee_remainder(BigExponentFloat::from(3))
    ///     ),
    ///     "-0x1.8"
    /// );
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder(self, other: Self) -> Self {
        rem_helper(Cow::Owned(self), &other, true, None).0
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation. The
    /// remainder is exact. The first [`BigExponentFloat`] is taken by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{round}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(10.5).ieee_remainder_val_ref(&BigExponentFloat::from(3))
    ///     ),
    ///     "-0x1.8"
    /// );
    /// ```
    #[inline]
    pub fn ieee_remainder_val_ref(self, other: &Self) -> Self {
        rem_helper(Cow::Owned(self), other, true, None).0
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation. The
    /// remainder is exact. The first [`BigExponentFloat`] is taken by reference and the second by
    /// value.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{round}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(10.5)).ieee_remainder_ref_val(BigExponentFloat::from(3))
    ///     ),
    ///     "-0x1.8"
    /// );
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_ref_val(&self, other: Self) -> Self {
        rem_helper(Cow::Borrowed(self), &other, true, None).0
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation. The
    /// remainder is exact. Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{round}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(10.5)).ieee_remainder_ref_ref(&BigExponentFloat::from(3))
    ///     ),
    ///     "-0x1.8"
    /// );
    /// ```
    #[inline]
    pub fn ieee_remainder_ref_ref(&self, other: &Self) -> Self {
        rem_helper(Cow::Borrowed(self), other, true, None).0
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation. The
    /// remainder is exact. The [`BigExponentFloat`] on the right-hand side is taken by value.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(10.5);
    /// x.ieee_remainder_assign(BigExponentFloat::from(3));
    /// assert_eq!(format!("{x:?}"), "-0x1.8");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = rem_helper(Cow::Owned(x), &other, true, None).0;
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the
    /// nearest integer (with ties to even), as for the IEEE 754 `remainder` operation. The
    /// remainder is exact. The [`BigExponentFloat`] on the right-hand side is taken by reference.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(10.5);
    /// x.ieee_remainder_assign_ref(&BigExponentFloat::from(3));
    /// assert_eq!(format!("{x:?}"), "-0x1.8");
    /// ```
    #[inline]
    pub fn ieee_remainder_assign_ref(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = rem_helper(Cow::Owned(x), other, true, None).0;
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the specified precision and with the specified rounding mode. Both
    /// [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793).rem_and_quotient_bits_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793).rem_and_quotient_bits_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793).rem_and_quotient_bits_prec_round(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793).rem_and_quotient_bits_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793).rem_and_quotient_bits_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793).rem_and_quotient_bits_prec_round(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_and_quotient_bits_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            &other,
            false,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            other,
            false,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            &other,
            false,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the specified precision and with the specified rounding mode. Both
    /// [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.fa25e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            other,
            false,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the nearest value of the specified precision. Both [`BigExponentFloat`]s
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_and_quotient_bits_prec(self, other: Self, prec: u64) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            &other,
            false,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the nearest value of the specified precision. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_and_quotient_bits_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_val_ref(
        self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            other,
            false,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the nearest value of the specified precision. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_and_quotient_bits_prec_ref_val(
        &self,
        other: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            &other,
            false,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`, rounding
    /// the remainder to the nearest value of the specified precision. Both [`BigExponentFloat`]s
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_and_quotient_bits_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.fa26");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1155);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            other,
            false,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`. The
    /// remainder is exact. Both [`BigExponentFloat`]s are taken by value.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = BigExponentFloat::from(10.5).rem_and_quotient_bits(BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "0x1.8");
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_and_quotient_bits(self, other: Self) -> (Self, i64) {
        let (r, _, q) =
            rem_with_quotient_bits(Cow::Owned(self), &other, false, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`. The
    /// remainder is exact. The first [`BigExponentFloat`] is taken by value and the second by
    /// reference.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) =
    ///     BigExponentFloat::from(10.5).rem_and_quotient_bits_val_ref(&BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "0x1.8");
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_val_ref(self, other: &Self) -> (Self, i64) {
        let (r, _, q) = rem_with_quotient_bits(Cow::Owned(self), other, false, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`. The
    /// remainder is exact. The first [`BigExponentFloat`] is taken by reference and the second by
    /// value.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = (&BigExponentFloat::from(10.5))
    ///     .rem_and_quotient_bits_ref_val(BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "0x1.8");
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_and_quotient_bits_ref_val(&self, other: Self) -> (Self, i64) {
        let (r, _, q) =
            rem_with_quotient_bits(Cow::Borrowed(self), &other, false, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for C's `fmod`, along with the low bits of that quotient, as for C's `remquo`. The
    /// remainder is exact. Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an
    /// enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::rem_prec_round`], and has the same special
    /// cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = (&BigExponentFloat::from(10.5))
    ///     .rem_and_quotient_bits_ref_ref(&BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "0x1.8");
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_ref_ref(&self, other: &Self) -> (Self, i64) {
        let (r, _, q) =
            rem_with_quotient_bits(Cow::Borrowed(self), other, false, None, QUOTIENT_BITS);
        (r, q)
    }
}

impl Rem<Self> for BigExponentFloat {
    type Output = Self;

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`. The remainder is exact, even
    /// when the quotient is enormous: the quotient is never formed. Both [`BigExponentFloat`]s are
    /// taken by value.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{trunc}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(10.5) % BigExponentFloat::from(3)
    ///     ),
    ///     "0x1.8"
    /// );
    ///
    /// // 2^(2^63 - 1) is 2 more than a multiple of 3, since 2^k mod 3 is 2 for every odd k.
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(i64::MAX) % BigExponentFloat::from(3),
    ///     BigExponentFloat::from(2)
    /// );
    /// ```
    #[inline]
    fn rem(self, other: Self) -> Self {
        rem_with_quotient_bits(Cow::Owned(self), &other, false, None, QUOTIENT_BITS).0
    }
}

impl Rem<&Self> for BigExponentFloat {
    type Output = Self;

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`. The remainder is exact, even
    /// when the quotient is enormous: the quotient is never formed. The first [`BigExponentFloat`]
    /// is taken by value and the second by reference.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{trunc}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(10.5) % &BigExponentFloat::from(3)
    ///     ),
    ///     "0x1.8"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: &Self) -> Self {
        rem_with_quotient_bits(Cow::Owned(self), other, false, None, QUOTIENT_BITS).0
    }
}

impl Rem<BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`. The remainder is exact, even
    /// when the quotient is enormous: the quotient is never formed. The first [`BigExponentFloat`]
    /// is taken by reference and the second by value.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{trunc}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(10.5)) % BigExponentFloat::from(3)
    ///     ),
    ///     "0x1.8"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: BigExponentFloat) -> BigExponentFloat {
        rem_with_quotient_bits(Cow::Borrowed(self), &other, false, None, QUOTIENT_BITS).0
    }
}

impl Rem<&BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`. The remainder is exact, even
    /// when the quotient is enormous: the quotient is never formed. Both [`BigExponentFloat`]s are
    /// taken by reference.
    ///
    /// $$
    /// f(x,y) = x-y\operatorname{trunc}(x/y).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(x,\text{NaN})=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$, rounded, if $x$ is finite
    /// - $f(0,y)=0$ if $y$ is not `NaN` or 0
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(10.5)) % &BigExponentFloat::from(3)
    ///     ),
    ///     "0x1.8"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: &BigExponentFloat) -> BigExponentFloat {
        rem_with_quotient_bits(Cow::Borrowed(self), other, false, None, QUOTIENT_BITS).0
    }
}

impl RemAssign<Self> for BigExponentFloat {
    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`. The remainder is exact, even
    /// when the quotient is enormous: the quotient is never formed. The [`BigExponentFloat`] on the
    /// right-hand side is taken by value.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(10.5);
    /// x %= BigExponentFloat::from(3);
    /// assert_eq!(format!("{x:?}"), "0x1.8");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = rem_with_quotient_bits(Cow::Owned(x), &other, false, None, QUOTIENT_BITS).0;
    }
}

impl RemAssign<&Self> for BigExponentFloat {
    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded toward zero,
    /// as for the `%` operator on primitive floats and C's `fmod`. The remainder is exact, even
    /// when the quotient is enormous: the quotient is never formed. The [`BigExponentFloat`] on the
    /// right-hand side is taken by reference.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(10.5);
    /// x %= &BigExponentFloat::from(3);
    /// assert_eq!(format!("{x:?}"), "0x1.8");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = rem_with_quotient_bits(Cow::Owned(x), other, false, None, QUOTIENT_BITS).0;
    }
}

impl BigExponentFloat {
    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            &other,
            true,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_round_val_ref(
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            other,
            true,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the specified precision and with the specified rounding mode. The first [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_val(
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_val(
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_val(
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_val(
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_val(
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_val(
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            &other,
            true,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the specified precision and with the specified rounding mode. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.b8");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb5");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_round_ref_ref(
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            other,
            true,
            Some((prec, rm)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec(
        self,
        other: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            &other,
            true,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the nearest value of the specified precision. The first [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_and_quotient_bits_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_val_ref(
        self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Owned(self),
            other,
            true,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the nearest value of the specified precision. The first [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_ref_val(
        &self,
        other: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            &other,
            true,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the remainder to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "-0x0.c");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_and_quotient_bits_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "-0x0.bdbb6");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1156);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_with_quotient_bits(
            Cow::Borrowed(self),
            other,
            true,
            Some((prec, Nearest)),
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`. The remainder is exact. Both [`BigExponentFloat`]s are taken by value.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = BigExponentFloat::from(10.5)
    ///     .ieee_remainder_and_quotient_bits(BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "-0x1.8");
    /// assert_eq!(q, 4);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits(self, other: Self) -> (Self, i64) {
        let (r, _, q) = rem_with_quotient_bits(Cow::Owned(self), &other, true, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`. The remainder is exact. The first [`BigExponentFloat`] is taken by value and the second by reference.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = BigExponentFloat::from(10.5)
    ///     .ieee_remainder_and_quotient_bits_val_ref(&BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "-0x1.8");
    /// assert_eq!(q, 4);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_val_ref(self, other: &Self) -> (Self, i64) {
        let (r, _, q) = rem_with_quotient_bits(Cow::Owned(self), other, true, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`. The remainder is exact. The first [`BigExponentFloat`] is taken by reference and the second by value.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = (&BigExponentFloat::from(10.5))
    ///     .ieee_remainder_and_quotient_bits_ref_val(BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "-0x1.8");
    /// assert_eq!(q, 4);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_ref_val(&self, other: Self) -> (Self, i64) {
        let (r, _, q) =
            rem_with_quotient_bits(Cow::Borrowed(self), &other, true, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of two [`BigExponentFloat`]s, with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`. The remainder is exact. Both [`BigExponentFloat`]s are taken by reference.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// The remainder is as for [`BigExponentFloat::ieee_remainder_prec_round`], and has the same special cases. In all of the special cases, the quotient bits are 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`, and $m$ is the difference of the exponents, or 1 if it is
    /// smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let (r, q) = (&BigExponentFloat::from(10.5))
    ///     .ieee_remainder_and_quotient_bits_ref_ref(&BigExponentFloat::from(3));
    /// assert_eq!(format!("{r:?}"), "-0x1.8");
    /// assert_eq!(q, 4);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_ref_ref(&self, other: &Self) -> (Self, i64) {
        let (r, _, q) =
            rem_with_quotient_bits(Cow::Borrowed(self), other, true, None, QUOTIENT_BITS);
        (r, q)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a `u64`, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x-n\operatorname{trunc}(x/n)+\varepsilon.
    /// $$
    /// - If $x-n\operatorname{trunc}(x/n)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-n\operatorname{trunc}(x/n)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-n\operatorname{trunc}(x/n)|\rfloor-p+1}$.
    /// - If $x-n\operatorname{trunc}(x/n)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-n\operatorname{trunc}(x/n)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p,m)=f(\pm\infty,n,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,n,p,m)=0$ if $n\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $n$ in magnitude. It is computed exactly before rounding, without forming the quotient.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rem_unsigned_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_unsigned_prec_round(7, 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_unsigned_prec_round(7, 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x5.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_unsigned_prec_round(7, 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_unsigned_prec_round(7, 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x5.97b8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_unsigned_prec_round(7, 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x5.97b88");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::from(3141.592653589793).rem_unsigned_prec_round(7, 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x5.97b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec_round(
        self,
        other: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_unsigned_helper(Cow::Owned(self), other, Some((prec, rm)))
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a `u64`, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x-n\operatorname{trunc}(x/n)+\varepsilon.
    /// $$
    /// - If $x-n\operatorname{trunc}(x/n)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-n\operatorname{trunc}(x/n)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-n\operatorname{trunc}(x/n)|\rfloor-p+1}$.
    /// - If $x-n\operatorname{trunc}(x/n)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-n\operatorname{trunc}(x/n)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p,m)=f(\pm\infty,n,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,n,p,m)=0$ if $n\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $n$ in magnitude. It is computed exactly before rounding, without forming the quotient.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).rem_unsigned_prec_round_ref(7, 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).rem_unsigned_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x5.c");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).rem_unsigned_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     (&BigExponentFloat::from(3141.592653589793)).rem_unsigned_prec_round_ref(7, 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x5.97b8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_unsigned_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x5.97b88");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_unsigned_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x5.97b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec_round_ref(
        &self,
        other: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_unsigned_helper(Cow::Borrowed(self), other, Some((prec, rm)))
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a `u64`, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x-n\operatorname{trunc}(x/n)+\varepsilon.
    /// $$
    /// - If $x-n\operatorname{trunc}(x/n)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-n\operatorname{trunc}(x/n)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-n\operatorname{trunc}(x/n)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p)=f(\pm\infty,n,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,n,p)=0$ if $n\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $n$ in magnitude. It is computed exactly before rounding, without forming the quotient.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rem_unsigned_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_unsigned_prec(7, 5);
    /// assert_eq!(format!("{r:?}"), "0x5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_unsigned_prec(7, 20);
    /// assert_eq!(format!("{r:?}"), "0x5.97b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec(self, other: u64, prec: u64) -> (Self, Ordering) {
        rem_unsigned_helper(Cow::Owned(self), other, Some((prec, Nearest)))
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a `u64`, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = x-n\operatorname{trunc}(x/n)+\varepsilon.
    /// $$
    /// - If $x-n\operatorname{trunc}(x/n)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-n\operatorname{trunc}(x/n)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-n\operatorname{trunc}(x/n)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p)=f(\pm\infty,n,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,n,p)=0$ if $n\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $n$ in magnitude. It is computed exactly before rounding, without forming the quotient.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_unsigned_prec_ref(7, 5);
    /// assert_eq!(format!("{r:?}"), "0x5.8");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_unsigned_prec_ref(7, 20);
    /// assert_eq!(format!("{r:?}"), "0x5.97b8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec_ref(&self, other: u64, prec: u64) -> (Self, Ordering) {
        rem_unsigned_helper(Cow::Borrowed(self), other, Some((prec, Nearest)))
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a `u64`, with the quotient rounded toward zero, as for C's `fmod`. The remainder is exact. The [`BigExponentFloat`] is taken by value.
    ///
    /// $$
    /// f(x,n) = x-n\operatorname{trunc}(x/n).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},n)=f(\pm\infty,n)=f(x,0)=\text{NaN}$
    /// - $f(0,n)=0$ if $n\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $n$ in magnitude. It is computed without forming the quotient, even when $x/n$ is enormous.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(10.5).rem_unsigned(3)),
    ///     "0x1.8"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from(10.5).rem_unsigned(0).to_string(),
    ///     "NaN"
    /// );
    /// ```
    #[inline]
    pub fn rem_unsigned(self, other: u64) -> Self {
        rem_unsigned_helper(Cow::Owned(self), other, None).0
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a `u64`, with the quotient rounded toward zero, as for C's `fmod`. The remainder is exact. The [`BigExponentFloat`] is taken by reference.
    ///
    /// $$
    /// f(x,n) = x-n\operatorname{trunc}(x/n).
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},n)=f(\pm\infty,n)=f(x,0)=\text{NaN}$
    /// - $f(0,n)=0$ if $n\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $n$ in magnitude. It is computed without forming the quotient, even when $x/n$ is enormous.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", (&BigExponentFloat::from(10.5)).rem_unsigned_ref(3)),
    ///     "0x1.8"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from(10.5).rem_unsigned_ref(0).to_string(),
    ///     "NaN"
    /// );
    /// ```
    #[inline]
    pub fn rem_unsigned_ref(&self, other: u64) -> Self {
        rem_unsigned_helper(Cow::Borrowed(self), other, None).0
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rem_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), &other, false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).rem_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), other, false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), &other, false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793)).rem_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), other, false, prec, rm, 0);
        (r, o)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_unsigneds(22u8, 7), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_unsigneds(22u8, 7), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.f");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_unsigneds(22u8, 7), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_unsigneds(22u8, 7), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dca");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_unsigneds(22u8, 7), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dcc");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_unsigneds(22u8, 7), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dcc");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), &other, false, prec, rm, 0);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] on the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign_ref(&Rational::from_unsigneds(22u8, 7), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign_ref(&Rational::from_unsigneds(22u8, 7), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.f");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign_ref(&Rational::from_unsigneds(22u8, 7), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign_ref(&Rational::from_unsigneds(22u8, 7), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dca");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign_ref(&Rational::from_unsigneds(22u8, 7), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dcc");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign_ref(&Rational::from_unsigneds(22u8, 7), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dcc");
    /// ```
    #[inline]
    pub fn rem_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), other, false, prec, rm, 0);
        *self = result;
        o
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rem_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_prec(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_prec(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), &other, false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_prec_val_ref(&Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_prec_val_ref(&Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), other, false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_prec_ref_val(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_prec_ref_val(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), &other, false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_prec_ref_ref(&Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_prec_ref_ref(&Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), other, false, prec, Nearest, 0);
        (r, o)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`Rational`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_assign(Rational::from_unsigneds(22u8, 7), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_assign(Rational::from_unsigneds(22u8, 7), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dcc");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), &other, false, prec, Nearest, 0);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`Rational`] on the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_assign_ref(&Rational::from_unsigneds(22u8, 7), 5),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.rem_rational_prec_assign_ref(&Rational::from_unsigneds(22u8, 7), 20),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x1.e0dcc");
    /// ```
    #[inline]
    pub fn rem_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), other, false, prec, Nearest, 0);
        *self = result;
        o
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rational_rem_big_exponent_float_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(&x, Cow::Owned(y), false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(x, Cow::Owned(y), false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(x, Cow::Borrowed(y), false, prec, rm, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rational_rem_big_exponent_float_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(&x, Cow::Owned(y), false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(x, Cow::Owned(y), false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, as for C's `fmod`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rational_rem_with_quotient_bits(x, Cow::Borrowed(y), false, prec, Nearest, 0);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rem_rational_and_quotient_bits_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round(Rational::from_unsigneds(22u8, 7), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Owned(self), &other, false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Owned(self), other, false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Borrowed(self), &other, false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.f");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dca");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Borrowed(self), other, false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rem_rational_and_quotient_bits_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec(
        self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Owned(self),
            &other,
            false,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_val_ref(&Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .rem_rational_and_quotient_bits_prec_val_ref(&Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Owned(self),
            other,
            false,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_ref_val(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_ref_val(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Borrowed(self),
            &other,
            false,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_ref_ref(&Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "0x1.e");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 999);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .rem_rational_and_quotient_bits_prec_ref_ref(&Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "0x1.e0dcc");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 999);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Borrowed(self),
            other,
            false,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Owned(y), false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Owned(y), false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Borrowed(y), false, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Owned(y), false, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), false, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Owned(y), false, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded toward zero, along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{trunc}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{trunc}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{trunc}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder has the sign of $x$ and is smaller than $y$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_rem_big_exponent_float_and_quotient_bits_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Borrowed(y), false, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ieee_remainder_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793).ieee_remainder_rational_prec_round(
    ///     Rational::from_unsigneds(22u8, 7),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), &other, true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_with_quotient_bits(Cow::Owned(self), other, true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), &other, true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), other, true, prec, rm, 1);
        (r, o)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.5");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.4");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.4");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b5a");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b58");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b5a");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), &other, true, prec, rm, 1);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] on the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.5");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.4");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.4");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b5a");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b58");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b5a");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), other, true, prec, rm, 1);
        *self = result;
        o
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::ieee_remainder_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), &other, true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_val_ref(&Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_prec_val_ref(&Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(self), other, true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_ref_val(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_ref_val(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), &other, true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_ref_ref(&Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_prec_ref_ref(&Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rem_rational_with_quotient_bits(Cow::Borrowed(self), other, true, prec, Nearest, 1);
        (r, o)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`Rational`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_assign(Rational::from_unsigneds(22u8, 7), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.4");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_assign(Rational::from_unsigneds(22u8, 7), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b5a");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), &other, true, prec, Nearest, 1);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the remainder of $x$ divided by a [`Rational`] $y$, with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`Rational`] on the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_assign_ref(&Rational::from_unsigneds(22u8, 7), 5),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.4");
    ///
    /// let mut x = BigExponentFloat::from(3141.592653589793);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_assign_ref(&Rational::from_unsigneds(22u8, 7), 20),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "-0x1.43b5a");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o, _) =
            rem_rational_with_quotient_bits(Cow::Owned(x), other, true, prec, Nearest, 1);
        *self = result;
        o
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(&x, Cow::Owned(y), true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(x, Cow::Owned(y), true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(31416u16, 7),
    ///         &BigExponentFloat::from(E),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(x, Cow::Borrowed(y), true, prec, rm, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(&x, Cow::Owned(y), true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_val_ref(
    ///     Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_val(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_with_quotient_bits(x, Cow::Owned(y), true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), as for C's `remainder`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     5,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_ref(
    ///     &Rational::from_unsigneds(31416u16, 7),
    ///     &BigExponentFloat::from(E),
    ///     20,
    /// );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        let (r, o, _) =
            rational_rem_with_quotient_bits(x, Cow::Borrowed(y), true, prec, Nearest, 1);
        (r, o)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ieee_remainder_rational_and_quotient_bits_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Owned(self), &other, true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Owned(self), other, true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Borrowed(self), &other, true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(\pm\infty,y,p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(0,y,p,m)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.5");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b58");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Borrowed(self), other, true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::ieee_remainder_rational_and_quotient_bits_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec(Rational::from_unsigneds(22u8, 7), 5);
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec(Rational::from_unsigneds(22u8, 7), 20);
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec(
        self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Owned(self),
            &other,
            true,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = BigExponentFloat::from(3141.592653589793)
    ///     .ieee_remainder_rational_and_quotient_bits_prec_val_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(Cow::Owned(self), other, true, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_ref_val(
    ///         Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Borrowed(self),
            &other,
            true,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`BigExponentFloat`] divided by a [`Rational`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`BigExponentFloat`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(0,y,p)=0$ if $y\neq0$
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(), other.significant_bits(), prec)`, and $m$ is the absolute value of `self`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.4");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1000);
    ///
    /// let (r, o, q) = (&BigExponentFloat::from(3141.592653589793))
    ///     .ieee_remainder_rational_and_quotient_bits_prec_ref_ref(
    ///         &Rational::from_unsigneds(22u8, 7),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "-0x1.43b5a");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1000);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rem_rational_with_quotient_bits(
            Cow::Borrowed(self),
            other,
            true,
            prec,
            Nearest,
            QUOTIENT_BITS,
        )
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Owned(y), true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Owned(y), true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the specified precision and with the specified rounding mode. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p+1}$.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(x,0,p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,p,m)=x$, rounded
    /// - $f(0,y,p,m)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the remainder cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1d");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{r:?}"), "0x0.1de02");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Borrowed(y), true, prec, rm, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         5,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) =
    ///     BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec(
    ///         Rational::from_unsigneds(31416u16, 7),
    ///         BigExponentFloat::from(E),
    ///         20,
    ///     );
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Owned(y), true, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by value and the [`BigExponentFloat`] by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_val_ref(Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(&x, Cow::Borrowed(y), true, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] is taken by reference and the [`BigExponentFloat`] by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_val(&Rational::from_unsigneds(31416u16, 7), BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Owned(y), true, prec, Nearest, QUOTIENT_BITS)
    }

    /// Computes the remainder of a [`Rational`] divided by a [`BigExponentFloat`], with the quotient rounded to the nearest integer (with ties to even), along with the low bits of that quotient, as for C's `remquo`, rounding the result to the nearest value of the specified precision. The [`Rational`] and the [`BigExponentFloat`] are both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign: it equals $\pm(|q|\bmod 2^{63})$. It is computed without forming $q$, which may have an enormous number of bits.
    ///
    /// $$
    /// f(x,y,p) = x-y\operatorname{round}(x/y)+\varepsilon.
    /// $$
    /// - If $x-y\operatorname{round}(x/y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x-y\operatorname{round}(x/y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |x-y\operatorname{round}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$, rounded
    /// - $f(0,y,p)=0$ if $y$ is not `NaN` or 0
    /// - In all of the above special cases, the quotient bits are 0.
    ///
    /// The remainder is at most $|y|/2$ in magnitude. It is computed exactly before rounding, even when $x/y$ is enormous: the quotient is never formed.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n \log m)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(x.significant_bits(), y.significant_bits(), prec)`, and $m$ is the absolute value of `y`'s exponent, or 1 if it is smaller.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::E;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{r:?}"), "0x0.1e");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    ///
    /// let (r, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_ref(&Rational::from_unsigneds(31416u16, 7), &BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{r:?}"), "0x0.1de022");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 1651);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        rational_rem_with_quotient_bits(x, Cow::Borrowed(y), true, prec, Nearest, QUOTIENT_BITS)
    }
}
