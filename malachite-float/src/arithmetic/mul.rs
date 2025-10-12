// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    Float, float_either_infinity, float_either_zero, float_infinity, float_nan,
    float_negative_infinity, float_negative_zero, float_zero,
};
use core::cmp::{
    Ordering::{self, *},
    max,
};
use core::mem::swap;
use core::ops::{Mul, MulAssign};
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, IsPowerOf2, NegAssign, Sign};
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_mul::{
    mul_float_significands_in_place, mul_float_significands_in_place_ref,
    mul_float_significands_ref_ref,
};
use malachite_q::Rational;

const MUL_RATIONAL_THRESHOLD: u64 = 50;

fn mul_rational_prec_round_assign_naive(
    x: &mut Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> Ordering {
    assert_ne!(prec, 0);
    match (&mut *x, y) {
        (float_nan!(), _) => Equal,
        (Float(Infinity { sign }), y) => {
            match y.sign() {
                Equal => *x = float_nan!(),
                Greater => {}
                Less => {
                    sign.not_assign();
                }
            };
            Equal
        }
        (Float(Zero { sign }), y) => {
            if y < 0 {
                sign.not_assign();
            };
            Equal
        }
        (x, y) => {
            let not_sign = *x < 0;
            let mut z = Float::ZERO;
            swap(x, &mut z);
            let (mut product, o) =
                Float::from_rational_prec_round(Rational::exact_from(z) * y, prec, rm);
            if product == 0u32 && not_sign {
                product.neg_assign();
            }
            *x = product;
            o
        }
    }
}

fn mul_rational_prec_round_assign_naive_ref(
    x: &mut Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Ordering {
    assert_ne!(prec, 0);
    match (&mut *x, y) {
        (float_nan!(), _) => Equal,
        (Float(Infinity { sign }), y) => {
            match y.sign() {
                Equal => *x = float_nan!(),
                Greater => {}
                Less => {
                    sign.not_assign();
                }
            };
            Equal
        }
        (Float(Zero { sign }), y) => {
            if *y < 0 {
                sign.not_assign();
            };
            Equal
        }
        (x, y) => {
            let not_sign = *x < 0;
            let mut z = Float::ZERO;
            swap(x, &mut z);
            let (mut product, o) =
                Float::from_rational_prec_round(Rational::exact_from(z) * y, prec, rm);
            if product == 0u32 && not_sign {
                product.neg_assign();
            }
            *x = product;
            o
        }
    }
}

pub_test! {mul_rational_prec_round_naive(
    mut x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = mul_rational_prec_round_assign_naive(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {mul_rational_prec_round_naive_val_ref(
    mut x: Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = mul_rational_prec_round_assign_naive_ref(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {mul_rational_prec_round_naive_ref_val(
    x: &Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (float_nan!(), _) => (float_nan!(), Equal),
        (Float(Infinity { sign }), y) => (
            match y.sign() {
                Equal => float_nan!(),
                Greater => Float(Infinity { sign: *sign }),
                Less => Float(Infinity { sign: !*sign }),
            },
            Equal,
        ),
        (Float(Zero { sign }), y) => (
            if y >= 0u32 {
                Float(Zero { sign: *sign })
            } else {
                Float(Zero { sign: !*sign })
            },
            Equal,
        ),
        (x, y) => {
            let (mut product, o) =
                Float::from_rational_prec_round(Rational::exact_from(x) * y, prec, rm);
            if product == 0u32 && *x < 0 {
                product.neg_assign();
            }
            (product, o)
        }
    }
}}

pub_test! {mul_rational_prec_round_naive_ref_ref(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (float_nan!(), _) => (float_nan!(), Equal),
        (Float(Infinity { sign }), y) => (
            match y.sign() {
                Equal => float_nan!(),
                Greater => Float(Infinity { sign: *sign }),
                Less => Float(Infinity { sign: !*sign }),
            },
            Equal,
        ),
        (Float(Zero { sign }), y) => (
            if *y >= 0u32 {
                Float(Zero { sign: *sign })
            } else {
                Float(Zero { sign: !*sign })
            },
            Equal,
        ),
        (x, y) => {
            let (mut product, o) =
                Float::from_rational_prec_round(Rational::exact_from(x) * y, prec, rm);
            if product == 0u32 && *x < 0 {
                product.neg_assign();
            }
            (product, o)
        }
    }
}}

fn mul_rational_prec_round_assign_direct(
    x: &mut Float,
    y: Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> Ordering {
    assert_ne!(prec, 0);
    let sign = y >= 0;
    let (n, d) = y.into_numerator_and_denominator();
    if !sign {
        rm.neg_assign();
    }
    let o = match (
        if n == 0 { None } else { n.checked_log_base_2() },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let o = x.set_prec_round(prec, rm);
            *x <<= log_n;
            *x >>= log_d;
            o
        }
        (None, Some(log_d)) => {
            let o = x.mul_prec_round_assign(Float::exact_from(n), prec, rm);
            *x >>= log_d;
            o
        }
        (Some(log_n), None) => {
            let o = x.div_prec_round_assign(Float::exact_from(d), prec, rm);
            *x <<= log_n;
            o
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + n.significant_bits();
            x.mul_prec_round_assign(n, mul_prec, Floor);
            x.div_prec_round_assign(d, prec, rm)
        }
    };
    if sign {
        o
    } else {
        x.neg_assign();
        o.reverse()
    }
}

fn mul_rational_prec_round_assign_direct_ref(
    x: &mut Float,
    y: &Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> Ordering {
    assert_ne!(prec, 0);
    let sign = *y >= 0;
    let (n, d) = y.numerator_and_denominator_ref();
    if !sign {
        rm.neg_assign();
    }
    let o = match (
        if *n == 0 {
            None
        } else {
            n.checked_log_base_2()
        },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let o = x.set_prec_round(prec, rm);
            *x <<= log_n;
            *x >>= log_d;
            o
        }
        (None, Some(log_d)) => {
            let o = x.mul_prec_round_assign(Float::exact_from(n), prec, rm);
            *x >>= log_d;
            o
        }
        (Some(log_n), None) => {
            let o = x.div_prec_round_assign(Float::exact_from(d), prec, rm);
            *x <<= log_n;
            o
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + n.significant_bits();
            x.mul_prec_round_assign(n, mul_prec, Floor);
            x.div_prec_round_assign(d, prec, rm)
        }
    };
    if sign {
        o
    } else {
        x.neg_assign();
        o.reverse()
    }
}

pub_test! {mul_rational_prec_round_direct(
    mut x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = mul_rational_prec_round_assign_direct(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {mul_rational_prec_round_direct_val_ref(
    mut x: Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = mul_rational_prec_round_assign_direct_ref(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {mul_rational_prec_round_direct_ref_val(
    x: &Float,
    y: Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = y >= 0;
    let (n, d) = y.into_numerator_and_denominator();
    if !sign {
        rm.neg_assign();
    }
    let (product, o) = match (
        if n == 0 { None } else { n.checked_log_base_2() },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (product, o) = Float::from_float_prec_round_ref(x, prec, rm);
            (product << log_n >> log_d, o)
        }
        (None, Some(log_d)) => {
            let (product, o) = x.mul_prec_round_ref_val(Float::exact_from(n), prec, rm);
            (product >> log_d, o)
        }
        (Some(log_n), None) => {
            let (product, o) = x.div_prec_round_ref_val(Float::exact_from(d), prec, rm);
            (product << log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + n.significant_bits();
            x.mul_prec_round_ref_val(n, mul_prec, Floor)
                .0
                .div_prec_round(d, prec, rm)
        }
    };
    if sign {
        (product, o)
    } else {
        (-product, o.reverse())
    }
}}

pub_test! {mul_rational_prec_round_direct_ref_ref(
    x: &Float,
    y: &Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = *y >= 0;
    let (n, d) = y.numerator_and_denominator_ref();
    if !sign {
        rm.neg_assign();
    }
    let (product, o) = match (
        if *n == 0 {
            None
        } else {
            n.checked_log_base_2()
        },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (product, o) = Float::from_float_prec_round_ref(x, prec, rm);
            (product << log_n >> log_d, o)
        }
        (None, Some(log_d)) => {
            let (product, o) =
                x.mul_prec_round_ref_val(Float::exact_from(n), prec, rm);
            (product >> log_d, o)
        }
        (Some(log_n), None) => {
            let (product, o) =
                x.div_prec_round_ref_val(Float::exact_from(d), prec, rm);
            (product << log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + n.significant_bits();
            x.mul_prec_round_ref_val(n, mul_prec, Floor)
                .0
                .div_prec_round(d, prec, rm)
        }
    };
    if sign {
        (product, o)
    } else {
        (-product, o.reverse())
    }
}}

impl Float {
    /// Multiplies two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded product is less than, equal to, or greater than the
    /// exact product. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm0.0,p,m)=f(\pm0.0,\pm\infty,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::mul_round`] instead. If both of these things are true, consider using `*`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round(Float::from(E), 5, Floor);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round(Float::from(E), 5, Ceiling);
    /// assert_eq!(product.to_string(), "9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round(Float::from(E), 5, Nearest);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round(Float::from(E), 20, Floor);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round(Float::from(E), 20, Ceiling);
    /// assert_eq!(product.to_string(), "8.53975");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round(Float::from(E), 20, Nearest);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round(
        mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = self.mul_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Multiplies two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is are taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm0.0,p,m)=f(\pm0.0,\pm\infty,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::mul_round_val_ref`] instead. If both of these things are true,
    /// consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_val_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(product.to_string(), "9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_val_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_val_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_val_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(product.to_string(), "8.53975");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_val_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round_val_ref(
        mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = self.mul_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Multiplies two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is are taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm0.0,p,m)=f(\pm0.0,\pm\infty,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::mul_round_ref_val`] instead. If both of these things are true,
    /// consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_val(Float::from(E), 5, Floor);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_val(Float::from(E), 5, Ceiling);
    /// assert_eq!(product.to_string(), "9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_val(Float::from(E), 5, Nearest);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_val(Float::from(E), 20, Floor);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_val(Float::from(E), 20, Ceiling);
    /// assert_eq!(product.to_string(), "8.53975");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_val(Float::from(E), 20, Nearest);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round_ref_val(
        &self,
        mut other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = other.mul_prec_round_assign_ref(self, prec, rm);
        (other, o)
    }

    /// Multiplies two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded product is less than, equal to, or greater than the
    /// exact product. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm0.0,p,m)=f(\pm0.0,\pm\infty,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(0.0,x,p,m)=f(x,0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or
    ///   $x>0.0$
    /// - $f(-0.0,x,p,m)=f(x,-0.0,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or
    ///   $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::mul_round_ref_ref`] instead. If both of these things are true,
    /// consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(product.to_string(), "9.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(product.to_string(), "8.53975");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_round_ref_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_zero!())
            | (float_either_zero!(), float_either_infinity!()) => (float_nan!(), Equal),
            (
                Self(Infinity { sign: x_sign }),
                Self(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Self(Finite { sign: x_sign, .. }), Self(Infinity { sign: y_sign })) => (
                Self(Infinity {
                    sign: x_sign == y_sign,
                }),
                Equal,
            ),
            (
                Self(Zero { sign: x_sign }),
                Self(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Self(Finite { sign: x_sign, .. }), Self(Zero { sign: y_sign })) => (
                Self(Zero {
                    sign: x_sign == y_sign,
                }),
                Equal,
            ),
            (
                Self(Finite {
                    sign: x_sign,
                    exponent: x_exp,
                    precision: x_prec,
                    significand: x,
                }),
                Self(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: y,
                }),
            ) => {
                let sign = x_sign == y_sign;
                let exp_sum = x_exp + y_exp;
                if exp_sum - 1 > Self::MAX_EXPONENT {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Ceiling | Up | Nearest) => (float_infinity!(), Greater),
                        (true, _) => (Self::max_finite_value_with_prec(prec), Less),
                        (false, Floor | Up | Nearest) => (float_negative_infinity!(), Less),
                        (false, _) => (-Self::max_finite_value_with_prec(prec), Greater),
                    };
                } else if exp_sum < Self::MIN_EXPONENT - 1 {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Floor | Down | Nearest) => (float_zero!(), Less),
                        (true, _) => (Self::min_positive_value_prec(prec), Greater),
                        (false, Ceiling | Down | Nearest) => (float_negative_zero!(), Greater),
                        (false, _) => (-Self::min_positive_value_prec(prec), Less),
                    };
                }
                let (product, exp_offset, o) = mul_float_significands_ref_ref(
                    x,
                    *x_prec,
                    y,
                    *y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                let exp = exp_sum.checked_add(exp_offset).unwrap();
                if exp > Self::MAX_EXPONENT {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Ceiling | Up | Nearest) => (float_infinity!(), Greater),
                        (true, _) => (Self::max_finite_value_with_prec(prec), Less),
                        (false, Floor | Up | Nearest) => (float_negative_infinity!(), Less),
                        (false, _) => (-Self::max_finite_value_with_prec(prec), Greater),
                    };
                } else if exp < Self::MIN_EXPONENT {
                    return if rm == Nearest
                        && exp == Self::MIN_EXPONENT - 1
                        && (o == Less || !product.is_power_of_2())
                    {
                        if sign {
                            (Self::min_positive_value_prec(prec), Greater)
                        } else {
                            (-Self::min_positive_value_prec(prec), Less)
                        }
                    } else {
                        match (sign, rm) {
                            (_, Exact) => panic!("Inexact Float multiplication"),
                            (true, Ceiling | Up) => (Self::min_positive_value_prec(prec), Greater),
                            (true, _) => (float_zero!(), Less),
                            (false, Floor | Up) => (-Self::min_positive_value_prec(prec), Less),
                            (false, _) => (float_negative_zero!(), Greater),
                        }
                    };
                }
                (
                    Self(Finite {
                        sign,
                        exponent: exp,
                        precision: prec,
                        significand: product,
                    }),
                    if sign { o } else { o.reverse() },
                )
            }
        }
    }

    /// Multiplies two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded product is less than, equal to, or greater than the exact product.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm0.0,p)=f(\pm0.0,\pm\infty,p) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec(Float::from(E), 5);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec(Float::from(E), 20);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.mul_prec_round(other, prec, Nearest)
    }

    /// Multiplies two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm0.0,p)=f(\pm0.0,\pm\infty,p) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.mul_prec_round_val_ref(other, prec, Nearest)
    }

    /// Multiplies two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm0.0,p)=f(\pm0.0,\pm\infty,p) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.mul_prec_round_ref_val(other, prec, Nearest)
    }

    /// Multiplies two [`Float`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`Float`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded product is less than, equal to, or greater than the exact
    /// product. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm0.0,p)=f(\pm0.0,\pm\infty,p) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x>0.0$
    /// - $f(\infty,x,p)=f(x,\infty,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,p)=f(x,0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,p)=f(x,-0.0,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(product.to_string(), "8.5");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(product.to_string(), "8.53973");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.mul_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Multiplies two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded product is less than, equal to, or greater than the exact product. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\pm\infty,\pm0.0,m)=f(\pm0.0,\pm\infty,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,m)=f(x,\infty,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `*`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_round(Float::from(E), Floor);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_round(Float::from(E), Ceiling);
    /// assert_eq!(product.to_string(), "8.539734222673568");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_round(Float::from(E), Nearest);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round(other, prec, rm)
    }

    /// Multiplies two [`Float`]s, rounding the result with the specified rounding mode. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded product is less than, equal to, or greater than the exact
    /// product. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\pm\infty,\pm0.0,m)=f(\pm0.0,\pm\infty,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,m)=f(x,\infty,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `*`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_round_val_ref(&Float::from(E), Floor);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_round_val_ref(&Float::from(E), Ceiling);
    /// assert_eq!(product.to_string(), "8.539734222673568");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_round_val_ref(&Float::from(E), Nearest);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_val_ref(other, prec, rm)
    }

    /// Multiplies two [`Float`]s, rounding the result with the specified rounding mode. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded product is less than, equal to, or greater than the exact
    /// product. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\pm\infty,\pm0.0,m)=f(\pm0.0,\pm\infty,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,m)=f(x,\infty,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `*`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_round_ref_val(Float::from(E), Floor);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_round_ref_val(Float::from(E), Ceiling);
    /// assert_eq!(product.to_string(), "8.539734222673568");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_round_ref_val(Float::from(E), Nearest);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_ref_val(other, prec, rm)
    }

    /// Multiplies two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded product is less than, equal to, or greater than the exact product. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(\pm\infty,\pm0.0,m)=f(\pm0.0,\pm\infty,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\infty$ if $x>0.0$
    /// - $f(\infty,x,m)=f(x,\infty,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x,m)=f(x,-\infty,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x,m)=f(x,0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x,m)=f(x,-0.0,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `*`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_round_ref_ref(&Float::from(E), Floor);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_round_ref_ref(&Float::from(E), Ceiling);
    /// assert_eq!(product.to_string(), "8.539734222673568");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_round_ref_ref(&Float::from(E), Nearest);
    /// assert_eq!(product.to_string(), "8.539734222673566");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_ref_ref(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::mul_round_assign`] instead. If both of these things are true,
    /// consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign(Float::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.5");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign(Float::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(product.to_string(), "9.0");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign(Float::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.5");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign(Float::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.53973");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign(Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(product.to_string(), "8.53975");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign(Float::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.53973");
    /// ```
    #[inline]
    pub fn mul_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_zero!())
            | (float_either_zero!(), float_either_infinity!()) => {
                *self = float_nan!();
                Equal
            }
            (
                Self(Infinity { sign: x_sign }),
                Self(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Self(Finite { sign: x_sign, .. }), Self(Infinity { sign: y_sign })) => {
                *self = Self(Infinity {
                    sign: *x_sign == y_sign,
                });
                Equal
            }
            (
                Self(Zero { sign: x_sign }),
                Self(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Self(Finite { sign: x_sign, .. }), Self(Zero { sign: y_sign })) => {
                *self = Self(Zero {
                    sign: *x_sign == y_sign,
                });
                Equal
            }
            (
                Self(Finite {
                    sign: x_sign,
                    exponent: x_exp,
                    precision: x_prec,
                    significand: x,
                }),
                Self(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: mut y,
                }),
            ) => {
                let sign = *x_sign == y_sign;
                let exp_sum = *x_exp + y_exp;
                if exp_sum - 1 > Self::MAX_EXPONENT {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Ceiling | Up | Nearest) => {
                            *self = float_infinity!();
                            Greater
                        }
                        (true, _) => {
                            *self = Self::max_finite_value_with_prec(prec);
                            Less
                        }
                        (false, Floor | Up | Nearest) => {
                            *self = float_negative_infinity!();
                            Less
                        }
                        (false, _) => {
                            *self = -Self::max_finite_value_with_prec(prec);
                            Greater
                        }
                    };
                } else if exp_sum < Self::MIN_EXPONENT - 1 {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Floor | Down | Nearest) => {
                            *self = float_zero!();
                            Less
                        }
                        (true, _) => {
                            *self = Self::min_positive_value_prec(prec);
                            Greater
                        }
                        (false, Ceiling | Down | Nearest) => {
                            *self = float_negative_zero!();
                            Greater
                        }
                        (false, _) => {
                            *self = -Self::min_positive_value_prec(prec);
                            Less
                        }
                    };
                }
                let (exp_offset, o) = mul_float_significands_in_place(
                    x,
                    *x_prec,
                    &mut y,
                    y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                *x_exp = exp_sum.checked_add(exp_offset).unwrap();
                if *x_exp > Self::MAX_EXPONENT {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Ceiling | Up | Nearest) => {
                            *self = float_infinity!();
                            Greater
                        }
                        (true, _) => {
                            *self = Self::max_finite_value_with_prec(prec);
                            Less
                        }
                        (false, Floor | Up | Nearest) => {
                            *self = float_negative_infinity!();
                            Less
                        }
                        (false, _) => {
                            *self = -Self::max_finite_value_with_prec(prec);
                            Greater
                        }
                    };
                } else if *x_exp < Self::MIN_EXPONENT {
                    return if rm == Nearest
                        && *x_exp == Self::MIN_EXPONENT - 1
                        && (o == Less || !x.is_power_of_2())
                    {
                        if sign {
                            *self = Self::min_positive_value_prec(prec);
                            Greater
                        } else {
                            *self = -Self::min_positive_value_prec(prec);
                            Less
                        }
                    } else {
                        match (sign, rm) {
                            (_, Exact) => panic!("Inexact Float multiplication"),
                            (true, Ceiling | Up) => {
                                *self = Self::min_positive_value_prec(prec);
                                Greater
                            }
                            (true, _) => {
                                *self = float_zero!();
                                Less
                            }
                            (false, Floor | Up) => {
                                *self = -Self::min_positive_value_prec(prec);
                                Less
                            }
                            (false, _) => {
                                *self = float_negative_zero!();
                                Greater
                            }
                        }
                    };
                }
                *x_sign = sign;
                *x_prec = prec;
                if sign { o } else { o.reverse() }
            }
        }
    }

    /// Multiplies a [`Float`] by a [`Float`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded product is
    /// less than, equal to, or greater than the exact product. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::mul_round_assign_ref`] instead. If both of these things are
    /// true, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign_ref(&Float::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.5");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign_ref(&Float::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(product.to_string(), "9.0");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign_ref(&Float::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.5");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign_ref(&Float::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.53973");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign_ref(&Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(product.to_string(), "8.53975");
    ///
    /// let mut product = Float::from(PI);
    /// assert_eq!(
    ///     product.mul_prec_round_assign_ref(&Float::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(product.to_string(), "8.53973");
    /// ```
    #[inline]
    pub fn mul_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_zero!())
            | (float_either_zero!(), float_either_infinity!()) => {
                *self = float_nan!();
                Equal
            }
            (
                Self(Infinity { sign: x_sign }),
                Self(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Self(Finite { sign: x_sign, .. }), Self(Infinity { sign: y_sign })) => {
                *self = Self(Infinity {
                    sign: *x_sign == *y_sign,
                });
                Equal
            }
            (
                Self(Zero { sign: x_sign }),
                Self(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Self(Finite { sign: x_sign, .. }), Self(Zero { sign: y_sign })) => {
                *self = Self(Zero {
                    sign: *x_sign == *y_sign,
                });
                Equal
            }
            (
                Self(Finite {
                    sign: x_sign,
                    exponent: x_exp,
                    precision: x_prec,
                    significand: x,
                }),
                Self(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: y,
                }),
            ) => {
                let sign = x_sign == y_sign;
                let exp_sum = *x_exp + y_exp;
                if exp_sum - 1 > Self::MAX_EXPONENT {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Ceiling | Up | Nearest) => {
                            *self = float_infinity!();
                            Greater
                        }
                        (true, _) => {
                            *self = Self::max_finite_value_with_prec(prec);
                            Less
                        }
                        (false, Floor | Up | Nearest) => {
                            *self = float_negative_infinity!();
                            Less
                        }
                        (false, _) => {
                            *self = -Self::max_finite_value_with_prec(prec);
                            Greater
                        }
                    };
                } else if exp_sum < Self::MIN_EXPONENT - 1 {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Floor | Down | Nearest) => {
                            *self = float_zero!();
                            Less
                        }
                        (true, _) => {
                            *self = Self::min_positive_value_prec(prec);
                            Greater
                        }
                        (false, Ceiling | Down | Nearest) => {
                            *self = float_negative_zero!();
                            Greater
                        }
                        (false, _) => {
                            *self = -Self::min_positive_value_prec(prec);
                            Less
                        }
                    };
                }
                let (exp_offset, o) = mul_float_significands_in_place_ref(
                    x,
                    *x_prec,
                    y,
                    *y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                *x_exp = exp_sum.checked_add(exp_offset).unwrap();
                if *x_exp > Self::MAX_EXPONENT {
                    return match (sign, rm) {
                        (_, Exact) => panic!("Inexact Float multiplication"),
                        (true, Ceiling | Up | Nearest) => {
                            *self = float_infinity!();
                            Greater
                        }
                        (true, _) => {
                            *self = Self::max_finite_value_with_prec(prec);
                            Less
                        }
                        (false, Floor | Up | Nearest) => {
                            *self = float_negative_infinity!();
                            Less
                        }
                        (false, _) => {
                            *self = -Self::max_finite_value_with_prec(prec);
                            Greater
                        }
                    };
                } else if *x_exp < Self::MIN_EXPONENT {
                    return if rm == Nearest
                        && *x_exp == Self::MIN_EXPONENT - 1
                        && (o == Less || !x.is_power_of_2())
                    {
                        if sign {
                            *self = Self::min_positive_value_prec(prec);
                            Greater
                        } else {
                            *self = -Self::min_positive_value_prec(prec);
                            Less
                        }
                    } else {
                        match (sign, rm) {
                            (_, Exact) => panic!("Inexact Float multiplication"),
                            (true, Ceiling | Up) => {
                                *self = Self::min_positive_value_prec(prec);
                                Greater
                            }
                            (true, _) => {
                                *self = float_zero!();
                                Less
                            }
                            (false, Floor | Up) => {
                                *self = -Self::min_positive_value_prec(prec);
                                Less
                            }
                            (false, _) => {
                                *self = float_negative_zero!();
                                Greater
                            }
                        }
                    };
                }
                *x_sign = sign;
                *x_prec = prec;
                if sign { o } else { o.reverse() }
            }
        }
    }

    /// Multiplies a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Float`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded product is less than, equal to, or
    /// greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_prec_assign(Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "8.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_prec_assign(Float::from(E), 20), Less);
    /// assert_eq!(x.to_string(), "8.53973");
    /// ```
    #[inline]
    pub fn mul_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.mul_prec_round_assign(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Float`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded product is less than, equal to, or
    /// greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_prec_assign_ref(&Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "8.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_prec_assign_ref(&Float::from(E), 20), Less);
    /// assert_eq!(x.to_string(), "8.53973");
    /// ```
    #[inline]
    pub fn mul_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.mul_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded product is less than, equal to, or greater than the
    /// exact product. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::mul_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `*=`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_round_assign(Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "8.539734222673566");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_round_assign(Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "8.539734222673568");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_round_assign(Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "8.539734222673566");
    /// ```
    #[inline]
    pub fn mul_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_assign(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Float`] in place, rounding the result with the specified
    /// rounding mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`]
    /// is returned, indicating whether the rounded product is less than, equal to, or greater than
    /// the exact product. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::mul_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_round_assign_ref(&Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "8.539734222673566");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_round_assign_ref(&Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "8.539734222673568");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_round_assign_ref(&Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "8.539734222673566");
    /// ```
    #[inline]
    pub fn mul_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_assign_ref(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_rational_prec`] instead.
    /// If you know that your target precision is the precision of the [`Float`] input, consider
    /// using [`Float::mul_rational_round`] instead. If both of these things are true, consider
    /// using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(product.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(product.to_string(), "1.047199");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round(
        mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = self.mul_rational_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_rational_prec_val_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::mul_rational_round_val_ref`] instead. If both of these things are
    /// true, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(product.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(product.to_string(), "1.047199");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_val_ref(
        mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = self.mul_rational_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_rational_prec_ref_val`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::mul_rational_round_ref_val`] instead. If both of these things are
    /// true, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(product.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(product.to_string(), "1.047199");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if max(self.complexity(), other.significant_bits()) < MUL_RATIONAL_THRESHOLD {
            mul_rational_prec_round_naive_ref_val(self, other, prec, rm)
        } else {
            mul_rational_prec_round_direct_ref_val(self, other, prec, rm)
        }
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded product is less
    /// than, equal to, or greater than the exact product. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x>0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_rational_prec_ref_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::mul_rational_round_ref_ref`] instead. If both of these things are
    /// true, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(product.to_string(), "1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(product.to_string(), "1.06");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(product.to_string(), "1.047199");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(product.to_string(), "1.047197");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        if max(self.complexity(), other.significant_bits()) < MUL_RATIONAL_THRESHOLD {
            mul_rational_prec_round_naive_ref_ref(self, other, prec, rm)
        } else {
            mul_rational_prec_round_direct_ref_ref(self, other, prec, rm)
        }
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_round`] instead. If you know that your target precision is the
    /// precision of the [`Float`] input, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec(Rational::exact_from(1.5), 5);
    /// assert_eq!(product.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec(Rational::exact_from(1.5), 20);
    /// assert_eq!(product.to_string(), "4.712387");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.mul_rational_prec_round(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_round_val_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_val_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(product.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_val_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(product.to_string(), "4.712387");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.mul_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_round_ref_val`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_ref_val(Rational::exact_from(1.5), 5);
    /// assert_eq!(product.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_ref_val(Rational::exact_from(1.5), 20);
    /// assert_eq!(product.to_string(), "4.712387");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.mul_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded product is less than, equal
    /// to, or greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x>0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x>0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x\geq0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_round_ref_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) = Float::from(PI).mul_rational_prec_ref_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(product.to_string(), "4.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_prec_ref_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(product.to_string(), "4.712387");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn mul_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.mul_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] and the [`Rational`] are both are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded product is less than, equal to, or greater
    /// than the exact product. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x>0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(product.to_string(), "1.047197551196597");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_rational_round(self, other: Rational, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.mul_rational_prec_round(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded product is less than, equal to, or greater
    /// than the exact product. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x>0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_rational_prec_round_val_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(product.to_string(), "1.047197551196597");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded product is less than, equal to, or greater
    /// than the exact product. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x>0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_rational_prec_round_ref_val`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(product.to_string(), "1.047197551196597");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Rational`], rounding the result with the specified rounding
    /// mode. The [`Float`] and the [`Rational`] are both are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded product is less than, equal to, or greater
    /// than the exact product. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x>0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x>0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x\geq0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_rational_prec_round_ref_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `*` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the [`Float`] input is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(product.to_string(), "1.047197551196597");
    /// assert_eq!(o, Less);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    ///
    /// let (product, o) =
    ///     Float::from(PI).mul_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(product.to_string(), "1.047197551196598");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn mul_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Rational`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded product is less than, equal to, or
    /// greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_rational_prec_assign`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::mul_rational_round_assign`] instead. If both of these things are
    /// true, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.047197");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.047199");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.047197");
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        if max(self.complexity(), other.significant_bits()) < MUL_RATIONAL_THRESHOLD {
            mul_rational_prec_round_assign_naive(self, other, prec, rm)
        } else {
            mul_rational_prec_round_assign_direct(self, other, prec, rm)
        }
    }

    /// Multiplies a [`Float`] by a [`Rational`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded product is less than, equal to, or
    /// greater than the exact product. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_rational_prec_assign_ref`] instead. If you know that your target precision is
    /// the precision of the [`Float`] input, consider using
    /// [`Float::mul_rational_round_assign_ref`] instead. If both of these things are true, consider
    /// using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact multiplication.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.06");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.047197");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.047199");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.047197");
    /// ```
    #[inline]
    pub fn mul_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        if max(self.complexity(), other.significant_bits()) < MUL_RATIONAL_THRESHOLD {
            mul_rational_prec_round_assign_naive_ref(self, other, prec, rm)
        } else {
            mul_rational_prec_round_assign_direct_ref(self, other, prec, rm)
        }
    }

    /// Multiplies a [`Float`] by a [`Rational`] in place, rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded product is less than, equal to, or greater than the exact
    /// product. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_assign(Rational::exact_from(1.5), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_assign(Rational::exact_from(1.5), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.712387");
    /// ```
    #[inline]
    pub fn mul_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.mul_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Rational`] in place, rounding the result to the nearest value
    /// of the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded product is less than, equal to, or greater than the
    /// exact product. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the product is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::mul_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_assign_ref(&Rational::exact_from(1.5), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "4.8");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_prec_assign_ref(&Rational::exact_from(1.5), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "4.712387");
    /// ```
    #[inline]
    pub fn mul_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.mul_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Multiplies a [`Float`] by a [`Rational`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded product is less than, equal to, or greater than the exact product.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::mul_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input [`Float`] is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_round_assign(Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.047197551196597");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_round_assign(Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.047197551196598");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_round_assign(Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.047197551196598");
    /// ```
    #[inline]
    pub fn mul_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_assign(other, prec, rm)
    }

    /// Multiplies a [`Float`] by a [`Rational`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded product is less than, equal to, or greater than the exact
    /// product. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $xy$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::mul_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `*=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input [`Float`] is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "1.047197551196597");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.047197551196598");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.047197551196598");
    /// ```
    #[inline]
    pub fn mul_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_assign_ref(other, prec, rm)
    }
}

