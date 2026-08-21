// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2016-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::add_mul::{add_scaled_round, float_sign};
use crate::{
    Float, emulate_float_float_float_float_to_float_fn, emulate_float_float_float_to_float_fn,
    float_either_infinity, float_either_zero, float_infinity, float_nan, float_negative_infinity,
    significand_bits,
};
use core::cmp::Ordering::{self, Equal};
use malachite_base::max;
use malachite_base::num::arithmetic::traits::{MulAddMul, MulAddMulAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Floor, Nearest};
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// This is mpfr_fmma and mpfr_fmms from fmma.c, MPFR 4.2.2, with the result's precision passed
// explicitly; `neg` distinguishes the two, as in the C code's mpfr_fmma_aux. The result is a * b +
// c * d (or a * b - c * d if `neg` is true), rounded to `prec` bits with rounding mode `rm`.
//
// Where the C code computes both products exactly as UBFs (unbounded floats) and lets an
// unbounded-exponent mpfr_add resolve every case, here the products are computed at prec(a) +
// prec(b) and prec(c) + prec(d) bits, which is exact unless a product's exponent leaves the
// representable range; the sum is then a single rounded addition. When a product does leave the
// range, both products are formed at the integer level and `add_scaled_round` performs the single
// rounding, standing in for the UBF machinery as in `add_mul_helper`. The C code's equal-precision
// shortcut through mpfr_set_1_2 is a performance spelling of the same computation and is omitted.
// The singular cases, which the C code delegates to the UBF product and addition rules, are spelled
// out explicitly and follow those rules: any NaN operand or any infinity-times-zero product is NaN,
// infinite products dominate (with opposite infinite products giving NaN), and the sign rules for
// zero products are those of Float addition.
pub(crate) fn mul_add_mul_helper(
    a: &Float,
    b: &Float,
    c: &Float,
    d: &Float,
    neg: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan() {
        return (float_nan!(), Equal);
    }
    let inf_zero = |x: &Float, y: &Float| {
        matches!(x, float_either_infinity!()) && matches!(y, float_either_zero!())
    };
    if inf_zero(a, b) || inf_zero(b, a) || inf_zero(c, d) || inf_zero(d, c) {
        return (float_nan!(), Equal);
    }
    let s1 = float_sign(a) == float_sign(b);
    let s2 = (float_sign(c) == float_sign(d)) != neg;
    let p1_inf = a.is_infinite() || b.is_infinite();
    let p2_inf = c.is_infinite() || d.is_infinite();
    if p1_inf || p2_inf {
        return if p1_inf && p2_inf && s1 != s2 {
            (float_nan!(), Equal)
        } else {
            let sp = if p1_inf { s1 } else { s2 };
            (
                if sp {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            )
        };
    }
    let p1_zero = matches!(a, float_either_zero!()) || matches!(b, float_either_zero!());
    let p2_zero = matches!(c, float_either_zero!()) || matches!(d, float_either_zero!());
    if p1_zero && p2_zero {
        // two zero products: positive unless both are negative, except under Floor, where it is
        // negative unless both are positive (the sign rules of Float addition)
        let sign = if rm == Floor { s1 && s2 } else { s1 || s2 };
        return (
            if sign {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            },
            Equal,
        );
    }
    if p1_zero {
        // the result is the rounded second product; a negated product is computed via the negation
        // identity
        return if neg {
            let (p, o) = c.mul_prec_round_ref_ref(d, prec, -rm);
            (-p, o.reverse())
        } else {
            c.mul_prec_round_ref_ref(d, prec, rm)
        };
    }
    if p2_zero {
        return a.mul_prec_round_ref_ref(b, prec, rm);
    }
    // At precisions prec(a) + prec(b) and prec(c) + prec(d) the products are exact unless their
    // exponents leave the representable range.
    let (
        Float(Finite {
            precision: a_prec, ..
        }),
        Float(Finite {
            precision: b_prec, ..
        }),
        Float(Finite {
            precision: c_prec, ..
        }),
        Float(Finite {
            precision: d_prec, ..
        }),
    ) = (a, b, c, d)
    else {
        unreachable!()
    };
    let (u1, o1) = a.mul_prec_ref_ref(b, a_prec + b_prec);
    let (u2, o2) = c.mul_prec_ref_ref(d, c_prec + d_prec);
    if o1 == Equal && o2 == Equal {
        let u2 = if neg { -u2 } else { u2 };
        return u1.add_prec_round(u2, prec, rm);
    }
    // a product's exponent left the range: form both products at the integer level
    let scaled = |x: &Float, y: &Float| {
        let (
            Float(Finite {
                exponent: x_exponent,
                significand: x_significand,
                ..
            }),
            Float(Finite {
                exponent: y_exponent,
                significand: y_significand,
                ..
            }),
        ) = (x, y)
        else {
            unreachable!()
        };
        (
            x_significand * y_significand,
            i64::from(*x_exponent) - i64::exact_from(significand_bits(x_significand))
                + i64::from(*y_exponent)
                - i64::exact_from(significand_bits(y_significand)),
        )
    };
    let (m1, e1) = scaled(a, b);
    let (m2, e2) = scaled(c, d);
    add_scaled_round(s1, &m1, e1, s2, &m2, e2, &Natural::ONE, prec, rm)
}

// The mixed Float-Rational counterpart of `mul_add_mul_helper`: the result is x * y + z * w (or x *
// y - z * w if `neg` is true) with the `Rational` w entering its product exactly, rounded to `prec`
// bits with rounding mode `rm`. Pre-rounding w to a `Float` would perturb the result by z times the
// conversion error; the identity xy + z(n/d) = (xyd + zn)/d keeps the whole computation exact until
// the single rounding at the end, in `add_scaled_round`. Since a nonzero `Rational` is generally
// not a dyadic, there is no exact-product fast path for the second product, and the first product
// is formed at the integer level along with it.
//
// A `Rational` zero has no sign and is treated as a positive zero in the product's sign rules.
pub(crate) fn mul_add_mul_rational_helper(
    x: &Float,
    y: &Float,
    z: &Float,
    w: &Rational,
    neg: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if x.is_nan() || y.is_nan() || z.is_nan() {
        return (float_nan!(), Equal);
    }
    let inf_zero = |u: &Float, v: &Float| {
        matches!(u, float_either_infinity!()) && matches!(v, float_either_zero!())
    };
    if inf_zero(x, y) || inf_zero(y, x) || matches!(z, float_either_infinity!()) && *w == 0u32 {
        return (float_nan!(), Equal);
    }
    let s1 = float_sign(x) == float_sign(y);
    // a zero Rational counts as positive, so >= rather than > (for a nonzero w the two comparisons
    // agree)
    let s2 = (float_sign(z) == (*w >= 0u32)) != neg;
    let p1_inf = x.is_infinite() || y.is_infinite();
    let p2_inf = z.is_infinite();
    if p1_inf || p2_inf {
        return if p1_inf && p2_inf && s1 != s2 {
            (float_nan!(), Equal)
        } else {
            let sp = if p1_inf { s1 } else { s2 };
            (
                if sp {
                    float_infinity!()
                } else {
                    float_negative_infinity!()
                },
                Equal,
            )
        };
    }
    let p1_zero = matches!(x, float_either_zero!()) || matches!(y, float_either_zero!());
    let p2_zero = matches!(z, float_either_zero!()) || *w == 0u32;
    if p1_zero && p2_zero {
        // two zero products: the sign rules of Float addition, a zero Rational counting as positive
        let sign = if rm == Floor { s1 && s2 } else { s1 || s2 };
        return (
            if sign {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            },
            Equal,
        );
    }
    if p1_zero {
        // the result is the rounded second product; a negated product is computed via the negation
        // identity
        return if neg {
            let (p, o) = z.mul_rational_prec_round_ref_ref(w, prec, -rm);
            (-p, o.reverse())
        } else {
            z.mul_rational_prec_round_ref_ref(w, prec, rm)
        };
    }
    if p2_zero {
        return x.mul_prec_round_ref_ref(y, prec, rm);
    }
    // all operands are finite and nonzero: xy + z(n/d) = (xyd + zn)/d, formed exactly
    let (
        Float(Finite {
            exponent: x_exponent,
            significand: x_significand,
            ..
        }),
        Float(Finite {
            exponent: y_exponent,
            significand: y_significand,
            ..
        }),
        Float(Finite {
            exponent: z_exponent,
            significand: z_significand,
            ..
        }),
    ) = (x, y, z)
    else {
        unreachable!()
    };
    let d = w.denominator_ref();
    add_scaled_round(
        s1,
        &(x_significand * y_significand * d),
        i64::from(*x_exponent) - i64::exact_from(significand_bits(x_significand))
            + i64::from(*y_exponent)
            - i64::exact_from(significand_bits(y_significand)),
        s2,
        &(z_significand * w.numerator_ref()),
        i64::from(*z_exponent) - i64::exact_from(significand_bits(z_significand)),
        d,
        prec,
        rm,
    )
}

impl Float {
    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. All four [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round(y.clone(), z.clone(), w.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round(y.clone(), z.clone(), w.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round(y.clone(), z.clone(), w.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round(y.clone(), z.clone(), w.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round(y.clone(), z.clone(), w.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round(y.clone(), z.clone(), w.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round(
        self,
        y: Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, &z, &w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The first three [`Float`]s are taken by value and
    /// the fourth by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_val_ref(y.clone(), z.clone(), &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, &z, w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The third [`Float`] is taken by reference and the
    /// others by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_val(y.clone(), &z, w.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, z, &w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The first two [`Float`]s are taken by value and the
    /// last two by reference. An [`Ordering`] is also returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, &y, z, w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The second [`Float`] is taken by reference and the
    /// others by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_val(&y, z.clone(), w.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, &z, &w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The second and fourth [`Float`]s are taken by
    /// reference and the others by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, &z, w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The second and third [`Float`]s are taken by
    /// reference and the others by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, z, &w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. The first [`Float`] is taken by value and the
    /// others by reference. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(&self, y, z, w, false, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the specified precision
    /// and with the specified rounding mode; the products are not rounded before the final
    /// addition, so there is a single rounding. All four [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec`] instead.
    /// If you know that your target precision is the maximum of the precisions of the inputs,
    /// consider using [`Float::mul_add_mul_round`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5200043");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_helper(self, y, z, w, false, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, &z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by reference
    /// and the others by value. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_val_ref(y.clone(), z.clone(), &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, &z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(y.clone(), &z, w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(y.clone(), &z, w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_val(y.clone(), &z, w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, &y, z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(&y, z.clone(), w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(&y, z.clone(), w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_val(&y, z.clone(), w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, &z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, &z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by value and
    /// the others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_round_assign`] instead. If both of these things
    /// are true, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_helper(self, y, z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. All four [`Float`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec(y.clone(), z.clone(), w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec(y.clone(), z.clone(), w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec(self, y: Self, z: Self, w: Self, prec: u64) -> (Self, Ordering) {
        self.mul_add_mul_prec_round(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The first three [`Float`]s are taken by value and the fourth by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_val_val_ref(y.clone(), z.clone(), &w, 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_val_val_ref(y.clone(), z.clone(), &w, 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The third [`Float`] is taken by reference and the others by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_val_ref_val(y.clone(), &z, w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_val_ref_val(y.clone(), &z, w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The first two [`Float`]s are taken by value and the last two by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_val_ref_ref(y.clone(), &z, &w, 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_val_ref_ref(y.clone(), &z, &w, 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The second [`Float`] is taken by reference and the others by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_val_val(&y, z.clone(), w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_val_val(&y, z.clone(), w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The second and fourth [`Float`]s are taken by reference and the others by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_val_ref(&y, z.clone(), &w, 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_val_ref(&y, z.clone(), &w, 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The second and third [`Float`]s are taken by reference and the others by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_ref_val(&y, &z, w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_ref_val(&y, &z, w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. The first [`Float`] is taken by value and the others by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_prec_val_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_val_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result to the nearest value of
    /// the specified precision; the products are not rounded before the final addition, so there is
    /// a single rounding. All four [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.mul_add_mul_prec_ref_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.mul_add_mul_prec_ref_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(sum.to_string(), "9.5199890");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_prec_round_ref_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign(y.clone(), z.clone(), w.clone(), 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign(y.clone(), z.clone(), w.clone(), 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign(&mut self, y: Self, z: Self, w: Self, prec: u64) -> Ordering {
        self.mul_add_mul_prec_round_assign(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by reference and the
    /// others by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_val_ref_ref(y.clone(), &z, &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_val_ref_ref(y.clone(), &z, &w, 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_val_ref(&y, z.clone(), &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_val_ref(&y, z.clone(), &w, 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_ref_val(&y, &z, w.clone(), 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_ref_val(&y, &z, w.clone(), 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_ref_ref(&y, &z, &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_prec_assign_ref_ref_ref(&y, &z, &w, 20), Less);
    /// assert_eq!(x.to_string(), "9.5199890");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_prec_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_prec_round_assign_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. All four [`Float`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round(y.clone(), z.clone(), w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round(y.clone(), z.clone(), w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round(y.clone(), z.clone(), w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round(
        self,
        y: Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The first three [`Float`]s are taken by value and the fourth by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_val_val_ref(y.clone(), z.clone(), &w, Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_round_val_val_val_ref(y.clone(), z.clone(), &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_round_val_val_val_ref(y.clone(), z.clone(), &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_val_val_ref(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The third [`Float`] is taken by reference and the others by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_val_ref_val(y.clone(), &z, w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_round_val_val_ref_val(y.clone(), &z, w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_round_val_val_ref_val(y.clone(), &z, w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_val_ref_val(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The first two [`Float`]s are taken by value and the last two by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_val_ref_ref(y.clone(), &z, &w, Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_val_ref_ref(y.clone(), &z, &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_val_ref_ref(y.clone(), &z, &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_val_ref_ref(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The second [`Float`] is taken by reference and the others by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_val_val(&y, z.clone(), w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_round_val_ref_val_val(&y, z.clone(), w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_round_val_ref_val_val(&y, z.clone(), w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_ref_val_val(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The second and fourth [`Float`]s are taken by reference and the others by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to,
    /// or greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_val_ref(&y, z.clone(), &w, Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_val_ref(&y, z.clone(), &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_val_ref(&y, z.clone(), &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_ref_val_ref(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The second and third [`Float`]s are taken by reference and the others by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_ref_val(&y, &z, w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_ref_val(&y, &z, w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_ref_val(&y, &z, w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_ref_ref_val(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. The first [`Float`] is taken by value and the others by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_round_val_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_val_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Adds the products of two pairs of [`Float`]s, rounding the result with the specified
    /// rounding mode; the products are not rounded before the final addition, so there is a single
    /// rounding. All four [`Float`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::mul_add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let (sum, o) = x.mul_add_mul_round_ref_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(sum.to_string(), "9.5199923661421124");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.mul_add_mul_round_ref_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.mul_add_mul_round_ref_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.5199923661421142");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_ref_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign(y.clone(), z.clone(), w.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign(y.clone(), z.clone(), w.clone(), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign(y.clone(), z.clone(), w.clone(), Nearest), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_val_val_ref(y.clone(), z.clone(), &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_round_assign_val_val_ref(y.clone(), z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_round_assign_val_val_ref(y.clone(), z.clone(), &w, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_val_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by reference and the others by value.
    /// An [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_val_ref_val(y.clone(), &z, w.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_round_assign_val_ref_val(y.clone(), &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_round_assign_val_ref_val(y.clone(), &z, w.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_val_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_val_ref_ref(y.clone(), &z, &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_val_ref_ref(y.clone(), &z, &w, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_val_ref_ref(y.clone(), &z, &w, Nearest), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_val_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_val_val(&y, z.clone(), w.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_round_assign_ref_val_val(&y, z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_round_assign_ref_val_val(&y, z.clone(), w.clone(), Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_ref_val_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by value and the others by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_val_ref(&y, z.clone(), &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_val_ref(&y, z.clone(), &w, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_val_ref(&y, z.clone(), &w, Nearest), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_ref_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_ref_val(&y, &z, w.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_ref_val(&y, &z, w.clone(), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_ref_val(&y, &z, w.clone(), Nearest), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_ref_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_ref_ref(&y, &z, &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.5199923661421124");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_ref_ref(&y, &z, &w, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_round_assign_ref_ref_ref(&y, &z, &w, Nearest), Greater);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_round_assign_ref_ref_ref(y, z, w, prec, rm)
    }
}

impl Float {
    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The [`Float`]s and the [`Rational`] are all taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round(y.clone(), z.clone(), w.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round(
        self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, &z, &w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The [`Float`]s are taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_val_ref(
    ///             y.clone(), z.clone(), &w, 5, Floor
    ///         );
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_val_ref(
    ///             y.clone(), z.clone(), &w, 5, Ceiling
    ///         );
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_val_ref(
    ///             y.clone(), z.clone(), &w, 5, Nearest
    ///         );
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_val_ref(
    ///             y.clone(), z.clone(), &w, 20, Floor
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_val_ref(
    ///             y.clone(), z.clone(), &w, 20, Ceiling
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_val_ref(
    ///             y.clone(), z.clone(), &w, 20, Nearest
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, &z, w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The third [`Float`] is taken by reference and the
    /// other operands by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_val(
    ///             y.clone(), &z, w.clone(), 5, Floor
    ///         );
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_val(
    ///             y.clone(), &z, w.clone(), 5, Ceiling
    ///         );
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_val(
    ///             y.clone(), &z, w.clone(), 5, Nearest
    ///         );
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_val(
    ///             y.clone(), &z, w.clone(), 20, Floor
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_val(
    ///             y.clone(), &z, w.clone(), 20, Ceiling
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_val(
    ///             y.clone(), &z, w.clone(), 20, Nearest
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, z, &w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The first two [`Float`]s are taken by value and the
    /// third [`Float`] and the [`Rational`] by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_val_ref_ref(y.clone(), &z, &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, &y, z, w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The second [`Float`] is taken by reference and the
    /// other operands by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_val(
    ///             &y, z.clone(), w.clone(), 5, Floor
    ///         );
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_val(
    ///             &y, z.clone(), w.clone(), 5, Ceiling
    ///         );
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_val(
    ///             &y, z.clone(), w.clone(), 5, Nearest
    ///         );
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_val(
    ///             &y, z.clone(), w.clone(), 20, Floor
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_val(
    ///             &y, z.clone(), w.clone(), 20, Ceiling
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_val(
    ///             &y, z.clone(), w.clone(), 20, Nearest
    ///         );
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, &z, &w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The second [`Float`] and the [`Rational`] are taken
    /// by reference and the other operands by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_val_ref(&y, z.clone(), &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, &z, w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The second and third [`Float`]s are taken by
    /// reference and the other operands by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone()
    ///         .mul_add_mul_rational_prec_round_val_ref_ref_val(&y, &z, w.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, z, &w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The first [`Float`] is taken by value and the other
    /// operands by reference. An [`Ordering`] is also returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_round_val_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(&self, y, z, w, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode; the
    /// [`Rational`] enters its product exactly and the products are not rounded before the final
    /// addition, so there is a single rounding. The [`Float`]s and the [`Rational`] are all taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,p,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p,m)=f(x,\text{NaN},z,w,p,m)=f(x,y,\text{NaN},w,p,m)=
    ///   f(x,y,z,\text{NaN},p,m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,p,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`,
    ///   $(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::mul_add_mul_rational_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::mul_add_mul_rational_round`] instead. If both of these
    /// things are true, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Floor);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "9.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 5, Nearest);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Floor);
    /// assert_eq!(sum.to_string(), "9.0111237");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.mul_add_mul_rational_prec_round_ref_ref_ref_ref(&y, &z, &w, 20, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        mul_add_mul_rational_helper(self, y, z, w, false, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign(y.clone(), z.clone(), w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, &z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by reference
    /// and the others by value. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_val_ref(
    ///         y.clone(), z.clone(), &w, 5, Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_val_ref(
    ///         y.clone(), z.clone(), &w, 5, Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_val_ref(
    ///         y.clone(), z.clone(), &w, 5, Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, &z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_ref_val(
    ///         y.clone(), &z, w.clone(), 5, Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_ref_val(
    ///         y.clone(), &z, w.clone(), 5, Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_ref_val(
    ///         y.clone(), &z, w.clone(), 5, Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_val_ref_ref(y.clone(), &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, &y, z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value. An [`Ordering`] is returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_val_val(
    ///         &y, z.clone(), w.clone(), 5, Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_val_val(
    ///         &y, z.clone(), w.clone(), 5, Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_val_val(
    ///         &y, z.clone(), w.clone(), 5, Nearest
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, &z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The middle [`Float`] on the right-hand side is taken by value
    /// and the others by reference. An [`Ordering`] is returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_val_ref(&y, z.clone(), &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, &z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The last [`Float`] on the right-hand side is taken by value and
    /// the others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_ref_val(&y, &z, w.clone(), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, z, &w, false, prec, rm);
        *self = s;
        o
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Float`]s on the right-hand side are all taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_assign`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`Float::mul_add_mul_rational_round_assign`] instead. If both of these things are true,
    /// consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused operation is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.50");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_round_assign_ref_ref_ref(&y, &z, &w, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = mul_add_mul_rational_helper(self, y, z, w, false, prec, rm);
        *self = s;
        o
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The [`Float`]s and the [`Rational`] are all taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec(y.clone(), z.clone(), w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec(y.clone(), z.clone(), w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec(
        self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The [`Float`]s are taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_val_val_val_ref(y.clone(), z.clone(), &w, 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_val_val_val_ref(y.clone(), z.clone(), &w, 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The third [`Float`] is taken by reference and the other operands by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_val_val_ref_val(y.clone(), &z, w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_val_val_ref_val(y.clone(), &z, w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The first two [`Float`]s are taken by value and the third [`Float`] and
    /// the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_val_ref_ref(y.clone(), &z, &w, 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_val_ref_ref(y.clone(), &z, &w, 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The second [`Float`] is taken by reference and the other operands by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_val_ref_val_val(&y, z.clone(), w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_prec_val_ref_val_val(&y, z.clone(), w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The second [`Float`] and the [`Rational`] are taken by reference and the
    /// other operands by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_ref_val_ref(&y, z.clone(), &w, 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_ref_val_ref(&y, z.clone(), &w, 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The second and third [`Float`]s are taken by reference and the other
    /// operands by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_ref_ref_val(&y, &z, w.clone(), 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_ref_ref_val(&y, &z, w.clone(), 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The first [`Float`] is taken by value and the other operands by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_prec_val_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_val_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision; the [`Rational`] enters
    /// its product exactly and the products are not rounded before the final addition, so there is
    /// a single rounding. The [`Float`]s and the [`Rational`] are all taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w,p) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,p)=f(x,\text{NaN},z,w,p)=f(x,y,\text{NaN},w,p)=
    ///   f(x,y,z,\text{NaN},p)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w,p)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.mul_add_mul_rational_prec_ref_ref_ref_ref(&y, &z, &w, 5);
    /// assert_eq!(sum.to_string(), "9.00");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.mul_add_mul_rational_prec_ref_ref_ref_ref(&y, &z, &w, 20);
    /// assert_eq!(sum.to_string(), "9.0111389");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.mul_add_mul_rational_prec_round_ref_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign(y.clone(), z.clone(), w.clone(), 5), Less);
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign(y.clone(), z.clone(), w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by reference and the
    /// others by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign_val_val_ref(y.clone(), z.clone(), &w, 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_val_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign_val_ref_val(y.clone(), &z, w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_val_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_val_ref_ref(y.clone(), &z, &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_val_ref_ref(y.clone(), &z, &w, 20), Greater);
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_val_ref_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] on the right-hand side is taken by reference and
    /// the others by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_prec_assign_ref_val_val(&y, z.clone(), w.clone(), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_ref_val_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The middle [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_ref_val_ref(&y, z.clone(), &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_ref_val_ref(&y, z.clone(), &w, 20), Greater);
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_ref_val_ref(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The last [`Float`] on the right-hand side is taken by value and the
    /// others by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_ref_ref_val(&y, &z, w.clone(), 5), Less);
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_ref_ref_val(&y, &z, w.clone(), 20), Greater);
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_ref_ref_val(y, z, w, prec, Nearest)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s on the right-hand side are all taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know that your target
    /// precision is the maximum of the precisions of the inputs, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_ref_ref_ref(&y, &z, &w, 5), Less);
    /// assert_eq!(x.to_string(), "9.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_prec_assign_ref_ref_ref(&y, &z, &w, 20), Greater);
    /// assert_eq!(x.to_string(), "9.0111389");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_prec_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Rational,
        prec: u64,
    ) -> Ordering {
        self.mul_add_mul_rational_prec_round_assign_ref_ref_ref(y, z, w, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The [`Float`]s and the [`Rational`] are all taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_round(y.clone(), z.clone(), w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round(y.clone(), z.clone(), w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round(y.clone(), z.clone(), w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round(
        self,
        y: Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The [`Float`]s are taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_val_ref(y.clone(), z.clone(), &w, Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_val_ref(y.clone(), z.clone(), &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_val_ref(y.clone(), z.clone(), &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_val_val_ref(
        self,
        y: Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_val_val_ref(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The third [`Float`] is taken by reference and the other operands by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_ref_val(y.clone(), &z, w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_ref_val(y.clone(), &z, w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_ref_val(y.clone(), &z, w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_val_ref_val(
        self,
        y: Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_val_ref_val(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The first two [`Float`]s are taken by value and the third [`Float`] and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_ref_ref(y.clone(), &z, &w, Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_ref_ref(y.clone(), &z, &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_val_ref_ref(y.clone(), &z, &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_val_ref_ref(
        self,
        y: Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_val_ref_ref(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The second [`Float`] is taken by reference and the other operands by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_val_val(&y, z.clone(), w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_val_val(&y, z.clone(), w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_val_val(&y, z.clone(), w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_ref_val_val(
        self,
        y: &Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_ref_val_val(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The second [`Float`] and the [`Rational`] are taken by reference and the other
    /// operands by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_val_ref(&y, z.clone(), &w, Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_val_ref(&y, z.clone(), &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_val_ref(&y, z.clone(), &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_ref_val_ref(
        self,
        y: &Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_ref_val_ref(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The second and third [`Float`]s are taken by reference and the other operands by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_ref_val(&y, &z, w.clone(), Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_ref_val(&y, &z, w.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     x.clone().mul_add_mul_rational_round_val_ref_ref_val(&y, &z, w.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_ref_ref_val(
        self,
        y: &Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_ref_ref_val(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The first [`Float`] is taken by value and the other operands by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_round_val_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_round_val_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().mul_add_mul_rational_round_val_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_val_ref_ref_ref(
        self,
        y: &Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_val_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Adds the product of two [`Float`]s and the product of a [`Float`] and a [`Rational`],
    /// rounding the result with the specified rounding mode; the [`Rational`] enters its product
    /// exactly and the products are not rounded before the final addition, so there is a single
    /// rounding. The [`Float`]s and the [`Rational`] are all taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,w,m) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    ///   $f(\text{NaN},y,z,w,m)=f(x,\text{NaN},z,w,m)=f(x,y,\text{NaN},w,m)=
    ///   f(x,y,z,\text{NaN},m)=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w,m)=0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is not
    ///   `Floor`
    /// - $f(x,y,z,w,m)=-0.0$ if $xy=-zw$, the products are finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,w,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,w,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul`](malachite_base::num::arithmetic::traits::MulAddMul::mul_add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let (sum, o) = x.mul_add_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, Floor);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.mul_add_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, Ceiling);
    /// assert_eq!(sum.to_string(), "9.0111387434645991");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.mul_add_mul_rational_round_ref_ref_ref_ref(&y, &z, &w, Nearest);
    /// assert_eq!(sum.to_string(), "9.0111387434645973");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_ref_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_ref_ref_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign(y.clone(), z.clone(), w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign(
        &mut self,
        y: Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_val_ref(y.clone(), z.clone(), &w, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_val_ref(y.clone(), z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_val_ref(y.clone(), z.clone(), &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_val_val_ref(
        &mut self,
        y: Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_val_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by reference and the others by value.
    /// An [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_ref_val(y.clone(), &z, w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_ref_val(y.clone(), &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_ref_val(y.clone(), &z, w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_val_ref_val(
        &mut self,
        y: Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_val_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_round_assign_val_ref_ref(y.clone(), &z, &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_ref_ref(y.clone(), &z, &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_val_ref_ref(y.clone(), &z, &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_val_ref_ref(
        &mut self,
        y: Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_val_ref_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The first [`Float`] on the right-hand side is taken by reference and the others by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_val_val(&y, z.clone(), w.clone(), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_val_val(&y, z.clone(), w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_val_val(&y, z.clone(), w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_ref_val_val(
        &mut self,
        y: &Self,
        z: Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_ref_val_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The middle [`Float`] on the right-hand side is taken by value and the others by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_round_assign_ref_val_ref(&y, z.clone(), &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_val_ref(&y, z.clone(), &w, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_val_ref(&y, z.clone(), &w, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_ref_val_ref(
        &mut self,
        y: &Self,
        z: Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_ref_val_ref(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The last [`Float`] on the right-hand side is taken by value and the others by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_round_assign_ref_ref_val(&y, &z, w.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_ref_val(&y, &z, w.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.mul_add_mul_rational_round_assign_ref_ref_val(&y, &z, w.clone(), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_ref_ref_val(
        &mut self,
        y: &Self,
        z: &Self,
        w: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_ref_ref_val(y, z, w, prec, rm)
    }

    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding, rounding the result with the specified rounding mode.
    /// The [`Float`]s on the right-hand side are all taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $xy+zw$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::mul_add_mul_rational_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using
    /// [`mul_add_mul_assign`](malachite_base::num::arithmetic::traits::MulAddMulAssign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the maximum precision of the inputs is not high enough to
    /// represent the output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_round_assign_ref_ref_ref(&y, &z, &w, Floor), Less);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_round_assign_ref_ref_ref(&y, &z, &w, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.0111387434645991");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.mul_add_mul_rational_round_assign_ref_ref_ref(&y, &z, &w, Nearest), Less);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn mul_add_mul_rational_round_assign_ref_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        w: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_round_assign_ref_ref_ref(y, z, w, prec, rm)
    }
}

impl MulAddMul<Self, Self, Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking all four by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(y, z, w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec(y, z, w, prec).0
    }
}

impl MulAddMul<Self, Self, &Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the first three
    /// by value and the fourth by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(y, z, &w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_val_val_ref(y, z, w, prec)
            .0
    }
}

impl MulAddMul<Self, &Self, Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the third by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(y, &z, w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_val_ref_val(y, z, w, prec)
            .0
    }
}

impl MulAddMul<Self, &Self, &Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the first two by
    /// value and the last two by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(y, &z, &w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_val_ref_ref(y, z, w, prec)
            .0
    }
}

impl MulAddMul<&Self, Self, Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the second by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, z, w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_ref_val_val(y, z, w, prec)
            .0
    }
}

impl MulAddMul<&Self, Self, &Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the second and
    /// fourth by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, z, &w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_ref_val_ref(y, z, w, prec)
            .0
    }
}

impl MulAddMul<&Self, &Self, Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the second and
    /// third by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, &z, w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_ref_ref_val(y, z, w, prec)
            .0
    }
}

impl MulAddMul<&Self, &Self, &Rational> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the first by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, &z, &w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: &Rational) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_val_ref_ref_ref(y, z, w, prec)
            .0
    }
}

impl MulAddMul<&Float, &Float, &Rational> for &Float {
    type Output = Float;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking all four by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`; a zero
    ///   [`Rational`] counts as an unsigned zero and a positive sign.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// assert_eq!(
    ///     &x.mul_add_mul(&y, &z, &w).to_string(),
    ///     "9.0111387434645973"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Float, z: &Float, w: &Rational) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_ref_ref_ref_ref(y, z, w, prec)
            .0
    }
}

impl MulAddMulAssign<Self, Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(y, z, w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign(y, z, w, prec);
    }
}

impl MulAddMulAssign<Self, Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(y, z, &w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_val_val_ref(y, z, w, prec);
    }
}

impl MulAddMulAssign<Self, &Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(y, &z, w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_val_ref_val(y, z, w, prec);
    }
}

impl MulAddMulAssign<Self, &Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(y, &z, &w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_val_ref_ref(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(&y, z, w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_ref_val_val(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(&y, z, &w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_ref_val_ref(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, &Self, Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(&y, &z, w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_ref_ref_val(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, &Self, &Rational> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the input [`Float`]s.
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_rational_round_assign`]. If you want to specify the output precision,
    /// consider using [`Float::mul_add_mul_prec_assign`]. If you want both of these things,
    /// consider using [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Rational::from_signeds(1, 3);
    /// x.mul_add_mul_assign(&y, &z, &w);
    /// assert_eq!(x.to_string(), "9.0111387434645973");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: &Rational) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.mul_add_mul_rational_prec_assign_ref_ref_ref(y, z, w, prec);
    }
}

impl MulAddMul<Self, Self, Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking all four by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(y, z, w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec(y, z, w, prec).0
    }
}

impl MulAddMul<Self, Self, &Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the first three
    /// by value and the fourth by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(y, z, &w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_val_val_ref(y, z, w, prec).0
    }
}

impl MulAddMul<Self, &Self, Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the third by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(y, &z, w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_val_ref_val(y, z, w, prec).0
    }
}

impl MulAddMul<Self, &Self, &Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the first two by
    /// value and the last two by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(y, &z, &w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_val_ref_ref(y, z, w, prec).0
    }
}

impl MulAddMul<&Self, Self, Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the second by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, z, w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_ref_val_val(y, z, w, prec).0
    }
}

impl MulAddMul<&Self, Self, &Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the second and
    /// fourth by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, z, &w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_ref_val_ref(y, z, w, prec).0
    }
}

impl MulAddMul<&Self, &Self, Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the second and
    /// third by reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, &z, w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_ref_ref_val(y, z, w, prec).0
    }
}

impl MulAddMul<&Self, &Self, &Self> for Float {
    type Output = Self;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking the first by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     x.mul_add_mul(&y, &z, &w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_val_ref_ref_ref(y, z, w, prec).0
    }
}

impl MulAddMul<&Float, &Float, &Float> for &Float {
    type Output = Float;
    /// Adds the products of two pairs of [`Float`]s with a single rounding, taking all four by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z,w) = xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    ///   $f(\text{NaN},y,z,w)=f(x,\text{NaN},z,w)=f(x,y,\text{NaN},w)=
    ///   f(x,y,z,\text{NaN})=\text{NaN}$
    /// - If either product multiplies an infinity by a zero, the result is `NaN`.
    /// - If exactly one product is infinite, the result is that product's infinity.
    /// - If both products are infinite, the result is their common infinity if their signs agree,
    ///   and `NaN` otherwise.
    /// - If both products are zeros, the sign rules of [`Float`] addition apply.
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$, the products are
    /// - $f(x,y,z,w)=0.0$ if $xy=-zw$ and the products are finite and nonzero
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,w)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,w)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,w)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,w)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,w)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,w)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::mul_add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// assert_eq!(
    ///     &x.mul_add_mul(&y, &z, &w).to_string(),
    ///     "9.5199923661421142"
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Float, z: &Float, w: &Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_ref_ref_ref_ref(y, z, w, prec).0
    }
}

impl MulAddMulAssign<Self, Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(y, z, w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign(y, z, w, prec);
    }
}

impl MulAddMulAssign<Self, Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(y, z, &w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_val_val_ref(y, z, w, prec);
    }
}

impl MulAddMulAssign<Self, &Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(y, &z, w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_val_ref_val(y, z, w, prec);
    }
}

impl MulAddMulAssign<Self, &Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(y, &z, &w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_val_ref_ref(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The first [`Float`] on the right-hand side is taken by
    /// reference and the others by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(&y, z, w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_ref_val_val(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The middle [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(&y, z, &w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_ref_val_ref(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, &Self, Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The last [`Float`] on the right-hand side is taken by
    /// value and the others by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(&y, &z, w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_ref_ref_val(y, z, w, prec);
    }
}

impl MulAddMulAssign<&Self, &Self, &Self> for Float {
    /// Multiplies a [`Float`] by another [`Float`] in place and adds the product of two more
    /// [`Float`]s, with a single rounding. The [`Float`]s on the right-hand side are all taken by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets xy+zw+\varepsilon.
    /// $$
    /// - If $xy+zw$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $xy+zw$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |xy+zw|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::mul_add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::mul_add_mul_round_assign`]. If you want to specify the output precision, consider
    /// using [`Float::mul_add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::mul_add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits() +
    /// y.significant_bits() + z.significant_bits() + w.significant_bits()`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::MulAddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// let w = Float::from(LN_2);
    /// x.mul_add_mul_assign(&y, &z, &w);
    /// assert_eq!(x.to_string(), "9.5199923661421142");
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits(),
            w.significant_bits()
        );
        self.mul_add_mul_prec_assign_ref_ref_ref(y, z, w, prec);
    }
}

/// Adds the products of two pairs of primitive floats with a single rounding, using emulated
/// [`Float`] arithmetic.
///
/// The products are not rounded before the addition, so the result is the true value of $xy+zw$
/// rounded once to the nearest representable value. No standard-library counterpart exists.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, LN_2, PI, SQRT_2};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::mul_add_mul::*;
///
/// assert_eq!(
///     NiceFloat(primitive_float_mul_add_mul(PI, E, SQRT_2, LN_2)),
///     NiceFloat(9.519992366142114)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_mul_add_mul<T: PrimitiveFloat>(x: T, y: T, z: T, w: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_float_float_to_float_fn(Float::mul_add_mul_prec, x, y, z, w)
}

/// Adds the product of two primitive floats and the product of a primitive float and a
/// [`Rational`], with a single rounding, using emulated [`Float`] arithmetic.
///
/// The [`Rational`] enters its product exactly, the products are not rounded before the addition,
/// and the result is the true value of $xy+zw$ rounded once to the nearest representable value.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `w.significant_bits()`.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, PI, SQRT_2};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::mul_add_mul::*;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_mul_add_mul_rational(
///         PI,
///         E,
///         SQRT_2,
///         &Rational::from_signeds(1, 3)
///     )),
///     NiceFloat(9.011138743464597)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_mul_add_mul_rational<T: PrimitiveFloat>(x: T, y: T, z: T, w: &Rational) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_float_to_float_fn(
        |x, y, z, prec| x.mul_add_mul_rational_prec_val_val_val_ref(y, z, w, prec),
        x,
        y,
        z,
    )
}
