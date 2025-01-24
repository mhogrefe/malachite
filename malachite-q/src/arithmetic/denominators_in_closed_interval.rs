// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::traits::{DenominatorsInClosedInterval, SimplestRationalInInterval};
use crate::exhaustive::{
    exhaustive_rationals_with_denominator_inclusive_range,
    exhaustive_rationals_with_denominator_range,
};
use crate::Rational;
use alloc::collections::BTreeSet;
use malachite_base::num::arithmetic::traits::{Ceiling, Reciprocal, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::factorization::traits::Primes;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// Returns a k such that for all n >= k, any closed interval with the given diameter is guaranteed
// to contain rationals with (reduced) denominator n.
fn smallest_guaranteed_denominator(interval_diameter: &Rational) -> Natural {
    if *interval_diameter >= 1u32 {
        return Natural::ONE;
    }
    let mut primorial = Natural::TWO;
    let mut pow = Natural::TWO;
    for p in Limb::primes().skip(1) {
        primorial *= Natural::from(p);
        pow <<= 1;
        let limit = Rational::from_naturals_ref(&pow, &primorial);
        if *interval_diameter >= limit {
            return primorial;
        }
    }
    panic!();
}

fn smallest_likely_denominator(interval_diameter: &Rational) -> Natural {
    interval_diameter.reciprocal().ceiling().unsigned_abs()
}

/// Returns an iterator of all denominators that appear in the [`Rational`]s contained in a closed
/// interval.
///
/// This `struct` is created by [`DenominatorsInClosedInterval::denominators_in_closed_interval`];
/// see its documentation for more.
#[derive(Clone, Debug)]
pub struct DenominatorsInClosedRationalInterval {
    a: Rational,
    b: Rational,
    low_threshold: Natural,
    high_threshold: Natural,
    current: Natural,
    points: BTreeSet<Rational>,
}

impl Iterator for DenominatorsInClosedRationalInterval {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.current >= self.high_threshold {
            self.points.clear();
            self.current += Natural::ONE;
            Some(self.current.clone())
        } else if self.current >= self.low_threshold {
            self.points.clear();
            loop {
                self.current += Natural::ONE;
                if exhaustive_rationals_with_denominator_inclusive_range(
                    self.current.clone(),
                    self.a.clone(),
                    self.b.clone(),
                )
                .next()
                .is_some()
                {
                    return Some(self.current.clone());
                }
            }
        } else if self.points.is_empty() {
            assert_eq!(self.current, 0u32);
            self.points.insert(self.a.clone());
            self.points.insert(self.b.clone());
            self.points
                .insert(Rational::simplest_rational_in_open_interval(
                    &self.a, &self.b,
                ));
            let mut min_denominator = self.a.denominator_ref();
            for p in &self.points {
                let pd = p.denominator_ref();
                if pd < min_denominator {
                    min_denominator = pd;
                }
            }
            self.current = min_denominator.clone();
            for p in exhaustive_rationals_with_denominator_range(
                self.current.clone(),
                self.a.clone(),
                self.b.clone(),
            ) {
                self.points.insert(p);
            }
            Some(self.current.clone())
        } else {
            let mut previous_point = None;
            let mut min_interior_denominator = None;
            for p in &self.points {
                if let Some(previous) = previous_point {
                    let interior_denominator =
                        Rational::simplest_rational_in_open_interval(previous, p)
                            .into_denominator();
                    if let Some(previous_min) = min_interior_denominator.as_ref() {
                        if interior_denominator < *previous_min {
                            min_interior_denominator = Some(interior_denominator);
                        }
                    } else {
                        min_interior_denominator = Some(interior_denominator);
                    }
                }
                previous_point = Some(p);
            }
            let min_interior_denominator = min_interior_denominator.unwrap();
            assert!(min_interior_denominator > self.current);
            let mut min_denominator = min_interior_denominator;
            let ad = self.a.denominator_ref();
            if *ad > self.current && *ad < min_denominator {
                min_denominator = ad.clone();
            }
            let bd = self.b.denominator_ref();
            if *bd > self.current && *bd < min_denominator {
                min_denominator = bd.clone();
            }
            self.current = min_denominator;
            for p in exhaustive_rationals_with_denominator_range(
                self.current.clone(),
                self.a.clone(),
                self.b.clone(),
            ) {
                self.points.insert(p);
            }
            Some(self.current.clone())
        }
    }
}

impl DenominatorsInClosedInterval for Rational {
    type Denominators = DenominatorsInClosedRationalInterval;

    /// Returns an iterator of all denominators that appear in the [`Rational`]s contained in a
    /// closed interval.
    ///
    /// For example, consider the interval $[1/3, 1/2]$. It contains no integers, so no
    /// [`Rational`]s with denominator 1. It does contain [`Rational`]s with denominators 2 and 3
    /// (the endpoints). It contains none with denominator 4, but it does contain $2/5$. It contains
    /// none with denominator 6 (though $1/3$ and $1/2$ are $2/6$ and $3/6$, those representations
    /// are not reduced). It contains $3/7$, $3/8$, and $4/9$ but none with denominator 10 ($0.4$
    /// does not count because it is $2/5$). It contains all denominators greater than 10, so the
    /// complete list is $2, 3, 5, 7, 8, 9, 11, 12, 13, \ldots$.
    ///
    /// # Worst-case complexity per iteration
    /// $T(n, i) = O(n + \log i)$
    ///
    /// $M(n, i) = O(n + \log i)$
    ///
    /// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $n$ is
    /// `max(a.significant_bits(), b.significant_bits())`.
    ///
    /// # Panics
    /// Panics if $a \geq b$.
    ///
    /// ```
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_q::arithmetic::traits::DenominatorsInClosedInterval;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     prefix_to_string(
    ///         Rational::denominators_in_closed_interval(Rational::ONE, Rational::TWO),
    ///         20
    ///     ),
    ///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, ...]"
    /// );
    /// assert_eq!(
    ///     prefix_to_string(
    ///         Rational::denominators_in_closed_interval(
    ///             Rational::from_signeds(1, 3),
    ///             Rational::from_signeds(1, 2)
    ///         ),
    ///         20
    ///     ),
    ///     "[2, 3, 5, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, ...]"
    /// );
    /// assert_eq!(
    ///     prefix_to_string(
    ///         Rational::denominators_in_closed_interval(
    ///             Rational::from_signeds(1, 1000001),
    ///             Rational::from_signeds(1, 1000000)
    ///         ),
    ///         20
    ///     ),
    ///     "[1000000, 1000001, 3000001, 3000002, 4000001, 4000003, 5000001, 5000002, 5000003, \
    ///     5000004, 6000001, 6000005, 7000001, 7000002, 7000003, 7000004, 7000005, 7000006, \
    ///     8000001, 8000003, ...]"
    /// );
    /// ```
    fn denominators_in_closed_interval(
        a: Rational,
        b: Rational,
    ) -> DenominatorsInClosedRationalInterval {
        assert!(a < b);
        let diameter = &b - &a;
        let (mut low_threshold, high_threshold) = if diameter >= 1u32 {
            (Natural::ZERO, Natural::ZERO)
        } else {
            (
                smallest_likely_denominator(&diameter),
                smallest_guaranteed_denominator(&diameter),
            )
        };
        if low_threshold < 100u32 {
            low_threshold = Natural::ZERO;
        }
        DenominatorsInClosedRationalInterval {
            a,
            b,
            low_threshold,
            high_threshold,
            current: Natural::ZERO,
            points: BTreeSet::new(),
        }
    }
}
