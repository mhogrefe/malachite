// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::modular::traits::ModPowerOf2CoprimeWith;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_coprime_with::coprime_or_non_invertible;
use crate::unsigned_polynomial::modular::mod_power_of_2_gcd::assert_reduced_mod_power_of_2;
use crate::unsigned_polynomial::modular::subproduct_tree::Modulus;
use alloc::vec::Vec;

/// The polynomial with the low bits of the coefficients `bits`, a polynomial modulo 2, without
/// zeros at the end.
///
/// This is not part of the public API; it is public so that `malachite-nz` can test coprimality of
/// `NaturalPolynomial`s modulo powers of 2.
#[doc(hidden)]
pub fn low_bits(bits: impl Iterator<Item = bool>) -> Vec<u8> {
    let mut xs: Vec<u8> = bits.map(u8::from).collect();
    while xs.last() == Some(&0) {
        xs.pop();
    }
    xs
}

/// Whether the polynomials with coefficients `xs` and `ys`, each 0 or 1 and neither ending with a
/// zero, are coprime modulo 2.
///
/// This is not part of the public API; it is public so that `malachite-nz` can test coprimality of
/// `NaturalPolynomial`s modulo powers of 2.
#[doc(hidden)]
#[inline]
pub fn coprime_mod_2(xs: &[u8], ys: &[u8]) -> bool {
    // 2 is prime, so every nonzero leading coefficient is invertible.
    coprime_or_non_invertible(xs, ys, &Modulus(2u8)).unwrap()
}

// Since 2 is nilpotent modulo $2^k$, polynomials are coprime modulo $2^k$ exactly when they are
// coprime modulo 2: if $sa + tb = 1 + 2h$, then $1 + 2h$ is invertible.
fn mod_power_of_2_coprime_with_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    assert_reduced_mod_power_of_2(p, q, pow);
    pow == 0
        || coprime_mod_2(
            &low_bits(p.coefficients.iter().map(|x| x.odd())),
            &low_bits(q.coefficients.iter().map(|x| x.odd())),
        )
}

impl<T: PrimitiveUnsigned> ModPowerOf2CoprimeWith<Self> for UnsignedPolynomial<T> {
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $2^k$, taking both by
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("4*x^2+x").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.mod_power_of_2_coprime_with(q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!p.mod_power_of_2_coprime_with(q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: Self, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CoprimeWith<&Self> for UnsignedPolynomial<T> {
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $2^k$, taking the first by
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("4*x^2+x").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.mod_power_of_2_coprime_with(&q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!p.mod_power_of_2_coprime_with(&q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: &Self, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CoprimeWith<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $2^k$, taking the first by
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("4*x^2+x").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_power_of_2_coprime_with(q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_power_of_2_coprime_with(q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: UnsignedPolynomial<T>, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CoprimeWith<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    /// Determines whether two [`UnsignedPolynomial`]s are coprime modulo $2^k$, taking both by
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2CoprimeWith;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("4*x^2+x").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).mod_power_of_2_coprime_with(&q, 3));
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// assert!(!(&p).mod_power_of_2_coprime_with(&q, 3));
    /// ```
    ///
    /// FLINT has no coprimality test for polynomials.
    #[inline]
    fn mod_power_of_2_coprime_with(self, other: &UnsignedPolynomial<T>, pow: u64) -> bool {
        mod_power_of_2_coprime_with_helper(self, other, pow)
    }
}
