// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN};
use crate::big_exponent_float::bracket::{
    Bracket, ZIV_GUARD_BITS, floor_bracket, rational_exponent_bound, rounding_possible,
    ziv_loop_certified,
};
use crate::big_exponent_float::exponential::lambert_w0::{
    LambertTarget, apriori_w0_pos, exponent_size, lambert_bracket, log_seed_step, max_of, min_of,
};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::FloorLogBase2;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, OneHalf, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The number of Newton steps taken in `f64` to make a seed.
const OMEGA_SEED_STEPS: usize = 8;

// The least argument for which a seed is made in `f64`: below it e^x leaves the normal range.
const OMEGA_SEED_MIN: f64 = -700.0;

// `OMEGA_SEED_STEPS` steps of Newton's iteration for y + ln y = x in `f64`, from a start value.
//
// The seed only has to be close: it is refined and then checked at the working precision, so its
// accuracy affects the number of steps taken, never the result. The exponential and the logarithm
// are therefore libm's, which are within a few ulps and a thousand times faster than Malachite's
// correctly rounded `primitive_float_exp` and `primitive_float_ln`.
//
// This is equivalent to `omegaFloat` from `AzFloat/WrightOmegaBracket.lean`, Azurite.
fn omega_float(x: f64, mut y: f64) -> f64 {
    for _ in 0..OMEGA_SEED_STEPS {
        y -= (y + libm::log(y) - x) * y / (1.0 + y);
    }
    y
}

// A seed for the candidate, with about 48 bits: Newton steps in `f64` from e^x (x < -1), from the
// tangent of omega at 0 (-1 <= x <= 2), or from x - ln x (x > 2). Outside the range of `f64`, and
// below `OMEGA_SEED_MIN`, the same start values at 64 bits: they are within a relative e^x,
// respectively (ln x) / x, of omega(x). As in `omega_float`, the exponential and the logarithm are
// libm's.
//
// This is equivalent to `omegaSeed` from `AzFloat/WrightOmegaBracket.lean`, Azurite.
fn omega_seed(x: &BigExponentFloat) -> BigExponentFloat {
    let xf = f64::rounding_from(x, Nearest).0;
    if xf.is_finite() && xf >= OMEGA_SEED_MIN {
        let start = if xf < -1.0 {
            libm::exp(xf)
        } else if xf <= 2.0 {
            0.5671432904097838 + 0.36 * xf
        } else {
            xf - libm::log(xf)
        };
        let y = omega_float(xf, start);
        if y.is_finite() && y > 0.0 {
            return BigExponentFloat::from(y);
        }
        // The start values are below e^(1 + x), where a Newton step stays positive.
        fail_on_untested_path("omega_seed, iteration not positive and finite");
    }
    if *x < 1u32 {
        x.exp_prec_ref(64).0
    } else {
        x.sub_prec_ref_val(x.ln_prec_ref(64).0, 64).0
    }
}

// Newton steps for y + ln y = x from a seed with about 48 bits up to `target` bits, planned
// backwards: the step to `target` bits starts from `target` / 2 + 8, so that every step is limited
// by its precision and not by the accuracy of its start. A step is taken at 8 bits beyond its
// target and the bit length of the exponent of the iterate.
//
// This is equivalent to `omegaRefine` from `AzFloat/WrightOmegaBracket.lean`, Azurite.
fn omega_refine(x: &BigExponentFloat, seed: BigExponentFloat, target: u64) -> BigExponentFloat {
    // The targets of the steps, from the last step back to the first, are `target`, `target` / 2 +
    // 8, and so on, down to the last one above 40.
    let mut steps = 0;
    let mut t = target;
    while t > 40 {
        steps += 1;
        t = (t >> 1) + 8;
    }
    let mut n = seed;
    for step in (0..steps).rev() {
        let mut t = target;
        for _ in 0..step {
            t = (t >> 1) + 8;
        }
        n = log_seed_step(x, &n, t + exponent_size(&n) + 8);
    }
    n
}

// The unverified candidate for omega(x) with about `target` correct bits.
//
// This is equivalent to `omegaCandidate` from `AzFloat/WrightOmegaBracket.lean`, Azurite.
fn omega_candidate(x: &LambertTarget, target: u64) -> BigExponentFloat {
    let prec = target + exponent_size(&x.lower(64)) + 80;
    let x = x.lower(prec);
    omega_refine(&x, omega_seed(&x), target)
}