impl Mul<Self> for Float {
    type Output = Self;

    /// Multiplies two [`Float`]s, taking both by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// product is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm0.0)=f(\pm0.0,\pm\infty) = \text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0.0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=f(x,0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x)=f(x,0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::mul_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::mul_round`].
    /// If you want both of these things, consider using [`Float::mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) * Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) * Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     Float::from(1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::from(-1.5) * Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::from(-1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((Float::INFINITY * Float::ZERO).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) * Float::from(2.5), 4.0);
    /// assert_eq!(Float::from(1.5) * Float::from(-2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) * Float::from(2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) * Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round(other, prec, Nearest).0
    }
}

impl Mul<&Self> for Float {
    type Output = Self;

    /// Multiplies two [`Float`]s, taking the first by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// product is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm0.0)=f(\pm0.0,\pm\infty) = \text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0.0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=f(x,0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x)=f(x,0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_val_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::mul_round_val_ref`]. If you want both of these things, consider using
    /// [`Float::mul_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) * &Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) * &Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     Float::from(1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::from(-1.5) * &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::from(-1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((Float::INFINITY * &Float::ZERO).is_nan());
    ///
    /// assert_eq!(Float::from(1.5) * &Float::from(2.5), 4.0);
    /// assert_eq!(Float::from(1.5) * &Float::from(-2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) * &Float::from(2.5), -4.0);
    /// assert_eq!(Float::from(-1.5) * &Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: &Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Mul<Float> for &Float {
    type Output = Float;

