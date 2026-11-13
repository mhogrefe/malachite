// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, float_to_finite, from_float_result, shift_finite,
};
use crate::big_exponent_float::arithmetic::reciprocal::{
    reciprocal_helper, reciprocal_prec_round_generic,
};
use crate::big_exponent_float::arithmetic::reciprocal_sqrt::{
    reciprocal_sqrt_helper, reciprocal_sqrt_prec_round_generic,
};
use crate::big_exponent_float::arithmetic::sqrt::{sqrt_helper, sqrt_prec_round_generic};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Abs, CheckedRoot, DivMod, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// x^(1/k), rounded to `prec` bits with `rm`. Taking k as an `i128` lets the `u64` and `i64` roots
// share this function.
pub(crate) fn root_helper(
    x: Cow<'_, BigExponentFloat>,
    k: i128,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    root_helper_with(x, k, prec, rm, true)
}

crate_test_fn! {
    // x^(1/k) rounded to `prec` bits with `rm`, without scaling x into `Float`'s exponent range:
    // the root is computed as x'^(1/k) * 2^(e/k) for x = x' * 2^e with 1/2 <= |x'| < 1, both
    // factors with `Float` arithmetic in a Ziv loop, after exact and midpoint results are detected
    // by integer arithmetic. Roots of order -2, -1, 1, and 2 use the generic algorithms of the
    // corresponding operations. Used to check and benchmark the kernel path, and also used by it
    // when x and k are both so large that x cannot be scaled by a multiple of k into `Float`'s
    // exponent range.
    root_prec_round_generic(
        x: &BigExponentFloat,
        k: i128,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        root_helper_with(Cow::Borrowed(x), k, prec, rm, false)
    }
}

