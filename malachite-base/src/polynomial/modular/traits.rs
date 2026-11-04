// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;

/// Removes the largest power of a polynomial that divides another modulo $2^k$, returning the
/// reduced polynomial and how many times the factor was removed. The coefficients must already be
/// reduced modulo $2^k$, the factor must have degree at least 1, and its leading coefficient must
/// be odd.
pub trait ModPowerOf2RemovePower<Rhs = Self> {
    type Output;

    /// Removes the largest power of a polynomial that divides another modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, e), \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q.
    /// $$
    fn mod_power_of_2_remove_power(self, other: Rhs, pow: u64) -> (Self::Output, u64);
}

/// Replaces a polynomial with itself divided by the largest power of another that divides it modulo
/// $2^k$, returning how many times the factor was removed. The coefficients must already be reduced
/// modulo $2^k$, the factor must have degree at least 1, and its leading coefficient must be odd.
pub trait ModPowerOf2RemovePowerAssign<Rhs = Self> {
    /// Replaces `self` with itself divided by the largest power of `other` that divides it modulo
    /// $2^k$, returning the exponent of that power.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{2^k}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    fn mod_power_of_2_remove_power_assign(&mut self, other: Rhs, pow: u64) -> u64;
}

/// Divides a polynomial by $x - c$ modulo $2^k$, returning the quotient and the remainder, which is
/// the value of the polynomial at $c$. The coefficients and $c$ must already be reduced modulo
/// $2^k$.
pub trait ModPowerOf2DivRoot<C> {
    type Output;
    type RemOutput;

    /// Divides a polynomial by $x - c$ modulo $2^k$, returning the quotient and the remainder.
    ///
    /// $$
    /// f(a, c, k) = (q, r), \quad a \equiv (x - c)q + r \pmod{2^k}, \quad r = a(c).
    /// $$
    fn mod_power_of_2_div_root(self, c: C, pow: u64) -> (Self::Output, Self::RemOutput);
}

/// Divides a polynomial by $x - c$ modulo $2^k$ in place, replacing it with the quotient and
/// returning the remainder, which is the value of the polynomial at $c$. The coefficients and $c$
/// must already be reduced modulo $2^k$.
pub trait ModPowerOf2DivRootAssign<C> {
    type RemOutput;

    /// Replaces a polynomial with its quotient by $x - c$ modulo $2^k$, returning the remainder.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{2^k},
    /// $$
    /// returning $r = a(c)$.
    fn mod_power_of_2_div_root_assign(&mut self, c: C, pow: u64) -> Self::RemOutput;
}

/// Divides one polynomial by another modulo $2^k$, assuming that the second divides the first. The
/// coefficients must already be reduced modulo $2^k$, and the leading coefficient of the divisor
/// must be odd.
///
/// If the divisor does not divide the dividend, the result is meaningless.
pub trait ModPowerOf2DivExact<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another modulo $2^k$, assuming that the second divides the first.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    fn mod_power_of_2_div_exact(self, other: Rhs, pow: u64) -> Self::Output;
}

/// Divides one polynomial by another modulo $2^k$ in place, assuming that the second divides the
/// first. The coefficients must already be reduced modulo $2^k$, and the leading coefficient of the
/// divisor must be odd.
///
/// If the divisor does not divide the dividend, the result is meaningless.
pub trait ModPowerOf2DivExactAssign<Rhs = Self> {
    /// Replaces `self` with its quotient by another polynomial modulo $2^k$, assuming that the
    /// second divides the first.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    fn mod_power_of_2_div_exact_assign(&mut self, other: Rhs, pow: u64);
}

/// Divides one polynomial by another modulo $2^k$ if the second divides the first, returning `None`
/// otherwise. The coefficients must already be reduced modulo $2^k$, and the leading coefficient of
/// the divisor must be odd.
pub trait CheckedModPowerOf2DivExact<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another modulo $2^k$ if the second divides the first, or returns
    /// `None`.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    fn checked_mod_power_of_2_div_exact(self, other: Rhs, pow: u64) -> Option<Self::Output>;
}

/// Determines whether one polynomial is divisible by another modulo $2^k$. The coefficients must
/// already be reduced modulo $2^k$, and the leading coefficient of the divisor, unless it is 0,
/// must be odd.
pub trait ModPowerOf2DivisibleBy<Rhs = Self> {
    /// Determines whether a polynomial is divisible by another modulo $2^k$: whether there is a
    /// polynomial $q$ with $a \equiv bq \pmod{2^k}$.
    fn mod_power_of_2_divisible_by(self, other: Rhs, pow: u64) -> bool;
}

/// Divides one polynomial by another modulo $2^k$, returning the quotient and remainder. The
/// coefficients must already be reduced modulo $2^k$, and the leading coefficient of the divisor
/// must be odd.
///
/// The quotient and remainder are the unique polynomials $q$ and $r$ with coefficients modulo $2^k$
/// such that $a = bq + r$ and $\deg r < \deg b$.
pub trait ModPowerOf2DivRem<Rhs = Self> {
    type DivOutput;
    type RemOutput;

    /// Divides a polynomial by another modulo $2^k$, returning the quotient and remainder.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_div_rem(self, other: Rhs, pow: u64) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides one polynomial by another modulo $2^k$ in place, replacing the first with the quotient
/// and returning the remainder. The coefficients must already be reduced modulo $2^k$, and the
/// leading coefficient of the divisor must be odd.
pub trait ModPowerOf2DivAssignRem<Rhs = Self> {
    type RemOutput;

    /// Replaces `self` with its quotient by another polynomial modulo $2^k$, returning the
    /// remainder.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    fn mod_power_of_2_div_assign_rem(&mut self, other: Rhs, pow: u64) -> Self::RemOutput;
}

/// Divides one polynomial by another modulo $2^k$, returning the quotient and remainder, or `None`
/// if the leading coefficient of the divisor is not odd. The coefficients must already be reduced
/// modulo $2^k$.
pub trait CheckedModPowerOf2DivRem<Rhs = Self> {
    type DivOutput;
    type RemOutput;

