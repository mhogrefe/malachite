// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::traits::{Approximate, ApproximateAssign};
use crate::Rational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivMod, Floor, Parity, Reciprocal, ShrRound, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

fn approximate_helper(q: &Rational, max_denominator: &Natural) -> Rational {
    let floor = q.floor();
    let mut x = (q - Rational::from(&floor)).reciprocal();
    let mut previous_numerator = Integer::ONE;
    let mut previous_denominator = Natural::ZERO;
    let mut numerator = floor;
    let mut denominator = Natural::ONE;
    let mut result = None;
    loop {
        let n;
        (n, x.numerator) = (&x.numerator).div_mod(&x.denominator);
        swap(&mut x.numerator, &mut x.denominator);
        let previous_previous_numerator = previous_numerator.clone();
        let previous_previous_denominator = previous_denominator.clone();
        previous_numerator.add_mul_assign(&numerator, Integer::from(&n));
        previous_denominator.add_mul_assign(&denominator, &n);
        if previous_denominator > *max_denominator {
            previous_numerator = previous_previous_numerator;
            previous_denominator = previous_previous_denominator;
            // We need a term m such that previous_denominator + denominator * m is as large as
            // possible without exceeding max_denominator.
            let m = (max_denominator - &previous_denominator) / &denominator;
            let half_n = (&n).shr_round(1, Ceiling).0;
            if m < half_n {
            } else if m == half_n && n.even() {
                let previous_convergent = Rational {
                    sign: numerator >= 0u32,
                    numerator: (&numerator).unsigned_abs(),
                    denominator: denominator.clone(),
                };
                previous_numerator.add_mul_assign(&numerator, Integer::from(&m));
                previous_denominator.add_mul_assign(&denominator, m);
                let candidate = Rational {
                    sign: previous_numerator >= 0u32,
                    numerator: previous_numerator.unsigned_abs(),
                    denominator: previous_denominator,
                };
                result = Some(if (q - &previous_convergent).lt_abs(&(q - &candidate)) {
                    previous_convergent
                } else {
                    candidate
                });
            } else {
                numerator *= Integer::from(&m);
                numerator += previous_numerator;
                denominator *= m;
                denominator += previous_denominator;
            }
            break;
        }
        swap(&mut numerator, &mut previous_numerator);
        swap(&mut denominator, &mut previous_denominator);
    }
    let result = if let Some(result) = result {
        result
    } else {
        Rational {
            sign: numerator >= 0u32,
            numerator: numerator.unsigned_abs(),
            denominator,
        }
    };
    // Suppose the input is (1/4, 2). The approximations 0 and 1/2 both satisfy the denominator
    // limit and are equidistant from 1/4, but we prefer 0 because it has the smaller denominator.
    // Unfortunately, the code above makes the wrong choice, so we need the following code to check
    // whether the approximation on the opposite side of `self` is better.
    let opposite: Rational = (q << 1) - &result;
    if result.denominator_ref() <= opposite.denominator_ref() {
        result
    } else {
        opposite
    }
}

impl Approximate for Rational {
    /// Finds the best approximation of a [`Rational`] using a denominator no greater than a
    /// specified maximum, taking the [`Rational`] by value.
    ///
    /// Let $f(x, d) = p/q$, with $p$ and $q$ relatively prime. Then the following properties hold:
    /// - $q \leq d$
    /// - For all $n \in \Z$ and all $m \in \Z$ with $0 < m \leq d$, $|x - p/q| \leq |x - n/m|$.
    /// - If $|x - n/m| = |x - p/q|$, then $q \leq m$.
    /// - If $|x - n/q| = |x - p/q|$, then $p$ is even and $n$ is either equal to $p$ or odd.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// - If `max_denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::arithmetic::traits::Approximate;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::exact_from(std::f64::consts::PI)
    ///         .approximate(&Natural::from(1000u32))
    ///         .to_string(),
    ///     "355/113"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(333i32, 1000)
    ///         .approximate(&Natural::from(100u32))
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    ///
    /// # Implementation notes
    /// This algorithm follows the description in
    /// <https://en.wikipedia.org/wiki/Continued_fraction#Best_rational_approximations>. One part of
    /// the algorithm not mentioned in that article is that if the last term $n$ in the continued
    /// fraction needs to be reduced, the optimal replacement term $m$ may be found using division.
    fn approximate(self, max_denominator: &Natural) -> Rational {
        assert_ne!(*max_denominator, 0);
        if self.denominator_ref() <= max_denominator {
            return self;
        }
        if *max_denominator == 1u32 {
            return Rational::from(Integer::rounding_from(self, Nearest).0);
        }
        approximate_helper(&self, max_denominator)
    }
}

impl<'a> Approximate for &'a Rational {
    /// Finds the best approximation of a [`Rational`] using a denominator no greater than a
    /// specified maximum, taking the [`Rational`] by reference.
    ///
    /// Let $f(x, d) = p/q$, with $p$ and $q$ relatively prime. Then the following properties hold:
    /// - $q \leq d$
    /// - For all $n \in \Z$ and all $m \in \Z$ with $0 < m \leq d$, $|x - p/q| \leq |x - n/m|$.
    /// - If $|x - n/m| = |x - p/q|$, then $q \leq m$.
    /// - If $|x - n/q| = |x - p/q|$, then $p$ is even and $n$ is either equal to $p$ or odd.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// - If `max_denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::arithmetic::traits::Approximate;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::exact_from(std::f64::consts::PI))
    ///         .approximate(&Natural::from(1000u32))
    ///         .to_string(),
    ///     "355/113"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(333i32, 1000))
    ///         .approximate(&Natural::from(100u32))
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    ///
    /// # Implementation notes
    /// This algorithm follows the description in
    /// <https://en.wikipedia.org/wiki/Continued_fraction#Best_rational_approximations>. One part of
    /// the algorithm not mentioned in that article is that if the last term $n$ in the continued
    /// fraction needs to be reduced, the optimal replacement term $m$ may be found using division.
    fn approximate(self, max_denominator: &Natural) -> Rational {
        assert_ne!(*max_denominator, 0);
        if self.denominator_ref() <= max_denominator {
            return self.clone();
        }
        if *max_denominator == 1u32 {
            return Rational::from(Integer::rounding_from(self, Nearest).0);
        }
        approximate_helper(self, max_denominator)
    }
}

impl ApproximateAssign for Rational {
    /// Finds the best approximation of a [`Rational`] using a denominator no greater than a
    /// specified maximum, mutating the [`Rational`] in place.
    ///
    /// See [`Rational::approximate`] for more information.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// - If `max_denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::arithmetic::traits::ApproximateAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::exact_from(std::f64::consts::PI);
    /// x.approximate_assign(&Natural::from(1000u32));
    /// assert_eq!(x.to_string(), "355/113");
    ///
    /// let mut x = Rational::from_signeds(333i32, 1000);
    /// x.approximate_assign(&Natural::from(100u32));
    /// assert_eq!(x.to_string(), "1/3");
    /// ```
    fn approximate_assign(&mut self, max_denominator: &Natural) {
        assert_ne!(*max_denominator, 0);
        if self.denominator_ref() <= max_denominator {
        } else if *max_denominator == 1u32 {
            *self = Rational::from(Integer::rounding_from(&*self, Nearest).0);
        } else {
            *self = approximate_helper(&*self, max_denominator);
        }
    }
}
