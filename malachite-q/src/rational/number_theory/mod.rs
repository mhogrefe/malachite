// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Approximate`](traits::Approximate) and
/// [`ApproximateAssign`](traits::ApproximateAssign), traits for approximating a
/// [`Rational`](super::Rational) by a [`Rational`](super::Rational) with a bounded denominator.
pub mod approximate;
/// A function for computing Dedekind sums.
pub mod dedekind_sum;
/// Getting all denominators of [`Rational`](super::Rational)s that appear in a given closed
/// interval.
pub mod denominators_in_closed_interval;
/// Implementations of [`ExtendedGcd`](malachite_base::num::number_theory::traits::ExtendedGcd), a
/// trait for computing the GCD of two numbers along with Bézout cofactors.
pub mod extended_gcd;
/// Finding the neighbors of a [`Rational`](super::Rational) in a Farey sequence.
pub mod farey_neighbors;
/// Implementations of [`Gcd`](malachite_base::num::number_theory::traits::Gcd) and
/// [`GcdAssign`](malachite_base::num::number_theory::traits::GcdAssign), traits for computing the
/// GCD (greatest common divisor) of two numbers.
pub mod gcd;
/// A function for computing harmonic numbers.
pub mod harmonic_number;
/// Functions for finding the simplest (lowest-denominator) [`Rational`](super::Rational) in an
/// interval.
pub mod simplest_rational_in_interval;
/// Traits for number-theoretic functions of [`Rational`](crate::Rational)s.
pub mod traits;