    /// Divides a polynomial by another modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } \operatorname{lc}(b) \text{ is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a \equiv bq + r \pmod{2^k}$ and $\deg r < \deg b$.
    fn checked_mod_power_of_2_div_rem(
        self,
        other: Rhs,
        pow: u64,
    ) -> Option<(Self::DivOutput, Self::RemOutput)>;
}

/// Finds the quotient of one polynomial divided by another modulo $2^k$. The coefficients must
/// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
///
/// This is the quotient of [`ModPowerOf2DivRem`]; computing it alone can be faster.
pub trait ModPowerOf2Quotient<Rhs = Self> {
    type Output;

    /// Finds the quotient of a polynomial divided by another modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_quotient(self, other: Rhs, pow: u64) -> Self::Output;
}

/// Replaces one polynomial with its quotient by another modulo $2^k$. The coefficients must already
/// be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
pub trait ModPowerOf2QuotientAssign<Rhs = Self> {
    /// Replaces `self` with its quotient by another polynomial modulo $2^k$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_quotient_assign(&mut self, other: Rhs, pow: u64);
}

/// Finds the quotient of one polynomial divided by another modulo $2^k$, or `None` if the leading
/// coefficient of the divisor is not odd. The coefficients must already be reduced modulo $2^k$.
pub trait CheckedModPowerOf2Quotient<Rhs = Self> {
    type Output;

    /// Finds the quotient of a polynomial divided by another modulo $2^k$, or `None` if the
    /// divisor's leading coefficient is not odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } \operatorname{lc}(b) \text{ is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a \equiv bq + r \pmod{2^k}$ and $\deg r < \deg b$.
    fn checked_mod_power_of_2_quotient(self, other: Rhs, pow: u64) -> Option<Self::Output>;
}

/// Finds the remainder of one polynomial divided by another modulo $2^k$. The coefficients must
/// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
pub trait ModPowerOf2Rem<Rhs = Self> {
    type Output;

    /// Finds the remainder of a polynomial divided by another modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_rem(self, other: Rhs, pow: u64) -> Self::Output;
}

/// Replaces one polynomial with its remainder by another modulo $2^k$. The coefficients must
/// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
pub trait ModPowerOf2RemAssign<Rhs = Self> {
    /// Replaces `self` with its remainder by another polynomial modulo $2^k$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_rem_assign(&mut self, other: Rhs, pow: u64);
}

/// Finds the remainder of one polynomial divided by another modulo $2^k$, or `None` if the leading
/// coefficient of the divisor is not odd. The coefficients must already be reduced modulo $2^k$.
pub trait CheckedModPowerOf2Rem<Rhs = Self> {
    type Output;

    /// Finds the remainder of a polynomial divided by another modulo $2^k$, or `None` if the
    /// divisor's leading coefficient is not odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } \operatorname{lc}(b) \text{ is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a \equiv bq + r \pmod{2^k}$ and $\deg r < \deg b$.
    fn checked_mod_power_of_2_rem(self, other: Rhs, pow: u64) -> Option<Self::Output>;
}

/// Removes the largest power of a polynomial that divides another modulo $m$, returning the reduced
/// polynomial and how many times the factor was removed. The coefficients must already be reduced
/// modulo $m$, the factor must have degree at least 1, and its leading coefficient must be
/// invertible modulo $m$.
pub trait ModRemovePower<Rhs = Self, M = Self> {
    type Output;

    /// Removes the largest power of a polynomial that divides another modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, e), \quad a \equiv b^e q \pmod{m}, \quad b \nmid q.
    /// $$
    fn mod_remove_power(self, other: Rhs, m: M) -> (Self::Output, u64);
}

/// Replaces a polynomial with itself divided by the largest power of another that divides it modulo
/// $m$, returning how many times the factor was removed. The coefficients must already be reduced
/// modulo $m$, the factor must have degree at least 1, and its leading coefficient must be
/// invertible modulo $m$.
pub trait ModRemovePowerAssign<Rhs = Self, M = Self> {
    /// Replaces `self` with itself divided by the largest power of `other` that divides it modulo
    /// $m$, returning the exponent of that power.
    ///
    /// $$
    /// a \gets q, \quad a \equiv b^e q \pmod{m}, \quad b \nmid q,
    /// $$
    /// returning $e$.
    fn mod_remove_power_assign(&mut self, other: Rhs, m: M) -> u64;
}

/// Divides a polynomial by $x - c$ modulo $m$, returning the quotient and the remainder, which is
/// the value of the polynomial at $c$. The coefficients and $c$ must already be reduced modulo $m$.
pub trait ModDivRoot<C, M = C> {
    type Output;
    type RemOutput;

    /// Divides a polynomial by $x - c$ modulo $m$, returning the quotient and the remainder.
    ///
    /// $$
    /// f(a, c, m) = (q, r), \quad a \equiv (x - c)q + r \pmod{m}, \quad r = a(c).
    /// $$
    fn mod_div_root(self, c: C, m: M) -> (Self::Output, Self::RemOutput);
}

/// Divides a polynomial by $x - c$ modulo $m$ in place, replacing it with the quotient and
/// returning the remainder, which is the value of the polynomial at $c$. The coefficients and $c$
/// must already be reduced modulo $m$.
pub trait ModDivRootAssign<C, M = C> {
    type RemOutput;

