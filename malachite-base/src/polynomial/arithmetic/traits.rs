// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;

/// Multiplies two polynomials, keeping only the coefficients of $x^i$ for $i$ in a given range, and
/// shifting them down to start at $x^0$.
///
/// This is the middle product: with $\ell$ equal to `low` and $h$ equal to `high`, it is $\lfloor
/// (pq \bmod x^h) / x^\ell \rfloor$. The polynomials need not be of any particular lengths, and a
/// range that extends past the end of the product is cut off there.
pub trait MulMiddle<Rhs = Self> {
    type Output;

    /// Multiplies two polynomials and returns the coefficients of $x^i$ for `low` $\leq i <$ `high`
    /// of the product, shifted down by `low`.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    fn mul_middle(self, other: Rhs, low: u64, high: u64) -> Self::Output;
}

/// Multiplies a polynomial by another in place, keeping only the coefficients of $x^i$ for $i$ in a
/// given range, and shifting them down to start at $x^0$.
///
/// This is the middle product: with $\ell$ equal to `low` and $h$ equal to `high`, it is $\lfloor
/// (pq \bmod x^h) / x^\ell \rfloor$. The polynomials need not be of any particular lengths, and a
/// range that extends past the end of the product is cut off there.
pub trait MulMiddleAssign<Rhs = Self> {
    /// Multiplies `self` by a polynomial and replaces it with the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` of the product, shifted down by `low`.
    ///
    /// $$
    /// p \gets \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    fn mul_middle_assign(&mut self, other: Rhs, low: u64, high: u64);
}

/// Finds the remainder of one polynomial divided by another, using data precomputed from the
/// divisor.
///
/// The data is precomputed once, with [`precompute_rem_data`](RemPrecomputed::precompute_rem_data),
/// and may be reused for any number of dividends.
pub trait RemPrecomputed<Rhs = Self> {
    type Output;
    type Data;

    /// Precomputes the data for dividing by `other`.
    fn precompute_rem_data(other: &Rhs) -> Self::Data;

    /// Finds the remainder of a polynomial divided by another, using data precomputed from the
    /// divisor.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    fn rem_precomputed(self, other: Rhs, data: &Self::Data) -> Self::Output;
}

/// Replaces one polynomial with its remainder by another, using data precomputed from the divisor.
///
/// For precomputing the data, use [`precompute_rem_data`](RemPrecomputed::precompute_rem_data).
pub trait RemPrecomputedAssign<Rhs = Self>: RemPrecomputed<Rhs> {
    /// Replaces `self` with its remainder by another polynomial, using data precomputed from the
    /// divisor.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    fn rem_precomputed_assign(&mut self, other: Rhs, data: &Self::Data);
}

/// Divides one polynomial by another if the second divides the first, returning `None` otherwise.
pub trait CheckedDivExact<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another if the second divides the first, or returns `None`.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a = bq, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    fn checked_div_exact(self, other: Rhs) -> Option<Self::Output>;
}

/// Divides a polynomial by $x - c$, returning the quotient and the remainder, which is the value of
/// the polynomial at $c$.
pub trait DivRoot<C> {
    type Output;
    type RemOutput;

    /// Divides a polynomial by $x - c$, returning the quotient and the remainder.
    ///
    /// $$
    /// f(a, c) = (q, r), \quad a = (x - c)q + r, \quad r = a(c).
    /// $$
    fn div_root(self, c: C) -> (Self::Output, Self::RemOutput);
}

/// Divides a polynomial by $x - c$ in place, replacing it with the quotient and returning the
/// remainder, which is the value of the polynomial at $c$.
pub trait DivRootAssign<C> {
    type RemOutput;

    /// Replaces a polynomial with its quotient by $x - c$, returning the remainder.
    ///
    /// $$
    /// a \gets q, \quad a = (x - c)q + r,
    /// $$
    /// returning $r = a(c)$.
    fn div_root_assign(&mut self, c: C) -> Self::RemOutput;
}

/// Divides one polynomial by another, returning the quotient and remainder, or `None` if the
/// division is impossible: if the divisor is 0, or if the quotient over the field of fractions does
/// not have coefficients in the coefficient ring.
pub trait CheckedDivRem<Rhs = Self> {
    type DivOutput;
    type RemOutput;

    /// Divides a polynomial by another, returning the quotient and remainder, or `None` if they do
    /// not have coefficients in the coefficient ring.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } a = bq + r \text{ with } \deg r < \deg b, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    fn checked_div_rem(self, other: Rhs) -> Option<(Self::DivOutput, Self::RemOutput)>;
}

/// Finds the remainder of one polynomial divided by another, or `None` if the division is
/// impossible, as for [`CheckedDivRem`].
pub trait CheckedRem<Rhs = Self> {
    type Output;

