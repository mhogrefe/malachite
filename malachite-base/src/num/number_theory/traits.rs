// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

pub trait BinomialCoefficient<T = Self> {
    fn binomial_coefficient(n: T, k: T) -> Self;
}

pub trait CheckedBinomialCoefficient<T = Self>: Sized {
    fn checked_binomial_coefficient(n: T, k: T) -> Option<Self>;
}

/// Determines whether two numbers are coprime.
pub trait CoprimeWith<RHS = Self> {
    fn coprime_with(self, other: RHS) -> bool;
}

/// Computes the GCD (greatest common divisor) of two numbers $a$ and $b$, and also the coefficients
/// $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$.
///
/// The are infinitely many $x$, $y$ that satisfy the identity, so the full specification is more
/// detailed:
///
/// - $f(0, 0) = (0, 0, 0)$.
/// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
/// - $f(a, ak) = (-a, -1, 0)$ if $a < 0$ and $k \neq 1$.
/// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
/// - $f(bk, b) = (-b, 0, -1)$ if $b < 0$.
/// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(|a|, |b|)$, where
///   $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq \lfloor a/g
///   \rfloor$.
pub trait ExtendedGcd<RHS = Self> {
    type Gcd;
    type Cofactor;

    fn extended_gcd(self, other: RHS) -> (Self::Gcd, Self::Cofactor, Self::Cofactor);
}

/// Computes the $n$th Bell number: the number of ways to partition a set of $n$ elements.
pub trait BellNumber {
    fn bell_number(n: u64) -> Self;
}

/// Computes the $n$th Bell number, returning `None` if the result is too large to be represented.
pub trait CheckedBellNumber: Sized {
    fn checked_bell_number(n: u64) -> Option<Self>;
}

/// Computes the Stirling number of the first kind $s(n, k) = (-1)^{n + k} \left[{n \atop
/// k}\right]$, the coefficient of $x^k$ in the falling factorial $x(x - 1) \cdots (x - n + 1)$.
pub trait StirlingNumberOfTheFirstKind {
    fn stirling_number_of_the_first_kind(n: u64, k: u64) -> Self;
}

/// Computes the Stirling number of the first kind $s(n, k)$, returning `None` if the result is too
/// large to be represented.
pub trait CheckedStirlingNumberOfTheFirstKind: Sized {
    fn checked_stirling_number_of_the_first_kind(n: u64, k: u64) -> Option<Self>;
}

/// Computes the unsigned Stirling number of the first kind $\left[{n \atop k}\right]$: the number
/// of permutations of $n$ elements with $k$ cycles.
pub trait UnsignedStirlingNumberOfTheFirstKind {
    fn unsigned_stirling_number_of_the_first_kind(n: u64, k: u64) -> Self;
}

/// Computes the unsigned Stirling number of the first kind $\left[{n \atop k}\right]$, returning
/// `None` if the result is too large to be represented.
pub trait CheckedUnsignedStirlingNumberOfTheFirstKind: Sized {
    fn checked_unsigned_stirling_number_of_the_first_kind(n: u64, k: u64) -> Option<Self>;
}

/// Computes the Stirling number of the second kind $S(n, k)$: the number of ways to partition a set
/// of $n$ elements into $k$ nonempty subsets.
pub trait StirlingNumberOfTheSecondKind {
    fn stirling_number_of_the_second_kind(n: u64, k: u64) -> Self;
}

/// Computes the Stirling number of the second kind $S(n, k)$, returning `None` if the result is too
/// large to be represented.
pub trait CheckedStirlingNumberOfTheSecondKind: Sized {
    fn checked_stirling_number_of_the_second_kind(n: u64, k: u64) -> Option<Self>;
}

/// Computes the factorial of a `u64`.
pub trait Factorial {
    fn factorial(n: u64) -> Self;
}

/// Computes the factorial of a `u64`, returning `None` if the result is too large to be
/// represented.
pub trait CheckedFactorial: Sized {
    fn checked_factorial(n: u64) -> Option<Self>;
}

/// Computes the double factorial of a `u64`. The double factorial of a non-negative integer is the
/// product of all the positive integers that are less than or equal to it and have the same parity
/// as it.
pub trait DoubleFactorial {
    fn double_factorial(n: u64) -> Self;
}

/// Computes the double factorial of a `u64`, returning `None` if the result is too large to be
/// represented. The double factorial of a non-negative integer is the product of all the positive
/// integers that are less than or equal to it and have the same parity as it.
pub trait CheckedDoubleFactorial: Sized {
    fn checked_double_factorial(n: u64) -> Option<Self>;
}

/// Computes the $m$-multifactorial of a `u64`. The $m$-multifactorial of a non-negative integer $n$
/// is the product of all integers $k$ such that $0<k\leq n$ and $k\equiv n \pmod m$.
pub trait Multifactorial {
    fn multifactorial(n: u64, m: u64) -> Self;
}