    /// Replaces a polynomial with its quotient by $x - c$ modulo $m$, returning the remainder.
    ///
    /// $$
    /// a \gets q, \quad a \equiv (x - c)q + r \pmod{m},
    /// $$
    /// returning $r = a(c)$.
    fn mod_div_root_assign(&mut self, c: C, m: M) -> Self::RemOutput;
}

/// Divides one polynomial by another modulo $m$, assuming that the second divides the first. The
/// coefficients must already be reduced modulo $m$, and the leading coefficient of the divisor must
/// be invertible modulo $m$.
///
/// If the divisor does not divide the dividend, the result is meaningless.
pub trait ModDivExact<Rhs = Self, M = Self> {
    type Output;

    /// Divides a polynomial by another modulo $m$, assuming that the second divides the first.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    fn mod_div_exact(self, other: Rhs, m: M) -> Self::Output;
}

/// Divides one polynomial by another modulo $m$ in place, assuming that the second divides the
/// first. The coefficients must already be reduced modulo $m$, and the leading coefficient of the
/// divisor must be invertible modulo $m$.
///
/// If the divisor does not divide the dividend, the result is meaningless.
pub trait ModDivExactAssign<Rhs = Self, M = Self> {
    /// Replaces `self` with its quotient by another polynomial modulo $m$, assuming that the second
    /// divides the first.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    fn mod_div_exact_assign(&mut self, other: Rhs, m: M);
}

/// Divides one polynomial by another modulo $m$ if the second divides the first, returning `None`
/// otherwise. The coefficients must already be reduced modulo $m$, and the leading coefficient of
/// the divisor must be invertible modulo $m$.
pub trait CheckedModDivExact<Rhs = Self, M = Self> {
    type Output;

    /// Divides a polynomial by another modulo $m$ if the second divides the first, or returns
    /// `None`.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    fn checked_mod_div_exact(self, other: Rhs, m: M) -> Option<Self::Output>;
}

/// Determines whether one polynomial is divisible by another modulo $m$. The coefficients must
/// already be reduced modulo $m$, and the leading coefficient of the divisor, unless it is 0, must
/// be invertible modulo $m$.
pub trait ModDivisibleBy<Rhs = Self, M = Self> {
    /// Determines whether a polynomial is divisible by another modulo $m$: whether there is a
    /// polynomial $q$ with $a \equiv bq \pmod{m}$.
    fn mod_divisible_by(self, other: Rhs, m: M) -> bool;
}

/// Divides one polynomial by another modulo $m$, returning the quotient and remainder. The
/// coefficients must already be reduced modulo $m$, and the leading coefficient of the divisor must
/// be invertible modulo $m$.
///
/// The quotient and remainder are the unique polynomials $q$ and $r$ with coefficients modulo $m$
/// such that $a = bq + r$ and $\deg r < \deg b$.
pub trait ModDivRem<Rhs = Self, M = Self> {
    type DivOutput;
    type RemOutput;

    /// Divides a polynomial by another modulo $m$, returning the quotient and remainder.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_div_rem(self, other: Rhs, m: M) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides one polynomial by another modulo $m$ in place, replacing the first with the quotient and
/// returning the remainder. The coefficients must already be reduced modulo $m$, and the leading
/// coefficient of the divisor must be invertible modulo $m$.
pub trait ModDivAssignRem<Rhs = Self, M = Self> {
    type RemOutput;

    /// Replaces `self` with its quotient by another polynomial modulo $m$, returning the remainder.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    fn mod_div_assign_rem(&mut self, other: Rhs, m: M) -> Self::RemOutput;
}

/// Divides one polynomial by another modulo $m$, returning the quotient and remainder, or `None` if
/// the leading coefficient of the divisor is not invertible modulo $m$. The coefficients must
/// already be reduced modulo $m$.
pub trait CheckedModDivRem<Rhs = Self, M = Self> {
    type DivOutput;
    type RemOutput;

    /// Divides a polynomial by another modulo $m$, returning the quotient and remainder, or `None`
    /// if the divisor's leading coefficient is not invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } \gcd(\operatorname{lc}(b), m) = 1, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a \equiv bq + r \pmod{m}$ and $\deg r < \deg b$.
    fn checked_mod_div_rem(self, other: Rhs, m: M) -> Option<(Self::DivOutput, Self::RemOutput)>;
}

/// Finds the quotient of one polynomial divided by another modulo $m$. The coefficients must
/// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
/// modulo $m$.
///
/// This is the quotient of [`ModDivRem`]; computing it alone can be faster.
pub trait ModQuotient<Rhs = Self, M = Self> {
    type Output;

    /// Finds the quotient of a polynomial divided by another modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_quotient(self, other: Rhs, m: M) -> Self::Output;
}

/// Replaces one polynomial with its quotient by another modulo $m$. The coefficients must already
/// be reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
pub trait ModQuotientAssign<Rhs = Self, M = Self> {
    /// Replaces `self` with its quotient by another polynomial modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_quotient_assign(&mut self, other: Rhs, m: M);
}

/// Finds the quotient of one polynomial divided by another modulo $m$, or `None` if the leading
/// coefficient of the divisor is not invertible modulo $m$. The coefficients must already be
/// reduced modulo $m$.
pub trait CheckedModQuotient<Rhs = Self, M = Self> {
    type Output;

    /// Finds the quotient of a polynomial divided by another modulo $m$, or `None` if the divisor's
    /// leading coefficient is not invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } \gcd(\operatorname{lc}(b), m) = 1, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a \equiv bq + r \pmod{m}$ and $\deg r < \deg b$.
    fn checked_mod_quotient(self, other: Rhs, m: M) -> Option<Self::Output>;
}

/// Finds the remainder of one polynomial divided by another modulo $m$. The coefficients must
/// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
/// modulo $m$.
pub trait ModRem<Rhs = Self, M = Self> {
    type Output;

