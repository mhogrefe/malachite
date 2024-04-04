use crate::test_util::common::rug_float_significant_bits;
use crate::Float;
use crate::InnerFloat::{Infinity, NaN, Zero};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AddAssignRound;
use std::cmp::Ordering;
use std::ops::AddAssign;

pub fn rug_add_round(x: rug::Float, y: rug::Float, rm: Round) -> (rug::Float, Ordering) {
    let xsb = rug_float_significant_bits(&x);
    let ysb = rug_float_significant_bits(&y);
    let mut sum = x;
    if sum == 0u32 || xsb < ysb {
        sum.set_prec(u32::exact_from(ysb));
    }
    let o = sum.add_assign_round(y, rm);
    (sum, o)
}

pub fn rug_add(x: rug::Float, y: rug::Float) -> rug::Float {
    let xsb = rug_float_significant_bits(&x);
    let ysb = rug_float_significant_bits(&y);
    let mut sum = x;
    if sum == 0u32 || xsb < ysb {
        sum.set_prec(u32::exact_from(ysb));
    }
    sum.add_assign(y);
    sum
}

pub fn rug_add_rational_round(
    x: rug::Float,
    y: rug::Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut sum = x;
    let o = sum.add_assign_round(y, rm);
    (sum, o)
}

pub fn rug_add_rational(x: rug::Float, y: rug::Rational) -> rug::Float {
    let mut sum = x;
    sum.add_assign(y);
    sum
}

pub fn add_prec_round_naive(x: Float, y: Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (float_nan!(), _)
        | (_, float_nan!())
        | (float_infinity!(), float_negative_infinity!())
        | (float_negative_infinity!(), float_infinity!()) => (float_nan!(), Ordering::Equal),
        (float_infinity!(), _) | (_, float_infinity!()) => (float_infinity!(), Ordering::Equal),
        (float_negative_infinity!(), _) | (_, float_negative_infinity!()) => {
            (float_negative_infinity!(), Ordering::Equal)
        }
        (float_zero!(), float_negative_zero!()) | (float_negative_zero!(), float_zero!()) => (
            if rm == RoundingMode::Floor {
                float_negative_zero!()
            } else {
                float_zero!()
            },
            Ordering::Equal,
        ),
        (float_either_zero!(), mut z) | (mut z, float_either_zero!()) => {
            let o = z.set_prec_round(prec, rm);
            (z, o)
        }
        (x, y) => {
            let (mut sum, o) = Float::from_rational_prec_round(
                Rational::exact_from(x) + Rational::exact_from(y),
                prec,
                rm,
            );
            if rm == RoundingMode::Floor && o == Ordering::Equal && sum == 0u32 {
                sum.neg_assign();
            }
            (sum, o)
        }
    }
}

pub fn add_rational_prec_round_naive(
    x: Float,
    y: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match (x, y) {
        (x @ float_nan!() | x @ float_infinity!() | x @ float_negative_infinity!(), _) => {
            (x, Ordering::Equal)
        }
        (float_negative_zero!(), y) => {
            if y == 0u32 {
                (float_negative_zero!(), Ordering::Equal)
            } else {
                Float::from_rational_prec_round(y, prec, rm)
            }
        }
        (float_zero!(), y) => Float::from_rational_prec_round(y, prec, rm),
        (x, y) => {
            let (mut sum, o) =
                Float::from_rational_prec_round(Rational::exact_from(x) + y, prec, rm);
            if rm == RoundingMode::Floor && o == Ordering::Equal && sum == 0u32 {
                sum.neg_assign();
            }
            (sum, o)
        }
    }
}
