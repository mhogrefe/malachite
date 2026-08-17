// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    Float, emulate_float_float_float_to_float_fn, float_either_infinity, float_either_zero,
    float_infinity, float_nan, float_negative_infinity, significand_bits,
};
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::{fail_on_untested_path, max};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// If the product's exponent reaches this bound, the sum overflows regardless of the addend, whose
// magnitude is less than 2^MAX_EXPONENT.
const SURE_OVERFLOW_EXPONENT: i64 = Float::MAX_EXPONENT_I64 + 3;

// The sign of a `Float` that is not NaN. `true` means positive.
fn float_sign(x: &Float) -> bool {
    match x {
        Float(Infinity { sign } | Zero { sign } | Finite { sign, .. }) => *sign,
        _ => panic!(),
    }
}

// Rounds A + P to `prec` bits with rounding mode `rm`, where A = ±ma * 2^ea and P = ±mp * 2^ep
// are exact scaled integers: ma and mp are positive, and ea and ep are the exponents of their least
// significant bits. This stands in for the UBF (unbounded-float) machinery that mpfr_fma uses when
// the product x * y lies outside the representable exponent range: the product is kept in exact
// integer form instead of as an unbounded float, and a single rounding produces the result.
//
// When the smaller operand lies entirely below both the larger operand's bits and its rounding
// window, only its sign and nonzeroness can affect the rounding. In that case the larger operand is
// padded with prec + 2 guard bits and a ±1 sticky bit is appended, which reproduces the exact
// sum's rounding and ordering while keeping the computation small. Otherwise the operands' bit
// ranges genuinely interact and the sum is formed exactly; its size is then bounded by the sizes of
// the operands and the output.
fn add_scaled_round(
    sa: bool,
    ma: &Natural,
    ea: i64,
    sp: bool,
    mp: Natural,
    ep: i64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let am = ea + i64::exact_from(ma.significant_bits());
    let pm = ep + i64::exact_from(mp.significant_bits());
    // the operand with the greater most-significant-bit exponent dominates: its magnitude is at
    // least 2^(dm - 1), and the other's is less than 2^tm <= 2^(dm - 1)
    let ((sd, md, ed, dm), (st, _, _, tm)) = if pm > am {
        ((sp, &mp, ep, pm), (sa, ma, ea, am))
    } else {
        ((sa, ma, ea, am), (sp, &mp, ep, pm))
    };
    let (v, m) = if i128::from(ed) - i128::from(tm) >= 3
        && i128::from(dm) - i128::from(tm) >= i128::from(prec) + 3
    {
        // The smaller operand is entirely below the larger one's least significant bit and below
        // its rounding window, so it acts as a sticky bit: pad the larger operand with prec + 2
        // guard bits and add or subtract 1, which lies strictly between the same two rounding
        // boundaries as the true sum.
        let padded = md << (prec + 2);
        let v = Integer::from_sign_and_abs(
            sd,
            if st == sd {
                padded + Natural::ONE
            } else {
                padded - Natural::ONE
            },
        );
        (v, ed - i64::exact_from(prec + 2))
    } else {
        // the bit ranges interact; form the sum exactly
        let m = min(ea, ep);
        let va = Integer::from_sign_and_abs(sa, ma << u64::exact_from(ea - m));
        let vp = Integer::from_sign_and_abs(sp, mp << u64::exact_from(ep - m));
        (va + vp, m)
    };
    if v == 0 {
        // Believed unreachable: this function is only called when the product's magnitude range and
        // the addend's are disjoint (the product's exponent is above the addend's whole range in
        // the overflow case and below it in the underflow case), so the two can never cancel
        // exactly. The zero handling is kept as a defensive fallback.
        fail_on_untested_path("add_scaled_round, exact cancellation");
        return (
            if rm == Floor {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            },
            Equal,
        );
    }
    // As in rem1_core: when the result's exponent is strictly inside the representable range, round
    // the integer once and shift exactly; at the range edges, fall back to the Rational conversion,
    // whose single rounding handles underflow and overflow.
    let e = i64::exact_from(v.significant_bits()) + m;
    if e > Float::MIN_EXPONENT_I64 && e < Float::MAX_EXPONENT_I64 {
        let (f, o) = Float::from_integer_prec_round(v, prec, rm);
        (f << m, o)
    } else {
        Float::from_rational_prec_round(Rational::from(v) << m, prec, rm)
    }
}

