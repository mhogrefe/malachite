use malachite_nz::natural::Natural;
use Rational;

pub trait ApproximateAssign {
    // Replaces `self` with the closest `Rational` whose denominator does not exceed the specified
    // maximum.
    fn approximate_assign(&mut self, max_denominator: &Natural);
}

pub trait Approximate {
    // Returns the closest `Rational` whose denominator does not exceed the specified maximum.
    fn approximate(self, max_denominator: &Natural) -> Rational;
}

pub trait SimplestRationalInInterval {
    // Finds the simplest `Rational` contained in an open interval.
    //
    // Simplicity is defined as follows: If two `Rational`s have different denominators, then the
    // one with the smaller denominator is simpler. If they have the same denominator, then the one
    // whose numerator is closer to zero is simpler. Finally, if $q > 0$, then $q$ is simpler than
    // $-q$.
    fn simplest_rational_in_open_interval(x: &Self, y: &Self) -> Rational;

    // Finds the simplest `Rational` contained in a closed interval.
    //
    // Simplicity is defined as follows: If two `Rational`s have different denominators, then the
    // one with the smaller denominator is simpler. If they have the same denominator, then the one
    // whose numerator is closer to zero is simpler. Finally, if $q > 0$, then $q$ is simpler than
    // $-q$.
    fn simplest_rational_in_closed_interval(x: &Self, y: &Self) -> Rational;
}