    /// Finds the remainder of a polynomial divided by another, or `None` if the division is
    /// impossible.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } a = bq + r \text{ with } \deg r < \deg b, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    fn checked_rem(self, other: Rhs) -> Option<Self::Output>;
}

/// Pseudo-divides one polynomial by another, returning the quotient, the remainder, and the
/// exponent $d$: with $\ell$ the leading coefficient of the divisor, the dividend times $\ell^d$ is
/// the divisor times the quotient plus the remainder, and the remainder's degree is less than the
/// divisor's. $d$ is the least exponent for which the quotient has integral coefficients.
pub trait PseudoDivRem<Rhs = Self> {
    type DivOutput;
    type RemOutput;

    /// Pseudo-divides a polynomial by another.
    ///
    /// $$
    /// f(a, b) = (q, r, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$ and $d$ is as small as possible.
    fn pseudo_div_rem(self, other: Rhs) -> (Self::DivOutput, Self::RemOutput, u64);
}

/// Pseudo-divides one polynomial by another in place, replacing the first with the quotient and
/// returning the remainder and the exponent $d$, as for [`PseudoDivRem`].
pub trait PseudoDivAssignRem<Rhs = Self> {
    type RemOutput;

    /// Replaces `self` with its pseudo-quotient by another polynomial, returning the remainder and
    /// the exponent.
    ///
    /// $$
    /// a \gets q, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $(r, d)$, where $\ell$ is the leading coefficient of $b$ and $d$ is as small as
    /// possible.
    fn pseudo_div_assign_rem(&mut self, other: Rhs) -> (Self::RemOutput, u64);
}

/// Pseudo-divides one polynomial by another, returning the quotient and the exponent $d$, as for
/// [`PseudoDivRem`].
pub trait PseudoDiv<Rhs = Self> {
    type Output;

    /// Pseudo-divides a polynomial by another, returning the quotient and the exponent.
    ///
    /// $$
    /// f(a, b) = (q, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$ and $d$ is as small as possible.
    fn pseudo_div(self, other: Rhs) -> (Self::Output, u64);
}

/// Replaces one polynomial with its pseudo-quotient by another, returning the exponent $d$, as for
/// [`PseudoDivRem`].
pub trait PseudoDivAssign<Rhs = Self> {
    /// Replaces `self` with its pseudo-quotient by another polynomial, returning the exponent.
    ///
    /// $$
    /// a \gets q, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $d$, where $\ell$ is the leading coefficient of $b$ and $d$ is as small as
    /// possible.
    fn pseudo_div_assign(&mut self, other: Rhs) -> u64;
}

/// Finds the pseudo-remainder of one polynomial divided by another, and the exponent $d$, as for
/// [`PseudoDivRem`].
pub trait PseudoRem<Rhs = Self> {
    type Output;

    /// Finds the pseudo-remainder of a polynomial divided by another, and the exponent.
    ///
    /// $$
    /// f(a, b) = (r, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$ and $d$ is as small as possible.
    fn pseudo_rem(self, other: Rhs) -> (Self::Output, u64);
}

/// Replaces one polynomial with its pseudo-remainder by another, returning the exponent $d$, as for
/// [`PseudoDivRem`].
pub trait PseudoRemAssign<Rhs = Self> {
    /// Replaces `self` with its pseudo-remainder by another polynomial, returning the exponent.
    ///
    /// $$
    /// a \gets r, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $d$, where $\ell$ is the leading coefficient of $b$ and $d$ is as small as
    /// possible.
    fn pseudo_rem_assign(&mut self, other: Rhs) -> u64;
}

/// Computes the square of a polynomial's $L^2$ norm: the sum of the squares of its coefficients.
///
/// This is exact, unlike the norm itself, which is usually irrational.
pub trait L2NormSquared {
    type Output;

    /// Computes the sum of the squares of a polynomial's coefficients.
    ///
    /// $$
    /// f(p) = \sum_i p_i^2.
    /// $$
    fn l2_norm_squared(self) -> Self::Output;
}

/// Computes the floor of a polynomial's $L^2$ norm: the floor of the square root of the sum of the
/// squares of its coefficients.
pub trait FloorL2Norm {
    type Output;

    /// Computes the floor of the square root of the sum of the squares of a polynomial's
    /// coefficients.
    ///
    /// $$
    /// f(p) = \left \lfloor \sqrt{\sum_i p_i^2} \right \rfloor.
    /// $$
    fn floor_l2_norm(self) -> Self::Output;
}

/// Multiplies a polynomial by $x^n$, which moves every coefficient up by $n$ places.
pub trait MulPowerOfX {
    type Output;

    /// Multiplies a polynomial by $x^n$.
    ///
    /// $$
    /// f(p, n) = x^np.
    /// $$
    fn mul_power_of_x(self, n: u64) -> Self::Output;
}