    /// Finds the remainder of a polynomial divided by another modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_rem(self, other: Rhs, m: M) -> Self::Output;
}

/// Replaces one polynomial with its remainder by another modulo $m$. The coefficients must already
/// be reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo $m$.
pub trait ModRemAssign<Rhs = Self, M = Self> {
    /// Replaces `self` with its remainder by another polynomial modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_rem_assign(&mut self, other: Rhs, m: M);
}

/// Finds the remainder of one polynomial divided by another modulo $m$, or `None` if the leading
/// coefficient of the divisor is not invertible modulo $m$. The coefficients must already be
/// reduced modulo $m$.
pub trait CheckedModRem<Rhs = Self, M = Self> {
    type Output;

    /// Finds the remainder of a polynomial divided by another modulo $m$, or `None` if the
    /// divisor's leading coefficient is not invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } \gcd(\operatorname{lc}(b), m) = 1, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a \equiv bq + r \pmod{m}$ and $\deg r < \deg b$.
    fn checked_mod_rem(self, other: Rhs, m: M) -> Option<Self::Output>;
}

/// Divides one polynomial by another modulo $m$, returning the quotient and remainder, using data
/// precomputed from the divisor and $m$. The coefficients must already be reduced modulo $m$, and
/// the leading coefficient of the divisor must be invertible modulo $m$.
///
/// If many polynomials are divided by the same divisor, it is quicker to precompute the data once,
/// with [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data), and
/// reuse it.
pub trait ModDivRemPrecomputed<Rhs = Self, M = Self> {
    type DivOutput;
    type RemOutput;
    type Data;

    /// Precomputes the data for dividing by `other` modulo `m`.
    fn precompute_mod_div_rem_data(other: &Rhs, m: &M) -> Self::Data;

    /// Divides a polynomial by another modulo $m$, returning the quotient and remainder, using data
    /// precomputed from the divisor and $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_div_rem_precomputed(
        self,
        other: Rhs,
        m: M,
        data: &Self::Data,
    ) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides one polynomial by another modulo $m$ in place, replacing the first with the quotient and
/// returning the remainder, using data precomputed from the divisor and $m$. The coefficients must
/// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
/// modulo $m$.
///
/// For precomputing the data, use
/// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data).
pub trait ModDivAssignRemPrecomputed<Rhs = Self, M = Self>: ModDivRemPrecomputed<Rhs, M> {
    /// Replaces `self` with its quotient by another polynomial modulo $m$, returning the remainder,
    /// using data precomputed from the divisor and $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    fn mod_div_assign_rem_precomputed(
        &mut self,
        other: Rhs,
        m: M,
        data: &Self::Data,
    ) -> Self::RemOutput;
}

/// Finds the quotient of one polynomial divided by another modulo $m$, using data precomputed from
/// the divisor and $m$. The coefficients must already be reduced modulo $m$, and the leading
/// coefficient of the divisor must be invertible modulo $m$.
///
/// If many polynomials are divided by the same divisor, it is quicker to precompute the data once,
/// with [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data), and
/// reuse it.
pub trait ModQuotientPrecomputed<Rhs = Self, M = Self> {
    type Output;
    type Data;

    /// Precomputes the data for dividing by `other` modulo `m`.
    fn precompute_mod_quotient_data(other: &Rhs, m: &M) -> Self::Data;

    /// Finds the quotient of a polynomial divided by another modulo $m$, using data precomputed
    /// from the divisor and $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_quotient_precomputed(self, other: Rhs, m: M, data: &Self::Data) -> Self::Output;
}

/// Replaces one polynomial with its quotient by another modulo $m$, using data precomputed from the
/// divisor and $m$. The coefficients must already be reduced modulo $m$, and the leading
/// coefficient of the divisor must be invertible modulo $m$.
///
/// For precomputing the data, use
/// [`precompute_mod_quotient_data`](ModQuotientPrecomputed::precompute_mod_quotient_data).
pub trait ModQuotientPrecomputedAssign<Rhs = Self, M = Self>:
    ModQuotientPrecomputed<Rhs, M>
{
    /// Replaces `self` with its quotient by another polynomial modulo $m$, using data precomputed
    /// from the divisor and $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_quotient_precomputed_assign(&mut self, other: Rhs, m: M, data: &Self::Data);
}

/// Finds the remainder of one polynomial divided by another modulo $m$, using data precomputed from
/// the divisor and $m$. The coefficients must already be reduced modulo $m$, and the leading
/// coefficient of the divisor must be invertible modulo $m$.
///
/// If many polynomials are reduced by the same divisor, it is quicker to precompute the data once,
/// with [`precompute_mod_rem_data`](ModRemPrecomputed::precompute_mod_rem_data), and reuse it.
pub trait ModRemPrecomputed<Rhs = Self, M = Self> {
    type Output;
    type Data;

    /// Precomputes the data for dividing by `other` modulo `m`.
    fn precompute_mod_rem_data(other: &Rhs, m: &M) -> Self::Data;

    /// Finds the remainder of a polynomial divided by another modulo $m$, using data precomputed
    /// from the divisor and $m$.
    ///
    /// $$
    /// f(a, b, m) = r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_rem_precomputed(self, other: Rhs, m: M, data: &Self::Data) -> Self::Output;
}

/// Replaces one polynomial with its remainder by another modulo $m$, using data precomputed from
/// the divisor and $m$. The coefficients must already be reduced modulo $m$, and the leading
/// coefficient of the divisor must be invertible modulo $m$.
///
/// For precomputing the data, use
/// [`precompute_mod_rem_data`](ModRemPrecomputed::precompute_mod_rem_data).
pub trait ModRemPrecomputedAssign<Rhs = Self, M = Self>: ModRemPrecomputed<Rhs, M> {
    /// Replaces `self` with its remainder by another polynomial modulo $m$, using data precomputed
    /// from the divisor and $m$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    fn mod_rem_precomputed_assign(&mut self, other: Rhs, m: M, data: &Self::Data);
}

/// Divides one polynomial by another modulo $2^k$, returning the quotient and remainder, using data
/// precomputed from the divisor and $k$. The coefficients must already be reduced modulo $2^k$, and
/// the leading coefficient of the divisor must be odd.
///
/// If many polynomials are divided by the same divisor, it is quicker to precompute the data once,
/// with [`precompute_mod_power_of_2_div_rem_data`](
/// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data), and reuse it.
pub trait ModPowerOf2DivRemPrecomputed<Rhs = Self> {
    type DivOutput;
    type RemOutput;
    type Data;

