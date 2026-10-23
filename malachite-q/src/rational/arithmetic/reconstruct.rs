// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2020 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::reconstruct::{
    rational_reconstruct_with_bounds, rational_reconstruction_balanced_bound,
};

impl Rational {
    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, subject to bounds on the
    /// numerator and denominator. The residue and modulus are taken by value and the bounds by
    /// reference.
    ///
    /// $f(a, m, N, D) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq D$, and
    /// $\gcd(n, d) = 1$.
    ///
    /// Whenever $2ND < m$, at most one fraction satisfies the constraints, and it is returned if it
    /// exists; `None` means no such fraction exists. If $2ND \geq m$ the fraction is no longer
    /// unique. In either case the function examines a single candidate, the continued-fraction
    /// approximant of $a/m$ whose remainder is the first to reach $N$ or below, and returns it
    /// exactly when it satisfies the constraints.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `a` is greater than or equal to `m`, or if `n_bound` or `d_bound` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds(
    ///         Natural::from(33u32),
    ///         Natural::from(97u32),
    ///         &Natural::from(6u32),
    ///         &Natural::from(6u32),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "2/3"
    /// );
    /// // 1/25 has residue 444 modulo 1009; denominator-heavy bounds find it...
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds(
    ///         Natural::from(444u32),
    ///         Natural::from(1009u32),
    ///         &Natural::ONE,
    ///         &Natural::from(30u32),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "1/25"
    /// );
    /// // ...but numerator-heavy bounds do not
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds(
    ///         Natural::from(444u32),
    ///         Natural::from(1009u32),
    ///         &Natural::from(30u32),
    ///         &Natural::ONE,
    ///     ),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0.
    #[inline]
    pub fn reconstruct_with_bounds(
        a: Natural,
        m: Natural,
        n_bound: &Natural,
        d_bound: &Natural,
    ) -> Option<Self> {
        rational_reconstruct_with_bounds(a, m, n_bound, d_bound).map(
            |(sign, numerator, denominator)| Self {
                sign,
                numerator,
                denominator,
            },
        )
    }

    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, subject to bounds on the
    /// numerator and denominator. All four [`Natural`]s are taken by reference.
    ///
    /// $f(a, m, N, D) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq D$, and
    /// $\gcd(n, d) = 1$.
    ///
    /// Whenever $2ND < m$, at most one fraction satisfies the constraints, and it is returned if it
    /// exists; `None` means no such fraction exists. If $2ND \geq m$ the fraction is no longer
    /// unique. In either case the function examines a single candidate, the continued-fraction
    /// approximant of $a/m$ whose remainder is the first to reach $N$ or below, and returns it
    /// exactly when it satisfies the constraints.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `a` is greater than or equal to `m`, or if `n_bound` or `d_bound` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds_ref(
    ///         &Natural::from(33u32),
    ///         &Natural::from(97u32),
    ///         &Natural::from(6u32),
    ///         &Natural::from(6u32),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "2/3"
    /// );
    /// // no fraction with numerator and denominator at most 6 has residue 44 modulo 97
    /// assert_eq!(
    ///     Rational::reconstruct_with_bounds_ref(
    ///         &Natural::from(44u32),
    ///         &Natural::from(97u32),
    ///         &Natural::from(6u32),
    ///         &Natural::from(6u32),
    ///     ),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz_2 from fmpq/reconstruct_fmpz_2.c, FLINT 3.6.0, where all
    /// inputs are taken by reference.
    #[inline]
    pub fn reconstruct_with_bounds_ref(
        a: &Natural,
        m: &Natural,
        n_bound: &Natural,
        d_bound: &Natural,
    ) -> Option<Self> {
        rational_reconstruct_with_bounds(a.clone(), m.clone(), n_bound, d_bound).map(
            |(sign, numerator, denominator)| Self {
                sign,
                numerator,
                denominator,
            },
        )
    }

    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, using balanced bounds on the
    /// numerator and denominator. Both [`Natural`]s are taken by value.
    ///
    /// $f(a, m) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq N$, and $\gcd(n, d)
    /// = 1$, with $N = \lfloor\sqrt{(m-1)/2}\rfloor$.
    ///
    /// The balanced bounds are the largest symmetric bounds satisfying $2N^2 < m$, so at most one
    /// fraction satisfies the constraints, and it is returned if it exists; `None` means no such
    /// fraction exists. To reconstruct fractions whose numerator and denominator have different
    /// sizes, use [`reconstruct_with_bounds`](Rational::reconstruct_with_bounds).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `m` is less than 3 or if `a` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct(Natural::from(33u32), Natural::from(97u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2/3"
    /// );
    /// // 1/25 has residue 444 modulo 1009, but the balanced bounds cannot see it: its
    /// // denominator outweighs its numerator
    /// assert_eq!(
    ///     Rational::reconstruct(Natural::from(444u32), Natural::from(1009u32)),
    ///     None
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz from fmpq/reconstruct_fmpz.c, FLINT 3.6.0.
    pub fn reconstruct(a: Natural, m: Natural) -> Option<Self> {
        assert!(m > 2u32, "m must be greater than 2");
        let b = rational_reconstruction_balanced_bound(&m);
        rational_reconstruct_with_bounds(a, m, &b, &b).map(|(sign, numerator, denominator)| Self {
            sign,
            numerator,
            denominator,
        })
    }

    /// Reconstructs a [`Rational`] from its residue $a$ modulo $m$, using balanced bounds on the
    /// numerator and denominator. Both [`Natural`]s are taken by reference.
    ///
    /// $f(a, m) = n/d$, where $n \equiv ad \pmod{m}$, $|n| \leq N$, $0 < d \leq N$, and $\gcd(n, d)
    /// = 1$, with $N = \lfloor\sqrt{(m-1)/2}\rfloor$.
    ///
    /// The balanced bounds are the largest symmetric bounds satisfying $2N^2 < m$, so at most one
    /// fraction satisfies the constraints, and it is returned if it exists; `None` means no such
    /// fraction exists. To reconstruct fractions whose numerator and denominator have different
    /// sizes, use [`reconstruct_with_bounds_ref`](Rational::reconstruct_with_bounds_ref).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`: the
    /// size-dispatched kernels culminate in a subquadratic half-gcd splitter, giving the gcd-class
    /// bound.
    ///
    /// # Panics
    /// Panics if `m` is less than 3 or if `a` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// // 2/3 has residue 33 modulo 97, since 33 * 3 = 99 ≡ 2 mod 97
    /// assert_eq!(
    ///     Rational::reconstruct_ref(&Natural::from(33u32), &Natural::from(97u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "2/3"
    /// );
    /// // 22/7 has residue 8818342134038800723104056361 modulo 12345678987654321012345678901
    /// assert_eq!(
    ///     Rational::reconstruct_ref(
    ///         &Natural::from_str("8818342134038800723104056361").unwrap(),
    ///         &Natural::from_str("12345678987654321012345678901").unwrap(),
    ///     )
    ///     .unwrap()
    ///     .to_string(),
    ///     "22/7"
    /// );
    /// ```
    ///
    /// This is fmpq_reconstruct_fmpz from fmpq/reconstruct_fmpz.c, FLINT 3.6.0, where all inputs
    /// are taken by reference.
    pub fn reconstruct_ref(a: &Natural, m: &Natural) -> Option<Self> {
        assert!(*m > 2u32, "m must be greater than 2");
        let b = rational_reconstruction_balanced_bound(m);
        rational_reconstruct_with_bounds(a.clone(), m.clone(), &b, &b).map(
            |(sign, numerator, denominator)| Self {
                sign,
                numerator,
                denominator,
            },
        )
    }
}
