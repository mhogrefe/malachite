use crate::Rational;
use malachite_nz::test_util::common::{
    bigint_to_integer, natural_to_bigint, natural_to_rug_integer, rug_integer_to_integer,
};
use num::BigRational;

pub fn bigrational_to_rational(n: &BigRational) -> Rational {
    Rational::from_integers(bigint_to_integer(n.numer()), bigint_to_integer(n.denom()))
}

pub fn rational_to_bigrational(n: &Rational) -> BigRational {
    let mut q = BigRational::new_raw(
        natural_to_bigint(n.numerator_ref()),
        natural_to_bigint(n.denominator_ref()),
    );
    if *n < 0 {
        q = -q;
    }
    q
}

pub fn rug_rational_to_rational(n: &rug::Rational) -> Rational {
    Rational::from_integers(
        rug_integer_to_integer(n.numer()),
        rug_integer_to_integer(n.denom()),
    )
}

pub fn rational_to_rug_rational(n: &Rational) -> rug::Rational {
    let mut q = rug::Rational::from((
        natural_to_rug_integer(n.numerator_ref()),
        natural_to_rug_integer(n.denominator_ref()),
    ));
    if *n < 0 {
        q = -q;
    }
    q
}