// The enclosure of omega(x) from an enclosure [`x_lo`, `x_hi`] of x and a positive float `n`, at
// precision `prec`. With the residual d = n + ln n - x of the candidate n, n - omega(x) = d * c,
// where c lies between t / (1 + t) at t = min(n, omega(x)) and at t = max(n, omega(x)). Crude
// bounds on omega(x) enclose c, and then omega(x) lies in n - [d] * [c], whose width is of the
// order d^2: the verification is also the last Newton step.
//
// This is equivalent to `omegaEncl` from `AzFloat/WrightOmegaBracket.lean`, Azurite, with the
// correctly rounded logarithm in place of Azurite's bracket.
fn omega_encl(
    x_lo: &BigExponentFloat,
    x_hi: &BigExponentFloat,
    n: &BigExponentFloat,
    prec: u64,
) -> Bracket {
    let ln_n = floor_bracket(n.ln_prec_round_ref(prec, Floor).0, prec);
    let d_lo = n
        .add_prec_round_ref_val(ln_n.0, prec, Floor)
        .0
        .sub_prec_round_val_ref(x_hi, prec, Floor)
        .0;
    let d_hi = n
        .add_prec_round_ref_val(ln_n.1, prec, Ceiling)
        .0
        .sub_prec_round_val_ref(x_lo, prec, Ceiling)
        .0;
    let d_pos = max_of(BigExponentFloat::ZERO, d_hi.clone());
    let d_neg = max_of(BigExponentFloat::ZERO, -&d_lo);
    // omega(x) lies in [lower, upper].
    let lower = max_of(
        BigExponentFloat::ZERO,
        n.sub_prec_round_ref_val(
            d_pos
                .mul_prec_round(min_of(BigExponentFloat::ONE, n.clone()), prec, Ceiling)
                .0,
            prec,
            Floor,
        )
        .0,
    );
    let k = if d_neg <= BigExponentFloat::ONE_HALF {
        min_of(BigExponentFloat::ONE, n << 1u32)
    } else {
        BigExponentFloat::ONE
    };
    let upper = n
        .add_prec_round_ref_val(d_neg.mul_prec_round(k, prec, Ceiling).0, prec, Ceiling)
        .0;
    let c_lo = lower
        .div_prec_round_ref_val(
            BigExponentFloat::ONE
                .add_prec_round_val_ref(&lower, prec, Ceiling)
                .0,
            prec,
            Floor,
        )
        .0;
    let c_hi = upper
        .div_prec_round_ref_val(
            BigExponentFloat::ONE
                .add_prec_round_val_ref(&upper, prec, Floor)
                .0,
            prec,
            Ceiling,
        )
        .0;
    let above = d_hi
        .mul_prec_round_ref_ref(if d_hi > 0u32 { &c_hi } else { &c_lo }, prec, Ceiling)
        .0;
    let below = d_lo
        .mul_prec_round_ref_ref(if d_lo < 0u32 { &c_hi } else { &c_lo }, prec, Floor)
        .0;
    (
        n.sub_prec_round_ref_val(above, prec, Floor).0,
        n.sub_prec_round_ref_val(below, prec, Ceiling).0,
    )
}

// The working precision at Ziv level `w` for a candidate `n`: `w` + 24 + 2 * floor(log_2(`w`)) bits
// beyond the bit length of the exponent of `n`, at least 64.
//
// This is equivalent to `omegaWork` from `AzFloat/WrightOmegaBracket.lean`, Azurite.
fn omega_work(n: &BigExponentFloat, w: u64) -> u64 {
    max(64, w + 24 + (w.floor_log_base_2() << 1) + exponent_size(n))
}

// The bracket of omega(x) at Ziv level `w`: for a very negative x, the one from e^x, given by
// `exp_x`; otherwise the enclosure around the candidate when it is narrower than 2^(-w) relative,
// and the bracket of W_0 at e^x when it is not.
//
// This is equivalent to `wrightBracket` from `AzFloat/WrightOmegaBracket.lean`, Azurite, except for
// the path for a very negative x. The bracket of omega(x) at Ziv level `w` for a very negative x,
// from an enclosure of E = e^x: E - E^2 < omega(x) < E - E^2 + 3/2 E^3. (omega = E e^(-omega), so
// omega < E; then e^(-omega) > 1 - E gives the lower bound, and e^(-omega) < 1 - omega + omega^2 /
// 2 with E - E^2 < omega < E gives the upper one.) The cost is one exponential.
//
// Azurite has no such path: there a very negative x goes through the candidate, whose residual
// cannot resolve a value as small as e^x, and then through the bracket of W_0.
fn wright_bracket_negative(exp_x: &LambertTarget, w: u64) -> Bracket {
    let prec = w + 16;
    let (e_lo, e_hi) = floor_bracket(exp_x.lower(prec).into_owned(), prec);
    let square_lo = e_lo.square_prec_round_ref(prec, Floor).0;
    let square_hi = e_hi.square_prec_round_ref(prec, Ceiling).0;
    let cube_hi = square_hi.mul_prec_round_ref_ref(&e_hi, prec, Ceiling).0;
    (
        e_lo.sub_prec_round(square_hi, prec, Floor).0,
        e_hi.sub_prec_round(square_lo, prec, Ceiling)
            .0
            .add_prec_round(cube_hi << 1u32, prec, Ceiling)
            .0,
    )
}

