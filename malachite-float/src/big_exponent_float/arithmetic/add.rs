// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::ops::{Add, AddAssign};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{SignificantBits, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float::add::add_float_significands_ref_ref;
use malachite_nz::natural::arithmetic::float::sub::sub_float_significands_ref_ref;
use malachite_nz::platform::Limb;

// The precision of a finite nonzero value's significand: the number of bits from its highest set
// bit to its lowest set bit. Since a significand is aligned to the top of its limbs and has no zero
// low limb, it occupies exactly as many limbs as a `Float` significand of this precision would.
pub(crate) fn significand_prec(significand: &Natural) -> u64 {
    // The significand is aligned to the top of its limbs, so its highest set bit is the highest bit
    // of its highest limb; and its lowest limb is nonzero, so it holds the lowest set bit.
    (significand.limb_count() << Limb::LOG_WIDTH)
        - TrailingZeros::trailing_zeros(significand.limbs()[0])
}

// The precision with which a finite nonzero significand is passed to one of `Float`'s kernels. Any
// precision at least the significand's own, and requiring the same number of limbs, describes the
// same significand; and the kernels have faster paths for inputs whose precision equals the output
// precision. So the output precision `out_prec` is used when it is such a precision.
pub(crate) fn kernel_prec(significand: &Natural, out_prec: Option<u64>) -> u64 {
    let prec = significand_prec(significand);
    match out_prec {
        Some(out_prec)
            if out_prec >= prec && significand.limb_count() == out_prec.div_ceil(Limb::WIDTH) =>
        {
            out_prec
        }
        _ => prec,
    }
}

// A significand produced by one of `Float`'s kernels, which has as many limbs as its output
// precision requires, converted to a `BigExponentFloat` significand by removing zero low limbs.
pub(crate) fn finish_significand(mut significand: Natural) -> Natural {
    BigExponentFloat::trim_significand(&mut significand);
    significand
}

// The exact sum of two finite nonzero values, each given as a sign (`true` for positive), an
// exponent, and a significand: the value is ±m * 2^(e - b), where b is the number of significant
// bits of m. The significands are aligned at the lower of the two lowest bit positions and added as
// `Integer`s.
pub(crate) fn add_finite_exact(
    x_sign: bool,
    x_exponent: &Integer,
    x_significand: &Natural,
    y_sign: bool,
    y_exponent: &Integer,
    y_significand: &Natural,
) -> BigExponentFloat {
    let x_low = x_exponent - Integer::from(x_significand.significant_bits());
    let y_low = y_exponent - Integer::from(y_significand.significant_bits());
    let (x, y, low) = if x_low <= y_low {
        let shift = u64::exact_from(&(y_low - &x_low));
        (
            Integer::from_sign_and_abs_ref(x_sign, x_significand),
            Integer::from_sign_and_abs(y_sign, y_significand << shift),
            x_low,
        )
    } else {
        let shift = u64::exact_from(&(x_low - &y_low));
        (
            Integer::from_sign_and_abs(x_sign, x_significand << shift),
            Integer::from_sign_and_abs_ref(y_sign, y_significand),
            y_low,
        )
    };
    let sum = x + y;
    if sum == 0u32 {
        return BigExponentFloat::ZERO;
    }
    let sign = sum > 0u32;
    let m = sum.unsigned_abs();
    let exponent = low + Integer::from(m.significant_bits());
    BigExponentFloat::from_parts(sign, exponent, m)
}

// The sum of two finite nonzero values, given as in `add_finite_exact`, rounded to `prec` bits with
// `rm`.
//
// Let x be the operand with the larger exponent e, and let g be the smaller of x's lowest bit
// position and e - prec - 2. Every value at which rounding to `prec` bits can change (the
// representable values and, for `Nearest`, the midpoints between them) is a multiple of 2^g, since
// the sum's exponent is at least e - 1; so is x. If the other operand y is smaller than 2^g in
// absolute value, x + y lies strictly between x and the next multiple of 2^g in y's direction, and
// so does x + s, where s = ±2^(g - 1) has y's sign; the two round to the same value, on the same
// side. Replacing y by s keeps the exact sum short however far apart the exponents are.
fn add_finite_prec_round_generic(
    x: (bool, &Integer, &Natural),
    y: (bool, &Integer, &Natural),
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (x, y) = if x.1 >= y.1 { (x, y) } else { (y, x) };
    let x_low = x.1 - Integer::from(x.2.significant_bits());
    let g = core::cmp::min(x_low, x.1 - Integer::from(prec) - Integer::TWO);
    let mut sum = if y.1 <= &g {
        add_finite_exact(x.0, x.1, x.2, y.0, &g, &Natural::ONE)
    } else {
        add_finite_exact(x.0, x.1, x.2, y.0, y.1, y.2)
    };
    let o = sum.round_to_prec_round_assign(prec, rm);
    (sum, o)
}

