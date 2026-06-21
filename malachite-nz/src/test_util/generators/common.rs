// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{FloorLogBase, Pow, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::It;
use num::{BigInt, BigUint};

pub fn natural_nrm(xs: It<Natural>) -> It<(BigUint, rug::Integer, Natural)> {
    Box::new(xs.map(|x| (BigUint::from(&x), rug::Integer::from(&x), x)))
}

pub fn natural_rm(xs: It<Natural>) -> It<(rug::Integer, Natural)> {
    Box::new(xs.map(|x| (rug::Integer::from(&x), x)))
}

#[allow(clippy::type_complexity)]
pub fn natural_pair_nrm(
    ps: It<(Natural, Natural)>,
) -> It<(
    (BigUint, BigUint),
    (rug::Integer, rug::Integer),
    (Natural, Natural),
)> {
    Box::new(ps.map(|(x, y)| {
        (
            (BigUint::from(&x), BigUint::from(&y)),
            (rug::Integer::from(&x), rug::Integer::from(&y)),
            (x, y),
        )
    }))
}

pub fn natural_pair_rm(
    ps: It<(Natural, Natural)>,
) -> It<((rug::Integer, rug::Integer), (Natural, Natural))> {
    Box::new(ps.map(|(x, y)| ((rug::Integer::from(&x), rug::Integer::from(&y)), (x, y))))
}

pub fn natural_pair_nm(ps: It<(Natural, Natural)>) -> It<((BigUint, BigUint), (Natural, Natural))> {
    Box::new(ps.map(|(x, y)| ((BigUint::from(&x), BigUint::from(&y)), (x, y))))
}

pub fn natural_pair_1_rm<T: 'static + Clone>(
    ps: It<(Natural, T)>,
) -> It<((rug::Integer, T), (Natural, T))> {
    Box::new(ps.map(|(x, y)| ((rug::Integer::from(&x), y.clone()), (x, y))))
}