fn wright_bracket(x: &LambertTarget, exp_x: &LambertTarget, w: u64) -> Bracket {
    // When x <= -(7 / 20) * (w + 8) - 1, E^2 = e^(2x) < 2^(-w - 8), so the bracket from the
    // exponential is narrow enough.
    if *x.upper(64) <= -BigExponentFloat::from(7 * (w + 8) / 20 + 1) {
        return wright_bracket_negative(exp_x, w);
    }
    let n = omega_candidate(x, (w >> 1) + 8);
    if n.is_finite() && n > 0u32 {
        let prec = omega_work(&n, w);
        let encl = omega_encl(&x.lower(prec), &x.upper(prec), &n, prec);
        if encl.1.sub_prec_round_ref_ref(&encl.0, 64, Ceiling).0 <= &encl.0 >> w {
            return encl;
        }
    }
    // This is the fallback that makes the bracket certain to be narrow enough whatever the
    // candidate is. No argument is known for which the candidate is not good enough, now that a
    // very negative x does not get here.
    fail_on_untested_path("wright_bracket, Lambert W fallback");
    lambert_bracket(true, exp_x, None, &apriori_w0_pos(exp_x, 64), w)
}

// The large-argument path, for an argument x with 1 < x < 2^e, given by the rounding procedure `nx`
// of -x, a float `nx_hi` >= -x, and a float `x_lo` <= x: since x - ln x < omega(x) < x, -omega(x)
// lies in (-x, -x + e), and its rounding is decided from the two ends when they round alike. The
// exact end x, often a rounding boundary, must be at the open side, and Ziv's loop would need about
// e bits to see on which side of it the value lies.
//
// This is equivalent to `wrightOmegaHuge?` from `AzFloat/WrightOmegaBracket.lean`, Azurite.
fn wright_omega_huge<F: Fn(u64, RoundingMode) -> (BigExponentFloat, Ordering)>(
    nx: &F,
    nx_hi: &BigExponentFloat,
    x_lo: &BigExponentFloat,
    e: &Integer,
    w: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    if *e >= w + 8 + e.significant_bits() && *x_lo > 1u32 {
        let hi = nx_hi
            .add_prec_round_ref_val(BigExponentFloat::from(e), w + 8, Ceiling)
            .0;
        rounding_possible(nx, &|p, m| hi.round_to_prec_round_ref(p, m), prec, -rm)
            .map(|(r, o)| (-r, o.reverse()))
    } else {
        None
    }
}

// omega(x), rounded to `prec` bits with `rm`.
//
// This is equivalent to `wrightOmegaPrecRound` from `AzFloat/WrightOmega.lean`, Azurite.
pub(crate) fn wright_omega_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => return (BigExponentFloat::INFINITY, Equal),
        BigExponentFloat(Infinity { sign: false }) => return (BigExponentFloat::ZERO, Equal),
        // omega(1) = 1
        _ if *x == 1u32 => return (BigExponentFloat::ONE, Equal),
        _ => {}
    }
    assert_ne!(rm, Exact, "Inexact Wright omega function");
    if let BigExponentFloat(Finite { exponent, .. }) = x {
        let nx = -x;
        if let Some(r) = wright_omega_huge(
            &|p, m| nx.round_to_prec_round_ref(p, m),
            &nx,
            x,
            exponent,
            prec + ZIV_GUARD_BITS,
            prec,
            rm,
        ) {
            return r;
        }
    }
    let target = LambertTarget::BigExponentFloat(x);
    let exp_target = LambertTarget::Exp(x);
    ziv_loop_certified(&|w| wright_bracket(&target, &exp_target, w), prec, rm)
}

// omega(q) for a `Rational` q, rounded to `prec` bits with `rm`.
//
// This is equivalent to `wrightOmegaRatPrecRound` from `AzFloat/WrightOmega.lean`, Azurite.
pub(crate) fn wright_omega_rational_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    // omega(1) = 1
    if *q == 1u32 {
        return (BigExponentFloat::ONE, Equal);
    }
    assert_ne!(rm, Exact, "Inexact Wright omega function");
    let bound = rational_exponent_bound(q);
    let w = prec + ZIV_GUARD_BITS;
    // The large-argument path applies under this condition, which is tested here as well to avoid
    // converting the argument when it does not hold.
    if bound >= w + 8 {
        let nq = -q;
        if let Some(r) = wright_omega_huge(
            &|p, m| BigExponentFloat::from_rational_prec_round_ref(&nq, p, m),
            &BigExponentFloat::from_rational_prec_round_ref(&nq, w + 8, Ceiling).0,
            &BigExponentFloat::from_rational_prec_round_ref(q, 64, Floor).0,
            &bound,
            w,
            prec,
            rm,
        ) {
            return r;
        }
    }
    let target = LambertTarget::Rational(q);
    let exp_target = LambertTarget::ExpRational(q);
    ziv_loop_certified(&|w| wright_bracket(&target, &exp_target, w), prec, rm)
}

