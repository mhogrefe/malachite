// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::is_power_of_2::abs_is_power_of_2;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{float_either_infinity, float_either_zero, float_nan, Float};
use core::cmp::max;
use core::cmp::Ordering::{self, *};
use core::mem::swap;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, IsPowerOf2, NegAssign, Sign};
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_div::{
    div_float_significands_in_place, div_float_significands_in_place_ref,
    div_float_significands_ref_ref, div_float_significands_ref_val,
};
use malachite_q::Rational;

const DIV_RATIONAL_THRESHOLD: u64 = 50;
const RATIONAL_DIV_THRESHOLD: u64 = 50;

fn div_rational_prec_round_assign_naive(
    x: &mut Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> Ordering {
    assert_ne!(prec, 0);
    match (&mut *x, y) {
        (float_nan!(), _) => Equal,
        (Float(Infinity { sign }), y) => {
            if y < 0 {
                sign.not_assign();
            };
            Equal
        }
        (Float(Zero { sign }), y) => {
            match y.sign() {
                Equal => *x = float_nan!(),
                Greater => {}
                Less => sign.not_assign(),
            }
            Equal
        }
        (x, y) => {
            if y == 0 {
                *x = Float(Infinity { sign: *x > 0u32 });
                Equal
            } else {
                let not_sign = *x < 0;
                let mut z = Float::ZERO;
                swap(x, &mut z);
                let (mut quotient, o) =
                    Float::from_rational_prec_round(Rational::exact_from(z) / y, prec, rm);
                if quotient == 0u32 && not_sign {
                    quotient.neg_assign();
                }
                *x = quotient;
                o
            }
        }
    }
}

fn div_rational_prec_round_assign_naive_ref(
    x: &mut Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Ordering {
    assert_ne!(prec, 0);
    match (&mut *x, y) {
        (float_nan!(), _) => Equal,
        (Float(Infinity { sign }), y) => {
            if *y < 0 {
                sign.not_assign();
            };
            Equal
        }
        (Float(Zero { sign }), y) => {
            match y.sign() {
                Equal => *x = float_nan!(),
                Greater => {}
                Less => sign.not_assign(),
            }
            Equal
        }
        (x, y) => {
            if *y == 0 {
                *x = Float(Infinity { sign: *x > 0u32 });
                Equal
            } else {
                let not_sign = *x < 0;
                let mut z = Float::ZERO;
                swap(x, &mut z);
                let (mut quotient, o) =
                    Float::from_rational_prec_round(Rational::exact_from(z) / y, prec, rm);
                if quotient == 0u32 && not_sign {
                    quotient.neg_assign();
                }
                *x = quotient;
                o
            }
        }
    }
}

pub_test! {div_rational_prec_round_naive(
    mut x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = div_rational_prec_round_assign_naive(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {div_rational_prec_round_naive_val_ref(
    mut x: Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = div_rational_prec_round_assign_naive_ref(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {div_rational_prec_round_naive_ref_val(
    x: &Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (float_nan!(), _) => (float_nan!(), Equal),
        (Float(Infinity { sign }), y) => (
            if y >= 0u32 {
                Float(Infinity { sign: *sign })
            } else {
                Float(Infinity { sign: !*sign })
            },
            Equal,
        ),
        (Float(Zero { sign }), y) => (
            match y.sign() {
                Equal => float_nan!(),
                Greater => Float(Zero { sign: *sign }),
                Less => Float(Zero { sign: !*sign }),
            },
            Equal,
        ),
        (x, y) => {
            if y == 0 {
                (Float(Infinity { sign: *x > 0u32 }), Equal)
            } else {
                let (mut quotient, o) =
                    Float::from_rational_prec_round(Rational::exact_from(x) / y, prec, rm);
                if quotient == 0u32 && *x < 0 {
                    quotient.neg_assign();
                }
                (quotient, o)
            }
        }
    }
}}

pub_test! {div_rational_prec_round_naive_ref_ref(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (float_nan!(), _) => (float_nan!(), Equal),
        (Float(Infinity { sign }), y) => (
            if *y >= 0u32 {
                Float(Infinity { sign: *sign })
            } else {
                Float(Infinity { sign: !*sign })
            },
            Equal,
        ),
        (Float(Zero { sign }), y) => (
            match y.sign() {
                Equal => float_nan!(),
                Greater => Float(Zero { sign: *sign }),
                Less => Float(Zero { sign: !*sign }),
            },
            Equal,
        ),
        (x, y) => {
            if *y == 0 {
                (Float(Infinity { sign: *x > 0u32 }), Equal)
            } else {
                let (mut quotient, o) =
                    Float::from_rational_prec_round(Rational::exact_from(x) / y, prec, rm);
                if quotient == 0u32 && *x < 0 {
                    quotient.neg_assign();
                }
                (quotient, o)
            }
        }
    }
}}

fn div_rational_prec_round_assign_direct(
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
            *x <<= log_d;
            *x >>= log_n;
            o
        }
        (None, Some(log_d)) => {
            let o = x.div_prec_round_assign(Float::exact_from(n), prec, rm);
            *x <<= log_d;
            o
        }
        (Some(log_n), None) => {
            let o = x.mul_prec_round_assign(Float::exact_from(d), prec, rm);
            *x >>= log_n;
            o
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + d.significant_bits();
            x.mul_prec_round_assign(d, mul_prec, Floor);
            x.div_prec_round_assign(n, prec, rm)
        }
    };
    if sign {
        o
    } else {
        x.neg_assign();
        o.reverse()
    }
}

fn div_rational_prec_round_assign_direct_ref(
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
            *x <<= log_d;
            *x >>= log_n;
            o
        }
        (None, Some(log_d)) => {
            let o = x.div_prec_round_assign(Float::exact_from(n), prec, rm);
            *x <<= log_d;
            o
        }
        (Some(log_n), None) => {
            let o = x.mul_prec_round_assign(Float::exact_from(d), prec, rm);
            *x >>= log_n;
            o
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + d.significant_bits();
            x.mul_prec_round_assign(d, mul_prec, Floor);
            x.div_prec_round_assign(n, prec, rm)
        }
    };
    if sign {
        o
    } else {
        x.neg_assign();
        o.reverse()
    }
}

pub_test! {div_rational_prec_round_direct(
    mut x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = div_rational_prec_round_assign_direct(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {div_rational_prec_round_direct_val_ref(
    mut x: Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let o = div_rational_prec_round_assign_direct_ref(&mut x, y, prec, rm);
    (x, o)
}}

pub_test! {div_rational_prec_round_direct_ref_val(
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
    let (quotient, o) = match (
        if n == 0 { None } else { n.checked_log_base_2() },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (quotient, o) = Float::from_float_prec_round_ref(x, prec, rm);
            (quotient << log_d >> log_n, o)
        }
        (None, Some(log_d)) => {
            let (quotient, o) = x.div_prec_round_ref_val(Float::exact_from(n), prec, rm);
            (quotient << log_d, o)
        }
        (Some(log_n), None) => {
            let (quotient, o) = x.mul_prec_round_ref_val(Float::exact_from(d), prec, rm);
            (quotient >> log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + d.significant_bits();
            x.mul_prec_round_ref_val(d, mul_prec, Floor)
                .0
                .div_prec_round(n, prec, rm)
        }
    };
    if sign {
        (quotient, o)
    } else {
        (-quotient, o.reverse())
    }
}}

pub_test! {div_rational_prec_round_direct_ref_ref(
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
    let (quotient, o) = match (
        if *n == 0 {
            None
        } else {
            n.checked_log_base_2()
        },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (quotient, o) = Float::from_float_prec_round_ref(x, prec, rm);
            (quotient << log_d >> log_n, o)
        }
        (None, Some(log_d)) => {
            let (quotient, o) =
                x.div_prec_round_ref_val(Float::exact_from(n), prec, rm);
            (quotient << log_d, o)
        }
        (Some(log_n), None) => {
            let (quotient, o) =
                x.mul_prec_round_ref_val(Float::exact_from(d), prec, rm);
            (quotient >> log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = x.get_min_prec().unwrap_or(1) + d.significant_bits();
            x.mul_prec_round_ref_val(d, mul_prec, Floor)
                .0
                .div_prec_round(n, prec, rm)
        }
    };
    if sign {
        (quotient, o)
    } else {
        (-quotient, o.reverse())
    }
}}

pub_test! {rational_div_float_prec_round_naive(
    x: Rational,
    y: Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (_, float_nan!()) => (float_nan!(), Equal),
        (x, Float(Infinity { sign })) => (
            if x >= 0u32 {
                Float(Zero { sign })
            } else {
                Float(Zero { sign: !sign })
            },
            Equal,
        ),
        (x, Float(Zero { sign })) => (
            match x.sign() {
                Equal => float_nan!(),
                Greater => Float(Infinity { sign }),
                Less => Float(Infinity { sign: !sign }),
            },
            Equal,
        ),
        (x, y) => {
            let not_sign = y < 0;
            let (mut quotient, o) =
                Float::from_rational_prec_round(x / Rational::exact_from(y), prec, rm);
            if quotient == 0u32 && not_sign {
                quotient.neg_assign();
            }
            (quotient, o)
        }
    }
}}

pub_test! {rational_div_float_prec_round_naive_val_ref(
    x: Rational,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (_, float_nan!()) => (float_nan!(), Equal),
        (x, Float(Infinity { sign })) => (
            if x >= 0u32 {
                Float(Zero { sign: *sign })
            } else {
                Float(Zero { sign: !*sign })
            },
            Equal,
        ),
        (x, Float(Zero { sign })) => (
            match x.sign() {
                Equal => float_nan!(),
                Greater => Float(Infinity { sign: *sign }),
                Less => Float(Infinity { sign: !*sign }),
            },
            Equal,
        ),
        (x, y) => {
            let (mut quotient, o) =
                Float::from_rational_prec_round(x / Rational::exact_from(y), prec, rm);
            if quotient == 0u32 && *y < 0 {
                quotient.neg_assign();
            }
            (quotient, o)
        }
    }
}}

pub_test! {rational_div_float_prec_round_naive_ref_val(
    x: &Rational,
    y: Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (_, float_nan!()) => (float_nan!(), Equal),
        (x, Float(Infinity { sign })) => (
            if *x >= 0u32 {
                Float(Zero { sign })
            } else {
                Float(Zero { sign: !sign })
            },
            Equal,
        ),
        (x, Float(Zero { sign })) => (
            match x.sign() {
                Equal => float_nan!(),
                Greater => Float(Infinity { sign }),
                Less => Float(Infinity { sign: !sign }),
            },
            Equal,
        ),
        (x, y) => {
            let not_sign = y < 0;
            let (mut quotient, o) =
                Float::from_rational_prec_round(x / Rational::exact_from(y), prec, rm);
            if quotient == 0u32 && not_sign {
                quotient.neg_assign();
            }
            (quotient, o)
        }
    }
}}

pub_test! {rational_div_float_prec_round_naive_ref_ref(
    x: &Rational,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (_, float_nan!()) => (float_nan!(), Equal),
        (x, Float(Infinity { sign })) => (
            if *x >= 0u32 {
                Float(Zero { sign: *sign })
            } else {
                Float(Zero { sign: !*sign })
            },
            Equal,
        ),
        (x, Float(Zero { sign })) => (
            match x.sign() {
                Equal => float_nan!(),
                Greater => Float(Infinity { sign: *sign }),
                Less => Float(Infinity { sign: !*sign }),
            },
            Equal,
        ),
        (x, y) => {
            let (mut quotient, o) =
                Float::from_rational_prec_round(x / Rational::exact_from(y), prec, rm);
            if quotient == 0u32 && *y < 0 {
                quotient.neg_assign();
            }
            (quotient, o)
        }
    }
}}

pub_test! {rational_div_float_prec_round_direct(
    x: Rational,
    y: Float,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = x >= 0;
    let (n, d) = x.into_numerator_and_denominator();
    if !sign {
        rm.neg_assign();
    }
    let (quotient, o) = match (
        if n == 0 { None } else { n.checked_log_base_2() },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (quotient, o) = Float::reciprocal_prec_round(y, prec, rm);
            (quotient << log_n >> log_d, o)
        }
        (None, Some(log_d)) => {
            let (quotient, o) = Float::exact_from(n).div_prec_round(y, prec, rm);
            (quotient >> log_d, o)
        }
        (Some(log_n), None) => {
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            let (quotient, o) = y
                .mul_prec_round(d, mul_prec, Floor)
                .0
                .reciprocal_prec_round(prec, rm);
            (quotient << log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            n.div_prec_round(y.mul_prec_round(d, mul_prec, Floor).0, prec, rm)
        }
    };
    if sign {
        (quotient, o)
    } else {
        (-quotient, o.reverse())
    }
}}

pub_test! {rational_div_float_prec_round_direct_val_ref(
    x: Rational,
    y: &Float,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = x >= 0;
    let (n, d) = x.into_numerator_and_denominator();
    if !sign {
        rm.neg_assign();
    }
    let (quotient, o) = match (
        if n == 0 { None } else { n.checked_log_base_2() },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (quotient, o) = Float::reciprocal_prec_round_ref(y, prec, rm);
            (quotient << log_n >> log_d, o)
        }
        (None, Some(log_d)) => {
            let (quotient, o) = Float::exact_from(n).div_prec_round_val_ref(y, prec, rm);
            (quotient >> log_d, o)
        }
        (Some(log_n), None) => {
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            let (quotient, o) = y
                .mul_prec_round_ref_val(d, mul_prec, Floor)
                .0
                .reciprocal_prec_round(prec, rm);
            (quotient << log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            n.div_prec_round(y.mul_prec_round_ref_val(d, mul_prec, Floor).0, prec, rm)
        }
    };
    if sign {
        (quotient, o)
    } else {
        (-quotient, o.reverse())
    }
}}

pub_test! {rational_div_float_prec_round_direct_ref_val(
    x: &Rational,
    y: Float,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = *x >= 0;
    let (n, d) = x.numerator_and_denominator_ref();
    if !sign {
        rm.neg_assign();
    }
    let (quotient, o) = match (
        if *n == 0 {
            None
        } else {
            n.checked_log_base_2()
        },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (quotient, o) = Float::reciprocal_prec_round(y, prec, rm);
            (quotient << log_n >> log_d, o)
        }
        (None, Some(log_d)) => {
            let (quotient, o) = Float::exact_from(n).div_prec_round(y, prec, rm);
            (quotient >> log_d, o)
        }
        (Some(log_n), None) => {
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            let (quotient, o) = y
                .mul_prec_round(d, mul_prec, Floor)
                .0
                .reciprocal_prec_round(prec, rm);
            (quotient << log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            n.div_prec_round(y.mul_prec_round(d, mul_prec, Floor).0, prec, rm)
        }
    };
    if sign {
        (quotient, o)
    } else {
        (-quotient, o.reverse())
    }
}}

pub_test! {rational_div_float_prec_round_direct_ref_ref(
    x: &Rational,
    y: &Float,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = *x >= 0;
    let (n, d) = x.numerator_and_denominator_ref();
    if !sign {
        rm.neg_assign();
    }
    let (quotient, o) = match (
        if *n == 0 {
            None
        } else {
            n.checked_log_base_2()
        },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => {
            let (quotient, o) = Float::reciprocal_prec_round_ref(y, prec, rm);
            (quotient << log_n >> log_d, o)
        }
        (None, Some(log_d)) => {
            let (quotient, o) =
                Float::exact_from(n).div_prec_round_val_ref(y, prec, rm);
            (quotient >> log_d, o)
        }
        (Some(log_n), None) => {
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            let (quotient, o) = y
                .mul_prec_round_ref_val(d, mul_prec, Floor)
                .0
                .reciprocal_prec_round(prec, rm);
            (quotient << log_n, o)
        }
        (None, None) => {
            let n = Float::exact_from(n);
            let d = Float::exact_from(d);
            let mul_prec = y.get_min_prec().unwrap_or(1) + d.significant_bits();
            n.div_prec_round(y.mul_prec_round_ref_val(d, mul_prec, Floor).0, prec, rm)
        }
    };
    if sign {
        (quotient, o)
    } else {
        (-quotient, o.reverse())
    }
}}

impl Float {
    /// Divides two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::div_round`] instead. If both of these things are true, consider using `/`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round(Float::from(E), 5, Floor);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round(Float::from(E), 5, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round(Float::from(E), 5, Nearest);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round(Float::from(E), 20, Floor);
    /// assert_eq!(quotient.to_string(), "1.155725");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round(Float::from(E), 20, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round(Float::from(E), 20, Nearest);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round(
        mut self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.div_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Divides two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is are taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::div_round_val_ref`] instead. If both of these things are true,
    /// consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_val_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_val_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_val_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_val_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(quotient.to_string(), "1.155725");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_val_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_val_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round_val_ref(
        mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.div_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Divides two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is are taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::div_round_ref_val`] instead. If both of these things are true,
    /// consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_val(Float::from(E), 5, Floor);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_val(Float::from(E), 5, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_val(Float::from(E), 5, Nearest);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_val(Float::from(E), 20, Floor);
    /// assert_eq!(quotient.to_string(), "1.155725");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_val(Float::from(E), 20, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_val(Float::from(E), 20, Nearest);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round_ref_val(
        &self,
        other: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => (float_nan!(), Equal),
            (
                Float(Infinity { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Zero { sign: y_sign })) => (
                Float(Infinity {
                    sign: *x_sign == y_sign,
                }),
                Equal,
            ),
            (
                Float(Zero { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Infinity { sign: y_sign })) => (
                Float(Zero {
                    sign: *x_sign == y_sign,
                }),
                Equal,
            ),
            (
                Float(Finite {
                    sign: x_sign,
                    exponent: x_exp,
                    precision: x_prec,
                    significand: x,
                }),
                Float(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: mut y,
                }),
            ) => {
                if y.is_power_of_2() {
                    let (mut quotient, mut o) =
                        Float::from_float_prec_round_ref(self, prec, if y_sign { rm } else { -rm });
                    if !y_sign {
                        quotient.neg_assign();
                        o = o.reverse();
                    }
                    return (quotient >> y_exp.checked_sub(1).unwrap(), o);
                }
                let sign = *x_sign == y_sign;
                let (quotient, exp_offset, o) = div_float_significands_ref_val(
                    x,
                    *x_prec,
                    &mut y,
                    y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                let exp = x_exp
                    .checked_sub(y_exp)
                    .unwrap()
                    .checked_add(i32::exact_from(exp_offset))
                    .unwrap();
                (
                    Float(Finite {
                        sign,
                        exponent: exp,
                        precision: prec,
                        significand: quotient,
                    }),
                    if sign { o } else { o.reverse() },
                )
            }
        }
    }

    /// Divides two [`Float`]s, rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::div_round_ref_ref`] instead. If both of these things are true,
    /// consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_ref(&Float::from(E), 5, Floor);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_ref(&Float::from(E), 5, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_ref(&Float::from(E), 5, Nearest);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_ref(&Float::from(E), 20, Floor);
    /// assert_eq!(quotient.to_string(), "1.155725");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_ref(&Float::from(E), 20, Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_round_ref_ref(&Float::from(E), 20, Nearest);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_round_ref_ref(
        &self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => (float_nan!(), Equal),
            (
                Float(Infinity { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Zero { sign: y_sign })) => (
                Float(Infinity {
                    sign: x_sign == y_sign,
                }),
                Equal,
            ),
            (
                Float(Zero { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Infinity { sign: y_sign })) => (
                Float(Zero {
                    sign: x_sign == y_sign,
                }),
                Equal,
            ),
            (
                Float(Finite {
                    sign: x_sign,
                    exponent: x_exp,
                    precision: x_prec,
                    significand: x,
                }),
                Float(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: y,
                }),
            ) => {
                if y.is_power_of_2() {
                    let (mut quotient, mut o) = Float::from_float_prec_round_ref(
                        self,
                        prec,
                        if *y_sign { rm } else { -rm },
                    );
                    if !y_sign {
                        quotient.neg_assign();
                        o = o.reverse();
                    }
                    return (quotient >> y_exp.checked_sub(1).unwrap(), o);
                }
                let sign = x_sign == y_sign;
                let (quotient, exp_offset, o) = div_float_significands_ref_ref(
                    x,
                    *x_prec,
                    y,
                    *y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                let exp = x_exp
                    .checked_sub(*y_exp)
                    .unwrap()
                    .checked_add(i32::exact_from(exp_offset))
                    .unwrap();
                (
                    Float(Finite {
                        sign,
                        exponent: exp,
                        precision: prec,
                        significand: quotient,
                    }),
                    if sign { o } else { o.reverse() },
                )
            }
        }
    }

    /// Divides two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec(Float::from(E), 5);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec(Float::from(E), 20);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec(self, other: Float, prec: u64) -> (Float, Ordering) {
        self.div_prec_round(other, prec, Nearest)
    }

    /// Divides two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_val_ref(&Float::from(E), 5);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_val_ref(&Float::from(E), 20);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_val_ref(self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.div_prec_round_val_ref(other, prec, Nearest)
    }

    /// Divides two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_ref_val(Float::from(E), 5);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_ref_val(Float::from(E), 20);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_ref_val(&self, other: Float, prec: u64) -> (Float, Ordering) {
        self.div_prec_round_ref_val(other, prec, Nearest)
    }

    /// Divides two [`Float`]s, rounding the result to the nearest value of the specified precision.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded quotient is less than, equal to, or greater than the exact quotient. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,p)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,p)=\infty$ if $x>0.0$
    /// - $f(x,0.0,p)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,p)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,p)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,p)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,p)=\infty$ if $x<0.0$
    /// - $f(0.0,x,p)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,p)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,p)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,p)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,p)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,p)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_ref_ref(&Float::from(E), 5);
    /// assert_eq!(quotient.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_prec_ref_ref(&Float::from(E), 20);
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn div_prec_ref_ref(&self, other: &Float, prec: u64) -> (Float, Ordering) {
        self.div_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Divides two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to specify an output precision, consider using [`Float::div_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `/`
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
    /// let (quotient, o) = Float::from(PI).div_round(Float::from(E), Floor);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_round(Float::from(E), Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727349790922");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_round(Float::from(E), Nearest);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_round(self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round(other, prec, rm)
    }

    /// Divides two [`Float`]s, rounding the result with the specified rounding mode. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded quotient is less than, equal to, or greater than the exact
    /// quotient. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to specify an output precision, consider using [`Float::div_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `/`
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
    /// let (quotient, o) = Float::from(PI).div_round_val_ref(&Float::from(E), Floor);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_round_val_ref(&Float::from(E), Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727349790922");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_round_val_ref(&Float::from(E), Nearest);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_round_val_ref(self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_val_ref(other, prec, rm)
    }

    /// Divides two [`Float`]s, rounding the result with the specified rounding mode. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded quotient is less than, equal to, or greater than the exact
    /// quotient. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to specify an output precision, consider using [`Float::div_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `/`
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
    /// let (quotient, o) = Float::from(PI).div_round_ref_val(Float::from(E), Floor);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_round_ref_val(Float::from(E), Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727349790922");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_round_ref_val(Float::from(E), Nearest);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_round_ref_val(&self, other: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_ref_val(other, prec, rm)
    }

    /// Divides two [`Float`]s, rounding the result with the specified rounding mode. Both
    /// [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded quotient is less than, equal to, or greater than the exact quotient. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},p,m)=f(\pm\infty,\pm\infty,p,m)=f(\pm0.0,\pm0.0,p,m) =
    ///   \text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x,m)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0,m)=\infty$ if $x>0.0$
    /// - $f(x,0.0,m)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x,m)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x,m)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0,m)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0,m)=\infty$ if $x<0.0$
    /// - $f(0.0,x,m)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x,m)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x,m)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x,m)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty,m)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty,m)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to specify an output precision, consider using [`Float::div_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `/`
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
    /// let (quotient, o) = Float::from(PI).div_round_ref_ref(&Float::from(E), Floor);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_round_ref_ref(&Float::from(E), Ceiling);
    /// assert_eq!(quotient.to_string(), "1.155727349790922");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_round_ref_ref(&Float::from(E), Nearest);
    /// assert_eq!(quotient.to_string(), "1.1557273497909217");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_round_ref_ref(&self, other: &Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_ref_ref(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] on the right-hand side is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded quotient is less than,
    /// equal to, or greater than the exact quotient. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::div_round_assign`] instead. If both of these things are true,
    /// consider using `/=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign(Float::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(quotient.to_string(), "1.12");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign(Float::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(quotient.to_string(), "1.19");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign(Float::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(quotient.to_string(), "1.12");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign(Float::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(quotient.to_string(), "1.155725");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign(Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(quotient.to_string(), "1.155727");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign(Float::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// ```
    #[inline]
    pub fn div_prec_round_assign(&mut self, other: Float, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => {
                *self = float_nan!();
                Equal
            }
            (
                Float(Infinity { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Zero { sign: y_sign })) => {
                *self = Float(Infinity {
                    sign: *x_sign == y_sign,
                });
                Equal
            }
            (
                Float(Zero { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Infinity { sign: y_sign })) => {
                *self = Float(Zero {
                    sign: *x_sign == y_sign,
                });
                Equal
            }
            (_, y) if abs_is_power_of_2(&y) => {
                if y < 0 {
                    self.neg_assign();
                }
                let o = self.set_prec_round(prec, rm);
                *self >>= y.get_exponent().unwrap().checked_sub(1).unwrap();
                o
            }
            (
                Float(Finite {
                    sign: ref mut x_sign,
                    exponent: ref mut x_exp,
                    precision: ref mut x_prec,
                    significand: ref mut x,
                }),
                Float(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: mut y,
                }),
            ) => {
                let sign = *x_sign == y_sign;
                let (exp_offset, o) = div_float_significands_in_place(
                    x,
                    *x_prec,
                    &mut y,
                    y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                *x_sign = sign;
                *x_exp = x_exp
                    .checked_sub(y_exp)
                    .unwrap()
                    .checked_add(i32::exact_from(exp_offset))
                    .unwrap();
                *x_prec = prec;
                if sign {
                    o
                } else {
                    o.reverse()
                }
            }
        }
    }

    /// Divides a [`Float`] by a [`Float`] in place, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::div_round_assign_ref`] instead. If both of these things are
    /// true, consider using `/=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign_ref(&Float::from(E), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(quotient.to_string(), "1.12");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign_ref(&Float::from(E), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(quotient.to_string(), "1.19");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign_ref(&Float::from(E), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(quotient.to_string(), "1.12");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign_ref(&Float::from(E), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(quotient.to_string(), "1.155725");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign_ref(&Float::from(E), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(quotient.to_string(), "1.155727");
    ///
    /// let mut quotient = Float::from(PI);
    /// assert_eq!(
    ///     quotient.div_prec_round_assign_ref(&Float::from(E), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(quotient.to_string(), "1.155727");
    /// ```
    #[inline]
    pub fn div_prec_round_assign_ref(
        &mut self,
        other: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        assert_ne!(prec, 0);
        match (&mut *self, other) {
            (float_nan!(), _)
            | (_, float_nan!())
            | (float_either_infinity!(), float_either_infinity!())
            | (float_either_zero!(), float_either_zero!()) => {
                *self = float_nan!();
                Equal
            }
            (
                Float(Infinity { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Zero { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Zero { sign: y_sign })) => {
                *self = Float(Infinity {
                    sign: x_sign == y_sign,
                });
                Equal
            }
            (
                Float(Zero { sign: x_sign }),
                Float(Finite { sign: y_sign, .. } | Infinity { sign: y_sign }),
            )
            | (Float(Finite { sign: x_sign, .. }), Float(Infinity { sign: y_sign })) => {
                *self = Float(Zero {
                    sign: x_sign == y_sign,
                });
                Equal
            }
            (_, y) if abs_is_power_of_2(y) => {
                if *y < 0 {
                    self.neg_assign();
                }
                let o = self.set_prec_round(prec, rm);
                *self >>= y.get_exponent().unwrap().checked_sub(1).unwrap();
                o
            }
            (
                Float(Finite {
                    sign: ref mut x_sign,
                    exponent: ref mut x_exp,
                    precision: ref mut x_prec,
                    significand: ref mut x,
                }),
                Float(Finite {
                    sign: y_sign,
                    exponent: y_exp,
                    precision: y_prec,
                    significand: y,
                }),
            ) => {
                let sign = x_sign == y_sign;
                let (exp_offset, o) = div_float_significands_in_place_ref(
                    x,
                    *x_prec,
                    y,
                    *y_prec,
                    prec,
                    if sign { rm } else { -rm },
                );
                *x_sign = sign;
                *x_exp = x_exp
                    .checked_sub(*y_exp)
                    .unwrap()
                    .checked_add(i32::exact_from(exp_offset))
                    .unwrap();
                *x_prec = prec;
                if sign {
                    o
                } else {
                    o.reverse()
                }
            }
        }
    }

    /// Divides a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of the
    /// specified precision. The [`Float`] on the right-hand side is taken by value. An [`Ordering`]
    /// is returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `/=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_prec_assign(Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_prec_assign(Float::from(E), 20), Greater);
    /// assert_eq!(x.to_string(), "1.155727");
    /// ```
    #[inline]
    pub fn div_prec_assign(&mut self, other: Float, prec: u64) -> Ordering {
        self.div_prec_round_assign(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Float`] in place, rounding the result to the nearest value of the
    /// specified precision. The [`Float`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded quotient is less than, equal to, or
    /// greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using `/=` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits(), `prec`)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_prec_assign_ref(&Float::from(E), 5), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_prec_assign_ref(&Float::from(E), 20), Greater);
    /// assert_eq!(x.to_string(), "1.155727");
    /// ```
    #[inline]
    pub fn div_prec_assign_ref(&mut self, other: &Float, prec: u64) -> Ordering {
        self.div_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Float`] in place, rounding the result with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded quotient is less than, equal to, or greater than the exact
    /// quotient. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::div_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using [`Float::div_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `/=`
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
    /// assert_eq!(x.div_round_assign(Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "1.1557273497909217");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_round_assign(Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.155727349790922");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_round_assign(Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1557273497909217");
    /// ```
    #[inline]
    pub fn div_round_assign(&mut self, other: Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_assign(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Float`] in place, rounding the result with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::div_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `/=` instead.
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
    /// assert_eq!(x.div_round_assign_ref(&Float::from(E), Floor), Less);
    /// assert_eq!(x.to_string(), "1.1557273497909217");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_round_assign_ref(&Float::from(E), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.155727349790922");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.div_round_assign_ref(&Float::from(E), Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1557273497909217");
    /// ```
    #[inline]
    pub fn div_round_assign_ref(&mut self, other: &Float, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_assign_ref(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=f(\pm0.0,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x>0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_rational_prec`] instead.
    /// If you know that your target precision is the precision of the [`Float`] input, consider
    /// using [`Float::div_rational_round`] instead. If both of these things are true, consider
    /// using `/` instead.
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
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(quotient.to_string(), "9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Ceiling);
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Nearest);
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Ceiling);
    /// assert_eq!(quotient.to_string(), "9.42479");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Nearest);
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round(
        mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.div_rational_prec_round_assign(other, prec, rm);
        (self, o)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=f(\pm0.0,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x>0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_rational_prec_val_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::div_rational_round_val_ref`] instead. If both of these things are
    /// true, consider using `/` instead.
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
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42479");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_val_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round_val_ref(
        mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let o = self.div_rational_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=f(\pm0.0,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x>0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_rational_prec_ref_val`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::div_rational_round_ref_val`] instead. If both of these things are
    /// true, consider using `/` instead.
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
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42479");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_val(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(self.complexity(), other.significant_bits()) < DIV_RATIONAL_THRESHOLD {
            div_rational_prec_round_naive_ref_val(self, other, prec, rm)
        } else {
            div_rational_prec_round_direct_ref_val(self, other, prec, rm)
        }
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Float`] and the [`Rational`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(\pm\infty,0,p,m)=f(\pm0.0,0,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p,m)=\infty$ if $x<0$
    /// - $f(0.0,x,p,m)=0.0$ if $x>0$
    /// - $f(0.0,x,p,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p,m)=0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_rational_prec_ref_ref`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::div_rational_round_ref_ref`] instead. If both of these things are
    /// true, consider using `/` instead.
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
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "9.0");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "9.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42479");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "9.42477");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(self.complexity(), other.significant_bits()) < DIV_RATIONAL_THRESHOLD {
            div_rational_prec_round_naive_ref_ref(self, other, prec, rm)
        } else {
            div_rational_prec_round_direct_ref_ref(self, other, prec, rm)
        }
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=f(\pm0.0,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x>0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_round`] instead. If you know that your target precision is the
    /// precision of the [`Float`] input, consider using `/` instead.
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
    /// let (quotient, o) = Float::from(PI).div_rational_prec(Rational::exact_from(1.5), 5);
    /// assert_eq!(quotient.to_string(), "2.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::from(PI).div_rational_prec(Rational::exact_from(1.5), 20);
    /// assert_eq!(quotient.to_string(), "2.094395");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec(self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.div_rational_prec_round(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=f(\pm0.0,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x>0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_round_val_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `/` instead.
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
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_val_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(quotient.to_string(), "2.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_val_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(quotient.to_string(), "2.094395");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.div_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference and the [`Rational`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=f(\pm0.0,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x>0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_round_ref_val`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `/` instead.
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
    /// let (quotient, o) = Float::from(PI).div_rational_prec_ref_val(Rational::exact_from(1.5), 5);
    /// assert_eq!(quotient.to_string(), "2.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_ref_val(Rational::exact_from(1.5), 20);
    /// assert_eq!(quotient.to_string(), "2.094395");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Float, Ordering) {
        self.div_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] and the [`Rational`] are both are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(\pm\infty,0,p)=f(\pm0.0,0,p)=\text{NaN}$
    /// - $f(\infty,x,p)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,p)=-\infty$ if $x<0$
    /// - $f(-\infty,x,p)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,p)=\infty$ if $x<0$
    /// - $f(0.0,x,p)=0.0$ if $x>0$
    /// - $f(0.0,x,p)=-0.0$ if $x<0$
    /// - $f(-0.0,x,p)=-0.0$ if $x>0$
    /// - $f(-0.0,x,p)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_round_ref_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using `/` instead.
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
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_ref_ref(&Rational::exact_from(1.5), 5);
    /// assert_eq!(quotient.to_string(), "2.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_prec_ref_ref(&Rational::exact_from(1.5), 20);
    /// assert_eq!(quotient.to_string(), "2.094395");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Float, Ordering) {
        self.div_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] and the [`Rational`] are both are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=f(\pm0.0,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x>0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_rational_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `/` instead.
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
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(quotient.to_string(), "9.42477796076939");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_round(self, other: Rational, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.div_rational_prec_round(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=f(\pm0.0,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x>0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_rational_prec_round_val_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `/` instead.
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
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(quotient.to_string(), "9.42477796076939");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_val_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.div_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=f(\pm0.0,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x>0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_rational_prec_round_ref_val`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `/` instead.
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
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(quotient.to_string(), "9.42477796076939");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_ref_val(Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.div_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Rational`], rounding the result with the specified rounding mode.
    /// The [`Float`] and the [`Rational`] are both are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(\pm\infty,0,m)=f(\pm0.0,0,m)=\text{NaN}$
    /// - $f(\infty,x,m)=\infty$ if $x\geq 0$
    /// - $f(\infty,x,m)=-\infty$ if $x<0$
    /// - $f(-\infty,x,m)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x,m)=\infty$ if $x<0$
    /// - $f(0.0,x,m)=0.0$ if $x>0$
    /// - $f(0.0,x,m)=-0.0$ if $x<0$
    /// - $f(-0.0,x,m)=-0.0$ if $x>0$
    /// - $f(-0.0,x,m)=0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_rational_prec_round_ref_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `/` instead.
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
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Floor);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Ceiling);
    /// assert_eq!(quotient.to_string(), "9.42477796076939");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::from(PI).div_rational_round_ref_ref(&Rational::from_unsigneds(1u8, 3), Nearest);
    /// assert_eq!(quotient.to_string(), "9.42477796076938");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn div_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.div_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Rational`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded quotient is less than, equal to, or
    /// greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::div_rational_prec_assign`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::div_rational_round_assign`] instead. If both of these things are
    /// true, consider using `/=` instead.
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
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
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
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.42479");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign(Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477");
    /// ```
    #[inline]
    pub fn div_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        if max(self.complexity(), other.significant_bits()) < DIV_RATIONAL_THRESHOLD {
            div_rational_prec_round_assign_naive(self, other, prec, rm)
        } else {
            div_rational_prec_round_assign_direct(self, other, prec, rm)
        }
    }

    /// Divides a [`Float`] by a [`Rational`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded quotient is less than, equal to, or
    /// greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_rational_prec_round`] documentation for information on special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::div_rational_prec_assign_ref`] instead. If you know that your target precision is
    /// the precision of the [`Float`] input, consider using
    /// [`Float::div_rational_round_assign_ref`] instead. If both of these things are true, consider
    /// using `/=` instead.
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
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
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
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.42479");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_round_assign_ref(&Rational::from_unsigneds(1u8, 3), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477");
    /// ```
    #[inline]
    pub fn div_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        if max(self.complexity(), other.significant_bits()) < DIV_RATIONAL_THRESHOLD {
            div_rational_prec_round_assign_naive_ref(self, other, prec, rm)
        } else {
            div_rational_prec_round_assign_direct_ref(self, other, prec, rm)
        }
    }

    /// Divides a [`Float`] by a [`Rational`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Rational`] is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded quotient is less than, equal to, or greater than the exact
    /// quotient. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `/=` instead.
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
    ///     x.div_rational_prec_assign(Rational::exact_from(1.5), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.1");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_assign(Rational::exact_from(1.5), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.094395");
    /// ```
    #[inline]
    pub fn div_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.div_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Rational`] in place, rounding the result to the nearest value of
    /// the specified precision. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::div_rational_prec`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_round_assign`] instead. If you know that your target precision is
    /// the maximum of the precisions of the two inputs, consider using `/=` instead.
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
    ///     x.div_rational_prec_assign_ref(&Rational::exact_from(1.5), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "2.1");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_prec_assign_ref(&Rational::exact_from(1.5), 20),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.094395");
    /// ```
    #[inline]
    pub fn div_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.div_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Divides a [`Float`] by a [`Rational`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded quotient is less than, equal to, or greater than the exact quotient.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::div_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `/=` instead.
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
    ///     x.div_rational_round_assign(Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477796076938");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_round_assign(Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.42477796076939");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_round_assign(Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477796076938");
    /// ```
    #[inline]
    pub fn div_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.div_rational_prec_round_assign(other, prec, rm)
    }

    /// Divides a [`Float`] by a [`Rational`] in place, rounding the result with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded quotient is less than, equal to, or greater than the exact
    /// quotient. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input [`Float`]. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the input [`Float`].
    ///
    /// See the [`Float::div_rational_round`] documentation for information on special cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::div_rational_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `/=` instead.
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
    ///     x.div_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477796076938");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "9.42477796076939");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(
    ///     x.div_rational_round_assign_ref(&Rational::from_unsigneds(1u8, 3), Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "9.42477796076938");
    /// ```
    #[inline]
    pub fn div_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.div_rational_prec_round_assign_ref(other, prec, rm)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Rational`] and the [`Float`] are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rational_div_float_prec`]
    /// instead. If you know that your target precision is the precision of the [`Float`] input,
    /// consider using [`Float::rational_div_float_round`] instead. If both of these things are
    /// true, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_round(Rational::from(3), Float::from(PI), 5, Floor);
    /// assert_eq!(quotient.to_string(), "0.94");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_round(Rational::from(3), Float::from(PI), 5, Ceiling);
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_round(Rational::from(3), Float::from(PI), 5, Nearest);
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_round(Rational::from(3), Float::from(PI), 20, Floor);
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_round(Rational::from(3), Float::from(PI), 20, Ceiling);
    /// assert_eq!(quotient.to_string(), "0.95493");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_round(Rational::from(3), Float::from(PI), 20, Nearest);
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_round(
        x: Rational,
        y: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(x.significant_bits(), y.complexity()) < RATIONAL_DIV_THRESHOLD {
            rational_div_float_prec_round_naive(x, y, prec, rm)
        } else {
            rational_div_float_prec_round_direct(x, y, prec, rm)
        }
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Rational`] is taken by value and the [`Float`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::rational_div_float_prec_val_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using
    /// [`Float::rational_div_float_round_val_ref`] instead. If both of these things are true,
    /// consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_val_ref(
    ///     Rational::from(3),
    ///     &Float::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "0.94");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_val_ref(
    ///     Rational::from(3),
    ///     &Float::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_val_ref(
    ///     Rational::from(3),
    ///     &Float::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_val_ref(
    ///     Rational::from(3),
    ///     &Float::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_val_ref(
    ///     Rational::from(3),
    ///     &Float::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "0.95493");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_val_ref(
    ///     Rational::from(3),
    ///     &Float::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_round_val_ref(
        x: Rational,
        y: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(x.significant_bits(), y.complexity()) < RATIONAL_DIV_THRESHOLD {
            rational_div_float_prec_round_naive_val_ref(x, y, prec, rm)
        } else {
            rational_div_float_prec_round_direct_val_ref(x, y, prec, rm)
        }
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Rational`] is taken by reference and the [`Float`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::rational_div_float_prec_ref_val`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using
    /// [`Float::rational_div_float_round_ref_val`] instead. If both of these things are true,
    /// consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_val(
    ///     &Rational::from(3),
    ///     Float::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "0.94");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_val(
    ///     &Rational::from(3),
    ///     Float::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_val(
    ///     &Rational::from(3),
    ///     Float::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_val(
    ///     &Rational::from(3),
    ///     Float::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_val(
    ///     &Rational::from(3),
    ///     Float::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "0.95493");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_val(
    ///     &Rational::from(3),
    ///     Float::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_round_ref_val(
        x: &Rational,
        y: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(x.significant_bits(), y.complexity()) < RATIONAL_DIV_THRESHOLD {
            rational_div_float_prec_round_naive_ref_val(x, y, prec, rm)
        } else {
            rational_div_float_prec_round_direct_ref_val(x, y, prec, rm)
        }
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The [`Rational`] and the [`Float`] are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded quotient is less
    /// than, equal to, or greater than the exact quotient. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$.
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::rational_div_float_prec_ref_ref`] instead. If you know that your target precision
    /// is the precision of the [`Float`] input, consider using
    /// [`Float::rational_div_float_round_ref_ref`] instead. If both of these things are true,
    /// consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact division.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_ref(
    ///     &Rational::from(3),
    ///     &Float::from(PI),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "0.94");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_ref(
    ///     &Rational::from(3),
    ///     &Float::from(PI),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_ref(
    ///     &Rational::from(3),
    ///     &Float::from(PI),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_ref(
    ///     &Rational::from(3),
    ///     &Float::from(PI),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_ref(
    ///     &Rational::from(3),
    ///     &Float::from(PI),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(quotient.to_string(), "0.95493");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec_round_ref_ref(
    ///     &Rational::from(3),
    ///     &Float::from(PI),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_round_ref_ref(
        x: &Rational,
        y: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(x.significant_bits(), y.complexity()) < RATIONAL_DIV_THRESHOLD {
            rational_div_float_prec_round_naive_ref_ref(x, y, prec, rm)
        } else {
            rational_div_float_prec_round_direct_ref_ref(x, y, prec, rm)
        }
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] and the [`Float`] are both are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(0,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\infty,x,p)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p)=0.0$ if $x>0$
    /// - $f(0,x,p)=-0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_div_float_prec_round`] instead. If you know that your target precision is
    /// the precision of the [`Float`] input, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) = Float::rational_div_float_prec(Rational::from(3), Float::from(PI), 5);
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) = Float::rational_div_float_prec(Rational::from(3), Float::from(PI), 20);
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec(x: Rational, y: Float, prec: u64) -> (Float, Ordering) {
        Float::rational_div_float_prec_round(x, y, prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] is taken by value and the [`Float`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(0,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\infty,x,p)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p)=0.0$ if $x>0$
    /// - $f(0,x,p)=-0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_div_float_prec_round_val_ref`] instead. If you know that your target
    /// precision is the precision of the [`Float`] input, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_val_ref(Rational::from(3), &Float::from(PI), 5);
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_val_ref(Rational::from(3), &Float::from(PI), 20);
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_val_ref(x: Rational, y: &Float, prec: u64) -> (Float, Ordering) {
        Float::rational_div_float_prec_round_val_ref(x, y, prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] is taken by reference and the [`Float`] by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(0,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\infty,x,p)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p)=0.0$ if $x>0$
    /// - $f(0,x,p)=-0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_div_float_prec_round_ref_val`] instead. If you know that your target
    /// precision is the precision of the [`Float`] input, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_ref_val(&Rational::from(3), Float::from(PI), 5);
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_ref_val(&Rational::from(3), Float::from(PI), 20);
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_ref_val(x: &Rational, y: Float, prec: u64) -> (Float, Ordering) {
        Float::rational_div_float_prec_round_ref_val(x, y, prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Rational`] and the [`Float`] are both are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded quotient is less than, equal
    /// to, or greater than the exact quotient. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the quotient is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(0,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\infty,x,p)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p)=0.0$ if $x>0$
    /// - $f(0,x,p)=-0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_div_float_prec_round_ref_ref`] instead. If you know that your target
    /// precision is the precision of the [`Float`] input, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_ref_ref(&Rational::from(3), &Float::from(PI), 5);
    /// assert_eq!(quotient.to_string(), "0.97");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_prec_ref_ref(&Rational::from(3), &Float::from(PI), 20);
    /// assert_eq!(quotient.to_string(), "0.954929");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_div_float_prec_ref_ref(
        x: &Rational,
        y: &Float,
        prec: u64,
    ) -> (Float, Ordering) {
        Float::rational_div_float_prec_round_ref_ref(x, y, prec, Nearest)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result with the specified rounding mode.
    /// The [`Rational`] and the [`Float`] are both are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},m)=f(0,\pm0.0,m)=\text{NaN}$
    /// - $f(x,\infty,x,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,m)=0.0$ if $x>0$
    /// - $f(0,x,m)=-0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rational_div_float_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits())`.
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
    /// let (quotient, o) =
    ///     Float::rational_div_float_round(Rational::from(3), Float::from(PI), Floor);
    /// assert_eq!(quotient.to_string(), "0.9549296585513716");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round(Rational::from(3), Float::from(PI), Ceiling);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round(Rational::from(3), Float::from(PI), Nearest);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_float_round(x: Rational, y: Float, rm: RoundingMode) -> (Float, Ordering) {
        let prec = y.significant_bits();
        Float::rational_div_float_prec_round(x, y, prec, rm)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result with the specified rounding mode.
    /// The [`Rational`] is taken by value and the [`Float`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},m)=f(0,\pm0.0,m)=\text{NaN}$
    /// - $f(x,\infty,x,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,m)=0.0$ if $x>0$
    /// - $f(0,x,m)=-0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rational_div_float_prec_round_val_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits())`.
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
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_val_ref(Rational::from(3), &Float::from(PI), Floor);
    /// assert_eq!(quotient.to_string(), "0.9549296585513716");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_val_ref(Rational::from(3), &Float::from(PI), Ceiling);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_val_ref(Rational::from(3), &Float::from(PI), Nearest);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_float_round_val_ref(
        x: Rational,
        y: &Float,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = y.significant_bits();
        Float::rational_div_float_prec_round_val_ref(x, y, prec, rm)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result with the specified rounding mode.
    /// The [`Rational`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},m)=f(0,\pm0.0,m)=\text{NaN}$
    /// - $f(x,\infty,x,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,m)=0.0$ if $x>0$
    /// - $f(0,x,m)=-0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rational_div_float_prec_round_ref_val`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits())`.
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
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_ref_val(&Rational::from(3), Float::from(PI), Floor);
    /// assert_eq!(quotient.to_string(), "0.9549296585513716");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_ref_val(&Rational::from(3), Float::from(PI), Ceiling);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_ref_val(&Rational::from(3), Float::from(PI), Nearest);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_float_round_ref_val(
        x: &Rational,
        y: Float,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = y.significant_bits();
        Float::rational_div_float_prec_round_ref_val(x, y, prec, rm)
    }

    /// Divides a [`Rational`] by a [`Float`], rounding the result with the specified rounding mode.
    /// The [`Rational`] and the [`Float`] are both are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded quotient is less than, equal to, or greater than
    /// the exact quotient. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the [`Float`] input. See [`RoundingMode`]
    /// for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p+1}$, where $p$ is the precision of the input [`Float`].
    /// - If $x/y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x/y|\rfloor-p}$, where $p$ is the precision of the input [`Float`].
    ///
    /// If the output has a precision, it is the precision of the [`Float`] input.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},m)=f(0,\pm0.0,m)=\text{NaN}$
    /// - $f(x,\infty,x,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,m)=0.0$ if $x>0$
    /// - $f(0,x,m)=-0.0$ if $x<0$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rational_div_float_prec_round_ref_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `/` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits())`.
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
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_ref_ref(&Rational::from(3), &Float::from(PI), Floor);
    /// assert_eq!(quotient.to_string(), "0.9549296585513716");
    /// assert_eq!(o, Less);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_ref_ref(&Rational::from(3), &Float::from(PI), Ceiling);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    ///
    /// let (quotient, o) =
    ///     Float::rational_div_float_round_ref_ref(&Rational::from(3), &Float::from(PI), Nearest);
    /// assert_eq!(quotient.to_string(), "0.9549296585513725");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_div_float_round_ref_ref(
        x: &Rational,
        y: &Float,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        let prec = y.significant_bits();
        Float::rational_div_float_prec_round_ref_ref(x, y, prec, rm)
    }
}