// The same as `add_finite_prec_round_generic`, using `Float`'s addition and subtraction kernels,
// which have fast paths for operands and outputs of one, two, or three limbs. The kernels take
// `i32` exponents, but only the difference between the operands' exponents matters, so x is given
// exponent 0 and y the negated gap, and the output exponent is added to x's. When the gap is larger
// than every precision involved plus 3, y lies below both x's lowest bit and x's rounding position,
// so, by the argument in `add_finite_prec_round_generic`, any gap at least that large gives the
// same result, and the gap is reduced to it. If even the reduced gap does not fit in an `i32`, the
// generic implementation is used.
fn add_finite_prec_round(
    x: (bool, &Integer, &Natural),
    y: (bool, &Integer, &Natural),
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let (x, y) = if x.1 >= y.1 { (x, y) } else { (y, x) };
    let x_prec = kernel_prec(x.2, Some(prec));
    let y_prec = kernel_prec(y.2, Some(prec));
    let bound = x_prec.max(y_prec).max(prec).saturating_add(3);
    let gap = x.1 - y.1;
    let gap = if gap > bound {
        bound
    } else {
        u64::exact_from(&gap)
    };
    let Ok(gap) = i32::try_from(gap) else {
        return add_finite_prec_round_generic(x, y, prec, rm);
    };
    let (x_sign, x_exponent, x_significand) = x;
    let (y_sign, _, y_significand) = y;
    let abs_rm = if x_sign { rm } else { -rm };
    if x_sign == y_sign {
        let (sum, sum_exp, o) = add_float_significands_ref_ref(
            x_significand,
            0,
            x_prec,
            y_significand,
            -gap,
            y_prec,
            prec,
            abs_rm,
        );
        (
            BigExponentFloat(Finite {
                sign: x_sign,
                exponent: x_exponent + Integer::from(sum_exp),
                significand: finish_significand(sum),
            }),
            if x_sign { o } else { o.reverse() },
        )
    } else {
        let (diff, diff_exp, o, neg) = sub_float_significands_ref_ref(
            x_significand,
            0,
            x_prec,
            y_significand,
            -gap,
            y_prec,
            prec,
            abs_rm,
        );
        if diff == 0u32 {
            return (BigExponentFloat::ZERO, Equal);
        }
        (
            BigExponentFloat(Finite {
                sign: x_sign != neg,
                exponent: x_exponent + Integer::from(diff_exp),
                significand: finish_significand(diff),
            }),
            if x_sign == neg { o.reverse() } else { o },
        )
    }
}

// `x` rounded with `prec_rm`, if it is `Some`, or `x` itself.
pub(crate) fn round_optional(
    mut x: BigExponentFloat,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    match prec_rm {
        Some((prec, rm)) => {
            let o = x.round_to_prec_round_assign(prec, rm);
            (x, o)
        }
        None => (x, Equal),
    }
}

// The sign of `x` if it is infinite.
pub(crate) const fn infinity_sign(x: &BigExponentFloat) -> Option<bool> {
    if let BigExponentFloat(Infinity { sign }) = x {
        Some(*sign)
    } else {
        None
    }
}

// x + y, or x - y if `negate_y` is true, rounded with `prec_rm` if it is `Some`, and exact
// otherwise. Taking the operands as `Cow`s lets every by-value and by-reference variant share this
// function, moving an operand out when it is the result.
pub(crate) fn add_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    negate_y: bool,
    prec_rm: Option<(u64, RoundingMode)>,
) -> (BigExponentFloat, Ordering) {
    add_helper_with(x, y, negate_y, prec_rm, true)
}

