// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
use malachite_base::unsigned_polynomial::modular::mod_power_of_2_coprime_with::{
    coprime_mod_2, low_bits,
};

// Since 2 is nilpotent modulo $2^k$, polynomials are coprime modulo $2^k$ exactly when they are
// coprime modulo 2.
fn mod_power_of_2_coprime_with_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> bool {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
    pow == 0
        || coprime_mod_2(
            &low_bits(p.coefficients.iter().map(Parity::odd)),
            &low_bits(q.coefficients.iter().map(Parity::odd)),
        )
}

impl ModPowerOf2CoprimeWith<Self> for NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $2^k$, taking both by
    /// value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{2^k} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// A polynomial is coprime with 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $k$
    /// is 0, every polynomial is 0, and the result is `true`.
    ///
    /// Since 2 is nilpotent modulo $2^k$, polynomials are coprime modulo $2^k$ exactly when they
    /// are coprime modulo 2, so only the low bits of the coefficients are used: their GCD modulo 2
    /// is found by Euclid's algorithm or, for long polynomials, the half-GCD algorithm.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+x").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.mod_power_of_2_coprime_with(q, 3));
    ///
    /// let p = NaturalPolynomial::from_str("x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!p.mod_power_of_2_coprime_with(q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: Self, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(&self, &other, pow)
    }
}

impl ModPowerOf2CoprimeWith<&Self> for NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $2^k$, taking the first by
    /// value and the second by reference. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{2^k} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// A polynomial is coprime with 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $k$
    /// is 0, every polynomial is 0, and the result is `true`.
    ///
    /// Since 2 is nilpotent modulo $2^k$, polynomials are coprime modulo $2^k$ exactly when they
    /// are coprime modulo 2, so only the low bits of the coefficients are used: their GCD modulo 2
    /// is found by Euclid's algorithm or, for long polynomials, the half-GCD algorithm.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+x").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.mod_power_of_2_coprime_with(&q, 3));
    ///
    /// let p = NaturalPolynomial::from_str("x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!p.mod_power_of_2_coprime_with(&q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: &Self, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(&self, other, pow)
    }
}

impl ModPowerOf2CoprimeWith<NaturalPolynomial> for &NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $2^k$, taking the first by
    /// reference and the second by value. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{2^k} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// A polynomial is coprime with 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $k$
    /// is 0, every polynomial is 0, and the result is `true`.
    ///
    /// Since 2 is nilpotent modulo $2^k$, polynomials are coprime modulo $2^k$ exactly when they
    /// are coprime modulo 2, so only the low bits of the coefficients are used: their GCD modulo 2
    /// is found by Euclid's algorithm or, for long polynomials, the half-GCD algorithm.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+x").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_power_of_2_coprime_with(q, 3));
    ///
    /// let p = NaturalPolynomial::from_str("x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_power_of_2_coprime_with(q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: NaturalPolynomial, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(self, &other, pow)
    }
}

impl ModPowerOf2CoprimeWith<&NaturalPolynomial> for &NaturalPolynomial {
    /// Determines whether two [`NaturalPolynomial`]s are coprime modulo $2^k$, taking both by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \text{true} & \text{if } sa + tb \equiv 1 \pmod{2^k} \text{ for some } s, t, \\\\
    ///     \text{false} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// A polynomial is coprime with 0 exactly when it is a unit, as $1 + 2x$ is modulo 4. When $k$
    /// is 0, every polynomial is 0, and the result is `true`.
    ///
    /// Since 2 is nilpotent modulo $2^k$, polynomials are coprime modulo $2^k$ exactly when they
    /// are coprime modulo 2, so only the low bits of the coefficients are used: their GCD modulo 2
    /// is found by Euclid's algorithm or, for long polynomials, the half-GCD algorithm.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+x").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_power_of_2_coprime_with(&q, 3));
    ///
    /// let p = NaturalPolynomial::from_str("x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_power_of_2_coprime_with(&q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: &NaturalPolynomial, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(self, other, pow)
    }
}