// The special cases follow `Float`'s roots, except at zero: with no negative zero, 0^(1/k) for
// negative odd k could be either infinity, and is NaN, like 1/0. For even negative k it is
// infinity, like FLINT's `arf_rsqrt` of 0. Negative x with odd k has a negative root (FLINT's
// `arf_root` gives NaN for any negative x).
//
// With `use_kernels`, x is scaled by a multiple of k into `Float`'s exponent range, its root is
// computed with `Float`'s root (or cube root) function, and the scaling is undone. The scaled
// operand has an exponent e with |e| < |k|, so the scaled root is between 2^(-1) and 2, far from
// overflow and underflow.
fn root_helper_with(
    x: Cow<'_, BigExponentFloat>,
    k: i128,
    prec: u64,
    rm: RoundingMode,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match k {
        0 => return (BigExponentFloat::NAN, Equal),
        1 => return x.into_owned().round_to_prec_round(prec, rm),
        -1 if use_kernels => return reciprocal_helper(x, prec, rm),
        -1 => return reciprocal_prec_round_generic(&x, prec, rm),
        2 if use_kernels => return sqrt_helper(x, prec, rm),
        2 => return sqrt_prec_round_generic(&x, prec, rm),
        -2 if use_kernels => return reciprocal_sqrt_helper(x, prec, rm),
        -2 => return reciprocal_sqrt_prec_round_generic(&x, prec, rm),
        _ => {}
    }
    let k_odd = k.odd();
    let exponent = match &*x {
        BigExponentFloat(NaN) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign }) => {
            return (
                if !sign && !k_odd {
                    BigExponentFloat::NAN
                } else if k < 0 {
                    BigExponentFloat::ZERO
                } else if *sign {
                    BigExponentFloat::INFINITY
                } else {
                    BigExponentFloat::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Zero) => {
            return (
                if k > 0 {
                    BigExponentFloat::ZERO
                } else if k_odd {
                    BigExponentFloat::NAN
                } else {
                    BigExponentFloat::INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Finite { sign: false, .. }) if !k_odd => {
            return (BigExponentFloat::NAN, Equal);
        }
        BigExponentFloat(Finite { exponent, .. }) => exponent,
    };
    if !use_kernels {
        return root_finite_generic(&x, k, prec, rm);
    }
    let k_abs = u64::exact_from(k.unsigned_abs());
    let (q, r) = exponent.div_mod(Integer::from(k_abs));
    // x = x' * 2^(q * |k|), where x' has exponent r. If r is too large for `Float`, r - |k| is
    // tried instead.
    let r = u64::exact_from(&r);
    let (float_exponent, q) = if let Ok(r) = i32::try_from(r)
        && r <= Float::MAX_EXPONENT
    {
        (r, q)
    } else if let Ok(r) = i32::try_from(i128::from(r) - i128::from(k_abs))
        && r >= Float::MIN_EXPONENT
    {
        (r, q + Integer::ONE)
    } else {
        return root_finite_generic(&x, k, prec, rm);
    };
    let x = finite_to_float(x, float_exponent);
    let frm = float_rm(rm);
    let result = if k == 3 {
        x.cbrt_prec_round(prec, frm)
    } else if k > 0 {
        x.root_u_prec_round(k_abs, prec, frm)
    } else {
        x.root_s_prec_round(i64::exact_from(k), prec, frm)
    };
    let (y, o) = from_float_result(result, Some(&if k > 0 { q } else { -q })).unwrap();
    assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat root");
    (y, o)
}

// x^(1/k) for finite nonzero x (positive if k is even) and |k| > 2, by the generic algorithm.
fn root_finite_generic(
    x: &BigExponentFloat,
    k: i128,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let BigExponentFloat(Finite {
        sign,
        exponent,
        significand,
    }) = x
    else {
        unreachable!()
    };
    let k_abs = u64::exact_from(k.unsigned_abs());
    let zeros = significand.trailing_zeros().unwrap();
    let m = significand >> zeros;
    let m_bits = m.significant_bits();
    // |x| = m * 2^t with m odd.
    let t = exponent - Integer::from(m_bits);
    let (t_q, t_r) = t.div_mod(Integer::from(k_abs));
    // The root is a dyadic rational iff k divides t and m is a perfect |k|th power; for negative k,
    // m must also be 1. Otherwise it is irrational, and so is never a rounding breakpoint. An odd m
    // greater than 1 with at most |k| bits is never a perfect |k|th power, since 3^|k| has more
    // than |k| bits.
    if t_r == 0u32 {
        let root = if m == 1u32 {
            Some(Natural::ONE)
        } else if k > 0 && m_bits > k_abs {
            (&m).checked_root(k_abs)
        } else {
            None
        };
        if let Some(root) = root {
            let shift = if k > 0 { t_q } else { -t_q };
            let exponent = shift + Integer::from(root.significant_bits());
            return BigExponentFloat::from_parts(*sign, exponent, root)
                .round_to_prec_round(prec, rm);
        }
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat root");
    // Rounding modes are inverted for negative x, since the loop below approximates the magnitude
    // of the root.
    let rm_abs = if *sign { rm } else { -rm };
    // |x| = x' * 2^exponent with 1/2 <= x' < 1, and exponent = q * |k| + r with 0 <= r < |k|, so
    // |x|^(1/k) = x'^(1/k) * 2^(r/k) * 2^(q/sign(k)).
    let x_prime = finite_to_float(Cow::Borrowed(x), 0).abs();
    let (q, r) = exponent.div_mod(Integer::from(k_abs));
    let r = Rational::from_unsigneds(u64::exact_from(&r), k_abs);
    let (shift, power) = if k > 0 { (q, r) } else { (-q, -r) };
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let root = if k > 0 {
            x_prime.root_u_prec_ref(k_abs, working_prec).0
        } else {
            x_prime.root_s_prec_ref(i64::exact_from(k), working_prec).0
        };
        let power_of_2 = Float::power_of_2_rational_prec_ref(&power, working_prec).0;
        let product = root.mul_prec(power_of_2, working_prec).0;
        // Each of the three roundings has a relative error of at most 2^(1 - working_prec), so the
        // total relative error is below 2^(3 - working_prec): less than 8 ulps. As in MPFR's
        // `MPFR_CAN_ROUND`, the test also makes sure that rounding the approximation gives the
        // right `Ordering`.
        if float_can_round(
            product.significand_ref().unwrap(),
            working_prec - 3,
            prec,
            rm_abs,
        ) {
            let exponent = Integer::from(product.get_exponent().unwrap());
            let product = shift_finite(float_to_finite(product, exponent), &shift);
            let (root, o) = product.round_to_prec_round(prec, rm_abs);
            return if *sign {
                (root, o)
            } else {
                (root.negate(), o.reverse())
            };
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl BigExponentFloat {
    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is a [`u64`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded root is less than, equal to, or greater than the exact root. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p+1}$.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k$ is odd
    /// - $f(-\infty,k,p,m)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p,m)=0$ if $k>0$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::root_u_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the root cannot be represented exactly
    /// with `prec` bits.
    ///
    /// This is FLINT's `arf_root`, except that `arf_root` gives NaN for every negative $x$.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec_round(4, 5, Floor);
    /// assert_eq!(format!("{root:?}"), "0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec_round(4, 5, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x1.6");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec_round(4, 5, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec_round(4, 20, Floor);
    /// assert_eq!(format!("{root:?}"), "0x1.54d26");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec_round(4, 20, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x1.54d28");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec_round(4, 20, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x1.54d26");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_u_prec_round(self, k: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_helper(Cow::Owned(self), i128::from(k), prec, rm)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is a [`u64`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded root is less than, equal to, or greater than the exact root. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p+1}$.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k$ is odd
    /// - $f(-\infty,k,p,m)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p,m)=0$ if $k>0$
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the root cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_round_ref(4, 5, Floor);
    /// assert_eq!(format!("{root:?}"), "0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_round_ref(4, 5, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x1.6");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_round_ref(4, 5, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_round_ref(4, 20, Floor);
    /// assert_eq!(format!("{root:?}"), "0x1.54d26");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_round_ref(4, 20, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x1.54d28");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_round_ref(4, 20, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x1.54d26");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_u_prec_round_ref(&self, k: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_helper(Cow::Borrowed(self), i128::from(k), prec, rm)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`] in place, where $k$ is a [`u64`], rounding
    /// the result to the specified precision and with the specified rounding mode. An [`Ordering`]
    /// is returned, indicating whether the rounded root is less than, equal to, or greater than the
    /// exact root. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p+1}$.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the root cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_round_assign(4, 5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_round_assign(4, 5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.6");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_round_assign(4, 5, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_round_assign(4, 20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.54d26");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_round_assign(4, 20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.54d28");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_round_assign(4, 20, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.54d26");
    /// ```
    #[inline]
    pub fn root_u_prec_round_assign(&mut self, k: u64, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = root_helper(Cow::Owned(x), i128::from(k), prec, rm);
        *self = result;
        o
    }

    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is a [`u64`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded root is less than,
    /// equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,k,p) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$
    /// - $f(-\infty,k,p)=-\infty$ if $k$ is odd
    /// - $f(-\infty,k,p)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p)=0$ if $k>0$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::root_u_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec(4, 5);
    /// assert_eq!(format!("{root:?}"), "0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_u_prec(4, 20);
    /// assert_eq!(format!("{root:?}"), "0x1.54d26");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_u_prec(self, k: u64, prec: u64) -> (Self, Ordering) {
        root_helper(Cow::Owned(self), i128::from(k), prec, Nearest)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is a [`u64`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded root is less
    /// than, equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,k,p) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$
    /// - $f(-\infty,k,p)=-\infty$ if $k$ is odd
    /// - $f(-\infty,k,p)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p)=0$ if $k>0$
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_ref(4, 5);
    /// assert_eq!(format!("{root:?}"), "0x1.5");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_u_prec_ref(4, 20);
    /// assert_eq!(format!("{root:?}"), "0x1.54d26");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_u_prec_ref(&self, k: u64, prec: u64) -> (Self, Ordering) {
        root_helper(Cow::Borrowed(self), i128::from(k), prec, Nearest)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`] in place, where $k$ is a [`u64`], rounding
    /// the result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded root is less than, equal to, or greater than the exact root.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^{1/k}|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_assign(4, 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.5");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_u_prec_assign(4, 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.54d26");
    /// ```
    #[inline]
    pub fn root_u_prec_assign(&mut self, k: u64, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = root_helper(Cow::Owned(x), i128::from(k), prec, Nearest);
        *self = result;
        o
    }
}

impl BigExponentFloat {
    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is an [`i64`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded root is less than, equal to, or greater than the exact root. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p+1}$.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$
    /// - $f(\infty,k,p,m)=0$ if $k<0$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k>0$ and $k$ is odd
    /// - $f(-\infty,k,p,m)=0$ if $k<0$ and $k$ is odd
    /// - $f(-\infty,k,p,m)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p,m)=0$ if $k>0$
    /// - $f(0,k,p,m)=\infty$ if $k<0$ and $k$ is even
    /// - $f(0,k,p,m)=\text{NaN}$ if $k<0$ and $k$ is odd
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::root_s_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the root cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec_round(-4, 5, Floor);
    /// assert_eq!(format!("{root:?}"), "0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec_round(-4, 5, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x0.c8");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec_round(-4, 5, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec_round(-4, 20, Floor);
    /// assert_eq!(format!("{root:?}"), "0x0.c049c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec_round(-4, 20, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x0.c049d");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec_round(-4, 20, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x0.c049c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_s_prec_round(self, k: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_helper(Cow::Owned(self), i128::from(k), prec, rm)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is an [`i64`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded root is less than, equal to, or greater than the exact root. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,k,p,m) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p+1}$.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=f(\text{NaN},k,p,m)=\text{NaN}$
    /// - $f(\infty,k,p,m)=\infty$ if $k>0$
    /// - $f(\infty,k,p,m)=0$ if $k<0$
    /// - $f(-\infty,k,p,m)=-\infty$ if $k>0$ and $k$ is odd
    /// - $f(-\infty,k,p,m)=0$ if $k<0$ and $k$ is odd
    /// - $f(-\infty,k,p,m)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p,m)=0$ if $k>0$
    /// - $f(0,k,p,m)=\infty$ if $k<0$ and $k$ is even
    /// - $f(0,k,p,m)=\text{NaN}$ if $k<0$ and $k$ is odd
    /// - $f(x,k,p,m)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the root cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_round_ref(-4, 5, Floor);
    /// assert_eq!(format!("{root:?}"), "0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_round_ref(-4, 5, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x0.c8");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_round_ref(-4, 5, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_round_ref(-4, 20, Floor);
    /// assert_eq!(format!("{root:?}"), "0x0.c049c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_round_ref(-4, 20, Ceiling);
    /// assert_eq!(format!("{root:?}"), "0x0.c049d");
    /// assert_eq!(o, Greater);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_round_ref(-4, 20, Nearest);
    /// assert_eq!(format!("{root:?}"), "0x0.c049c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_s_prec_round_ref(&self, k: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        root_helper(Cow::Borrowed(self), i128::from(k), prec, rm)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`] in place, where $k$ is an [`i64`],
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded root is less than, equal to, or
    /// greater than the exact root. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p+1}$.
    /// - If $x^{1/k}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^{1/k}|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the root cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_round_assign(-4, 5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_round_assign(-4, 5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.c8");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_round_assign(-4, 5, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_round_assign(-4, 20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.c049c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_round_assign(-4, 20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x0.c049d");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_round_assign(-4, 20, Nearest), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.c049c");
    /// ```
    #[inline]
    pub fn root_s_prec_round_assign(&mut self, k: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = root_helper(Cow::Owned(x), i128::from(k), prec, rm);
        *self = result;
        o
    }

    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is an [`i64`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded root is less than,
    /// equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,k,p) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$
    /// - $f(\infty,k,p)=0$ if $k<0$
    /// - $f(-\infty,k,p)=-\infty$ if $k>0$ and $k$ is odd
    /// - $f(-\infty,k,p)=0$ if $k<0$ and $k$ is odd
    /// - $f(-\infty,k,p)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p)=0$ if $k>0$
    /// - $f(0,k,p)=\infty$ if $k<0$ and $k$ is even
    /// - $f(0,k,p)=\text{NaN}$ if $k<0$ and $k$ is odd
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::root_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec(-4, 5);
    /// assert_eq!(format!("{root:?}"), "0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = BigExponentFloat::from(PI).root_s_prec(-4, 20);
    /// assert_eq!(format!("{root:?}"), "0x0.c049c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_s_prec(self, k: i64, prec: u64) -> (Self, Ordering) {
        root_helper(Cow::Owned(self), i128::from(k), prec, Nearest)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`], where $k$ is an [`i64`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded root is less
    /// than, equal to, or greater than the exact root. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,k,p) = x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^{1/k}|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p)=f(\text{NaN},k,p)=\text{NaN}$
    /// - $f(\infty,k,p)=\infty$ if $k>0$
    /// - $f(\infty,k,p)=0$ if $k<0$
    /// - $f(-\infty,k,p)=-\infty$ if $k>0$ and $k$ is odd
    /// - $f(-\infty,k,p)=0$ if $k<0$ and $k$ is odd
    /// - $f(-\infty,k,p)=\text{NaN}$ if $k$ is even
    /// - $f(0,k,p)=0$ if $k>0$
    /// - $f(0,k,p)=\infty$ if $k<0$ and $k$ is even
    /// - $f(0,k,p)=\text{NaN}$ if $k<0$ and $k$ is odd
    /// - $f(x,k,p)=\text{NaN}$ if $x<0$ and $k$ is even
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_ref(-4, 5);
    /// assert_eq!(format!("{root:?}"), "0x0.c");
    /// assert_eq!(o, Less);
    ///
    /// let (root, o) = (&BigExponentFloat::from(PI)).root_s_prec_ref(-4, 20);
    /// assert_eq!(format!("{root:?}"), "0x0.c049c");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn root_s_prec_ref(&self, k: i64, prec: u64) -> (Self, Ordering) {
        root_helper(Cow::Borrowed(self), i128::from(k), prec, Nearest)
    }

    /// Computes the $k$th root of a [`BigExponentFloat`] in place, where $k$ is an [`i64`],
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded root is less than, equal to, or greater than the
    /// exact root. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets x^{1/k}+\varepsilon.
    /// $$
    /// - If $x^{1/k}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^{1/k}$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^{1/k}|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_assign(-4, 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.root_s_prec_assign(-4, 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x0.c049c");
    /// ```
    #[inline]
    pub fn root_s_prec_assign(&mut self, k: i64, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = root_helper(Cow::Owned(x), i128::from(k), prec, Nearest);
        *self = result;
        o
    }
}