    /// Multiplies two [`Float`]s, taking the first by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// product is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm0.0)=f(\pm0.0,\pm\infty) = \text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0.0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=f(x,0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x)=f(x,0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_ref_val`] instead. If you want to specify the output precision, consider
    /// using [`Float::mul_round_ref_val`]. If you want both of these things, consider using
    /// [`Float::mul_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) * Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) * Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     &Float::from(1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::from(-1.5) * Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::from(-1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((&Float::INFINITY * Float::ZERO).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) * Float::from(2.5), 4.0);
    /// assert_eq!(&Float::from(1.5) * Float::from(-2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) * Float::from(2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) * Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Mul<&Float> for &Float {
    type Output = Float;

    /// Multiplies two [`Float`]s, taking both by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// product is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm0.0)=f(\pm0.0,\pm\infty) = \text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x>0.0$
    /// - $f(\infty,x)=f(x,\infty)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x>0.0$
    /// - $f(-\infty,x)=f(x,-\infty)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=f(x,0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(0.0,x)=f(x,0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(-0.0,x)=f(x,-0.0)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_ref_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::mul_round_ref_ref`]. If you want both of these things, consider using
    /// [`Float::mul_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) * &Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) * &Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     &Float::from(1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::from(-1.5) * &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::from(-1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert!((&Float::INFINITY * &Float::ZERO).is_nan());
    ///
    /// assert_eq!(&Float::from(1.5) * &Float::from(2.5), 4.0);
    /// assert_eq!(&Float::from(1.5) * &Float::from(-2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) * &Float::from(2.5), -4.0);
    /// assert_eq!(&Float::from(-1.5) * &Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl MulAssign<Self> for Float {
    /// Multiplies a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side
    /// by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// product is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the `*` documentation for information on special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::mul_round_assign`]. If you want both of these things, consider using
    /// [`Float::mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x *= Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x *= Float::INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x *= Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= Float::INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x *= Float::ZERO;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x *= Float::from(2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x *= Float::from(-2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= Float::from(2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= Float::from(-2.5);
    /// assert_eq!(x, 4.0);
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_assign(other, prec, Nearest);
    }
}