crate_test_fn! {
    // x + y, or x - y if `negate_y` is true, rounded to `prec` bits with `rm`, without `Float`'s
    // kernels: the exact sum (of x and a short stand-in for y when y is far below x) is formed with
    // `Natural` arithmetic and then rounded. Used to check and benchmark the kernel path.
    add_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        negate_y: bool,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        add_helper_with(Cow::Borrowed(x), Cow::Borrowed(y), negate_y, Some((prec, rm)), false)
    }
}

fn add_helper_with(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    negate_y: bool,
    prec_rm: Option<(u64, RoundingMode)>,
    use_kernels: bool,
) -> (BigExponentFloat, Ordering) {
    if let Some((prec, _)) = prec_rm {
        assert_ne!(prec, 0);
    }
    if x.is_nan() || y.is_nan() {
        return (BigExponentFloat::NAN, Equal);
    }
    match (infinity_sign(&x), infinity_sign(&y)) {
        (Some(x_sign), Some(y_sign)) => {
            return if x_sign == (y_sign != negate_y) {
                (x.into_owned(), Equal)
            } else {
                (BigExponentFloat::NAN, Equal)
            };
        }
        (Some(_), None) => return (x.into_owned(), Equal),
        (None, Some(_)) => {
            let y = y.into_owned();
            return (if negate_y { y.negate() } else { y }, Equal);
        }
        (None, None) => {}
    }
    if y.is_zero() {
        return round_optional(x.into_owned(), prec_rm);
    }
    if x.is_zero() {
        let y = y.into_owned();
        return round_optional(if negate_y { y.negate() } else { y }, prec_rm);
    }
    if let (
        BigExponentFloat(Finite {
            sign: x_sign,
            exponent: x_exponent,
            significand: x_significand,
        }),
        BigExponentFloat(Finite {
            sign: y_sign,
            exponent: y_exponent,
            significand: y_significand,
        }),
    ) = (&*x, &*y)
    {
        let y_sign = *y_sign != negate_y;
        if let Some((prec, rm)) = prec_rm {
            let x = (*x_sign, x_exponent, x_significand);
            let y = (y_sign, y_exponent, y_significand);
            if use_kernels {
                add_finite_prec_round(x, y, prec, rm)
            } else {
                add_finite_prec_round_generic(x, y, prec, rm)
            }
        } else {
            (
                add_finite_exact(
                    *x_sign,
                    x_exponent,
                    x_significand,
                    y_sign,
                    y_exponent,
                    y_significand,
                ),
                Equal,
            )
        }
    } else {
        unreachable!()
    }
}