    /// Precomputes the data for dividing by `other` modulo $2^k$.
    fn precompute_mod_power_of_2_div_rem_data(other: &Rhs, pow: u64) -> Self::Data;

    /// Divides a polynomial by another modulo $2^k$, returning the quotient and remainder, using
    /// data precomputed from the divisor and $k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_div_rem_precomputed(
        self,
        other: Rhs,
        pow: u64,
        data: &Self::Data,
    ) -> (Self::DivOutput, Self::RemOutput);
}

/// Divides one polynomial by another modulo $2^k$ in place, replacing the first with the quotient
/// and returning the remainder, using data precomputed from the divisor and $k$. The coefficients
/// must already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
///
/// For precomputing the data, use [`precompute_mod_power_of_2_div_rem_data`](
/// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data).
pub trait ModPowerOf2DivAssignRemPrecomputed<Rhs = Self>:
    ModPowerOf2DivRemPrecomputed<Rhs>
{
    /// Replaces `self` with its quotient by another polynomial modulo $2^k$, returning the
    /// remainder, using data precomputed from the divisor and $k$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    fn mod_power_of_2_div_assign_rem_precomputed(
        &mut self,
        other: Rhs,
        pow: u64,
        data: &Self::Data,
    ) -> Self::RemOutput;
}

/// Finds the quotient of one polynomial divided by another modulo $2^k$, using data precomputed
/// from the divisor and $k$. The coefficients must already be reduced modulo $2^k$, and the leading
/// coefficient of the divisor must be odd.
///
/// If many polynomials are divided by the same divisor, it is quicker to precompute the data once,
/// with [`precompute_mod_power_of_2_quotient_data`](
/// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data), and reuse it.
pub trait ModPowerOf2QuotientPrecomputed<Rhs = Self> {
    type Output;
    type Data;

    /// Precomputes the data for dividing by `other` modulo $2^k$.
    fn precompute_mod_power_of_2_quotient_data(other: &Rhs, pow: u64) -> Self::Data;

    /// Finds the quotient of a polynomial divided by another modulo $2^k$, using data precomputed
    /// from the divisor and $k$.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_quotient_precomputed(
        self,
        other: Rhs,
        pow: u64,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Replaces one polynomial with its quotient by another modulo $2^k$, using data precomputed from
/// the divisor and $k$. The coefficients must already be reduced modulo $2^k$, and the leading
/// coefficient of the divisor must be odd.
///
/// For precomputing the data, use [`precompute_mod_power_of_2_quotient_data`](
/// ModPowerOf2QuotientPrecomputed::precompute_mod_power_of_2_quotient_data).
pub trait ModPowerOf2QuotientPrecomputedAssign<Rhs = Self>:
    ModPowerOf2QuotientPrecomputed<Rhs>
{
    /// Replaces `self` with its quotient by another polynomial modulo $2^k$, using data precomputed
    /// from the divisor and $k$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_quotient_precomputed_assign(
        &mut self,
        other: Rhs,
        pow: u64,
        data: &Self::Data,
    );
}

/// Finds the remainder of one polynomial divided by another modulo $2^k$, using data precomputed
/// from the divisor and $k$. The coefficients must already be reduced modulo $2^k$, and the leading
/// coefficient of the divisor must be odd.
///
/// If many polynomials are reduced by the same divisor, it is quicker to precompute the data once,
/// with [`precompute_mod_power_of_2_rem_data`](
/// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data), and reuse it.
pub trait ModPowerOf2RemPrecomputed<Rhs = Self> {
    type Output;
    type Data;

    /// Precomputes the data for dividing by `other` modulo $2^k$.
    fn precompute_mod_power_of_2_rem_data(other: &Rhs, pow: u64) -> Self::Data;

    /// Finds the remainder of a polynomial divided by another modulo $2^k$, using data precomputed
    /// from the divisor and $k$.
    ///
    /// $$
    /// f(a, b, k) = r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_rem_precomputed(
        self,
        other: Rhs,
        pow: u64,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Replaces one polynomial with its remainder by another modulo $2^k$, using data precomputed from
/// the divisor and $k$. The coefficients must already be reduced modulo $2^k$, and the leading
/// coefficient of the divisor must be odd.
///
/// For precomputing the data, use [`precompute_mod_power_of_2_rem_data`](
/// ModPowerOf2RemPrecomputed::precompute_mod_power_of_2_rem_data).
pub trait ModPowerOf2RemPrecomputedAssign<Rhs = Self>: ModPowerOf2RemPrecomputed<Rhs> {
    /// Replaces `self` with its remainder by another polynomial modulo $2^k$, using data
    /// precomputed from the divisor and $k$.
    ///
    /// $$
    /// a \gets r, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    fn mod_power_of_2_rem_precomputed_assign(&mut self, other: Rhs, pow: u64, data: &Self::Data);
}

/// Composes two polynomials modulo $m$, giving $p(q(x)) \bmod m$. The coefficients of both must
/// already be reduced modulo $m$.
pub trait ModCompose<Rhs = Self, M = Self> {
    type Output;