impl MulAssign<&Self> for Float {
    /// Multiplies a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side
    /// by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// product is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the `*` documentation for information on special cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::mul_round_assign`]. If you want both of these things, consider using
    /// [`Float::mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x *= &Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x *= &Float::INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x *= &Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= &Float::INFINITY;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= &Float::NEGATIVE_INFINITY;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x *= &Float::ZERO;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x *= &Float::from(2.5);
    /// assert_eq!(x, 4.0);
    ///
    /// let mut x = Float::from(1.5);
    /// x *= &Float::from(-2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= &Float::from(2.5);
    /// assert_eq!(x, -4.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x *= &Float::from(-2.5);
    /// assert_eq!(x, 4.0);
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.mul_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Mul<Rational> for Float {
    type Output = Self;

    /// Multiplies a [`Float`] by a [`Rational`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x>0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x\geq0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec`] instead. If you want to specify the output precision, consider
    /// using [`Float::mul_rational_round`]. If you want both of these things, consider using
    /// [`Float::mul_rational_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN * Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(Float::INFINITY * Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY * Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::INFINITY * Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY * Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(Float::from(2.5) * Rational::exact_from(1.5), 4.0);
    /// assert_eq!(Float::from(2.5) * Rational::exact_from(-1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) * Rational::exact_from(1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) * Rational::exact_from(-1.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: Rational) -> Self {
        let prec = self.significant_bits();
        self.mul_rational_prec_round(other, prec, Nearest).0
    }
}

