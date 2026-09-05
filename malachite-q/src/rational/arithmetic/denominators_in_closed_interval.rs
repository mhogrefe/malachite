// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::arithmetic::traits::{
    DenominatorsInClosedInterval, SimplestRationalInInterval,
};
use alloc::collections::BinaryHeap;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::cmp::{Ordering, Reverse};
use malachite_base::num::arithmetic::traits::{
    Ceiling, CoprimeWith, Floor, Reciprocal, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::factorization::traits::Primes;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// Returns a k such that for all n >= k, any closed interval with the given diameter is guaranteed
// to contain rationals with (reduced) denominator n.
//
// An interval of diameter s is guaranteed to contain a reduced fraction with denominator d whenever
// s >= g(d)/d, where g is the Jacobsthal function: the largest gap between integers coprime to d.
// So a d that can fail satisfies g(d) > s * d, and by Kanold's bound g(d) <= 2^w, where w is the
// number of distinct prime factors of d, it satisfies d < 2^w/s. The smallest number with w
// distinct prime factors is the primorial p_w#, so failures with w factors exist only while p_w# <
// 2^w/s; the primorials outgrow the powers of two, so only finitely many w qualify, and every
// failing d lies below the largest of their bounds. This is tighter than rounding up to the next
// primorial, which is what this function used to do: for a diameter of 0.42 it returns 10 rather
// than 30.
crate_test_fn! {smallest_guaranteed_denominator(interval_diameter: &Rational) -> Natural {
    if *interval_diameter >= 1u32 {
        return Natural::ONE;
    }
    let mut primorial = Natural::ONE;
    let mut pow = Natural::ONE;
    let mut best = Natural::ONE;
    for p in Limb::primes() {
        // Failing denominators with this many distinct prime factors lie below this bound. The
        // bound doubles with each factor, so the last feasible one dominates.
        let bound = Rational::from(&pow) / interval_diameter;
        if bound <= primorial {
            // No number with this many distinct prime factors is small enough to fail, and each
            // extra factor at least triples the primorial while only doubling the bound, so larger
            // factor counts cannot fail either.
            break;
        }
        best = bound.floor().unsigned_abs() + Natural::ONE;
        primorial *= Natural::from(p);
        pow <<= 1u32;
    }
    best
}}

// Whether the closed interval [a, b] contains a fraction with the given reduced denominator: some
// integer in [ceil(ad), floor(bd)] coprime to d. The window holds about (b - a)d + 1 integers, so
// in the scan phase, where d is at most a small multiple of 1/(b - a), it is only a few gcds wide;
// asking the full per-denominator generator whether it is empty would construct an iterator, and
// clone both endpoints, per candidate.
fn denominator_present(a: &Rational, b: &Rational, d: &Natural) -> bool {
    let mut n = (a * Rational::from(d)).ceiling();
    let hi = (b * Rational::from(d)).floor();
    while n <= hi {
        if n.unsigned_abs_ref().coprime_with(d) {
            return true;
        }
        n += Integer::ONE;
    }
    false
}

fn smallest_likely_denominator(interval_diameter: &Rational) -> Natural {
    interval_diameter.reciprocal().ceiling().unsigned_abs()
}

// A gap between two adjacent known fractions, together with the simplest fraction in its open
// interior, which is the next fraction the gap will produce. Interior gaps — those whose ends are
// both produced fractions rather than the original endpoints — always have Farey-adjacent ends
// (the determinant of adjacent produced fractions is 1, by induction: the fractions simpler than
// both ends were produced first, so each end is a best approximation from its side), and the
// simplest fraction between Farey neighbors is their mediant. So interior gaps find their candidate
// with a single addition, and only the two gaps touching the original endpoints pay for a
// continued-fraction computation.
#[derive(Clone, Debug)]
struct Gap {
    candidate: Rational,
    lo: Rational,
    hi: Rational,
    lo_is_endpoint: bool,
    hi_is_endpoint: bool,
}

impl Gap {
    fn new(lo: Rational, hi: Rational, lo_is_endpoint: bool, hi_is_endpoint: bool) -> Self {
        let candidate = if lo_is_endpoint || hi_is_endpoint {
            Rational::simplest_rational_in_open_interval(&lo, &hi)
        } else {
            // The ends are Farey neighbors, so the mediant is the simplest fraction between them,
            // and the same determinant argument shows it is already in lowest terms.
            let n = Integer::from_sign_and_abs_ref(lo.sign, &lo.numerator)
                + Integer::from_sign_and_abs_ref(hi.sign, &hi.numerator);
            Rational {
                sign: n >= 0u32,
                numerator: n.unsigned_abs(),
                denominator: &lo.denominator + &hi.denominator,
            }
        };
        Self {
            candidate,
            lo,
            hi,
            lo_is_endpoint,
            hi_is_endpoint,
        }
    }
}

// Gaps are ordered by candidate denominator, so that a min-heap of them yields denominators in
// increasing order; the candidate value breaks ties arbitrarily but totally.
impl PartialEq for Gap {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.candidate == other.candidate
    }
}

