// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{BinomialCoefficient, Parity};
use malachite_base::num::basic::traits::One;

impl BinomialCoefficient for Integer {
    /// Computes the binomial coefficient of two [`Integer`]s, taking both by value.
    ///
    /// The second argument must be non-negative, but the first may be negative. If it is, the
    /// identity $\binom{-n}{k} = (-1)^k \binom{n+k-1}{k}$ is used.
    ///
    /// $$
    /// f(n, k) = \\begin{cases}
    ///     \binom{n}{k} & \text{if} \\quad n \geq 0, \\\\
    ///     (-1)^k \binom{-n+k-1}{k} & \text{if} \\quad n < 0.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if $k$ is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(4), Integer::from(0)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(4), Integer::from(1)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(4), Integer::from(2)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(4), Integer::from(3)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(4), Integer::from(4)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(10), Integer::from(5)),
    ///     252
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(100), Integer::from(50)).to_string(),
    ///     "100891344545564193334812497256"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(-3), Integer::from(0)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(-3), Integer::from(1)),
    ///     -3
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(-3), Integer::from(2)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(Integer::from(-3), Integer::from(3)),
    ///     -10
    /// );
    /// ```
    fn binomial_coefficient(n: Integer, k: Integer) -> Integer {
        assert!(k.sign);
        if n.sign {
            Integer::from(Natural::binomial_coefficient(n.abs, k.abs))
        } else {
            let k_abs = k.abs;
            Integer {
                sign: k_abs.even(),
                abs: Natural::binomial_coefficient(n.abs + &k_abs - Natural::ONE, k_abs),
            }
        }
    }
}

impl<'a> BinomialCoefficient<&'a Integer> for Integer {
    /// Computes the binomial coefficient of two [`Integer`]s, taking both by reference.
    ///
    /// The second argument must be non-negative, but the first may be negative. If it is, the
    /// identity $\binom{-n}{k} = (-1)^k \binom{n+k-1}{k}$ is used.
    ///
    /// $$
    /// f(n, k) = \\begin{cases}
    ///     \binom{n}{k} & \text{if} \\quad n \geq 0, \\\\
    ///     (-1)^k \binom{-n+k-1}{k} & \text{if} \\quad n < 0.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if $k$ is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BinomialCoefficient;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(4), &Integer::from(0)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(4), &Integer::from(1)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(4), &Integer::from(2)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(4), &Integer::from(3)),
    ///     4
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(4), &Integer::from(4)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(10), &Integer::from(5)),
    ///     252
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(100), &Integer::from(50)).to_string(),
    ///     "100891344545564193334812497256"
    /// );
    ///
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(-3), &Integer::from(0)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(-3), &Integer::from(1)),
    ///     -3
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(-3), &Integer::from(2)),
    ///     6
    /// );
    /// assert_eq!(
    ///     Integer::binomial_coefficient(&Integer::from(-3), &Integer::from(3)),
    ///     -10
    /// );
    /// ```
    fn binomial_coefficient(n: &'a Integer, k: &'a Integer) -> Integer {
        assert!(k.sign);
        if n.sign {
            Integer::from(Natural::binomial_coefficient(&n.abs, &k.abs))
        } else {
            let k_abs = &k.abs;
            Integer {
                sign: k_abs.even(),
                abs: Natural::binomial_coefficient(&(&n.abs + k_abs - Natural::ONE), k_abs),
            }
        }
    }
}
