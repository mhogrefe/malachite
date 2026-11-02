// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Computes the derivative of a polynomial, $\sum_i ia_ix^{i-1}$.
pub trait Derivative {
    type Output;

    /// Computes the derivative of a polynomial.
    ///
    /// $$
    /// f(p) = p'.
    /// $$
    fn derivative(self) -> Self::Output;
}

/// Replaces a polynomial with its derivative, $\sum_i ia_ix^{i-1}$.
pub trait DerivativeAssign {
    /// Replaces a polynomial with its derivative.
    ///
    /// $$
    /// p \gets p'.
    /// $$
    fn derivative_assign(&mut self);
}

/// Computes the integral of a polynomial whose constant term is zero, $\sum_i a_ix^{i+1}/(i+1)$.
pub trait Integral {
    type Output;

    /// Computes the integral of a polynomial whose constant term is zero.
    ///
    /// $$
    /// f(p) = \int_0^x p(t)\,dt.
    /// $$
    fn integral(self) -> Self::Output;
}

/// Replaces a polynomial with its integral whose constant term is zero, $\sum_i a_ix^{i+1}/(i+1)$.
pub trait IntegralAssign {
    /// Replaces a polynomial with its integral whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt.
    /// $$
    fn integral_assign(&mut self);
}

/// Computes the derivative of a polynomial modulo $m$. The coefficients must already be reduced
/// modulo $m$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod m$, which can be zero even when $a_i$ is not, so the
/// derivative can lose any number of degrees.
pub trait ModDerivative<M> {
    type Output;

    /// Computes the derivative of a polynomial modulo $m$.
    ///
    /// $$
    /// f(p, m) = p' \bmod m.
    /// $$
    fn mod_derivative(self, m: M) -> Self::Output;
}

/// Replaces a polynomial with its derivative modulo $m$. The coefficients must already be reduced
/// modulo $m$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod m$, which can be zero even when $a_i$ is not, so the
/// derivative can lose any number of degrees.
pub trait ModDerivativeAssign<M> {
    /// Replaces a polynomial with its derivative modulo $m$.
    ///
    /// $$
    /// p \gets p' \bmod m.
    /// $$
    fn mod_derivative_assign(&mut self, m: M);
}

/// Computes the integral modulo $m$ of a polynomial whose constant term is zero. The coefficients
/// must already be reduced modulo $m$.
///
/// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which that
/// coefficient is nonzero must be a unit modulo $m$.
pub trait ModIntegral<M> {
    type Output;

    /// Computes the integral modulo $m$ of a polynomial whose constant term is zero.
    ///
    /// $$
    /// f(p, m) = \int_0^x p(t)\,dt \bmod m.
    /// $$
    fn mod_integral(self, m: M) -> Self::Output;
}

/// Replaces a polynomial with its integral modulo $m$ whose constant term is zero. The coefficients
/// must already be reduced modulo $m$.
///
/// The coefficient of $x^{k-1}$ is divided by $k$ and moved to $x^k$, so every $k$ for which that
/// coefficient is nonzero must be a unit modulo $m$.
pub trait ModIntegralAssign<M> {
    /// Replaces a polynomial with its integral modulo $m$ whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod m.
    /// $$
    fn mod_integral_assign(&mut self, m: M);
}

/// Computes the derivative of a polynomial modulo $2^k$. The coefficients must already be reduced
/// modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod 2^k$, which can be zero even when $a_i$ is not, so
/// the derivative can lose any number of degrees.
pub trait ModPowerOf2Derivative {
    type Output;