impl BigExponentFloat {
    /// Computes $\omega(x)$, the Wright omega function, of a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0$
    /// - $f(1,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_prec_round), this function never
    /// underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_prec_round`]: an argument with exponent $e > 0$ takes about $e$ bits
    /// of $\log 2$. The time and memory needed for a negative argument therefore grow with its
    /// exponent, without any limit.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "1.557144");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        wright_omega_helper(&self, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`BigExponentFloat`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0$
    /// - $f(1,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_prec_round), this function never
    /// underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_prec_round`]: an argument with exponent $e > 0$ takes about $e$ bits
    /// of $\log 2$. The time and memory needed for a negative argument therefore grow with its
    /// exponent, without any limit.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "1.557144");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        wright_omega_helper(self, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`BigExponentFloat`] in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::wright_omega_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.5");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.557144");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5571461");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5571461");
    /// ```
    #[inline]
    pub fn wright_omega_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut g, o) = wright_omega_helper(self, prec, rm);
        swap(self, &mut g);
        o
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0$
    /// - $f(1,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_prec_round), this function never
    /// underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_prec_round`]: an argument with exponent $e > 0$ takes about $e$ bits
    /// of $\log 2$. The time and memory needed for a negative argument therefore grow with its
    /// exponent, without any limit.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec(5);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec(20);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    ///
    /// let x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// let (w, o) = x.wright_omega_prec(20);
    /// assert_eq!(w.to_string(), "1.0782e4342944819");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec(self, prec: u64) -> (Self, Ordering) {
        self.wright_omega_prec_round(prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`BigExponentFloat`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0$
    /// - $f(1,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_prec_round), this function never
    /// underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_prec_round`]: an argument with exponent $e > 0$ takes about $e$ bits
    /// of $\log 2$. The time and memory needed for a negative argument therefore grow with its
    /// exponent, without any limit.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_ref(5);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::TWO.wright_omega_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    ///
    /// let x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// let (w, o) = x.wright_omega_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.0782e4342944819");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.wright_omega_prec_round_ref(prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`BigExponentFloat`] in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::wright_omega_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = BigExponentFloat::TWO;
    /// assert_eq!(x.wright_omega_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.5571461");
    ///
    /// let mut x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// assert_eq!(x.wright_omega_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.0782e4342944819");
    /// ```
    #[inline]
    pub fn wright_omega_prec_assign(&mut self, prec: u64) -> Ordering {
        self.wright_omega_prec_round_assign(prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(1,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_rational_prec_round), this
    /// function never underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_rational_prec_round`]: an argument with exponent $e > 0$ takes about
    /// $e$ bits of $\log 2$. The time and memory needed for a negative argument therefore grow with
    /// its exponent, without any limit.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.8103218");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn wright_omega_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        wright_omega_rational_helper(&x, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(1,p,m)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_rational_prec_round), this
    /// function never underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_rational_prec_round`]: an argument with exponent $e > 0$ takes about
    /// $e$ bits of $\log 2$. The time and memory needed for a negative argument therefore grow with
    /// its exponent, without any limit.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.8103218");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        wright_omega_rational_helper(x, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(1,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_rational_prec_round), this
    /// function never underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_rational_prec_round`]: an argument with exponent $e > 0$ takes about
    /// $e$ bits of $\log 2$. The time and memory needed for a negative argument therefore grow with
    /// its exponent, without any limit.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (w, o) =
    ///     BigExponentFloat::wright_omega_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::wright_omega_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::wright_omega_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(1,p)=1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::wright_omega_rational_prec_round), this
    /// function never underflows.
    ///
    /// Warning: for a negative $x$ the result is close to $e^x$, and it is computed like
    /// [`BigExponentFloat::exp_rational_prec_round`]: an argument with exponent $e > 0$ takes about
    /// $e$ bits of $\log 2$. The time and memory needed for a negative argument therefore grow with
    /// its exponent, without any limit.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::wright_omega_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of $x$.
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
    /// let (w, o) =
    ///     BigExponentFloat::wright_omega_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::wright_omega_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::wright_omega_rational_prec_round_ref(x, prec, Nearest)
    }
}