impl Mul<&Rational> for Float {
    type Output = Self;

    /// Multiplies a [`Float`] by a [`Rational`], taking the first by value and the second by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x>0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x\geq0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_val_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::mul_rational_round_val_ref`]. If you want both of these things,
    /// consider using [`Float::mul_rational_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Float::NAN * &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     Float::INFINITY * &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY * &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::INFINITY * &Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY * &Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(Float::from(2.5) * &Rational::exact_from(1.5), 4.0);
    /// assert_eq!(Float::from(2.5) * &Rational::exact_from(-1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) * &Rational::exact_from(1.5), -4.0);
    /// assert_eq!(Float::from(-2.5) * &Rational::exact_from(-1.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: &Rational) -> Self {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Mul<Rational> for &Float {
    type Output = Float;

    /// Multiplies a [`Float`] by a [`Rational`], taking the first by reference and the second by
    /// value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x>0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x\geq0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_ref_val`] instead. If you want to specify the output precision,
    /// consider using [`Float::mul_rational_round_ref_val`]. If you want both of these things,
    /// consider using [`Float::mul_rational_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN * Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY * Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY * Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::INFINITY * Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY * Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(&Float::from(2.5) * Rational::exact_from(1.5), 4.0);
    /// assert_eq!(&Float::from(2.5) * Rational::exact_from(-1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) * Rational::exact_from(1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) * Rational::exact_from(-1.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Mul<&Rational> for &Float {
    type Output = Float;