    /// Composes two polynomials modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = p(q(x)) \bmod m.
    /// $$
    fn mod_compose(self, other: Rhs, m: M) -> Self::Output;
}

/// Composes two polynomials modulo $m$ in place, replacing $p$ with $p(q(x)) \bmod m$. The
/// coefficients of both must already be reduced modulo $m$.
pub trait ModComposeAssign<Rhs = Self, M = Self> {
    /// Composes two polynomials modulo $m$ in place.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod m.
    /// $$
    fn mod_compose_assign(&mut self, other: Rhs, m: M);
}

/// Composes two polynomials modulo $2^k$, giving $p(q(x)) \bmod 2^k$. The coefficients of both must
/// already be reduced modulo $2^k$.
pub trait ModPowerOf2Compose<Rhs = Self> {
    type Output;

    /// Composes two polynomials modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p(q(x)) \bmod 2^k.
    /// $$
    fn mod_power_of_2_compose(self, other: Rhs, pow: u64) -> Self::Output;
}

/// Composes two polynomials modulo $2^k$ in place, replacing $p$ with $p(q(x)) \bmod 2^k$. The
/// coefficients of both must already be reduced modulo $2^k$.
pub trait ModPowerOf2ComposeAssign<Rhs = Self> {
    /// Composes two polynomials modulo $2^k$ in place.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod 2^k.
    /// $$
    fn mod_power_of_2_compose_assign(&mut self, other: Rhs, pow: u64);
}

/// Shifts a polynomial by a constant modulo $2^k$, giving $p(x + c) \bmod 2^k$. The coefficients
/// and the constant must be already reduced modulo $2^k$.
pub trait ModPowerOf2TaylorShift<C> {
    type Output;

    /// Shifts a polynomial by a constant modulo $2^k$.
    ///
    /// $$
    /// f(p, c, k) = p(x + c) \bmod 2^k.
    /// $$
    fn mod_power_of_2_taylor_shift(self, c: C, pow: u64) -> Self::Output;
}

/// Shifts a polynomial by a constant modulo $2^k$ in place, replacing $p$ with $p(x + c) \bmod
/// 2^k$. The coefficients and the constant must be already reduced modulo $2^k$.
pub trait ModPowerOf2TaylorShiftAssign<C> {
    /// Shifts a polynomial by a constant modulo $2^k$ in place.
    ///
    /// $$
    /// p \gets p(x + c) \bmod 2^k.
    /// $$
    fn mod_power_of_2_taylor_shift_assign(&mut self, c: C, pow: u64);
}

/// Shifts a polynomial by a constant modulo $m$, giving $p(x + c) \bmod m$. The coefficients and
/// the constant must be already reduced modulo $m$.
pub trait ModTaylorShift<C, M = C> {
    type Output;

    /// Shifts a polynomial by a constant modulo $m$.
    ///
    /// $$
    /// f(p, c, m) = p(x + c) \bmod m.
    /// $$
    fn mod_taylor_shift(self, c: C, m: M) -> Self::Output;
}

/// Shifts a polynomial by a constant modulo $m$ in place, replacing $p$ with $p(x + c) \bmod m$.
/// The coefficients and the constant must be already reduced modulo $m$.
pub trait ModTaylorShiftAssign<C, M = C> {
    /// Shifts a polynomial by a constant modulo $m$ in place.
    ///
    /// $$
    /// p \gets p(x + c) \bmod m.
    /// $$
    fn mod_taylor_shift_assign(&mut self, c: C, m: M);
}

/// Makes a polynomial monic modulo $m$, by multiplying it by the inverse of its leading
/// coefficient.
///
/// The polynomial's coefficients must already be reduced modulo $m$. If the leading coefficient is
/// not invertible modulo $m$, its greatest common divisor with $m$, a nontrivial factor of $m$, is
/// returned as the error. The zero polynomial is left as it is.
pub trait ModMakeMonic<M> {
    /// The type of the monic polynomial.
    type Output;
    /// The type of the factor of $m$ returned when the leading coefficient is not invertible.
    type Factor;

    /// Makes a polynomial monic modulo `m`.
    fn mod_make_monic(self, m: M) -> Result<Self::Output, Self::Factor>;
}

/// Makes a polynomial monic modulo $m$ in place.
///
/// See [`ModMakeMonic`]. If the leading coefficient is not invertible, the polynomial is left
/// unchanged.
pub trait ModMakeMonicAssign<M> {
    /// The type of the factor of $m$ returned when the leading coefficient is not invertible.
    type Factor;

    /// Makes a polynomial monic modulo `m` in place.
    fn mod_make_monic_assign(&mut self, m: M) -> Result<(), Self::Factor>;
}

/// Evaluates a polynomial at a value, modulo $2^k$. The polynomial's coefficients and the value
/// must already be reduced modulo $2^k$.
pub trait ModPowerOf2Evaluate<T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`, modulo $2^k$.
    ///
    /// $$
    /// f(p, x, k) = \sum_{i=0}^{n-1} c_i x^i \bmod 2^k,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_power_of_2_evaluate(self, x: T, pow: u64) -> Self::Output;
}

/// Evaluates a polynomial at a value, modulo $m$. The value must already be reduced modulo $m$, and
/// so must the polynomial's coefficients, unless an implementation says otherwise.
pub trait ModEvaluate<T, M = T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`, modulo `m`.
    ///
    /// $$
    /// f(p, x, m) = \sum_{i=0}^{n-1} c_i x^i \bmod m,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_evaluate(self, x: T, m: M) -> Self::Output;
}

/// Evaluates a polynomial at each of several values, modulo $m$. The values must already be reduced
/// modulo $m$, and so must the polynomial's coefficients, unless an implementation says otherwise.
pub trait ModEvaluateMany<T, M = T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at each value in `xs`, modulo `m`, returning the values in the same
    /// order.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m
    /// \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_evaluate_many(self, xs: &[T], m: M) -> Vec<Self::Output>;
}