/// Computes the $m$-multifactorial of a `u64`, returning `None` if the result is too large to be
/// represented. The $m$-multifactorial of a non-negative integer $n$ is the product of all integers
/// $k$ such that $0<k\leq n$ and $k\equiv n \pmod m$.
pub trait CheckedMultifactorial: Sized {
    fn checked_multifactorial(n: u64, m: u64) -> Option<Self>;
}

/// Computes the subfactorial of a `u64`. The subfactorial of a non-negative integer $n$ counts the
/// number of derangements of $n$ elements, which are the permutations in which no element is fixed.
pub trait Subfactorial {
    fn subfactorial(n: u64) -> Self;
}

/// Computes the subfactorial of a `u64`, returning `None` if the result is too large to be
/// represented. The subfactorial of a non-negative integer $n$ counts the number of derangements of
/// $n$ elements, which are the permutations in which no element is fixed.
pub trait CheckedSubfactorial: Sized {
    fn checked_subfactorial(n: u64) -> Option<Self>;
}

/// Computes the rising factorial of a number: the product of the `n` consecutive numbers starting
/// at `self`, or 1 when `n` is 0.
pub trait RisingFactorial {
    type Output;

    fn rising_factorial(self, n: u64) -> Self::Output;
}

/// Computes the rising factorial of a number, returning `None` if the result cannot be represented.
pub trait CheckedRisingFactorial: Sized {
    fn checked_rising_factorial(self, n: u64) -> Option<Self>;
}

/// Computes the falling factorial of a number: the product of the `n` consecutive numbers counting
/// down from `self`, or 1 when `n` is 0.
pub trait FallingFactorial {
    type Output;

    fn falling_factorial(self, n: u64) -> Self::Output;
}

/// Computes the falling factorial of a number, returning `None` if the result cannot be
/// represented.
pub trait CheckedFallingFactorial: Sized {
    fn checked_falling_factorial(self, n: u64) -> Option<Self>;
}

/// Computes the $n$th Fibonacci number, either alone or paired with its predecessor:
/// `fibonacci_pair(n)` returns $(F(n), F(n-1))$.
pub trait Fibonacci: Sized {
    fn fibonacci(n: u64) -> Self;

    fn fibonacci_pair(n: u64) -> (Self, Self);
}

/// Computes the $n$th Fibonacci number, either alone or paired with its predecessor, returning
/// `None` if the result is too large to be represented.
pub trait CheckedFibonacci: Sized {
    fn checked_fibonacci(n: u64) -> Option<Self>;

    fn checked_fibonacci_pair(n: u64) -> Option<(Self, Self)>;
}

/// Calculates the GCD (greatest common divisor) of two numbers.
pub trait Gcd<RHS = Self> {
    type Output;

    fn gcd(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the GCD (greatest common divisor) of it and another number.
pub trait GcdAssign<RHS = Self> {
    fn gcd_assign(&mut self, other: RHS);
}

/// Calculates the LCM (least common multiple) of two numbers.
pub trait Lcm<RHS = Self> {
    type Output;

    fn lcm(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the LCM (least common multiple) of it and another number.
pub trait LcmAssign<RHS = Self> {
    fn lcm_assign(&mut self, other: RHS);
}

/// Calculates the LCM (least common multiple) of two numbers, returning `None` if the result is not
/// representable.
pub trait CheckedLcm<RHS = Self> {
    type Output;

    fn checked_lcm(self, other: RHS) -> Option<Self::Output>;
}

/// Calculates the Legendre symbol of two numbers. Typically the implementations will be identical
/// to those of [`JacobiSymbol`].
pub trait LegendreSymbol<RHS = Self> {
    fn legendre_symbol(self, other: RHS) -> i8;
}

/// Calculates the Jacobi symbol of two numbers.
pub trait JacobiSymbol<RHS = Self> {
    fn jacobi_symbol(self, other: RHS) -> i8;
}

/// Calculates the Kronecker symbol of two numbers.
pub trait KroneckerSymbol<RHS = Self> {
    fn kronecker_symbol(self, other: RHS) -> i8;
}

pub trait Primorial {
    fn primorial(n: u64) -> Self;

    fn product_of_first_n_primes(n: u64) -> Self;
}

pub trait CheckedPrimorial: Sized {
    fn checked_primorial(n: u64) -> Option<Self>;

    fn checked_product_of_first_n_primes(n: u64) -> Option<Self>;
}

/// Computes the $n$th Lucas number, either alone or paired with its predecessor:
/// `lucas_number_pair(n)` returns $(L(n), L(n-1))$.
pub trait LucasNumber: Sized {
    fn lucas_number(n: u64) -> Self;

    fn lucas_number_pair(n: u64) -> (Self, Self);
}

/// Computes the $n$th Lucas number, either alone or paired with its predecessor, returning `None`
/// if the result is too large to be represented.
pub trait CheckedLucasNumber: Sized {
    fn checked_lucas_number(n: u64) -> Option<Self>;

    fn checked_lucas_number_pair(n: u64) -> Option<(Self, Self)>;
}