/// Multiplies a polynomial by $x^n$ in place, which moves every coefficient up by $n$ places.
pub trait MulPowerOfXAssign {
    /// Multiplies a polynomial by $x^n$ in place.
    ///
    /// $$
    /// p \gets x^np.
    /// $$
    fn mul_power_of_x_assign(&mut self, n: u64);
}

/// Divides a polynomial by $x^n$, discarding the remainder: every coefficient moves down by $n$
/// places, and the lowest $n$ are dropped.
pub trait DivPowerOfX {
    type Output;

    /// Divides a polynomial by $x^n$, discarding the remainder.
    ///
    /// $$
    /// f(p, n) = \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    fn div_power_of_x(self, n: u64) -> Self::Output;
}

/// Divides a polynomial by $x^n$ in place, discarding the remainder: every coefficient moves down
/// by $n$ places, and the lowest $n$ are dropped.
pub trait DivPowerOfXAssign {
    /// Divides a polynomial by $x^n$ in place, discarding the remainder.
    ///
    /// $$
    /// p \gets \sum_{i \geq n} p_ix^{i-n}.
    /// $$
    fn div_power_of_x_assign(&mut self, n: u64);
}

/// Composes two polynomials, giving $p(q(x))$.
pub trait Compose<Rhs = Self> {
    type Output;

    /// Composes two polynomials.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    fn compose(self, other: Rhs) -> Self::Output;
}

/// Composes two polynomials in place, replacing $p$ with $p(q(x))$.
pub trait ComposeAssign<Rhs = Self> {
    /// Composes two polynomials in place.
    ///
    /// $$
    /// p \gets p(q(x)).
    /// $$
    fn compose_assign(&mut self, other: Rhs);
}

/// Composes a polynomial with $x^k$, giving $p(x^k)$: the coefficient of $x^i$ moves to $x^{ik}$.
///
/// When $k$ is 0, the result is the constant $p(1)$, the sum of the coefficients.
pub trait ComposePowerOfX {
    type Output;

    /// Composes a polynomial with $x^k$.
    ///
    /// $$
    /// f(p, k) = p(x^k).
    /// $$
    fn compose_power_of_x(self, k: u64) -> Self::Output;
}

/// Composes a polynomial with $x^k$ in place, replacing $p$ with $p(x^k)$.
///
/// When $k$ is 0, the result is the constant $p(1)$, the sum of the coefficients.
pub trait ComposePowerOfXAssign {
    /// Composes a polynomial with $x^k$ in place.
    ///
    /// $$
    /// p \gets p(x^k).
    /// $$
    fn compose_power_of_x_assign(&mut self, k: u64);
}

/// Shifts a polynomial by a constant, giving $p(x + c)$.
///
/// This is also called a Taylor shift: the coefficient of $x^k$ in $p(x + c)$ is $p^{(k)}(c)/k!$,
/// the $k$th coefficient of the Taylor expansion of $p$ around $c$. The shift preserves the degree
/// and the leading coefficient, and shifting by $-c$ undoes it.
pub trait TaylorShift<C> {
    type Output;

    /// Shifts a polynomial by a constant.
    ///
    /// $$
    /// f(p, c) = p(x + c).
    /// $$
    fn taylor_shift(self, c: C) -> Self::Output;
}

/// Shifts a polynomial by a constant in place, replacing $p$ with $p(x + c)$.
pub trait TaylorShiftAssign<C> {
    /// Shifts a polynomial by a constant in place.
    ///
    /// $$
    /// p \gets p(x + c).
    /// $$
    fn taylor_shift_assign(&mut self, c: C);
}

/// Computes the greatest common divisor of the exponents at which a polynomial has nonzero
/// coefficients.
///
/// This is the largest $k$ such that $p(x) = q(x^k)$ for some polynomial $q$, when $p$ is not
/// constant. A constant polynomial, including zero, gives 0: its only exponent with a nonzero
/// coefficient, if any, is 0, and the GCD of $\\{0\\}$ and of the empty set are both 0.
pub trait ExponentGcd {
    /// Computes the greatest common divisor of the exponents at which a polynomial has nonzero
    /// coefficients.
    ///
    /// $$
    /// f(p) = \gcd \\{i : p_i \neq 0\\}.
    /// $$
    fn exponent_gcd(&self) -> u64;
}

/// Deflates a polynomial by $n$, giving the polynomial $q$ with $q(x^n) = p(x)$: the coefficient of
/// $x^{in}$ moves to $x^i$.
///
/// This is the inverse of [`ComposePowerOfX`]. It exists only when every exponent at which $p$ has
/// a nonzero coefficient is a multiple of $n$, that is, when $n$ divides
/// [`exponent_gcd`](ExponentGcd::exponent_gcd); implementations panic otherwise, and when $n$ is 0.
/// A constant polynomial deflates to itself for every positive $n$.
pub trait DeflatePowerOfX {
    type Output;

