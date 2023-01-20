use crate::Rational;
use malachite_nz::integer::Integer;
use num::{BigInt, BigRational};

impl From<&BigRational> for Rational {
    fn from(n: &BigRational) -> Rational {
        Rational::from_integers(Integer::from(n.numer()), Integer::from(n.denom()))
    }
}

impl From<&Rational> for BigRational {
    fn from(n: &Rational) -> BigRational {
        let mut q = BigRational::new_raw(
            BigInt::from(n.numerator_ref()),
            BigInt::from(n.denominator_ref()),
        );
        if *n < 0 {
            q = -q;
        }
        q
    }
}

impl From<&rug::Rational> for Rational {
    fn from(n: &rug::Rational) -> Rational {
        Rational::from_integers(Integer::from(n.numer()), Integer::from(n.denom()))
    }
}

impl From<&Rational> for rug::Rational {
    fn from(n: &Rational) -> rug::Rational {
        let mut q = rug::Rational::from((
            rug::Integer::from(n.numerator_ref()),
            rug::Integer::from(n.denominator_ref()),
        ));
        if *n < 0 {
            q = -q;
        }
        q
    }
}