/// Evaluates a polynomial at the first terms of a geometric progression starting at 1, modulo $m$.
/// The ratio must already be reduced modulo $m$, and so must the polynomial's coefficients.
pub trait ModEvaluateGeometric<T, M = T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at $1, q, q^2, \ldots, q^{k-1}$, modulo `m`.
    ///
    /// $$
    /// f(p, q, k, m) = \left ( \sum_{i=0}^{n-1} c_i q^{ij} \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_evaluate_geometric(self, q: T, k: u64, m: M) -> Vec<Self::Output>;
}

/// Builds the monic polynomial with given roots modulo $m$: $(x - r_0)(x - r_1) \cdots (x -
/// r_{n-1}) \bmod m$. The roots must already be reduced modulo $m$.
pub trait ModFromRoots<T, M = T> {
    /// Builds the monic polynomial, with coefficients reduced modulo $m$, whose roots, with
    /// multiplicity, are the values in `roots`.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, m) = \prod_{i=0}^{n-1} (x - r_i) \bmod m.
    /// $$
    fn mod_from_roots(roots: &[T], m: M) -> Self;
}

/// Builds the monic polynomial with given roots modulo $2^k$: $(x - r_0)(x - r_1) \cdots (x -
/// r_{n-1}) \bmod 2^k$. The roots must already be reduced modulo $2^k$.
pub trait ModPowerOf2FromRoots<T> {
    /// Builds the monic polynomial, with coefficients reduced modulo $2^k$, whose roots, with
    /// multiplicity, are the values in `roots`.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, k) = \prod_{i=0}^{n-1} (x - r_i) \bmod 2^k.
    /// $$
    fn mod_power_of_2_from_roots(roots: &[T], pow: u64) -> Self;
}

/// Evaluates polynomials modulo $m$ at a fixed set of values, using data precomputed from the
/// values and $m$: the subproduct tree of the linear factors $x - x_j$.
pub trait ModEvaluateManyPrecomputed<T, M = T> {
    /// The type of the polynomial's values.
    type Output;
    type Data;

    /// Precomputes the data for evaluating at each value in `xs` modulo `m`.
    fn precompute_mod_evaluate_many_data(xs: &[T], m: &M) -> Self::Data;

    /// Evaluates a polynomial modulo `m` at each value the data was precomputed from, returning the
    /// values in the same order.
    ///
    /// $$
    /// f(p, m) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod m \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the $x_j$ are the
    /// values the data was precomputed from.
    fn mod_evaluate_many_precomputed(self, m: M, data: &Self::Data) -> Vec<Self::Output>;
}

/// Evaluates a polynomial modulo $2^k$ at several values of the coefficient type. The coefficients
/// and the values must already be reduced modulo $2^k$.
pub trait ModPowerOf2EvaluateMany<T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at each value in `xs`, modulo $2^k$, returning the values in the same
    /// order.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{l-1}, k) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod 2^k
    /// \right )_{j=0}^{l-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn mod_power_of_2_evaluate_many(self, xs: &[T], pow: u64) -> Vec<Self::Output>;
}

/// Evaluates polynomials modulo $2^k$ at a fixed set of values, using data precomputed from the
/// values and $k$: the subproduct tree of the linear factors $x - x_j$.
pub trait ModPowerOf2EvaluateManyPrecomputed<T> {
    /// The type of the polynomial's values.
    type Output;
    type Data;

    /// Precomputes the data for evaluating at each value in `xs` modulo $2^k$.
    fn precompute_mod_power_of_2_evaluate_many_data(xs: &[T], pow: u64) -> Self::Data;

    /// Evaluates a polynomial modulo $2^k$ at each value the data was precomputed from, returning
    /// the values in the same order.
    ///
    /// $$
    /// f(p, k) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \bmod 2^k \right )_{j=0}^{l-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$, $n$ is its length, and the $x_j$ are the $l$
    /// values the data was precomputed from.
    fn mod_power_of_2_evaluate_many_precomputed(
        self,
        pow: u64,
        data: &Self::Data,
    ) -> Vec<Self::Output>;
}

/// Computes the monic greatest common divisor of two polynomials modulo $m$. The coefficients must
/// already be reduced modulo $m$, and every leading coefficient met by Euclid's algorithm must be
/// invertible modulo $m$, as it always is when $m$ is prime.
pub trait ModGcd<Rhs = Self, M = Self> {
    type Output;

    /// Computes the monic GCD of two polynomials modulo $m$; the GCD of 0 and 0 is 0.
    ///
    /// $$
    /// f(a, b, m) = g, \quad g \mid a, \quad g \mid b, \quad (g) = (a, b) \pmod{m}.
    /// $$
    fn mod_gcd(self, other: Rhs, m: M) -> Self::Output;
}

/// Replaces a polynomial with the monic greatest common divisor of it and another polynomial modulo
/// $m$. The coefficients must already be reduced modulo $m$, and every leading coefficient met by
/// Euclid's algorithm must be invertible modulo $m$.
pub trait ModGcdAssign<Rhs = Self, M = Self> {
    /// Replaces `self` with its monic GCD with another polynomial modulo $m$.
    fn mod_gcd_assign(&mut self, other: Rhs, m: M);
}

/// Computes the monic greatest common divisor of two polynomials modulo $m$, or `None` if Euclid's
/// algorithm meets a leading coefficient that is not invertible modulo $m$. The coefficients must
/// already be reduced modulo $m$.
pub trait CheckedModGcd<Rhs = Self, M = Self> {
    type Output;

    /// Computes the monic GCD of two polynomials modulo $m$, or `None` if a leading coefficient met
    /// along the way is not invertible.
    fn checked_mod_gcd(self, other: Rhs, m: M) -> Option<Self::Output>;
}

/// Computes the monic greatest common divisor of two polynomials modulo $2^k$, or `None` if
/// Euclid's algorithm meets an even leading coefficient. The coefficients must already be reduced
/// modulo $2^k$.
pub trait CheckedModPowerOf2Gcd<Rhs = Self> {
    type Output;

