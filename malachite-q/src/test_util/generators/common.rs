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
