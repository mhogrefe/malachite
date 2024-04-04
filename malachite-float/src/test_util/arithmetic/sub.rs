use crate::test_util::common::rug_float_significant_bits;
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::SubAssignRound;
use std::cmp::Ordering;
use std::ops::SubAssign;

pub fn rug_sub_round(x: rug::Float, y: rug::Float, rm: Round) -> (rug::Float, Ordering) {
    let xsb = rug_float_significant_bits(&x);
    let ysb = rug_float_significant_bits(&y);
    let mut diff = x;
    if diff == 0u32 || xsb < ysb {
        diff.set_prec(u32::exact_from(ysb));
    }
    let o = diff.sub_assign_round(y, rm);
    (diff, o)
}

pub fn rug_sub(x: rug::Float, y: rug::Float) -> rug::Float {
    let xsb = rug_float_significant_bits(&x);
    let ysb = rug_float_significant_bits(&y);
    let mut diff = x;
    if diff == 0u32 || xsb < ysb {
        diff.set_prec(u32::exact_from(ysb));
    }
    diff.sub_assign(y);
    diff
}

pub fn rug_sub_rational_round(
    x: rug::Float,
    y: rug::Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut diff = x;
    let o = diff.sub_assign_round(y, rm);
    (diff, o)
}

pub fn rug_sub_rational(x: rug::Float, y: rug::Rational) -> rug::Float {
    let mut diff = x;
    diff.sub_assign(y);
    diff
}
