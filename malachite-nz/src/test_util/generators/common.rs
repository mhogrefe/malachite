use crate::integer::Integer;
use malachite_base::test_util::generators::common::It;
use crate::natural::Natural;
use num::{BigInt, BigUint};
use crate::test_util::common::{
    integer_to_bigint, integer_to_rug_integer, natural_to_biguint, natural_to_rug_integer,
};

pub fn natural_nrm(xs: It<Natural>) -> It<(BigUint, rug::Integer, Natural)> {
    Box::new(xs.map(|x| (natural_to_biguint(&x), natural_to_rug_integer(&x), x)))
}

pub fn natural_rm(xs: It<Natural>) -> It<(rug::Integer, Natural)> {
    Box::new(xs.map(|x| (natural_to_rug_integer(&x), x)))
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
            (natural_to_biguint(&x), natural_to_biguint(&y)),
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn natural_pair_rm(
    ps: It<(Natural, Natural)>,
) -> It<((rug::Integer, rug::Integer), (Natural, Natural))> {
    Box::new(ps.map(|(x, y)| {
        (
            (natural_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn natural_pair_1_rm<T: 'static + Clone>(
    ps: It<(Natural, T)>,
) -> It<((rug::Integer, T), (Natural, T))> {
    Box::new(ps.map(|(x, y)| ((natural_to_rug_integer(&x), y.clone()), (x, y))))
}

pub fn natural_pair_1_nm<T: 'static + Clone>(
    ps: It<(Natural, T)>,
) -> It<((BigUint, T), (Natural, T))> {
    Box::new(ps.map(|(x, y)| ((natural_to_biguint(&x), y.clone()), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn natural_pair_1_nrm<T: 'static + Clone>(
    ps: It<(Natural, T)>,
) -> It<((BigUint, T), (rug::Integer, T), (Natural, T))> {
    Box::new(ps.map(|(x, y)| {
        (
            (natural_to_biguint(&x), y.clone()),
            (natural_to_rug_integer(&x), y.clone()),
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
            (
                natural_to_biguint(&x),
                natural_to_biguint(&y),
                natural_to_biguint(&z),
            ),
            (
                natural_to_rug_integer(&x),
                natural_to_rug_integer(&y),
                natural_to_rug_integer(&z),
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
                natural_to_rug_integer(&x),
                natural_to_rug_integer(&y),
                natural_to_rug_integer(&z),
            ),
            (x, y, z),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn natural_triple_1_rm<T: 'static + Clone, U: 'static + Clone>(
    ts: It<(Natural, T, U)>,
) -> It<((rug::Integer, T, U), (Natural, T, U))> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (natural_to_rug_integer(&x), y.clone(), z.clone()),
            (x, y, z),
        )
    }))
}

pub fn integer_rm(xs: It<Integer>) -> It<(rug::Integer, Integer)> {
    Box::new(xs.map(|x| (integer_to_rug_integer(&x), x)))
}

pub fn integer_nrm(xs: It<Integer>) -> It<(BigInt, rug::Integer, Integer)> {
    Box::new(xs.map(|x| (integer_to_bigint(&x), integer_to_rug_integer(&x), x)))
}

pub fn integer_pair_rm(
    ps: It<(Integer, Integer)>,
) -> It<((rug::Integer, rug::Integer), (Integer, Integer))> {
    Box::new(ps.map(|(x, y)| {
        (
            (integer_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
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
            (integer_to_bigint(&x), integer_to_bigint(&y)),
            (integer_to_rug_integer(&x), integer_to_rug_integer(&y)),
            (x, y),
        )
    }))
}

pub fn integer_pair_1_rm<T: 'static + Clone>(
    ps: It<(Integer, T)>,
) -> It<((rug::Integer, T), (Integer, T))> {
    Box::new(ps.map(|(x, y)| ((integer_to_rug_integer(&x), y.clone()), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn integer_pair_1_nrm<T: 'static + Clone>(
    ps: It<(Integer, T)>,
) -> It<((BigInt, T), (rug::Integer, T), (Integer, T))> {
    Box::new(ps.map(|(x, y)| {
        (
            (integer_to_bigint(&x), y.clone()),
            (integer_to_rug_integer(&x), y.clone()),
            (x, y),
        )
    }))
}

#[allow(clippy::type_complexity)]
pub fn integer_triple_1_rm<T: 'static + Clone, U: 'static + Clone>(
    ts: It<(Integer, T, U)>,
) -> It<((rug::Integer, T, U), (Integer, T, U))> {
    Box::new(ts.map(|(x, y, z)| {
        (
            (integer_to_rug_integer(&x), y.clone(), z.clone()),
            (x, y, z),
        )
    }))
}

pub fn integer_natural_pair_rm(
    ps: It<(Integer, Natural)>,
) -> It<((rug::Integer, rug::Integer), (Integer, Natural))> {
    Box::new(ps.map(|(x, y)| {
        (
            (integer_to_rug_integer(&x), natural_to_rug_integer(&y)),
            (x, y),
        )
    }))
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
                integer_to_rug_integer(&x),
                integer_to_rug_integer(&y),
                natural_to_rug_integer(&z),
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
            (
                natural_to_rug_integer(&x),
                natural_to_rug_integer(&y),
                z.clone(),
            ),
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
            (
                integer_to_rug_integer(&x),
                integer_to_rug_integer(&y),
                z.clone(),
            ),
            (x, y, z),
        )
    }))
}