pub fn natural_pair_1_nm<T: 'static + Clone>(
    ps: It<(Natural, T)>,
) -> It<((BigUint, T), (Natural, T))> {
    Box::new(ps.map(|(x, y)| ((BigUint::from(&x), y.clone()), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn natural_pair_1_nrm<T: 'static + Clone>(
    ps: It<(Natural, T)>,
) -> It<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Box::new(ps.map(|(x, y)| {
        (
            (BigUint::from(&x), y.clone()),
            (rug::Integer::from(&x), y.clone()),
            (x, y),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn natural_triple_nrm(
    ts: It<(Natural, Natural, Natural)>,
) -> It<(
    (BigUint, BigUint, BigUint),
    (rug::Integer, rug::Integer, rug::Integer),
    (Natural, Natural, Natural),
)> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (BigUint::from(&x), BigUint::from(&y), BigUint::from(&z)),
            (
                rug::Integer::from(&x),
                rug::Integer::from(&y),
                rug::Integer::from(&z),
            ),
            (x, y, z),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn natural_triple_rm(
    ts: It<(Natural, Natural, Natural)>,
) -> It<(
    (rug::Integer, rug::Integer, rug::Integer),
    (Natural, Natural, Natural),
)> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (
                rug::Integer::from(&x),
                rug::Integer::from(&y),
                rug::Integer::from(&z),
            ),
            (x, y, z),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn natural_triple_1_rm<T: 'static + Clone, U: 'static + Clone>(
    ts: It<(Natural, T, U)>,
) -> It<((rug::Integer, T, U), (Natural, T, U))> {
    Box::new(ts.map(|(x, y, z)| ((rug::Integer::from(&x), y.clone(), z.clone()), (x, y, z))))
}

pub fn integer_rm(xs: It<Integer>) -> It<(rug::Integer, Integer)> {
    Box::new(xs.map(|x| (rug::Integer::from(&x), x)))
}

pub fn integer_nrm(xs: It<Integer>) -> It<(BigInt, rug::Integer, Integer)> {
    Box::new(xs.map(|x| (BigInt::from(&x), rug::Integer::from(&x), x)))
}

pub fn integer_pair_rm(
    ps: It<(Integer, Integer)>,
) -> It<((rug::Integer, rug::Integer), (Integer, Integer))> {
    Box::new(ps.map(|(x, y)| ((rug::Integer::from(&x), rug::Integer::from(&y)), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn integer_pair_nrm(
    ps: It<(Integer, Integer)>,
) -> It<(
    (BigInt, BigInt),
    (rug::Integer, rug::Integer),
    (Integer, Integer),
)> {
    Box::new(ps.map(|(x, y)| {
        (
            (BigInt::from(&x), BigInt::from(&y)),
            (rug::Integer::from(&x), rug::Integer::from(&y)),
            (x, y),
        )
    }))
}

pub fn integer_pair_nm(ps: It<(Integer, Integer)>) -> It<((BigInt, BigInt), (Integer, Integer))> {
    Box::new(ps.map(|(x, y)| ((BigInt::from(&x), BigInt::from(&y)), (x, y))))
}

pub fn integer_pair_1_rm<T: 'static + Clone>(
    ps: It<(Integer, T)>,
) -> It<((rug::Integer, T), (Integer, T))> {
    Box::new(ps.map(|(x, y)| ((rug::Integer::from(&x), y.clone()), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn integer_pair_1_nrm<T: 'static + Clone>(
    ps: It<(Integer, T)>,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Box::new(ps.map(|(x, y)| {
        (
            (BigInt::from(&x), y.clone()),
            (rug::Integer::from(&x), y.clone()),
            (x, y),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn integer_triple_1_rm<T: 'static + Clone, U: 'static + Clone>(
    ts: It<(Integer, T, U)>,
) -> It<((rug::Integer, T, U), (Integer, T, U))> {
    Box::new(ts.map(|(x, y, z)| ((rug::Integer::from(&x), y.clone(), z.clone()), (x, y, z))))
}

pub fn integer_natural_pair_rm(
    ps: It<(Integer, Natural)>,
) -> It<((rug::Integer, rug::Integer), (Integer, Natural))> {
    Box::new(ps.map(|(x, y)| ((rug::Integer::from(&x), rug::Integer::from(&y)), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn integer_integer_natural_triple_rm(
    ts: It<(Integer, Integer, Natural)>,
) -> It<(
    (rug::Integer, rug::Integer, rug::Integer),
    (Integer, Integer, Natural),
)> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (
                rug::Integer::from(&x),
                rug::Integer::from(&y),
                rug::Integer::from(&z),
            ),
            (x, y, z),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn natural_natural_triple_1_2_rm<T: 'static + Clone>(
    ts: It<(Natural, Natural, T)>,
) -> It<((rug::Integer, rug::Integer, T), (Natural, Natural, T))> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (rug::Integer::from(&x), rug::Integer::from(&y), z.clone()),
            (x, y, z),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn integer_integer_triple_1_2_rm<T: 'static + Clone>(
    ts: It<(Integer, Integer, T)>,
) -> It<((rug::Integer, rug::Integer, T), (Integer, Integer, T))> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (rug::Integer::from(&x), rug::Integer::from(&y), z.clone()),
            (x, y, z),
        )
    }))
}

pub fn integer_vec_nrm(
    xss: It<Vec<Integer>>,
) -> It<(Vec<BigInt>, Vec<rug::Integer>, Vec<Integer>)> {
    Box::new(xss.map(|xs| {
        (
            xs.iter().map(BigInt::from).collect(),
            xs.iter().map(rug::Integer::from).collect(),
            xs,
        )
    }))
}

pub fn natural_vec_nrm(
    xss: It<Vec<Natural>>,
) -> It<(Vec<BigUint>, Vec<rug::Integer>, Vec<Natural>)> {
    Box::new(xss.map(|xs| {
        (
            xs.iter().map(BigUint::from).collect(),
            xs.iter().map(rug::Integer::from).collect(),
            xs,
        )
    }))
}

// Maps raw components `(r, v1, v2, v3, rnd)` (from any generation mode) to a valid input tuple `(r,
// f, e, b0, m, rnd)` for `limbs_get_str_aux`. `r` is normalized (the most significant bit of its
// top limb is set); `f`, `e`, and the base `b0` are reduced into their valid ranges; and `m` is
// derived from the base-`b0` digit count of `N = floor(r * 2 ^ f)` so that the precondition `b0 ^
// (m - 1) <= Y < b0 ^ (m + 1)` holds by construction (no rejection sampling). `e` is negative (the
// "exact" case) part of the time; otherwise it spans the roundable and non-roundable cases.
#[allow(clippy::type_complexity)]
pub fn get_str_aux_inputs(
    (mut r, v1, v2, v3, rnd): (Vec<Limb>, u64, u64, u64, RoundingMode),
) -> (Vec<Limb>, u64, i64, i64, usize, RoundingMode) {
    let width = Limb::WIDTH;
    let n_width = u64::exact_from(r.len()) * width;
    // Normalize r.
    *r.last_mut().unwrap() |= Limb::power_of_2(width - 1);
    // neg_f = -f in [0, n_width - 1] (the function takes the magnitude of the non-positive f).
    let neg_f = v1 % n_width;
    // base: a non-power-of-2 in 3..=62.
    let mut b0 = (v3 % 60) + 3;
    if b0.is_power_of_two() {
        b0 += 1;
    }
    // N = r >> neg_f = floor(r * 2 ^ -neg_f) has base-b0 digit count m0, with b0 ^ (m0 - 1) <= N <
    // b0 ^ m0.
    let big_n = Natural::from_limbs_asc(&r) >> neg_f;
    let m0 = usize::exact_from(big_n.floor_log_base(&Natural::from(b0)) + 1);
    // The real precondition is `b0 ^ (m - 1) <= Y < 2 * b0 ^ m` (tighter than the `< b0 ^ (m + 1)`
    // in `limbs_get_str_aux`'s header): the round-away carry can't propagate past a leading digit
    // of 1. `m = m0` always satisfies this (`N < b0 ^ m0`); `m = m0 - 1` does exactly when N's
    // leading base-b0 digit is 1, i.e. `N < 2 * b0 ^ (m0 - 1)` — that case exercises the carry
    // loop.
    let m = if m0 >= 2
        && (v1 ^ v2 ^ v3) & 1 == 1
        && big_n < (Natural::from(b0).pow(u64::exact_from(m0 - 1)) << 1)
    {
        m0 - 1
    } else {
        m0
    };
    // e < 0 (exact) part of the time; otherwise an error exponent in [3, n_width + 2]. The minimum
    // of 3 matches the smallest error `limbs_float_exp` ever reports: `round_helper_2` (like the C
    // `mpfr_round_p`) reads out of bounds when the claimed error is so small that `n_width - e`
    // reaches the full precision, which never happens for a genuine approximation. Larger values
    // exercise both the roundable and non-roundable (MPFR_ROUND_FAILED) outcomes.
    let e = if v2 % 4 == 0 {
        -1
    } else {
        i64::exact_from(3 + v2 % n_width)
    };
    (r, neg_f, e, i64::exact_from(b0), m, rnd)
}