    /// Computes the monic GCD of two polynomials modulo $2^k$, or `None` if a leading coefficient
    /// met along the way is even.
    fn checked_mod_power_of_2_gcd(self, other: Rhs, pow: u64) -> Option<Self::Output>;
}

/// Computes the monic greatest common divisor of two polynomials modulo $m$, together with
/// cofactors. The coefficients must already be reduced modulo $m$, and every leading coefficient
/// met by Euclid's algorithm must be invertible modulo $m$, as it always is when $m$ is prime.
pub trait ModExtendedGcd<Rhs = Self, M = Self> {
    type Gcd;
    type Cofactor;

    /// Computes the monic GCD $g$ of two polynomials $a$ and $b$ modulo $m$, together with
    /// cofactors $s$ and $t$.
    ///
    /// $$
    /// f(a, b, m) = (g, s, t), \quad sa + tb \equiv g \pmod{m}.
    /// $$
    fn mod_extended_gcd(self, other: Rhs, m: M) -> (Self::Gcd, Self::Cofactor, Self::Cofactor);
}

/// Computes the monic greatest common divisor of two polynomials modulo $m$, together with
/// cofactors, or `None` if Euclid's algorithm meets a leading coefficient that is not invertible
/// modulo $m$. The coefficients must already be reduced modulo $m$.
pub trait CheckedModExtendedGcd<Rhs = Self, M = Self> {
    type Gcd;
    type Cofactor;

    /// Computes the monic GCD of two polynomials modulo $m$, together with cofactors, or `None` if
    /// a leading coefficient met along the way is not invertible.
    #[allow(clippy::type_complexity)]
    fn checked_mod_extended_gcd(
        self,
        other: Rhs,
        m: M,
    ) -> Option<(Self::Gcd, Self::Cofactor, Self::Cofactor)>;
}

/// Computes the monic greatest common divisor of two polynomials modulo $2^k$, together with
/// cofactors, or `None` if Euclid's algorithm meets an even leading coefficient. The coefficients
/// must already be reduced modulo $2^k$.
pub trait CheckedModPowerOf2ExtendedGcd<Rhs = Self> {
    type Gcd;
    type Cofactor;

    /// Computes the monic GCD of two polynomials modulo $2^k$, together with cofactors, or `None`
    /// if a leading coefficient met along the way is even.
    #[allow(clippy::type_complexity)]
    fn checked_mod_power_of_2_extended_gcd(
        self,
        other: Rhs,
        pow: u64,
    ) -> Option<(Self::Gcd, Self::Cofactor, Self::Cofactor)>;
}

/// Computes the monic least common multiple of two polynomials modulo $m$. The coefficients must
/// already be reduced modulo $m$, and every leading coefficient met along the way must be
/// invertible modulo $m$, as it always is when $m$ is prime.
pub trait ModLcm<Rhs = Self, M = Self> {
    type Output;

    /// Computes the monic LCM of two polynomials modulo $m$; the LCM of 0 and any polynomial is 0.
    ///
    /// $$
    /// f(a, b, m) = l, \quad a \mid l, \quad b \mid l, \quad (l) = (a) \cap (b) \pmod{m}.
    /// $$
    fn mod_lcm(self, other: Rhs, m: M) -> Self::Output;
}

/// Replaces a polynomial with the monic least common multiple of it and another polynomial modulo
/// $m$. The coefficients must already be reduced modulo $m$, and every leading coefficient met
/// along the way must be invertible modulo $m$.
pub trait ModLcmAssign<Rhs = Self, M = Self> {
    /// Replaces `self` with its monic LCM with another polynomial modulo $m$.
    fn mod_lcm_assign(&mut self, other: Rhs, m: M);
}

/// Computes the monic least common multiple of two polynomials modulo $m$, or `None` if a leading
/// coefficient met along the way is not invertible modulo $m$. The coefficients must already be
/// reduced modulo $m$.
pub trait CheckedModLcm<Rhs = Self, M = Self> {
    type Output;

    /// Computes the monic LCM of two polynomials modulo $m$, or `None` if a leading coefficient met
    /// along the way is not invertible.
    fn checked_mod_lcm(self, other: Rhs, m: M) -> Option<Self::Output>;
}

/// Computes the monic least common multiple of two polynomials modulo $2^k$, or `None` if a leading
/// coefficient met along the way is even. The coefficients must already be reduced modulo $2^k$.
pub trait CheckedModPowerOf2Lcm<Rhs = Self> {
    type Output;

    /// Computes the monic LCM of two polynomials modulo $2^k$, or `None` if a leading coefficient
    /// met along the way is even.
    fn checked_mod_power_of_2_lcm(self, other: Rhs, pow: u64) -> Option<Self::Output>;
}

/// Determines whether two polynomials are coprime modulo $m$: whether the ideal they generate is
/// the whole ring, so that $sa + tb \equiv 1 \pmod{m}$ for some polynomials $s$ and $t$. The
/// coefficients must already be reduced modulo $m$.
pub trait ModCoprimeWith<Rhs = Self, M = Self> {
    /// Determines whether two polynomials are coprime modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (\exists s, t : sa + tb \equiv 1 \pmod{m}).
    /// $$
    fn mod_coprime_with(self, other: Rhs, m: M) -> bool;
}

/// Determines whether two polynomials are coprime modulo $2^k$: whether the ideal they generate is
/// the whole ring, so that $sa + tb \equiv 1 \pmod{2^k}$ for some polynomials $s$ and $t$. The
/// coefficients must already be reduced modulo $2^k$.
pub trait ModPowerOf2CoprimeWith<Rhs = Self> {
    /// Determines whether two polynomials are coprime modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (\exists s, t : sa + tb \equiv 1 \pmod{2^k}).
    /// $$
    fn mod_power_of_2_coprime_with(self, other: Rhs, pow: u64) -> bool;
}