    /// Multiplies a [`Float`] by a [`Rational`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x>0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x>0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x\geq0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x\geq0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_ref_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::mul_rational_round_ref_ref`]. If you want both of these things,
    /// consider using [`Float::mul_rational_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Float::NAN * &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY * &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY * &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::INFINITY * &Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY * &Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(&Float::from(2.5) * &Rational::exact_from(1.5), 4.0);
    /// assert_eq!(&Float::from(2.5) * &Rational::exact_from(-1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) * &Rational::exact_from(1.5), -4.0);
    /// assert_eq!(&Float::from(-2.5) * &Rational::exact_from(-1.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl MulAssign<Rational> for Float {
    /// Multiplies a [`Float`] by a [`Rational`] in place, taking the [`Rational`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `*` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::mul_rational_round_assign`]. If you want both of these things,
    /// consider using [`Float::mul_rational_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x *= Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x *= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x *= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x *= Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x *= Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x *= Rational::exact_from(1.5);
    /// assert_eq!(x, 4.0);
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_assign(other, prec, Nearest);
    }
}

impl MulAssign<&Rational> for Float {
    /// Multiplies a [`Float`] by a [`Rational`] in place, taking the [`Rational`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `*` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_rational_prec_assign_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::mul_rational_round_assign_ref`]. If you want both of
    /// these things, consider using [`Float::mul_rational_prec_round_assign_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::NAN;
    /// x *= &Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x *= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x *= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x *= &Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x *= &Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x *= &Rational::exact_from(1.5);
    /// assert_eq!(x, 4.0);
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.mul_rational_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Mul<Float> for Rational {
    type Output = Float;

    /// Multiplies a [`Rational`] by a [`Float`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(0,\pm\infty)=\text{NaN}$
    /// - $f(x,\infty)=\infty$ if $x>0$
    /// - $f(x,\infty)=-\infty$ if $x<0$
    /// - $f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(x,-\infty)=\infty$ if $x<0$
    /// - $f(x,0.0)=0.0$ if $x\geq0$
    /// - $f(x,0.0)=-0.0$ if $x<0$
    /// - $f(x,-0.0)=-0.0$ if $x\geq0$
    /// - $f(x,-0.0)=0.0$ if $x<0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) * Float::NAN).is_nan());
    /// assert_eq!(Rational::exact_from(1.5) * Float::INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     Rational::exact_from(1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) * Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(Rational::exact_from(1.5) * Float::from(2.5), 4.0);
    /// assert_eq!(Rational::exact_from(-1.5) * Float::from(2.5), -4.0);
    /// assert_eq!(Rational::exact_from(1.5) * Float::from(-2.5), -4.0);
    /// assert_eq!(Rational::exact_from(-1.5) * Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: Float) -> Float {
        let prec = other.significant_bits();
        other.mul_rational_prec_round(self, prec, Nearest).0
    }
}

