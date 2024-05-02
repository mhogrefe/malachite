// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
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