    /// Deflates a polynomial by $n$.
    ///
    /// $$
    /// f(p, n) = q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    fn deflate_power_of_x(self, n: u64) -> Self::Output;
}

/// Deflates a polynomial by $n$ in place, replacing $p$ with the polynomial $q$ such that $q(x^n) =
/// p(x)$.
///
/// This is the inverse of [`ComposePowerOfXAssign`]. It exists only when every exponent at which
/// $p$ has a nonzero coefficient is a multiple of $n$, that is, when $n$ divides
/// [`exponent_gcd`](ExponentGcd::exponent_gcd); implementations panic otherwise, and when $n$ is 0.
/// A constant polynomial deflates to itself for every positive $n$.
pub trait DeflatePowerOfXAssign {
    /// Deflates a polynomial by $n$ in place.
    ///
    /// $$
    /// p \gets q, \quad \text{where} \quad q(x^n) = p(x).
    /// $$
    fn deflate_power_of_x_assign(&mut self, n: u64);
}

/// Packs the coefficients of a polynomial into a single number, placing the coefficient of $x^i$ at
/// bit $ib$; that is, evaluates the polynomial at $2^b$.
///
/// When every coefficient's absolute value is less than $2^b$, the coefficients occupy disjoint
/// $b$-bit fields, with a borrow from the next field above each negative one; this is the
/// representation that Kronecker substitution uses to reduce polynomial multiplication to integer
/// multiplication. Wider coefficients overlap the fields above them, and the result is still
/// $p(2^b)$.
pub trait BitPack {
    type Output;

    /// Packs the coefficients of a polynomial into fields of `bits` bits.
    ///
    /// $$
    /// f(p, b) = p(2^b) = \sum_i a_i2^{ib}.
    /// $$
    fn bit_pack(self, bits: u64) -> Self::Output;
}

/// Unpacks a polynomial from the fixed-width fields of a single number, reading the coefficient of
/// $x^i$ from bit $ib$; the result $p$ satisfies $p(2^b) = n$.
///
/// This inverts [`BitPack`] on polynomials whose coefficients fit their fields.
pub trait BitUnpack<T>: Sized {
    /// Unpacks a polynomial from fields of `bits` bits.
    ///
    /// $$
    /// f(n, b) = p, \quad \text{where} \quad p(2^b) = n.
    /// $$
    fn bit_unpack(n: T, bits: u64) -> Self;
}

/// Makes a polynomial monic, by dividing it by its leading coefficient.
///
/// The zero polynomial has no leading coefficient, and is left as it is.
pub trait MakeMonic {
    /// The type of the monic polynomial.
    type Output;

    /// Makes a polynomial monic.
    fn make_monic(self) -> Self::Output;
}

/// Makes a polynomial monic in place, by dividing it by its leading coefficient.
///
/// See [`MakeMonic`].
pub trait MakeMonicAssign {
    /// Makes a polynomial monic in place.
    fn make_monic_assign(&mut self);
}

/// Evaluates a polynomial at a value.
///
/// Evaluation substitutes a value for the variable. It maps out of the polynomial rather than
/// staying inside it, so the type of the value decides the type of the result, and a polynomial
/// type may implement this trait for several value types.
pub trait Evaluate<T> {
    /// The type of the polynomial's value.
    type Output;

    /// Evaluates a polynomial at `x`.
    ///
    /// $$
    /// f(p, x) = \sum_{i=0}^{n-1} c_i x^i,
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate(self, x: T) -> Self::Output;
}

/// Evaluates a polynomial at each of several values.
pub trait EvaluateMany<T> {
    /// The type of the polynomial's values.
    type Output;

    /// Evaluates a polynomial at each value in `xs`, returning the values in the same order.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    fn evaluate_many(self, xs: &[T]) -> Vec<Self::Output>;
}

/// Builds the monic polynomial with given roots: $(x - r_0)(x - r_1) \cdots (x - r_{n-1})$.
pub trait FromRoots<T> {
    /// Builds the monic polynomial whose roots, with multiplicity, are the values in `roots`.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} (x - r_i).
    /// $$
    fn from_roots(roots: &[T]) -> Self;
}

/// Builds the primitive polynomial with integer coefficients and given rational roots: $(q_0 x -
/// p_0)(q_1 x - p_1) \cdots (q_{n-1} x - p_{n-1})$, where $r_i = p_i/q_i$ in lowest terms.
pub trait FromRationalRoots<T> {
    /// Builds the primitive integer polynomial, with positive leading coefficient, whose roots,
    /// with multiplicity, are the values in `roots`.
    ///
    /// $$
    /// f((p_i/q_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} (q_i x - p_i).
    /// $$
    fn from_rational_roots(roots: &[T]) -> Self;
}
