// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::test_util::generators::common::It;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::BigRational;

pub fn rational_rm(xs: It<Rational>) -> It<(rug::Rational, Rational)> {
    Box::new(xs.map(|x| (rug::Rational::from(&x), x)))
}

pub fn rational_nrm(xs: It<Rational>) -> It<(BigRational, rug::Rational, Rational)> {
    Box::new(xs.map(|x| (BigRational::from(&x), rug::Rational::from(&x), x)))
}

pub fn rational_pair_rm(
    ps: It<(Rational, Rational)>,
) -> It<((rug::Rational, rug::Rational), (Rational, Rational))> {
    Box::new(ps.map(|(x, y)| ((rug::Rational::from(&x), rug::Rational::from(&y)), (x, y))))
}

pub fn rational_pair_nm(
    ps: It<(Rational, Rational)>,
) -> It<((BigRational, BigRational), (Rational, Rational))> {
    Box::new(ps.map(|(x, y)| ((BigRational::from(&x), BigRational::from(&y)), (x, y))))
}

#[allow(clippy::type_complexity)]
pub fn rational_pair_nrm(
    ps: It<(Rational, Rational)>,
) -> It<(
    (BigRational, BigRational),
    (rug::Rational, rug::Rational),
    (Rational, Rational),
)> {
    Box::new(ps.map(|(x, y)| {
        (
            (BigRational::from(&x), BigRational::from(&y)),
            (rug::Rational::from(&x), rug::Rational::from(&y)),
            (x, y),
        )
    }))
}

pub fn rational_integer_pair_rm(
    ps: It<(Rational, Integer)>,
) -> It<((rug::Rational, rug::Integer), (Rational, Integer))> {
    Box::new(ps.map(|(x, y)| ((rug::Rational::from(&x), rug::Integer::from(&y)), (x, y))))
}

pub fn rational_natural_pair_rm(
    ps: It<(Rational, Natural)>,
) -> It<((rug::Rational, rug::Integer), (Rational, Natural))> {
    Box::new(ps.map(|(x, y)| ((rug::Rational::from(&x), rug::Integer::from(&y)), (x, y))))
}

pub fn rational_pair_1_rm<T: 'static + Clone>(
    ps: It<(Rational, T)>,
) -> It<((rug::Rational, T), (Rational, T))> {
    Box::new(ps.map(|(x, y)| ((rug::Rational::from(&x), y.clone()), (x, y))))
}

pub fn rational_pair_1_nrm<T: 'static + Clone>(
    ps: It<(Rational, T)>,
) -> It<((BigRational, T), (rug::Rational, T), (Rational, T))> {
    Box::new(ps.map(|(x, y)| {
        (
            (BigRational::from(&x), y.clone()),
            (rug::Rational::from(&x), y.clone()),
            (x, y),
        )
    }))
}

pub fn rational_vec_nrm(
    xss: It<Vec<Rational>>,
) -> It<(Vec<BigRational>, Vec<rug::Rational>, Vec<Rational>)> {
    Box::new(xss.map(|xs| {
        (
            xs.iter().map(BigRational::from).collect(),
            xs.iter().map(rug::Rational::from).collect(),
            xs,
        )
    }))
}