// As in mpfr_overflow: toward-zero modes give the largest finite value with the overflow's sign,
// and the other modes give an infinity. `Exact` panics, since an overflow is always inexact.
fn overflow_result(sp: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match (sp, rm) {
        (_, Exact) => panic!("Inexact Float addition"),
        (true, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
        (true, _) => (float_infinity!(), Greater),
        (false, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
        (false, _) => (float_negative_infinity!(), Less),
    }
}

// The exact integer-level fallback for a product whose exponent left the representable range:
// decomposes the finite nonzero operands and forms the sum in `add_scaled_round`.
fn scaled_path(
    a: &Float,
    b: &Float,
    c: &Float,
    sp: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let (
        Float(Finite {
            sign: a_sign,
            exponent: a_exponent,
            significand: a_significand,
            ..
        }),
        Float(Finite {
            exponent: b_exponent,
            significand: b_significand,
            ..
        }),
        Float(Finite {
            exponent: c_exponent,
            significand: c_significand,
            ..
        }),
    ) = (a, b, c)
    else {
        unreachable!()
    };
    add_scaled_round(
        *a_sign,
        a_significand,
        i64::from(*a_exponent) - i64::exact_from(significand_bits(a_significand)),
        sp,
        b_significand * c_significand,
        i64::from(*b_exponent) - i64::exact_from(significand_bits(b_significand))
            + i64::from(*c_exponent)
            - i64::exact_from(significand_bits(c_significand)),
        prec,
        rm,
    )
}

// This is mpfr_fma from fma.c, MPFR 4.2.2, with the result's precision passed explicitly and the
// singular cases from mpfr_fma_singular inlined. `neg_p` negates the product, which also covers
// mpfr_fms from fms.c: fms.c negates its addend to compute x * y - z, while Malachite's sub_mul
// computes self - y * z, which is the same fused operation with the product negated instead.
//
// The result is a + b * c (or a - b * c if `neg_p` is true), rounded to `prec` bits with rounding
// mode `rm`. If we take the product's precision to be prec(b) + prec(c), the product b * c is
// exact, except in case of overflow or underflow, so the fused operation is a single rounded
// addition. MPFR's same-precision limb-level fast paths are omitted: they are performance shortcuts
// for the same exact-product-then-add computation, which Malachite's mul already optimizes. The
// pointer-equality x == y square shortcut is omitted for the same reason.
//
// Where MPFR resolves an overflowed or underflowed product with UBF arithmetic, here the two easy
// cases are handled as in the C code (a definite overflow, and a product so far below the addend
// that a minimal-value sentinel with the product's sign rounds identically), and the remaining
// cases form the sum exactly at the integer level in `add_scaled_round`.
pub(crate) fn add_mul_helper(
    a: &Float,
    b: &Float,
    c: &Float,
    neg_p: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (a, b, c) {
        (Float(NaN), _, _) | (_, Float(NaN), _) | (_, _, Float(NaN)) => (float_nan!(), Equal),
        (_, float_either_infinity!(), _) | (_, _, float_either_infinity!()) => {
            // cases Inf*0 + a, 0*Inf + a, Inf - Inf
            if matches!(b, float_either_zero!()) || matches!(c, float_either_zero!()) {
                return (float_nan!(), Equal);
            }
            let sp = (float_sign(b) == float_sign(c)) != neg_p;
            match a {
                float_either_infinity!() if float_sign(a) != sp => (float_nan!(), Equal),
                _ => (
                    // an infinite addend with the same sign as the infinite product, or a finite
                    // addend: the result is an infinity with the product's sign
                    if sp {
                        float_infinity!()
                    } else {
                        float_negative_infinity!()
                    },
                    Equal,
                ),
            }
        }
        // now b and c are finite
        (float_either_infinity!(), _, _) => (
            if float_sign(a) {
                float_infinity!()
            } else {
                float_negative_infinity!()
            },
            Equal,
        ),
        (_, float_either_zero!(), _) | (_, _, float_either_zero!()) => {
            // The product is a signed zero, and mpfr_fma_singular's rules for combining it with the
            // addend (including the zero-plus-zero sign rules) are exactly the addition rules, so
            // the addition does the work.
            let sp = (float_sign(b) == float_sign(c)) != neg_p;
            a.add_prec_round_ref_val(
                if sp {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                },
                prec,
                rm,
            )
        }
        (float_either_zero!(), _, _) => {
            // the result is the rounded product; a negated product is computed via the negation
            // identity
            if neg_p {
                let (p, o) = b.mul_prec_round_ref_ref(c, prec, -rm);
                (-p, o.reverse())
            } else {
                b.mul_prec_round_ref_ref(c, prec, rm)
            }
        }
        (
            Float(Finite {
                sign: a_sign,
                exponent: a_exponent,
                precision: a_precision,
                ..
            }),
            Float(Finite {
                sign: b_sign,
                exponent: b_exponent,
                precision: b_precision,
                ..
            }),
            Float(Finite {
                sign: c_sign,
                exponent: c_exponent,
                precision: c_precision,
                ..
            }),
        ) => {
            // At precision prec(b) + prec(c) the product is exact unless its exponent leaves the
            // representable range, and Nearest overflows to an infinity, so an inexact product
            // means overflow if infinite and underflow otherwise.
            let (u, o) = b.mul_prec_ref_ref(c, b_precision + c_precision);
            if o == Equal {
                let u = if neg_p { -u } else { u };
                return a.add_prec_round_ref_val(u, prec, rm);
            }
            let sp = (*b_sign == *c_sign) != neg_p;
            let sa = *a_sign;
            if u.is_infinite() {
                // The product overflows. If it has the addend's sign, no cancellation is possible.
                // Also, |a| < 2^MAX_EXPONENT, so if the product's exponent is at least MAX_EXPONENT
                // + 3, |b * c| >= 2^(MAX_EXPONENT + 1) and the sum still overflows.
                let e = i64::from(*b_exponent) + i64::from(*c_exponent);
                if sp == sa || e >= SURE_OVERFLOW_EXPONENT {
                    return overflow_result(sp, prec, rm);
                }
            } else {
                // The product underflows: |b * c| < 2^(MIN_EXPONENT - 1). When that is at most half
                // an ulp of both the addend and the result, the product can be replaced by a
                // minimal-value sentinel with its sign; this is even true in case of equality for
                // Nearest thanks to the even-rounding rule. The + 1 on prec is necessary because
                // the exponent of the result can be one less than the addend's.
                if u64::exact_from(i64::from(*a_exponent) - Float::MIN_EXPONENT_I64)
                    >= max(*a_precision, prec.saturating_add(1))
                {
                    let sent = if sp {
                        Float::MIN_POSITIVE
                    } else {
                        -Float::MIN_POSITIVE
                    };
                    return a.add_prec_round_ref_val(sent, prec, rm);
                }
            }
            // the remaining overflow and underflow cases: form the sum exactly
            scaled_path(a, b, c, sp, prec, rm)
        }
    }
}

// Like `add_mul_helper`, but taking the addend by value, so that the additions in the main path can
// reuse its storage. The singular cases don't benefit from ownership and are delegated to the
// by-reference helper.
pub(crate) fn add_mul_val_helper(
    a: Float,
    b: &Float,
    c: &Float,
    neg_p: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let (
        Float(Finite {
            sign: a_sign,
            exponent: a_exponent,
            precision: a_precision,
            ..
        }),
        Float(Finite {
            sign: b_sign,
            exponent: b_exponent,
            precision: b_precision,
            ..
        }),
        Float(Finite {
            sign: c_sign,
            exponent: c_exponent,
            precision: c_precision,
            ..
        }),
    ) = (&a, b, c)
    else {
        return add_mul_helper(&a, b, c, neg_p, prec, rm);
    };
    let (sa, ae, ap) = (*a_sign, i64::from(*a_exponent), *a_precision);
    let sp = (*b_sign == *c_sign) != neg_p;
    let e = i64::from(*b_exponent) + i64::from(*c_exponent);
    let (u, o) = b.mul_prec_ref_ref(c, b_precision + c_precision);
    if o == Equal {
        let u = if neg_p { -u } else { u };
        return a.add_prec_round(u, prec, rm);
    }
    if u.is_infinite() {
        // as in the by-reference helper
        if sp == sa || e >= SURE_OVERFLOW_EXPONENT {
            return overflow_result(sp, prec, rm);
        }
    } else if u64::exact_from(ae - Float::MIN_EXPONENT_I64) >= max(ap, prec.saturating_add(1)) {
        let sent = if sp {
            Float::MIN_POSITIVE
        } else {
            -Float::MIN_POSITIVE
        };
        return a.add_prec_round(sent, prec, rm);
    }
    scaled_path(&a, b, c, sp, prec, rm)
}

impl Float {
    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. All three [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_round(y.clone(), z.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round(y.clone(), z.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round(y.clone(), z.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round(y.clone(), z.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round(y.clone(), z.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round(y.clone(), z.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round(
        self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, &y, &z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first two [`Float`]s are taken
    /// by value and the third by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_val_ref(y.clone(), &z, 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_val_ref(y.clone(), &z, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_val_ref(y.clone(), &z, 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_val_ref(y.clone(), &z, 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_val_ref(y.clone(), &z, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_val_ref(y.clone(), &z, 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_val_val_ref(
        self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, &y, z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first and third [`Float`]s are
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_val(&y, z.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_val(&y, z.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_val(&y, z.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_val(&y, z.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_val(&y, z.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_val(&y, z.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_val_ref_val(
        self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, y, &z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// value and the second and third by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_round_val_ref_ref(&y, &z, 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_round_val_ref_ref(&y, &z, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_round_val_ref_ref(&y, &z, 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_round_val_ref_ref(&y, &z, 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_ref(&y, &z, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x
    ///     .clone()
    ///     .add_mul_prec_round_val_ref_ref(&y, &z, 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_val_ref_ref(
        self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_val_helper(self, y, z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// reference and the second and third by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_val(y.clone(), z.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_ref_val_val(
        &self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, &y, &z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first and third [`Float`]s are
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sum is less than, equal to, or greater than the exact sum. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_ref(y.clone(), &z, 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_ref(y.clone(), &z, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_ref(y.clone(), &z, 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_ref(y.clone(), &z, 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_ref(y.clone(), &z, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_val_ref(y.clone(), &z, 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_ref_val_ref(
        &self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, &y, z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first two [`Float`]s are taken
    /// by reference and the third by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_val(&y, z.clone(), 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_val(&y, z.clone(), 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_val(&y, z.clone(), 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_val(&y, z.clone(), 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_val(&y, z.clone(), 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_val(&y, z.clone(), 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_ref_ref_val(
        &self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, y, &z, false, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. All three [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,p,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p,m)=f(x,\text{NaN},z,p,m)=f(x,y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p,m)=f(x,\pm0.0,\pm\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,z,p,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is not `Floor`
    /// - $f(0.0,y,z,p,m)=f(-0.0,y,z,p,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$
    ///   is `Floor`
    /// - $f(x,y,z,p,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,p,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec`] instead. If
    /// you know that your target precision is the maximum of the precisions of the inputs, consider
    /// using [`Float::add_mul_round`] instead. If both of these things are true, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, 5, Floor);
    /// assert_eq!(sum.to_string(), "6.75");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, 5, Ceiling);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, 5, Nearest);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, 20, Floor);
    /// assert_eq!(sum.to_string(), "6.9858170");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, 20, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, 20, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_mul_helper(self, y, z, false, prec, rm)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`Float`]s on the right-hand
    /// side are taken by value. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::add_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(y.clone(), z.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(y.clone(), z.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign(y.clone(), z.clone(), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_assign(
        &mut self,
        y: Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, &y, &z, false, prec, rm);
        *self = s;
        o
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] on the
    /// right-hand side is taken by value and the second by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::add_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(y.clone(), &z, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(y.clone(), &z, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_val_ref(y.clone(), &z, 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, &y, z, false, prec, rm);
        *self = s;
        o
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] on the
    /// right-hand side is taken by reference and the second by value. An [`Ordering`] is returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::add_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(&y, z.clone(), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(&y, z.clone(), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_val(&y, z.clone(), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, y, &z, false, prec, rm);
        *self = s;
        o
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`Float`]s on the right-hand
    /// side are taken by reference. An [`Ordering`] is returned, indicating whether the rounded sum
    /// is less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::add_mul_prec_assign`]
    /// instead. If you know that your target precision is the maximum of the precisions of the
    /// inputs, consider using [`Float::add_mul_round_assign`] instead. If both of these things are
    /// true, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the fused multiply-add is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_round_assign_ref_ref(&y, &z, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "6.75");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(&y, &z, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_prec_round_assign_ref_ref(&y, &z, 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "7.00");
    /// ```
    #[inline]
    pub fn add_mul_prec_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (s, o) = add_mul_helper(self, y, z, false, prec, rm);
        *self = s;
        o
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. All three [`Float`]s are taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_prec(y.clone(), z.clone(), 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_prec(y.clone(), z.clone(), 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec(self, y: Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. The first two [`Float`]s are taken by value and the third
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_val_val_ref(y.clone(), &z, 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_val_val_ref(y.clone(), &z, 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_val_val_ref(self, y: Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_val_val_ref(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. The first and third [`Float`]s are taken by value and the
    /// second by reference. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_val_ref_val(&y, z.clone(), 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_val_ref_val(&y, z.clone(), 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_val_ref_val(self, y: &Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_val_ref_val(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. The first [`Float`] is taken by value and the second and
    /// third by reference. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_val_ref_ref(&y, &z, 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_prec_val_ref_ref(&y, &z, 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_val_ref_ref(self, y: &Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_val_ref_ref(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. The first [`Float`] is taken by reference and the second
    /// and third by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_val_val(y.clone(), z.clone(), 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_val_val(y.clone(), z.clone(), 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_ref_val_val(&self, y: Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_ref_val_val(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. The first and third [`Float`]s are taken by reference and
    /// the second by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_val_ref(y.clone(), &z, 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_val_ref(y.clone(), &z, 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_ref_val_ref(&self, y: Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_ref_val_ref(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. The first two [`Float`]s are taken by reference and the
    /// third by value. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_ref_val(&y, z.clone(), 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_ref_val(&y, z.clone(), 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_ref_ref_val(&self, y: &Self, z: Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_ref_ref_val(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result to the nearest
    /// value of the specified precision. All three [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,z,p) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,p)=f(x,\text{NaN},z,p)=f(x,y,\text{NaN},p)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,p)=f(x,\pm0.0,\pm\infty,p)=\text{NaN}$
    /// - $f(\infty,y,z,p)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,p)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,p)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,p)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,p)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,p)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,p)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,p)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,p)=f(-0.0,y,z,p)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z,p)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round`] instead. If you know that your target precision is the maximum
    /// of the precisions of the inputs, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_ref_ref(&y, &z, 5);
    /// assert_eq!(sum.to_string(), "7.00");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_prec_ref_ref_ref(&y, &z, 20);
    /// assert_eq!(sum.to_string(), "6.9858246");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn add_mul_prec_ref_ref_ref(&self, y: &Self, z: &Self, prec: u64) -> (Self, Ordering) {
        self.add_mul_prec_round_ref_ref_ref(y, z, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// nearest value of the specified precision. Both [`Float`]s on the right-hand side are taken
    /// by value. An [`Ordering`] is returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign(y.clone(), z.clone(), 5), Greater);
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign(y.clone(), z.clone(), 20), Greater);
    /// assert_eq!(x.to_string(), "6.9858246");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_assign(&mut self, y: Self, z: Self, prec: u64) -> Ordering {
        self.add_mul_prec_round_assign(y, z, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] on the right-hand side is
    /// taken by value and the second by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign_val_ref(y.clone(), &z, 5), Greater);
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign_val_ref(y.clone(), &z, 20), Greater);
    /// assert_eq!(x.to_string(), "6.9858246");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_assign_val_ref(&mut self, y: Self, z: &Self, prec: u64) -> Ordering {
        self.add_mul_prec_round_assign_val_ref(y, z, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] on the right-hand side is
    /// taken by reference and the second by value. An [`Ordering`] is returned, indicating whether
    /// the rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function assigns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign_ref_val(&y, z.clone(), 5), Greater);
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign_ref_val(&y, z.clone(), 20), Greater);
    /// assert_eq!(x.to_string(), "6.9858246");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_prec_assign_ref_val(&mut self, y: &Self, z: Self, prec: u64) -> Ordering {
        self.add_mul_prec_round_assign_ref_val(y, z, prec, Nearest)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result to the
    /// nearest value of the specified precision. Both [`Float`]s on the right-hand side are taken
    /// by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// If the sum is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the inputs, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `max(self.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign_ref_ref(&y, &z, 5), Greater);
    /// assert_eq!(x.to_string(), "7.00");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_prec_assign_ref_ref(&y, &z, 20), Greater);
    /// assert_eq!(x.to_string(), "6.9858246");
    /// ```
    #[inline]
    pub fn add_mul_prec_assign_ref_ref(&mut self, y: &Self, z: &Self, prec: u64) -> Ordering {
        self.add_mul_prec_round_assign_ref_ref(y, z, prec, Nearest)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. All three [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_round(y.clone(), z.clone(), Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().add_mul_round(y.clone(), z.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_round(y.clone(), z.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round(self, y: Self, z: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. The first two [`Float`]s are taken by value and the third by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_val_ref(y.clone(), &z, Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_val_ref(y.clone(), &z, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_val_ref(y.clone(), &z, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_val_val_ref(
        self,
        y: Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_val_val_ref(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. The first and third [`Float`]s are taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_ref_val(&y, z.clone(), Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_ref_val(&y, z.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_ref_val(&y, z.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_val_ref_val(
        self,
        y: &Self,
        z: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_val_ref_val(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second and third by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_ref_ref(&y, &z, Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_ref_ref(&y, &z, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.clone().add_mul_round_val_ref_ref(&y, &z, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_mul_round_val_ref_ref(
        self,
        y: &Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_val_ref_ref(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second and third
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_round_ref_val_val(y.clone(), z.clone(), Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_round_ref_val_val(y.clone(), z.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_round_ref_val_val(y.clone(), z.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_ref_val_val(
        &self,
        y: Self,
        z: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_ref_val_val(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. The first and third [`Float`]s are taken by reference and the
    /// second by value. An [`Ordering`] is also returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_round_ref_val_ref(y.clone(), &z, Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_round_ref_val_ref(y.clone(), &z, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_round_ref_val_ref(y.clone(), &z, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_ref_val_ref(
        &self,
        y: Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_ref_val_ref(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. The first two [`Float`]s are taken by reference and the third by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_round_ref_ref_val(&y, z.clone(), Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_round_ref_ref_val(&y, z.clone(), Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_round_ref_ref_val(&y, z.clone(), Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_ref_ref_val(
        &self,
        y: &Self,
        z: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_ref_ref_val(y, z, prec, rm)
    }

    /// Adds a [`Float`] and the product of two other [`Float`]s, rounding the result with the
    /// specified rounding mode. All three [`Float`]s are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,z,m) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z,m)=f(x,\text{NaN},z,m)=f(x,y,\text{NaN},m)=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0,m)=f(x,\pm0.0,\pm\infty,m)=\text{NaN}$
    /// - $f(\infty,y,z,m)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z,m)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z,m)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z,m)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z,m)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z,m)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z,m)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z,m)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   not `Floor`
    /// - $f(0.0,y,z,m)=f(-0.0,y,z,m)=-0.0$ if $x$ and $yz$ are zeros of different signs and $m$ is
    ///   `Floor`
    /// - $f(x,y,z,m)=0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is not `Floor`
    /// - $f(x,y,z,m)=-0.0$ if $x=-yz$, $x$ is finite and nonzero, and $m$ is `Floor`
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the output.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,z,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the output.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,z,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::add_mul_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`add_mul`](malachite_base::num::arithmetic::traits::AddMul::add_mul) instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let (sum, o) = x.add_mul_round_ref_ref_ref(&y, &z, Floor);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = x.add_mul_round_ref_ref_ref(&y, &z, Ceiling);
    /// assert_eq!(sum.to_string(), "6.9858236817489106");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = x.add_mul_round_ref_ref_ref(&y, &z, Nearest);
    /// assert_eq!(sum.to_string(), "6.9858236817489097");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_mul_round_ref_ref_ref(
        &self,
        y: &Self,
        z: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_ref_ref_ref(y, z, prec, rm)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result with the
    /// specified rounding mode. Both [`Float`]s on the right-hand side are taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign(y.clone(), z.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_round_assign(y.clone(), z.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.9858236817489106");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign(y.clone(), z.clone(), Nearest), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_assign(&mut self, y: Self, z: Self, rm: RoundingMode) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_assign(y, z, prec, rm)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result with the
    /// specified rounding mode. The first [`Float`] on the right-hand side is taken by value and
    /// the second by reference. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_val_ref(y.clone(), &z, Floor), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_round_assign_val_ref(y.clone(), &z, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.9858236817489106");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_val_ref(y.clone(), &z, Nearest), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_assign_val_ref(
        &mut self,
        y: Self,
        z: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_assign_val_ref(y, z, prec, rm)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result with the
    /// specified rounding mode. The first [`Float`] on the right-hand side is taken by reference
    /// and the second by value. An [`Ordering`] is returned, indicating whether the rounded sum is
    /// less than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_ref_val(&y, z.clone(), Floor), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.add_mul_round_assign_ref_val(&y, z.clone(), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.9858236817489106");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_ref_val(&y, z.clone(), Nearest), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn add_mul_round_assign_ref_val(
        &mut self,
        y: &Self,
        z: Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_assign_ref_val(y, z, prec, rm)
    }

    /// Adds the product of two [`Float`]s to a [`Float`] in place, rounding the result with the
    /// specified rounding mode. Both [`Float`]s on the right-hand side are taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function assigns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x+yz$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::add_mul_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using
    /// [`add_mul_assign`](malachite_base::num::arithmetic::traits::AddMulAssign::add_mul_assign)
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_ref_ref(&y, &z, Floor), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_ref_ref(&y, &z, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "6.9858236817489106");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.add_mul_round_assign_ref_ref(&y, &z, Nearest), Less);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    pub fn add_mul_round_assign_ref_ref(
        &mut self,
        y: &Self,
        z: &Self,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_round_assign_ref_ref(y, z, prec, rm)
    }
}

impl AddMul<Self, Self> for Float {
    type Output = Self;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking all three by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.add_mul(y, z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: Self, z: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec(y, z, prec).0
    }
}

impl AddMul<Self, &Self> for Float {
    type Output = Self;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking the first two by value and
    /// the third by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.add_mul(y, &z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: Self, z: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_val_val_ref(y, z, prec).0
    }
}

impl AddMul<&Self, Self> for Float {
    type Output = Self;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking the first and third by
    /// value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.add_mul(&y, z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: &Self, z: Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_val_ref_val(y, z, prec).0
    }
}

impl AddMul<&Self, &Self> for Float {
    type Output = Self;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking the first by value and the
    /// second and third by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(x.add_mul(&y, &z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: &Self, z: &Self) -> Self {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_val_ref_ref(y, z, prec).0
    }
}

impl AddMul<Float, Float> for &Float {
    type Output = Float;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking the first by reference and
    /// the second and third by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.add_mul(y, z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: Float, z: Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_ref_val_val(y, z, prec).0
    }
}

impl AddMul<Float, &Float> for &Float {
    type Output = Float;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking the first and third by
    /// reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.add_mul(y, &z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: Float, z: &Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_ref_val_ref(y, z, prec).0
    }
}

impl AddMul<&Float, Float> for &Float {
    type Output = Float;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking the first two by reference
    /// and the third by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.add_mul(&y, z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: &Float, z: Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_ref_ref_val(y, z, prec).0
    }
}

impl AddMul<&Float, &Float> for &Float {
    type Output = Float;
    /// Adds a [`Float`] and the product of two other [`Float`]s, taking all three by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y,z) = x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,z)=f(x,\text{NaN},z)=f(x,y,\text{NaN})=\text{NaN}$
    /// - $f(x,\pm\infty,\pm0.0)=f(x,\pm0.0,\pm\infty)=\text{NaN}$
    /// - $f(\infty,y,z)=\text{NaN}$ if $yz=-\infty$
    /// - $f(-\infty,y,z)=\text{NaN}$ if $yz=\infty$
    /// - $f(\infty,y,z)=\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq-\infty$
    /// - $f(-\infty,y,z)=-\infty$ if neither $y$ nor $z$ is `NaN` and $yz\neq\infty$
    /// - $f(x,y,z)=\infty$ if $x$ is finite and $yz=\infty$
    /// - $f(x,y,z)=-\infty$ if $x$ is finite and $yz=-\infty$
    /// - $f(0.0,y,z)=0.0$ if $yz=0.0$
    /// - $f(-0.0,y,z)=-0.0$ if $yz=-0.0$
    /// - $f(0.0,y,z)=f(-0.0,y,z)=0.0$ if $x$ and $yz$ are zeros of different signs
    /// - $f(x,y,z)=0.0$ if $x=-yz$, $x$ is finite and nonzero,
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,z)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,z)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,y,z)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,z)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,z)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,z)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// assert_eq!(&x.add_mul(&y, &z).to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul(self, y: &Float, z: &Float) -> Float {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_ref_ref_ref(y, z, prec).0
    }
}

impl AddMulAssign<Self, Self> for Float {
    /// Adds the product of two [`Float`]s to a [`Float`] in place, both [`Float`]s on the
    /// right-hand side being taken by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.add_mul_assign(y, z);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: Self, z: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_assign(y, z, prec);
    }
}

impl AddMulAssign<Self, &Self> for Float {
    /// Adds the product of two [`Float`]s to a [`Float`] in place, the first [`Float`] on the
    /// right-hand side being taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.add_mul_assign(y, &z);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: Self, z: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_assign_val_ref(y, z, prec);
    }
}

impl AddMulAssign<&Self, Self> for Float {
    /// Adds the product of two [`Float`]s to a [`Float`] in place, the first [`Float`] on the
    /// right-hand side being taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.add_mul_assign(&y, z);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: &Self, z: Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_assign_ref_val(y, z, prec);
    }
}

impl AddMulAssign<&Self, &Self> for Float {
    /// Adds the product of two [`Float`]s to a [`Float`] in place, both [`Float`]s on the
    /// right-hand side being taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the sum
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets x+yz+\varepsilon.
    /// $$
    /// - If $x+yz$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+yz|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::add_mul_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::add_mul_round_assign`]. If you want to specify the output precision, consider using
    /// [`Float::add_mul_prec_assign`]. If you want both of these things, consider using
    /// [`Float::add_mul_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `y.significant_bits() +
    /// z.significant_bits()`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(PI);
    /// let y = Float::from(E);
    /// let z = Float::from(SQRT_2);
    /// x.add_mul_assign(&y, &z);
    /// assert_eq!(x.to_string(), "6.9858236817489097");
    /// ```
    #[inline]
    fn add_mul_assign(&mut self, y: &Self, z: &Self) {
        let prec = max!(
            self.significant_bits(),
            y.significant_bits(),
            z.significant_bits()
        );
        self.add_mul_prec_assign_ref_ref(y, z, prec);
    }
}

/// Adds a primitive float and the product of two other primitive floats with a single rounding,
/// using emulated [`Float`] arithmetic.
///
/// This is a correctly-rounded fused multiply-add: the product is not rounded before the addition,
/// so the result is the true value of $x+yz$ rounded once to the nearest representable value. It
/// agrees with the standard library's hardware-backed `mul_add`, up to argument order.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use core::f64::consts::{E, PI, SQRT_2};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::add_mul::*;
///
/// assert_eq!(
///     NiceFloat(primitive_float_add_mul(PI, E, SQRT_2)),
///     NiceFloat(6.98582368174891)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_add_mul<T: PrimitiveFloat>(x: T, y: T, z: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_float_to_float_fn(Float::add_mul_prec, x, y, z)
}