    /// Computes the derivative of a polynomial modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p' \bmod 2^k.
    /// $$
    fn mod_power_of_2_derivative(self, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its derivative modulo $2^k$. The coefficients must already be reduced
/// modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $ia_i \bmod 2^k$, which can be zero even when $a_i$ is not, so
/// the derivative can lose any number of degrees.
pub trait ModPowerOf2DerivativeAssign {
    /// Replaces a polynomial with its derivative modulo $2^k$.
    ///
    /// $$
    /// p \gets p' \bmod 2^k.
    /// $$
    fn mod_power_of_2_derivative_assign(&mut self, pow: u64);
}

/// Computes the integral modulo $2^k$ of a polynomial whose constant term is zero. The coefficients
/// must already be reduced modulo $2^k$.
///
/// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are units
/// modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$.
pub trait ModPowerOf2Integral {
    type Output;

    /// Computes the integral modulo $2^k$ of a polynomial whose constant term is zero.
    ///
    /// $$
    /// f(p, k) = \int_0^x p(t)\,dt \bmod 2^k.
    /// $$
    fn mod_power_of_2_integral(self, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its integral modulo $2^k$ whose constant term is zero. The
/// coefficients must already be reduced modulo $2^k$.
///
/// The coefficient of $x^{i-1}$ is divided by $i$ and moved to $x^i$. Only odd numbers are units
/// modulo $2^k$, so every nonzero coefficient must belong to an even power of $x$.
pub trait ModPowerOf2IntegralAssign {
    /// Replaces a polynomial with its integral modulo $2^k$ whose constant term is zero.
    ///
    /// $$
    /// p \gets \int_0^x p(t)\,dt \bmod 2^k.
    /// $$
    fn mod_power_of_2_integral_assign(&mut self, pow: u64);
}

/// Computes the $n$th derivative of a polynomial, $\sum_i i^{\underline n}a_ix^{i-n}$, where
/// $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial.
pub trait NthDerivative {
    type Output;

    /// Computes the $n$th derivative of a polynomial.
    ///
    /// $$
    /// f(p, n) = p^{(n)}.
    /// $$
    fn nth_derivative(self, n: u64) -> Self::Output;
}

/// Replaces a polynomial with its $n$th derivative, $\sum_i i^{\underline n}a_ix^{i-n}$, where
/// $i^{\underline n} = i(i-1)\cdots(i-n+1)$ is a falling factorial.
pub trait NthDerivativeAssign {
    /// Replaces a polynomial with its $n$th derivative.
    ///
    /// $$
    /// p \gets p^{(n)}.
    /// $$
    fn nth_derivative_assign(&mut self, n: u64);
}

/// Computes the $n$th derivative of a polynomial modulo $m$. The coefficients must already be
/// reduced modulo $m$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod m$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $m$ divides $n!$, the
/// result is zero.
pub trait ModNthDerivative<M> {
    type Output;

    /// Computes the $n$th derivative of a polynomial modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = p^{(n)} \bmod m.
    /// $$
    fn mod_nth_derivative(self, n: u64, m: M) -> Self::Output;
}

/// Replaces a polynomial with its $n$th derivative modulo $m$. The coefficients must already be
/// reduced modulo $m$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod m$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $m$ divides $n!$, the
/// result is zero.
pub trait ModNthDerivativeAssign<M> {
    /// Replaces a polynomial with its $n$th derivative modulo $m$.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod m.
    /// $$
    fn mod_nth_derivative_assign(&mut self, n: u64, m: M);
}

/// Computes the $n$th derivative of a polynomial modulo $2^k$. The coefficients must already be
/// reduced modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod 2^k$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $2^k$ divides $n!$, the
/// result is zero.
pub trait ModPowerOf2NthDerivative {
    type Output;

    /// Computes the $n$th derivative of a polynomial modulo $2^k$.
    ///
    /// $$
    /// f(p, n, k) = p^{(n)} \bmod 2^k.
    /// $$
    fn mod_power_of_2_nth_derivative(self, n: u64, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its $n$th derivative modulo $2^k$. The coefficients must already be
/// reduced modulo $2^k$.
///
/// Each coefficient $a_i$ becomes $i^{\underline n}a_i \bmod 2^k$, where $i^{\underline n}$ is a
/// falling factorial; this can be zero even when $a_i$ is not, so the derivative can lose any
/// number of degrees. Every $i^{\underline n}$ is a multiple of $n!$, so if $2^k$ divides $n!$, the
/// result is zero.
pub trait ModPowerOf2NthDerivativeAssign {
    /// Replaces a polynomial with its $n$th derivative modulo $2^k$.
    ///
    /// $$
    /// p \gets p^{(n)} \bmod 2^k.
    /// $$
    fn mod_power_of_2_nth_derivative_assign(&mut self, n: u64, pow: u64);
}