impl Mul<&Float> for Rational {
    type Output = Float;

    /// Multiplies a [`Rational`] by a [`Float`], taking the [`Rational`] by value and the [`Float`]
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(0,\pm\infty)=\text{NaN}$
    /// - $f(x,\infty)=\infty$ if $x>0$
    /// - $f(x,\infty)=-\infty$ if $x<0$
    /// - $f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(x,-\infty)=\infty$ if $x<0$
    /// - $f(x,0.0)=0.0$ if $x\geq0$
    /// - $f(x,0.0)=-0.0$ if $x<0$
    /// - $f(x,-0.0)=-0.0$ if $x\geq0$
    /// - $f(x,-0.0)=0.0$ if $x<0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) * &Float::NAN).is_nan());
    /// assert_eq!(
    ///     Rational::exact_from(1.5) * &Float::INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) * &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(Rational::exact_from(1.5) * &Float::from(2.5), 4.0);
    /// assert_eq!(Rational::exact_from(-1.5) * &Float::from(2.5), -4.0);
    /// assert_eq!(Rational::exact_from(1.5) * &Float::from(-2.5), -4.0);
    /// assert_eq!(Rational::exact_from(-1.5) * &Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        other.mul_rational_prec_round_ref_val(self, prec, Nearest).0
    }
}