impl Eq for Gap {}

impl PartialOrd for Gap {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Gap {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.candidate
            .denominator
            .cmp(&other.candidate.denominator)
            .then_with(|| self.candidate.cmp(&other.candidate))
    }
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
    gaps: BinaryHeap<Reverse<Gap>>,
    endpoint_denominators: Vec<Natural>,
}

impl DenominatorsInClosedRationalInterval {
    // Splits every gap whose candidate has the given denominator. The children's candidates are
    // strictly more complex than the parent's, so the heap minimum strictly increases.
    fn split_gaps_with_denominator(&mut self, d: &Natural) {
        while let Some(Reverse(g)) = self.gaps.peek() {
            if g.candidate.denominator_ref() != d {
                break;
            }
            let Reverse(g) = self.gaps.pop().unwrap();
            self.gaps.push(Reverse(Gap::new(
                g.lo,
                g.candidate.clone(),
                g.lo_is_endpoint,
                false,
            )));
            self.gaps.push(Reverse(Gap::new(
                g.candidate,
                g.hi,
                false,
                g.hi_is_endpoint,
            )));
        }
    }
}

impl Iterator for DenominatorsInClosedRationalInterval {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.current >= self.high_threshold {
            self.gaps.clear();
            self.endpoint_denominators.clear();
            self.current += Natural::ONE;
            Some(self.current.clone())
        } else if self.current >= self.low_threshold {
            self.gaps.clear();
            self.endpoint_denominators.clear();
            loop {
                self.current += Natural::ONE;
                if denominator_present(&self.a, &self.b, &self.current) {
                    return Some(self.current.clone());
                }
            }
        } else {
            if self.gaps.is_empty() {
                assert_eq!(self.current, 0u32);
                let ad = self.a.denominator_ref();
                let bd = self.b.denominator_ref();
                self.endpoint_denominators = match ad.cmp(bd) {
                    Equal => alloc::vec![ad.clone()],
                    Less => alloc::vec![ad.clone(), bd.clone()],
                    Greater => alloc::vec![bd.clone(), ad.clone()],
                };
                self.gaps.push(Reverse(Gap::new(
                    self.a.clone(),
                    self.b.clone(),
                    true,
                    true,
                )));
            }
            // The next denominator is the smaller of the least unconsumed endpoint denominator and
            // the least gap candidate's denominator. When they are equal, both are consumed: the
            // denominator is present both as an endpoint and as one or more interior fractions, and
            // is reported once.
            let heap_denominator = self
                .gaps
                .peek()
                .unwrap()
                .0
                .candidate
                .denominator_ref()
                .clone();
            if self
                .endpoint_denominators
                .first()
                .is_some_and(|pd| *pd <= heap_denominator)
            {
                let pd = self.endpoint_denominators.remove(0);
                if pd == heap_denominator {
                    self.split_gaps_with_denominator(&heap_denominator);
                }
                self.current = pd.clone();
                Some(pd)
            } else {
                self.split_gaps_with_denominator(&heap_denominator);
                self.current = heap_denominator.clone();
                Some(heap_denominator)
            }
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
    /// $T(n, i) = O(m (\log m)^2 \log\log m)$
    ///
    /// $M(n, i) = O(n + \log i)$
    ///
    /// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $n$ is
    /// `max(a.significant_bits(), b.significant_bits())`, and $m$ is $n + \log i$. Most iterations
    /// are cheaper: a continued-fraction computation is only needed when a gap adjacent to one of
    /// the original endpoints splits, and interior gaps find their next fraction with a single
    /// addition, since adjacent produced fractions are always Farey neighbors and the simplest
    /// fraction between Farey neighbors is their mediant.
    ///
    /// # Panics
    /// Panics if $a \geq b$.
    ///
    /// ```
    /// use malachite_base::iterators::prefix_to_string;
    /// use malachite_base::num::basic::traits::{One, OneHalf, Two};
    /// use malachite_q::Rational;
    /// use malachite_q::rational::arithmetic::traits::DenominatorsInClosedInterval;
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
    ///             Rational::ONE_HALF
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
    ///     "[1000000, 1000001, 2000001, 3000001, 3000002, 4000001, 4000003, 5000001, 5000002, \
    ///     5000003, 5000004, 6000001, 6000005, 7000001, 7000002, 7000003, 7000004, 7000005, \
    ///     7000006, 8000001, ...]"
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
            gaps: BinaryHeap::new(),
            endpoint_denominators: Vec::new(),
        }
    }
}