impl BigExponentFloat {
    /// Adds two [`BigExponentFloat`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p,m)=0$ if $x$ is finite
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::add_prec`]
    /// instead. If you want the exact sum, consider using `+` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// addition.
    ///
    /// This is FLINT's `arf_add`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round(BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round(BigExponentFloat::from(E), 5, Ceiling);
    /// assert_eq!(format!("{sum:?}"), "0x6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round(BigExponentFloat::from(E), 5, Nearest);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round(BigExponentFloat::from(E), 20, Floor);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round(BigExponentFloat::from(E), 20, Ceiling);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc21");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round(BigExponentFloat::from(E), 20, Nearest);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        add_helper(Cow::Owned(self), Cow::Owned(other), false, Some((prec, rm)))
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`BigExponentFloat`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     BigExponentFloat::from(PI).add_prec_round_val_ref(&BigExponentFloat::from(E), 5, Floor);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc21");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_round_val_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`BigExponentFloat`] is taken by reference and the second
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded sum is less than,
    /// equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc21");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_val(
    ///     BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded sum is less than, equal to, or greater than
    /// the exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\infty,-\infty,p,m)=f(-\infty,\infty,p,m)=
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p,m)=f(x,-\infty,p,m)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p,m)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x6.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc21");
    /// assert_eq!(o, Greater);
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_round_ref_ref(
    ///     &BigExponentFloat::from(E),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            false,
            Some((prec, rm)),
        )
    }

    /// Adds a [`BigExponentFloat`] to a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.dc208");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.dc21");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign(BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.dc208");
    /// ```
    #[inline]
    pub fn add_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(Cow::Owned(x), Cow::Owned(other), false, Some((prec, rm)));
        *self = result;
        o
    }

    /// Adds a [`BigExponentFloat`] to a [`BigExponentFloat`] in place, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded sum is less than, equal to, or greater than the exact sum. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`]
    /// to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p+1}$.
    /// - If $x+y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x+y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `prec` is too small for an exact
    /// addition.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x6.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&BigExponentFloat::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.dc208");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.dc21");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(
    ///     x.add_prec_round_assign_ref(&BigExponentFloat::from(E), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(format!("{x:?}"), "0x5.dc208");
    /// ```
    #[inline]
    pub fn add_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(Cow::Owned(x), Cow::Borrowed(other), false, Some((prec, rm)));
        *self = result;
        o
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sum is less than, equal to, or greater than the exact sum.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p)=0$ if $x$ is finite
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::add_prec_round`] instead. If you want the exact sum, consider using `+`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Owned(self),
            Cow::Owned(other),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`BigExponentFloat`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_val_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) = BigExponentFloat::from(PI).add_prec_val_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Owned(self),
            Cow::Borrowed(other),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the nearest value of the specified
    /// precision. The first [`BigExponentFloat`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sum is less than, equal to, or
    /// greater than the exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) = (&BigExponentFloat::from(PI)).add_prec_ref_val(BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     (&BigExponentFloat::from(PI)).add_prec_ref_val(BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Owned(other),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds two [`BigExponentFloat`]s, rounding the result to the nearest value of the specified
    /// precision. Both [`BigExponentFloat`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sum is less than, equal to, or greater than the
    /// exact sum. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\infty,-\infty,p)=f(-\infty,\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x,p)=f(x,-\infty,p)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x,p)=0$ if $x$ is finite
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sum, o) =
    ///     (&BigExponentFloat::from(PI)).add_prec_ref_ref(&BigExponentFloat::from(E), 5);
    /// assert_eq!(format!("{sum:?}"), "0x5.c");
    /// assert_eq!(o, Less);
    ///
    /// let (sum, o) =
    ///     (&BigExponentFloat::from(PI)).add_prec_ref_ref(&BigExponentFloat::from(E), 20);
    /// assert_eq!(format!("{sum:?}"), "0x5.dc208");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn add_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        add_helper(
            Cow::Borrowed(self),
            Cow::Borrowed(other),
            false,
            Some((prec, Nearest)),
        )
    }

    /// Adds a [`BigExponentFloat`] to a [`BigExponentFloat`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand side is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.add_prec_assign(BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x5.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.add_prec_assign(BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x5.dc208");
    /// ```
    #[inline]
    pub fn add_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(
            Cow::Owned(x),
            Cow::Owned(other),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }

    /// Adds a [`BigExponentFloat`] to a [`BigExponentFloat`] in place, rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] on the right-hand side is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded sum is less
    /// than, equal to, or greater than the exact sum. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// x \gets x+y+\varepsilon.
    /// $$
    /// - If $x+y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x+y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x+y|\rfloor-p}$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.add_prec_assign_ref(&BigExponentFloat::from(E), 5), Less);
    /// assert_eq!(format!("{x:?}"), "0x5.c");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.add_prec_assign_ref(&BigExponentFloat::from(E), 20), Less);
    /// assert_eq!(format!("{x:?}"), "0x5.dc208");
    /// ```
    #[inline]
    pub fn add_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = add_helper(
            Cow::Owned(x),
            Cow::Borrowed(other),
            false,
            Some((prec, Nearest)),
        );
        *self = result;
        o
    }
}

impl Add<Self> for BigExponentFloat {
    type Output = Self;

    /// Adds two [`BigExponentFloat`]s. Both [`BigExponentFloat`]s are taken by value. The sum is
    /// exact.
    ///
    /// $$
    /// f(x,y) = x+y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the sum spans the gap between
    /// their exponents, so values whose exponents are far apart give a large sum. To bound its
    /// size, use [`BigExponentFloat::add_prec_round`] or [`BigExponentFloat::add_prec`] instead.
    ///
    /// This is FLINT's `arf_add` with precision `ARF_PREC_EXACT`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5) + BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.c"
    /// );
    /// ```
    #[inline]
    fn add(self, other: Self) -> Self {
        add_helper(Cow::Owned(self), Cow::Owned(other), false, None).0
    }
}