impl Mul<Float> for &Rational {
    type Output = Float;

    /// Multiplies a [`Rational`] by a [`Float`], taking the [`Rational`] by reference and the
    /// [`Float`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(0,\pm\infty)=\text{NaN}$
    /// - $f(x,\infty)=\infty$ if $x>0$
    /// - $f(x,\infty)=-\infty$ if $x<0$
    /// - $f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(x,-\infty)=\infty$ if $x<0$
    /// - $f(x,0.0)=0.0$ if $x\geq0$
    /// - $f(x,0.0)=-0.0$ if $x<0$
    /// - $f(x,-0.0)=-0.0$ if $x\geq0$
    /// - $f(x,-0.0)=0.0$ if $x<0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) * Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) * Float::INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) * Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) * Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(&Rational::exact_from(1.5) * Float::from(2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) * Float::from(2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(1.5) * Float::from(-2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) * Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: Float) -> Float {
        let prec = other.significant_bits();
        other.mul_rational_prec_round_val_ref(self, prec, Nearest).0
    }
}

impl Mul<&Float> for &Rational {
    type Output = Float;

    /// Multiplies a [`Rational`] by a [`Float`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the product is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = xy+\varepsilon.
    /// $$
    /// - If $xy$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |xy|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(0,\pm\infty)=\text{NaN}$
    /// - $f(x,\infty)=\infty$ if $x>0$
    /// - $f(x,\infty)=-\infty$ if $x<0$
    /// - $f(x,-\infty)=-\infty$ if $x>0$
    /// - $f(x,-\infty)=\infty$ if $x<0$
    /// - $f(x,0.0)=0.0$ if $x\geq0$
    /// - $f(x,0.0)=-0.0$ if $x<0$
    /// - $f(x,-0.0)=-0.0$ if $x\geq0$
    /// - $f(x,-0.0)=0.0$ if $x<0$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) * &Float::NAN).is_nan());
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) * &Float::INFINITY,
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) * &Float::INFINITY,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) * &Float::NEGATIVE_INFINITY,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(&Rational::exact_from(1.5) * &Float::from(2.5), 4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) * &Float::from(2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(1.5) * &Float::from(-2.5), -4.0);
    /// assert_eq!(&Rational::exact_from(-1.5) * &Float::from(-2.5), 4.0);
    /// ```
    #[inline]
    fn mul(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        other.mul_rational_prec_round_ref_ref(self, prec, Nearest).0
    }
}
