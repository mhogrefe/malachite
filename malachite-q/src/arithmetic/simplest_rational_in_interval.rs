// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::traits::SimplestRationalInInterval;
use crate::conversion::continued_fraction::to_continued_fraction::RationalContinuedFraction;
use crate::conversion::traits::ContinuedFraction;
use crate::Rational;
use core::cmp::{
    max, min,
    Ordering::{self, *},
};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{AddMul, Ceiling, Floor, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::IsInteger;
use malachite_nz::natural::Natural;

fn min_helper_oo<'a>(ox: &'a Option<Natural>, oy: &'a Option<Natural>) -> &'a Natural {
    if let Some(x) = ox.as_ref() {
        if let Some(y) = oy.as_ref() {
            min(x, y)
        } else {
            x
        }
    } else {
        oy.as_ref().unwrap()
    }
}

fn min_helper_xo<'a>(x: &'a Natural, oy: &'a Option<Natural>) -> &'a Natural {
    if let Some(y) = oy.as_ref() {
        min(x, y)
    } else {
        x
    }
}

fn simplest_rational_one_alt_helper(
    x: &Natural,
    oy_n: &Option<Natural>,
    mut cf_y: RationalContinuedFraction,
    numerator: &Natural,
    denominator: &Natural,
    previous_numerator: &Natural,
    previous_denominator: &Natural,
) -> Rational {
    // use [a_0; a_1, ... a_k - 1, 1] and [b_0; b_1, ... b_k]
    let (n, d) = if oy_n.is_some() && x - Natural::ONE == *oy_n.as_ref().unwrap() {
        let next_numerator = previous_numerator.add_mul(numerator, oy_n.as_ref().unwrap());
        let next_denominator = previous_denominator.add_mul(denominator, oy_n.as_ref().unwrap());
        let next_oy_n = cf_y.next();
        if next_oy_n == Some(Natural::ONE) {
            let next_next_numerator = numerator + &next_numerator;
            let next_next_denominator = denominator + &next_denominator;
            // since y_n = 1, cf_y is not exhausted yet
            let y_n = cf_y.next().unwrap() + Natural::ONE;
            (
                next_numerator.add_mul(next_next_numerator, &y_n),
                next_denominator.add_mul(next_next_denominator, y_n),
            )
        } else {
            (
                numerator + (next_numerator << 1),
                denominator + (next_denominator << 1),
            )
        }
    } else {
        let ox_n_m_1 = x - Natural::ONE;
        let m = min_helper_xo(&ox_n_m_1, oy_n);
        let next_numerator = previous_numerator.add_mul(numerator, m);
        let next_denominator = previous_denominator.add_mul(denominator, m);
        (
            numerator + (next_numerator << 1),
            denominator + (next_denominator << 1),
        )
    };
    Rational {
        sign: true,
        numerator: n,
        denominator: d,
    }
}

fn update_best(best: &mut Option<Rational>, x: &Rational, y: &Rational, candidate: Rational) {
    if best.is_none() && candidate > *x && candidate < *y {
        *best = Some(candidate);
    }
}

impl Rational {
    /// Compares two [`Rational`]s according to their complexity.
    ///
    /// Complexity is defined as follows: If two [`Rational`]s have different denominators, then the
    /// one with the larger denominator is more complex. If they have the same denominator, then the
    /// one whose numerator is further from zero is more complex. Finally, if $q > 0$, then $q$ is
    /// simpler than $-q$.
    ///
    /// The [`Rational`]s ordered by complexity look like this:
    /// $$
    /// 0, 1, -1, 2, -2, \ldots, 1/2, -1/2, 3/2, -3/2, \ldots, 1/3, -1/3, 2/3, -2/3, \ldots, \ldots.
    /// $$
    /// This order is a well-order, and the order type of the [`Rational`]s under this order is
    /// $\omega^2$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(1, 2).cmp_complexity(&Rational::from_signeds(1, 3)),
    ///     Less
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(1, 2).cmp_complexity(&Rational::from_signeds(3, 2)),
    ///     Less
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(1, 2).cmp_complexity(&Rational::from_signeds(-1, 2)),
    ///     Less
    /// );
    /// ```
    pub fn cmp_complexity(&self, other: &Rational) -> Ordering {
        self.denominator_ref()
            .cmp(other.denominator_ref())
            .then_with(|| self.numerator_ref().cmp(other.numerator_ref()))
            .then_with(|| (*self < 0u32).cmp(&(*other < 0u32)))
    }
}

impl SimplestRationalInInterval for Rational {
    /// Finds the simplest [`Rational`] contained in an open interval.
    ///
    /// Let $f(x, y) = p/q$, with $p$ and $q$ relatively prime. Then the following properties hold:
    /// - $x < p/q < y$
    /// - If $x < m/n < y$, then $n \geq q$
    /// - If $x < m/q < y$, then $|p| \leq |m|$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if $x \geq y$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::arithmetic::traits::SimplestRationalInInterval;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::simplest_rational_in_open_interval(
    ///         &Rational::from_signeds(1, 3),
    ///         &Rational::from_signeds(1, 2)
    ///     ),
    ///     Rational::from_signeds(2, 5)
    /// );
    /// assert_eq!(
    ///     Rational::simplest_rational_in_open_interval(
    ///         &Rational::from_signeds(-1, 3),
    ///         &Rational::from_signeds(1, 3)
    ///     ),
    ///     Rational::ZERO
    /// );
    /// assert_eq!(
    ///     Rational::simplest_rational_in_open_interval(
    ///         &Rational::from_signeds(314, 100),
    ///         &Rational::from_signeds(315, 100)
    ///     ),
    ///     Rational::from_signeds(22, 7)
    /// );
    /// ```
    fn simplest_rational_in_open_interval(x: &Rational, y: &Rational) -> Rational {
        assert!(x < y);
        if *x < 0u32 && *y > 0u32 {
            return Rational::ZERO;
        }
        let neg_x;
        let neg_y;
        let (neg, x, y) = if *x < 0u32 {
            neg_x = -x;
            neg_y = -y;
            (true, &neg_y, &neg_x)
        } else {
            (false, x, y)
        };
        let (floor_x, mut cf_x) = x.continued_fraction();
        let floor_x = floor_x.unsigned_abs();
        let (floor_y, mut cf_y) = y.continued_fraction();
        let floor_y = floor_y.unsigned_abs();
        let mut best = None;
        if floor_x == floor_y {
            let floor = floor_x;
            let mut previous_numerator = Natural::ONE;
            let mut previous_denominator = Natural::ZERO;
            let mut numerator = floor;
            let mut denominator = Natural::ONE;
            let mut ox_n = cf_x.next();
            let mut oy_n = cf_y.next();
            while ox_n == oy_n {
                // They are both Some
                swap(&mut numerator, &mut previous_numerator);
                swap(&mut denominator, &mut previous_denominator);
                numerator = (&numerator).add_mul(&previous_numerator, &ox_n.unwrap());
                denominator = (&denominator).add_mul(&previous_denominator, &oy_n.unwrap());
                ox_n = cf_x.next();
                oy_n = cf_y.next();
            }
            // use [x_0; x_1, ... x_k] and [y_0; y_1, ... y_k]
            let m = min_helper_oo(&ox_n, &oy_n) + Natural::ONE;
            let n = (&previous_numerator).add_mul(&numerator, &m);
            let d = (&previous_denominator).add_mul(&denominator, &m);
            let candidate = Rational {
                sign: true,
                numerator: n,
                denominator: d,
            };
            update_best(&mut best, x, y, candidate);
            if let Some(x_n) = ox_n.as_ref() {
                if cf_x.is_done() {
                    update_best(
                        &mut best,
                        x,
                        y,
                        simplest_rational_one_alt_helper(
                            x_n,
                            &oy_n,
                            cf_y.clone(),
                            &numerator,
                            &denominator,
                            &previous_numerator,
                            &previous_denominator,
                        ),
                    );
                }
            }
            if let Some(y_n) = oy_n.as_ref() {
                if cf_y.is_done() {
                    update_best(
                        &mut best,
                        x,
                        y,
                        simplest_rational_one_alt_helper(
                            y_n,
                            &ox_n,
                            cf_x.clone(),
                            &numerator,
                            &denominator,
                            &previous_numerator,
                            &previous_denominator,
                        ),
                    );
                }
            }
            if ox_n.is_some() && oy_n.is_some() && cf_x.is_done() != cf_y.is_done() {
                if cf_y.is_done() {
                    swap(&mut ox_n, &mut oy_n);
                    swap(&mut cf_y, &mut cf_x);
                }
                let x_n = ox_n.unwrap();
                let y_n = oy_n.unwrap();
                if y_n == x_n - Natural::ONE {
                    let next_y_n = cf_y.next().unwrap();
                    let next_numerator = (&previous_numerator).add_mul(&numerator, &y_n);
                    let next_denominator = (&previous_denominator).add_mul(&denominator, &y_n);
                    let (n, d) = if cf_y.is_done() && next_y_n == 2u32 {
                        (
                            next_numerator * Natural::from(3u32) + (numerator << 1),
                            next_denominator * Natural::from(3u32) + (denominator << 1),
                        )
                    } else {
                        (
                            previous_numerator + (numerator << 1),
                            previous_denominator + (denominator << 1),
                        )
                    };
                    let candidate = Rational {
                        sign: true,
                        numerator: n,
                        denominator: d,
                    };
                    update_best(&mut best, x, y, candidate);
                }
            }
        } else {
            let candidate = if floor_y - Natural::ONE != floor_x || !cf_y.is_done() {
                Rational::from(floor_x + Natural::ONE)
            } else {
                let floor = floor_x;
                // [f; x_1, x_2, x_3...] and [f + 1]. But to get any good candidates, we need [f;
                // x_1, x_2, x_3...] and [f; 1]. If x_1 does not exist, the result is [f; 2].
                let (n, d) = if cf_x.is_done() {
                    ((floor << 1) | Natural::ONE, Natural::TWO)
                } else {
                    let x_1 = cf_x.next().unwrap();
                    if x_1 > Natural::ONE {
                        if x_1 == 2u32 && cf_x.is_done() {
                            // [f; 1, 1] and [f; 1], so [f; 1, 2] is a candidate.
                            (
                                floor * Natural::from(3u32) + Natural::TWO,
                                Natural::from(3u32),
                            )
                        } else {
                            // If x_1 > 1, we have [f; 2] as a candidate.
                            ((floor << 1) | Natural::ONE, Natural::TWO)
                        }
                    } else {
                        // x_2 exists since x_1 was 1
                        let x_2 = cf_x.next().unwrap();
                        // [f; 1, x_2] and [f; 1], so [f; 1, x_2 + 1] is a candidate. [f; 1, x_2 -
                        // 1, 1] and [f; 1], but [f; 1, x_2] is not in the interval
                        let k = &x_2 + Natural::ONE;
                        (&floor * &k + floor + k, x_2 + Natural::TWO)
                    }
                };
                Rational {
                    sign: true,
                    numerator: n,
                    denominator: d,
                }
            };
            update_best(&mut best, x, y, candidate);
        }
        let best = best.unwrap();
        if neg {
            -best
        } else {
            best
        }
    }

    /// Finds the simplest [`Rational`] contained in a closed interval.
    ///
    /// Let $f(x, y) = p/q$, with $p$ and $q$ relatively prime. Then the following properties hold:
    /// - $x \leq p/q \leq y$
    /// - If $x \leq m/n \leq y$, then $n \geq q$
    /// - If $x \leq m/q \leq y$, then $|p| \leq |m|$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if $x > y$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::arithmetic::traits::SimplestRationalInInterval;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::simplest_rational_in_closed_interval(
    ///         &Rational::from_signeds(1, 3),
    ///         &Rational::from_signeds(1, 2)
    ///     ),
    ///     Rational::from_signeds(1, 2)
    /// );
    /// assert_eq!(
    ///     Rational::simplest_rational_in_closed_interval(
    ///         &Rational::from_signeds(-1, 3),
    ///         &Rational::from_signeds(1, 3)
    ///     ),
    ///     Rational::ZERO
    /// );
    /// assert_eq!(
    ///     Rational::simplest_rational_in_closed_interval(
    ///         &Rational::from_signeds(314, 100),
    ///         &Rational::from_signeds(315, 100)
    ///     ),
    ///     Rational::from_signeds(22, 7)
    /// );
    /// ```
    fn simplest_rational_in_closed_interval(x: &Rational, y: &Rational) -> Rational {
        assert!(x <= y);
        if x == y {
            return x.clone();
        } else if *x <= 0u32 && *y >= 0u32 {
            return Rational::ZERO;
        } else if x.is_integer() {
            return if y.is_integer() {
                if *x >= 0u32 {
                    min(x, y).clone()
                } else {
                    max(x, y).clone()
                }
            } else if *x >= 0u32 {
                x.clone()
            } else {
                Rational::from(y.floor())
            };
        } else if y.is_integer() {
            return if *x >= 0u32 {
                Rational::from(x.ceiling())
            } else {
                y.clone()
            };
        }
        let mut best = Rational::simplest_rational_in_open_interval(x, y);
        for q in [x, y] {
            if q.cmp_complexity(&best) == Less {
                best = q.clone();
            }
        }
        best
    }
}