impl Div<Float> for Float {
    type Output = Float;

    /// Divides two [`Float`]s, taking both by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// quotient is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm\infty)=f(\pm0.0,\pm0.0) = \text{NaN}$
    /// - $f(\infty,x)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0)=\infty$ if $x>0.0$
    /// - $f(x,0.0)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::div_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::div_round`].
    /// If you want both of these things, consider using [`Float::div_prec_round`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) / Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) / Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     Float::from(1.5) / Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(Float::from(-1.5) / Float::ZERO, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Float::from(-1.5) / Float::NEGATIVE_ZERO, Float::INFINITY);
    /// assert!((Float::ZERO / Float::ZERO).is_nan());
    ///
    /// assert_eq!((Float::from(1.5) / Float::from(2.5)).to_string(), "0.6");
    /// assert_eq!((Float::from(1.5) / Float::from(-2.5)).to_string(), "-0.6");
    /// assert_eq!((Float::from(-1.5) / Float::from(2.5)).to_string(), "-0.6");
    /// assert_eq!((Float::from(-1.5) / Float::from(-2.5)).to_string(), "0.6");
    /// ```
    #[inline]
    fn div(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round(other, prec, Nearest).0
    }
}

impl Div<&Float> for Float {
    type Output = Float;

    /// Divides two [`Float`]s, taking the first by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// quotient is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm\infty)=f(\pm0.0,\pm0.0) = \text{NaN}$
    /// - $f(\infty,x)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0)=\infty$ if $x>0.0$
    /// - $f(x,0.0)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_val_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::div_round_val_ref`]. If you want both of these things, consider using
    /// [`Float::div_prec_round_val_ref`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!((Float::from(1.5) / &Float::NAN).is_nan());
    /// assert_eq!(Float::from(1.5) / &Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     Float::from(1.5) / &Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(Float::from(-1.5) / &Float::ZERO, Float::NEGATIVE_INFINITY);
    /// assert_eq!(Float::from(-1.5) / &Float::NEGATIVE_ZERO, Float::INFINITY);
    /// assert!((Float::ZERO / &Float::ZERO).is_nan());
    ///
    /// assert_eq!((Float::from(1.5) / &Float::from(2.5)).to_string(), "0.6");
    /// assert_eq!((Float::from(1.5) / &Float::from(-2.5)).to_string(), "-0.6");
    /// assert_eq!((Float::from(-1.5) / &Float::from(2.5)).to_string(), "-0.6");
    /// assert_eq!((Float::from(-1.5) / &Float::from(-2.5)).to_string(), "0.6");
    /// ```
    #[inline]
    fn div(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Div<Float> for &Float {
    type Output = Float;

    /// Divides two [`Float`]s, taking the first by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// quotient is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm\infty)=f(\pm0.0,\pm0.0) = \text{NaN}$
    /// - $f(\infty,x)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0)=\infty$ if $x>0.0$
    /// - $f(x,0.0)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_ref_val`] instead. If you want to specify the output precision, consider
    /// using [`Float::div_round_ref_val`]. If you want both of these things, consider using
    /// [`Float::div_prec_round_ref_val`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) / Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) / Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     &Float::from(1.5) / Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(&Float::from(-1.5) / Float::ZERO, Float::NEGATIVE_INFINITY);
    /// assert_eq!(&Float::from(-1.5) / Float::NEGATIVE_ZERO, Float::INFINITY);
    /// assert!((&Float::ZERO / Float::ZERO).is_nan());
    ///
    /// assert_eq!((&Float::from(1.5) / Float::from(2.5)).to_string(), "0.6");
    /// assert_eq!((&Float::from(1.5) / Float::from(-2.5)).to_string(), "-0.6");
    /// assert_eq!((&Float::from(-1.5) / Float::from(2.5)).to_string(), "-0.6");
    /// assert_eq!((&Float::from(-1.5) / Float::from(-2.5)).to_string(), "0.6");
    /// ```
    #[inline]
    fn div(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Div<&Float> for &Float {
    type Output = Float;

    /// Divides two [`Float`]s, taking both by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// quotient is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(\pm\infty,\pm\infty)=f(\pm0.0,\pm0.0) = \text{NaN}$
    /// - $f(\infty,x)=\infty$ if $0.0<x<\infty$
    /// - $f(\infty,x)=-\infty$ if $-\infty<x<0.0$
    /// - $f(x,0.0)=\infty$ if $x>0.0$
    /// - $f(x,0.0)=-\infty$ if $x<0.0$
    /// - $f(-\infty,x)=-\infty$ if $0.0<x<\infty$
    /// - $f(-\infty,x)=\infty$ if $-\infty<x<0.0$
    /// - $f(x,-0.0)=-\infty$ if $x>0.0$
    /// - $f(x,-0.0)=\infty$ if $x<0.0$
    /// - $f(0.0,x)=0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(0.0,x)=-0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    /// - $f(-0.0,x)=-0.0$ if $x$ is not NaN and $x>0.0$
    /// - $f(-0.0,x)=0.0$ if $x$ is not NaN and $x<0.0$
    /// - $f(x,-\infty)=-0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=0.0$ or $x>0.0$
    /// - $f(x,-\infty)=0.0$ if $x$ is not NaN or $\pm\infty$, and if $x=-0.0$ or $x<0.0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_ref_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::div_round_ref_ref`]. If you want both of these things, consider using
    /// [`Float::div_prec_round_ref_ref`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::from(1.5) / &Float::NAN).is_nan());
    /// assert_eq!(&Float::from(1.5) / &Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     &Float::from(1.5) / &Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(&Float::from(-1.5) / &Float::ZERO, Float::NEGATIVE_INFINITY);
    /// assert_eq!(&Float::from(-1.5) / &Float::NEGATIVE_ZERO, Float::INFINITY);
    /// assert!((&Float::ZERO / &Float::ZERO).is_nan());
    ///
    /// assert_eq!((&Float::from(1.5) / &Float::from(2.5)).to_string(), "0.6");
    /// assert_eq!((&Float::from(1.5) / &Float::from(-2.5)).to_string(), "-0.6");
    /// assert_eq!((&Float::from(-1.5) / &Float::from(2.5)).to_string(), "-0.6");
    /// assert_eq!((&Float::from(-1.5) / &Float::from(-2.5)).to_string(), "0.6");
    /// ```
    #[inline]
    fn div(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl DivAssign<Float> for Float {
    /// Divides a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side by
    /// value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// quotient is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the `/` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::div_round_assign`]. If you want both of these things, consider using
    /// [`Float::div_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x /= Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x /= Float::ZERO;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x /= Float::NEGATIVE_ZERO;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= Float::ZERO;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= Float::NEGATIVE_ZERO;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x /= Float::INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x /= Float::from(2.5);
    /// assert_eq!(x.to_string(), "0.6");
    ///
    /// let mut x = Float::from(1.5);
    /// x /= Float::from(-2.5);
    /// assert_eq!(x.to_string(), "-0.6");
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= Float::from(2.5);
    /// assert_eq!(x.to_string(), "-0.6");
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= Float::from(-2.5);
    /// assert_eq!(x.to_string(), "0.6");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_assign(other, prec, Nearest);
    }
}

impl DivAssign<&Float> for Float {
    /// Divides a [`Float`] by a [`Float`] in place, taking the [`Float`] on the right-hand side by
    /// reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// quotient is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x\gets = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the `/` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::div_round_assign`]. If you want both of these things, consider using
    /// [`Float::div_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(1.5);
    /// x /= &Float::NAN;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x /= &Float::ZERO;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x /= &Float::NEGATIVE_ZERO;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= &Float::ZERO;
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= &Float::NEGATIVE_ZERO;
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x /= &Float::INFINITY;
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x /= &Float::from(2.5);
    /// assert_eq!(x.to_string(), "0.6");
    ///
    /// let mut x = Float::from(1.5);
    /// x /= &Float::from(-2.5);
    /// assert_eq!(x.to_string(), "-0.6");
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= &Float::from(2.5);
    /// assert_eq!(x.to_string(), "-0.6");
    ///
    /// let mut x = Float::from(-1.5);
    /// x /= &Float::from(-2.5);
    /// assert_eq!(x.to_string(), "0.6");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Float) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.div_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Div<Rational> for Float {
    type Output = Float;

    /// Divides a [`Float`] by a [`Rational`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=f(\pm0.0,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x\geq 0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x>0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x>0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec`] instead. If you want to specify the output precision, consider
    /// using [`Float::div_rational_round`]. If you want both of these things, consider using
    /// [`Float::div_rational_prec_round`].
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
    /// assert!((Float::NAN / Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(Float::INFINITY / Rational::exact_from(1.5), Float::INFINITY);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY / Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::INFINITY / Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY / Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (Float::from(2.5) / Rational::exact_from(1.5)).to_string(),
    ///     "1.8"
    /// );
    /// assert_eq!(
    ///     (Float::from(2.5) / Rational::exact_from(-1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (Float::from(-2.5) / Rational::exact_from(1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (Float::from(-2.5) / Rational::exact_from(-1.5)).to_string(),
    ///     "1.8"
    /// );
    /// ```
    #[inline]
    fn div(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.div_rational_prec_round(other, prec, Nearest).0
    }
}

impl Div<&Rational> for Float {
    type Output = Float;

    /// Divides a [`Float`] by a [`Rational`], taking the first by value and the second by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=f(\pm0.0,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x\geq 0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x>0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x>0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_val_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::div_rational_round_val_ref`]. If you want both of these things,
    /// consider using [`Float::div_rational_prec_round_val_ref`].
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
    /// assert!((Float::NAN / &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     Float::INFINITY / &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY / &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::INFINITY / &Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY / &Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (Float::from(2.5) / &Rational::exact_from(1.5)).to_string(),
    ///     "1.8"
    /// );
    /// assert_eq!(
    ///     (Float::from(2.5) / &Rational::exact_from(-1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (Float::from(-2.5) / &Rational::exact_from(1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (Float::from(-2.5) / &Rational::exact_from(-1.5)).to_string(),
    ///     "1.8"
    /// );
    /// ```
    #[inline]
    fn div(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.div_rational_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Div<Rational> for &Float {
    type Output = Float;

    /// Divides a [`Float`] by a [`Rational`], taking the first by reference and the second by
    /// value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=f(\pm0.0,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x\geq 0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x>0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x>0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_ref_val`] instead. If you want to specify the output precision,
    /// consider using [`Float::div_rational_round_ref_val`]. If you want both of these things,
    /// consider using [`Float::div_rational_prec_round_ref_val`].
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
    /// assert!((&Float::NAN / Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY / Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY / Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::INFINITY / Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY / Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (&Float::from(2.5) / Rational::exact_from(1.5)).to_string(),
    ///     "1.8"
    /// );
    /// assert_eq!(
    ///     (&Float::from(2.5) / Rational::exact_from(-1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (&Float::from(-2.5) / Rational::exact_from(1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (&Float::from(-2.5) / Rational::exact_from(-1.5)).to_string(),
    ///     "1.8"
    /// );
    /// ```
    #[inline]
    fn div(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.div_rational_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Div<&Rational> for &Float {
    type Output = Float;

    /// Divides a [`Float`] by a [`Rational`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(\pm\infty,0)=f(\pm0.0,0)=\text{NaN}$
    /// - $f(\infty,x)=\infty$ if $x\geq 0$
    /// - $f(\infty,x)=-\infty$ if $x<0$
    /// - $f(-\infty,x)=-\infty$ if $x\geq 0$
    /// - $f(-\infty,x)=\infty$ if $x<0$
    /// - $f(0.0,x)=0.0$ if $x>0$
    /// - $f(0.0,x)=-0.0$ if $x<0$
    /// - $f(-0.0,x)=-0.0$ if $x>0$
    /// - $f(-0.0,x)=0.0$ if $x<0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_ref_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::div_rational_round_ref_ref`]. If you want both of these things,
    /// consider using [`Float::div_rational_prec_round_ref_ref`].
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
    /// assert!((&Float::NAN / &Rational::exact_from(1.5)).is_nan());
    /// assert_eq!(
    ///     &Float::INFINITY / &Rational::exact_from(1.5),
    ///     Float::INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY / &Rational::exact_from(1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::INFINITY / &Rational::exact_from(-1.5),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Float::NEGATIVE_INFINITY / &Rational::exact_from(-1.5),
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (&Float::from(2.5) / &Rational::exact_from(1.5)).to_string(),
    ///     "1.8"
    /// );
    /// assert_eq!(
    ///     (&Float::from(2.5) / &Rational::exact_from(-1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (&Float::from(-2.5) / &Rational::exact_from(1.5)).to_string(),
    ///     "-1.8"
    /// );
    /// assert_eq!(
    ///     (&Float::from(-2.5) / &Rational::exact_from(-1.5)).to_string(),
    ///     "1.8"
    /// );
    /// ```
    #[inline]
    fn div(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.div_rational_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl DivAssign<Rational> for Float {
    /// Divides a [`Float`] by a [`Rational`] in place, taking the [`Rational`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `/` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::div_rational_round_assign`]. If you want both of these things,
    /// consider using [`Float::div_rational_prec_round_assign`].
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
    /// x /= Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x /= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x /= Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x /= Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x /= Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x /= Rational::exact_from(1.5);
    /// assert_eq!(x.to_string(), "1.8");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.div_rational_prec_round_assign(other, prec, Nearest);
    }
}

impl DivAssign<&Rational> for Float {
    /// Divides a [`Float`] by a [`Rational`] in place, taking the [`Rational`] by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// See the `/` documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::div_rational_prec_assign_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::div_rational_round_assign_ref`]. If you want both of
    /// these things, consider using [`Float::div_rational_prec_round_assign_ref`].
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
    /// x /= &Rational::exact_from(1.5);
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x /= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x /= &Rational::exact_from(1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::INFINITY;
    /// x /= &Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x /= &Rational::exact_from(-1.5);
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(2.5);
    /// x /= &Rational::exact_from(1.5);
    /// assert_eq!(x.to_string(), "1.8");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.div_rational_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Div<Float> for Rational {
    type Output = Float;

    /// Divides a [`Rational`] by a [`Float`], taking both by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) / Float::NAN).is_nan());
    /// assert_eq!(Rational::exact_from(1.5) / Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     Rational::exact_from(1.5) / Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) / Float::ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) / Float::NEGATIVE_ZERO,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (Rational::exact_from(1.5) / Float::from(2.5)).to_string(),
    ///     "0.6"
    /// );
    /// assert_eq!(
    ///     (Rational::exact_from(-1.5) / Float::from(2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (Rational::exact_from(1.5) / Float::from(-2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (Rational::exact_from(-1.5) / Float::from(-2.5)).to_string(),
    ///     "0.6"
    /// );
    /// ```
    #[inline]
    fn div(self, other: Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_div_float_prec_round(self, other, prec, Nearest).0
    }
}

impl Div<&Float> for Rational {
    type Output = Float;

    /// Divides a [`Rational`] by a [`Float`], taking the [`Rational`] by value and the [`Float`] by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((Rational::exact_from(1.5) / &Float::NAN).is_nan());
    /// assert_eq!(Rational::exact_from(1.5) / &Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     Rational::exact_from(1.5) / &Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) / &Float::ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     Rational::exact_from(-1.5) / &Float::NEGATIVE_ZERO,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (Rational::exact_from(1.5) / &Float::from(2.5)).to_string(),
    ///     "0.6"
    /// );
    /// assert_eq!(
    ///     (Rational::exact_from(-1.5) / &Float::from(2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (Rational::exact_from(1.5) / &Float::from(-2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (Rational::exact_from(-1.5) / &Float::from(-2.5)).to_string(),
    ///     "0.6"
    /// );
    /// ```
    #[inline]
    fn div(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_div_float_prec_round_val_ref(self, other, prec, Nearest).0
    }
}

impl Div<Float> for &Rational {
    type Output = Float;

    /// Divides a [`Rational`] by a [`Float`], taking the [`Rational`] by reference and the
    /// [`Float`] by value.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) / Float::NAN).is_nan());
    /// assert_eq!(&Rational::exact_from(1.5) / Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) / Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) / Float::ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) / Float::NEGATIVE_ZERO,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (&Rational::exact_from(1.5) / Float::from(2.5)).to_string(),
    ///     "0.6"
    /// );
    /// assert_eq!(
    ///     (&Rational::exact_from(-1.5) / Float::from(2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (&Rational::exact_from(1.5) / Float::from(-2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (&Rational::exact_from(-1.5) / Float::from(-2.5)).to_string(),
    ///     "0.6"
    /// );
    /// ```
    #[inline]
    fn div(self, other: Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_div_float_prec_round_ref_val(self, other, prec, Nearest).0
    }
}

impl Div<&Float> for &Rational {
    type Output = Float;

    /// Divides a [`Rational`] by a [`Float`], taking both by reference.
    ///
    /// If the output has a precision, it is the precision of the input [`Float`]. If the quotient
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x/y+\varepsilon.
    /// $$
    /// - If $x/y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x/y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x/y|\rfloor-p}$,
    ///   where $p$ is the precision of the input [`Float`].
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p,m)=f(0,\pm0.0,p,m)=\text{NaN}$
    /// - $f(x,\infty,x,p,m)=0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,\infty,x,p,m)=-0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(x,-\infty,x,p,m)=-0.0$ if $x>0.0$ or $x=0.0$
    /// - $f(x,-\infty,x,p,m)=0.0$ if $x<0.0$ or #x=-0.0$
    /// - $f(0,x,p,m)=0.0$ if $x>0$
    /// - $f(0,x,p,m)=-0.0$ if $x<0$
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, Zero,
    /// };
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!((&Rational::exact_from(1.5) / &Float::NAN).is_nan());
    /// assert_eq!(&Rational::exact_from(1.5) / &Float::ZERO, Float::INFINITY);
    /// assert_eq!(
    ///     &Rational::exact_from(1.5) / &Float::NEGATIVE_ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) / &Float::ZERO,
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     &Rational::exact_from(-1.5) / &Float::NEGATIVE_ZERO,
    ///     Float::INFINITY
    /// );
    ///
    /// assert_eq!(
    ///     (&Rational::exact_from(1.5) / &Float::from(2.5)).to_string(),
    ///     "0.6"
    /// );
    /// assert_eq!(
    ///     (&Rational::exact_from(-1.5) / &Float::from(2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (&Rational::exact_from(1.5) / &Float::from(-2.5)).to_string(),
    ///     "-0.6"
    /// );
    /// assert_eq!(
    ///     (&Rational::exact_from(-1.5) / &Float::from(-2.5)).to_string(),
    ///     "0.6"
    /// );
    /// ```
    #[inline]
    fn div(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_div_float_prec_round_ref_ref(self, other, prec, Nearest).0
    }
}
