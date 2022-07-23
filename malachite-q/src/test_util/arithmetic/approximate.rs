use crate::Rational;
use malachite_base::num::arithmetic::traits::{Reciprocal, RoundToMultiple};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::exhaustive::exhaustive_natural_inclusive_range;
use malachite_nz::natural::Natural;

// Slow! Only use for small `max_denominator`s
pub fn approximate_naive(x: &Rational, max_denominator: &Natural) -> Rational {
    let mut nearest = Rational::ZERO;
    for d in exhaustive_natural_inclusive_range(Natural::ONE, max_denominator.clone()) {
        let q = x.round_to_multiple(Rational::from(d).reciprocal(), RoundingMode::Nearest);
        if (x - &q).lt_abs(&(x - &nearest)) {
            nearest = q;
        }
    }
    nearest
}