impl Add<&Self> for BigExponentFloat {
    type Output = Self;

    /// Adds two [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by value and the
    /// second by reference. The sum is exact.
    ///
    /// $$
    /// f(x,y) = x+y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the sum spans the gap between
    /// their exponents, so values whose exponents are far apart give a large sum. To bound its
    /// size, use [`BigExponentFloat::add_prec_round`] or [`BigExponentFloat::add_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         BigExponentFloat::from(1.5) + &BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.c"
    /// );
    /// ```
    #[inline]
    fn add(self, other: &Self) -> Self {
        add_helper(Cow::Owned(self), Cow::Borrowed(other), false, None).0
    }
}

impl Add<BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds two [`BigExponentFloat`]s. The first [`BigExponentFloat`] is taken by reference and the
    /// second by value. The sum is exact.
    ///
    /// $$
    /// f(x,y) = x+y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the sum spans the gap between
    /// their exponents, so values whose exponents are far apart give a large sum. To bound its
    /// size, use [`BigExponentFloat::add_prec_round`] or [`BigExponentFloat::add_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)) + BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.c"
    /// );
    /// ```
    #[inline]
    fn add(self, other: BigExponentFloat) -> BigExponentFloat {
        add_helper(Cow::Borrowed(self), Cow::Owned(other), false, None).0
    }
}

impl Add<&BigExponentFloat> for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Adds two [`BigExponentFloat`]s. Both [`BigExponentFloat`]s are taken by reference. The sum
    /// is exact.
    ///
    /// $$
    /// f(x,y) = x+y.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\infty,-\infty)=f(-\infty,\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\infty$ if $x$ is not NaN or $-\infty$
    /// - $f(-\infty,x)=f(x,-\infty)=-\infty$ if $x$ is not NaN or $\infty$
    /// - $f(x,-x)=0$ if $x$ is finite
    ///
    /// When both values are finite and nonzero, the significand of the sum spans the gap between
    /// their exponents, so values whose exponents are far apart give a large sum. To bound its
    /// size, use [`BigExponentFloat::add_prec_round`] or [`BigExponentFloat::add_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         (&BigExponentFloat::from(1.5)) + &BigExponentFloat::from(0.25)
    ///     ),
    ///     "0x1.c"
    /// );
    /// ```
    #[inline]
    fn add(self, other: &BigExponentFloat) -> BigExponentFloat {
        add_helper(Cow::Borrowed(self), Cow::Borrowed(other), false, None).0
    }
}

impl AddAssign<Self> for BigExponentFloat {
    /// Adds a [`BigExponentFloat`] to a [`BigExponentFloat`] in place. The [`BigExponentFloat`] on
    /// the right-hand side is taken by value. The sum is exact.
    ///
    /// $$
    /// x \gets x+y.
    /// $$
    ///
    /// See the `+` documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x += BigExponentFloat::from(0.25);
    /// assert_eq!(format!("{x:?}"), "0x1.c");
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_helper(Cow::Owned(x), Cow::Owned(other), false, None).0;
    }
}

impl AddAssign<&Self> for BigExponentFloat {
    /// Adds a [`BigExponentFloat`] to a [`BigExponentFloat`] in place. The [`BigExponentFloat`] on
    /// the right-hand side is taken by reference. The sum is exact.
    ///
    /// $$
    /// x \gets x+y.
    /// $$
    ///
    /// See the `+` documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())` plus the absolute difference of the two exponents.
    ///
    /// # Panics
    /// Panics if the exponents differ by $2^{64}$ or more and both values are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(1.5);
    /// x += &BigExponentFloat::from(0.25);
    /// assert_eq!(format!("{x:?}"), "0x1.c");
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        let x = core::mem::replace(self, Self::ZERO);
        *self = add_helper(Cow::Owned(x), Cow::Borrowed(other), false, None).0;
    }
}
