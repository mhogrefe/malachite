// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_nz::natural::Natural;

/// Replaces a number with the closest [`Rational`] whose denominator does not exceed the specified
/// maximum.
pub trait ApproximateAssign {
    fn approximate_assign(&mut self, max_denominator: &Natural);
}

/// Returns the closest [`Rational`] whose denominator does not exceed the specified maximum.
pub trait Approximate {
    fn approximate(self, max_denominator: &Natural) -> Rational;
}

/// Finds the simplest [`Rational`] contained in an interval.
pub trait SimplestRationalInInterval {
    /// Finds the simplest [`Rational`] contained in an open interval.
    ///
    /// Simplicity is defined as follows: If two [`Rational`]s have different denominators, then the
    /// one with the smaller denominator is simpler. If they have the same denominator, then the one
    /// whose numerator is closer to zero is simpler. Finally, if $q > 0$, then $q$ is simpler than
    /// $-q$.
    fn simplest_rational_in_open_interval(x: &Self, y: &Self) -> Rational;

    /// Finds the simplest [`Rational`] contained in a closed interval.
    ///
    /// Simplicity is defined as follows: If two [`Rational`]s have different denominators, then the
    /// one with the smaller denominator is simpler. If they have the same denominator, then the one
    /// whose numerator is closer to zero is simpler. Finally, if $q > 0$, then $q$ is simpler than
    /// $-q$.
    fn simplest_rational_in_closed_interval(x: &Self, y: &Self) -> Rational;
}

// Returns an iterator of all denominators that appear in the [`Rational`]s contained in a closed
// interval.
pub trait DenominatorsInClosedInterval {
    type Denominators: Iterator<Item = Natural>;

    fn denominators_in_closed_interval(a: Rational, b: Rational) -> Self::Denominators;
}
