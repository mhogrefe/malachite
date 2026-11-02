// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Determines whether two polynomials agree below a given power of the variable.
///
/// This is equality of the two polynomials truncated to their first `len` coefficients, decided
/// without building either truncation.
pub trait EqTruncated<Rhs: ?Sized = Self> {
    /// Determines whether two polynomials have the same coefficient of $x^i$ for every $i$ less
    /// than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = (p \bmod x^n = q \bmod x^n).
    /// $$
    fn eq_truncated(&self, other: &Rhs, len: u64) -> bool;
}

/// Adds two polynomials, keeping only the coefficients of $x^i$ for $i$ less than a given length.
///
/// This is the sum of the two polynomials truncated to their first `len` coefficients, which is
/// also the truncation of their sum.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait AddTruncated<Rhs = Self> {
    type Output;

    /// Adds two polynomials and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    fn add_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Adds a polynomial to another in place, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait AddTruncatedAssign<Rhs = Self> {
    /// Adds a polynomial to `self` and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// p \gets (p + q) \bmod x^n.
    /// $$
    fn add_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Subtracts one polynomial from another, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length.
///
/// This is the difference of the two polynomials truncated to their first `len` coefficients, which
/// is also the truncation of their difference.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait SubTruncated<Rhs = Self> {
    type Output;

    /// Subtracts one polynomial from another and truncates the difference to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n) = (p - q) \bmod x^n.
    /// $$
    fn sub_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Subtracts a polynomial from another in place, keeping only the coefficients of $x^i$ for $i$
/// less than a given length.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials modulo $x^n$, applied to
/// the images of the two polynomials there. The polynomials need not already be truncated: they may
/// have any number of coefficients, and only the first `len` of each are read.
pub trait SubTruncatedAssign<Rhs = Self> {
    /// Subtracts a polynomial from `self` and truncates the difference to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (p - q) \bmod x^n.
    /// $$
    fn sub_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Multiplies two polynomials, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials modulo $x^n$, applied
/// to the images of the two polynomials there. The polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait MulTruncated<Rhs = Self> {
    type Output;

    /// Multiplies two polynomials and truncates the product to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    fn mul_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Multiplies a polynomial by another in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials modulo $x^n$, applied
/// to the images of the two polynomials there. The polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait MulTruncatedAssign<Rhs = Self> {
    /// Multiplies `self` by a polynomial and truncates the product to its first `len` coefficients.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    fn mul_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Inverts a polynomial as a power series, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length.
///
/// With $n$ equal to `len`, this is inversion in the ring of polynomials modulo $x^n$. Over the
/// integers, the inverse exists exactly when the constant coefficient is $\pm 1$; otherwise `None`
/// is returned.
pub trait InverseTruncated {
    type Output;

    /// Inverts a polynomial as a power series, truncated to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n) = q \bmod x^n, \quad pq \equiv 1 \pmod{x^n}.
    /// $$
    fn inverse_truncated(self, len: u64) -> Option<Self::Output>;
}

/// Finds the compositional inverse of a polynomial as a power series, truncated to its first `len`
/// coefficients: the series $q$ with $p(q(x)) = x$. The constant coefficient of $p$ must be 0 and
/// its coefficient of $x$ must be a unit; otherwise there is no inverse.
pub trait CompositionalInverseTruncated {
    type Output;

    /// Finds the compositional inverse of a polynomial as a power series, truncated to its first
    /// `len` coefficients.
    ///
    /// $$
    /// f(p, n) = q \bmod x^n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n}.
    /// $$
    fn compositional_inverse_truncated(self, len: u64) -> Option<Self::Output>;
}

/// Divides one polynomial by another as power series, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The constant coefficient of the divisor must be nonzero, and the
/// quotient must have coefficients in the polynomials' coefficient ring.
///
/// With $n$ equal to `len`, this is division in the ring of polynomials modulo $x^n$.
pub trait DivTruncated<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another as power series, truncating the quotient to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    fn div_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Divides one polynomial by another as power series in place, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length. The constant coefficient of the divisor must be nonzero,
/// and the quotient must have coefficients in the polynomials' coefficient ring.
pub trait DivTruncatedAssign<Rhs = Self> {
    /// Divides `self` by another polynomial as power series, truncating the quotient to its first
    /// `len` coefficients.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n}.
    /// $$
    fn div_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Divides one polynomial by another as power series, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length, returning `None` if the constant coefficient of the divisor is
/// zero or the quotient does not have coefficients in the polynomials' coefficient ring.
pub trait CheckedDivTruncated<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another as power series, truncating the quotient to its first `len`
    /// coefficients, or returns `None` if there is no such quotient.
    ///
    /// $$
    /// f(p, q, n) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \neq 0 \text{ and } r \text{ exists}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n}$.
    fn checked_div_truncated(self, other: Rhs, len: u64) -> Option<Self::Output>;
}

/// Squares a polynomial, keeping only the coefficients of $x^i$ for $i$ less than a given length.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait SquareTruncated {
    type Output;

    /// Squares a polynomial and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n) = p^2 \bmod x^n.
    /// $$
    fn square_truncated(self, len: u64) -> Self::Output;
}

/// Squares a polynomial in place, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait SquareTruncatedAssign {
    /// Squares `self` and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// p \gets p^2 \bmod x^n.
    /// $$
    fn square_truncated_assign(&mut self, len: u64);
}

/// Raises a polynomial to a power, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait PowTruncated {
    type Output;

    /// Raises a polynomial to the power `exp` and truncates the power to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    fn pow_truncated(self, exp: u64, len: u64) -> Self::Output;
}

/// Raises a polynomial to a power in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials modulo $x^n$, applied to
/// the image of the polynomial there. The polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait PowTruncatedAssign {
    /// Raises `self` to the power `exp` and truncates the power to its first `len` coefficients.
    ///
    /// $$
    /// p \gets p^e \bmod x^n.
    /// $$
    fn pow_truncated_assign(&mut self, exp: u64, len: u64);
}

/// Adds two polynomials modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2AddTruncated<Rhs = Self> {
    type Output;

    /// Adds two polynomials modulo $2^k$ and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_add_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Adds a polynomial to another modulo $2^k$ in place, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2AddTruncatedAssign<Rhs = Self> {
    /// Adds a polynomial to `self` modulo $2^k$ and truncates the sum to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_add_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Subtracts one polynomial from another modulo $2^k$, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2SubTruncated<Rhs = Self> {
    type Output;

    /// Subtracts one polynomial from another modulo $2^k$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = ((p - q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_sub_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Subtracts a polynomial from another modulo $2^k$ in place, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length. The coefficients of both must already be reduced modulo
/// $2^k$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2SubTruncatedAssign<Rhs = Self> {
    /// Subtracts a polynomial from `self` modulo $2^k$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// p \gets ((p - q) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_sub_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Multiplies two polynomials modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2MulTruncated<Rhs = Self> {
    type Output;

    /// Multiplies two polynomials modulo $2^k$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_mul_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Multiplies a polynomial by another modulo $2^k$ in place, keeping only the coefficients of $x^i$
/// for $i$ less than a given length. The coefficients of both must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $2^k$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModPowerOf2MulTruncatedAssign<Rhs = Self> {
    /// Multiplies `self` by a polynomial modulo $2^k$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Squares a polynomial modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the
/// coefficients, which must already be reduced, the polynomial need not already be truncated: it
/// may have any number of coefficients, and only the first `len` are read.
pub trait ModPowerOf2SquareTruncated {
    type Output;

    /// Squares a polynomial modulo $2^k$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_square_truncated(self, len: u64, pow: u64) -> Self::Output;
}

/// Squares a polynomial modulo $2^k$ in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the
/// coefficients, which must already be reduced, the polynomial need not already be truncated: it
/// may have any number of coefficients, and only the first `len` are read.
pub trait ModPowerOf2SquareTruncatedAssign {
    /// Squares `self` modulo $2^k$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_square_truncated_assign(&mut self, len: u64, pow: u64);
}

/// Raises a polynomial to a power modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the
/// coefficients, which must already be reduced, the polynomial need not already be truncated: it
/// may have any number of coefficients, and only the first `len` are read.
pub trait ModPowerOf2PowTruncated {
    type Output;

    /// Raises a polynomial to the power `exp` modulo $2^k$ and truncates the power to its first
    /// `len` coefficients.
    ///
    /// $$
    /// f(p, e, n, k) = (p^e \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_pow_truncated(self, exp: u64, len: u64, pow: u64) -> Self::Output;
}

/// Raises a polynomial to a power modulo $2^k$ in place, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the
/// coefficients, which must already be reduced, the polynomial need not already be truncated: it
/// may have any number of coefficients, and only the first `len` are read.
pub trait ModPowerOf2PowTruncatedAssign {
    /// Raises `self` to the power `exp` modulo $2^k$ and truncates the power to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (p^e \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_pow_truncated_assign(&mut self, exp: u64, len: u64, pow: u64);
}

/// Inverts a polynomial as a power series modulo $2^k$, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients must already be reduced modulo $2^k$.
///
/// With $n$ equal to `len`, this is inversion in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$. The inverse exists exactly when the constant coefficient is odd;
/// otherwise `None` is returned.
pub trait ModPowerOf2InverseTruncated {
    type Output;

    /// Inverts a polynomial as a power series modulo $2^k$, truncated to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, n, k) = q \bmod x^n, \quad pq \equiv 1 \pmod{x^n, 2^k}.
    /// $$
    fn mod_power_of_2_inverse_truncated(self, len: u64, pow: u64) -> Option<Self::Output>;
}

/// Finds the compositional inverse of a polynomial as a power series modulo $2^k$, truncated to its
/// first `len` coefficients. The constant coefficient must be 0 and the coefficient of $x$ must be
/// odd; otherwise there is no inverse.
pub trait ModPowerOf2CompositionalInverseTruncated {
    type Output;

    /// Finds the compositional inverse of a polynomial as a power series modulo $2^k$, truncated to
    /// its first `len` coefficients.
    ///
    /// $$
    /// f(p, n, k) = q \bmod x^n, \quad p(q(x)) \equiv x \pmod{x^n, 2^k}.
    /// $$
    fn mod_power_of_2_compositional_inverse_truncated(
        self,
        len: u64,
        pow: u64,
    ) -> Option<Self::Output>;
}

/// Divides one polynomial by another as power series modulo $2^k$, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length. The coefficients must already be reduced modulo $2^k$,
/// and the constant coefficient of the divisor must be odd.
///
/// With $n$ equal to `len`, this is division in the ring of polynomials with coefficients modulo
/// $2^k$, taken modulo $x^n$, where a series is invertible exactly when its constant coefficient is
/// odd.
pub trait ModPowerOf2DivTruncated<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another as power series modulo $2^k$, truncating the quotient to its
    /// first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    fn mod_power_of_2_div_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Divides one polynomial by another as power series modulo $2^k$ in place, keeping only the
/// coefficients of $x^i$ for $i$ less than a given length. The coefficients must already be reduced
/// modulo $2^k$, and the constant coefficient of the divisor must be odd.
pub trait ModPowerOf2DivTruncatedAssign<Rhs = Self> {
    /// Divides `self` by another polynomial as power series modulo $2^k$, truncating the quotient
    /// to its first `len` coefficients.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, 2^k}.
    /// $$
    fn mod_power_of_2_div_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}

/// Divides one polynomial by another as power series modulo $2^k$, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length, returning `None` if the constant coefficient of the
/// divisor is even. The coefficients must already be reduced modulo $2^k$.
pub trait CheckedModPowerOf2DivTruncated<Rhs = Self> {
    type Output;

    /// Divides a polynomial by another as power series modulo $2^k$, truncating the quotient to its
    /// first `len` coefficients, or returns `None` if the divisor is not invertible.
    ///
    /// $$
    /// f(p, q, n, k) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \text{ is odd}, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n, 2^k}$.
    fn checked_mod_power_of_2_div_truncated(
        self,
        other: Rhs,
        len: u64,
        pow: u64,
    ) -> Option<Self::Output>;
}

/// Adds two polynomials modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModAddTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Adds two polynomials modulo $m$ and truncates the sum to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = ((p + q) \bmod x^n) \bmod m.
    /// $$
    fn mod_add_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Adds a polynomial to another modulo $m$ in place, keeping only the coefficients of $x^i$ for $i$
/// less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is addition in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModAddTruncatedAssign<Rhs = Self, M = Self> {
    /// Adds a polynomial to `self` modulo $m$ and truncates the sum to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod m.
    /// $$
    fn mod_add_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Subtracts one polynomial from another modulo $m$, keeping only the coefficients of $x^i$ for $i$
/// less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModSubTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Subtracts one polynomial from another modulo $m$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = ((p - q) \bmod x^n) \bmod m.
    /// $$
    fn mod_sub_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Subtracts a polynomial from another modulo $m$ in place, keeping only the coefficients of $x^i$
/// for $i$ less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is subtraction in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModSubTruncatedAssign<Rhs = Self, M = Self> {
    /// Subtracts a polynomial from `self` modulo $m$ and truncates the difference to its first
    /// `len` coefficients.
    ///
    /// $$
    /// p \gets ((p - q) \bmod x^n) \bmod m.
    /// $$
    fn mod_sub_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Multiplies two polynomials modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModMulTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Multiplies two polynomials modulo $m$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
    /// $$
    fn mod_mul_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Multiplies a polynomial by another modulo $m$ in place, keeping only the coefficients of $x^i$
/// for $i$ less than a given length. The coefficients of both must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is multiplication in the ring of polynomials with coefficients
/// modulo $m$, taken modulo $x^n$, applied to the images of the two polynomials there. Unlike the
/// coefficients, which must already be reduced, the polynomials need not already be truncated: they
/// may have any number of coefficients, and only the first `len` of each are read.
pub trait ModMulTruncatedAssign<Rhs = Self, M = Self> {
    /// Multiplies `self` by a polynomial modulo $m$ and truncates the product to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
    /// $$
    fn mod_mul_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Squares a polynomial modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the coefficients,
/// which must already be reduced, the polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait ModSquareTruncated<M = Self> {
    type Output;

    /// Squares a polynomial modulo $m$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
    /// $$
    fn mod_square_truncated(self, len: u64, m: M) -> Self::Output;
}

/// Squares a polynomial modulo $m$ in place, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is squaring in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the coefficients,
/// which must already be reduced, the polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait ModSquareTruncatedAssign<M = Self> {
    /// Squares `self` modulo $m$ and truncates the square to its first `len` coefficients.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod m.
    /// $$
    fn mod_square_truncated_assign(&mut self, len: u64, m: M);
}

/// Raises a polynomial to a power modulo $m$, keeping only the coefficients of $x^i$ for $i$ less
/// than a given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the coefficients,
/// which must already be reduced, the polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait ModPowTruncated<M = Self> {
    type Output;

    /// Raises a polynomial to the power `exp` modulo $m$ and truncates the power to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, e, n, m) = (p^e \bmod x^n) \bmod m.
    /// $$
    fn mod_pow_truncated(self, exp: u64, len: u64, m: M) -> Self::Output;
}

/// Raises a polynomial to a power modulo $m$ in place, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is powering in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, applied to the image of the polynomial there. Unlike the coefficients,
/// which must already be reduced, the polynomial need not already be truncated: it may have any
/// number of coefficients, and only the first `len` are read.
pub trait ModPowTruncatedAssign<M = Self> {
    /// Raises `self` to the power `exp` modulo $m$ and truncates the power to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (p^e \bmod x^n) \bmod m.
    /// $$
    fn mod_pow_truncated_assign(&mut self, exp: u64, len: u64, m: M);
}

/// Inverts a polynomial as a power series modulo $m$, keeping only the coefficients of $x^i$ for
/// $i$ less than a given length. The coefficients must already be reduced modulo $m$.
///
/// With $n$ equal to `len`, this is inversion in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$. The inverse exists exactly when the constant coefficient is invertible
/// modulo $m$; otherwise `None` is returned.
pub trait ModInverseTruncated<M = Self> {
    type Output;

    /// Inverts a polynomial as a power series modulo $m$, truncated to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, n, m) = q \bmod x^n, \quad pq \equiv 1 \pmod{x^n, m}.
    /// $$
    fn mod_inverse_truncated(self, len: u64, m: M) -> Option<Self::Output>;
}

/// Finds the compositional inverse of a polynomial as a power series modulo $m$, truncated to its
/// first `len` coefficients. The constant coefficient must be 0 and the coefficient of $x$ must be
/// invertible modulo $m$; otherwise there is no inverse.
pub trait ModCompositionalInverseTruncated<M = Self> {
    type Output;

    /// Finds the compositional inverse of a polynomial as a power series modulo $m$, truncated to
    /// its first `len` coefficients.
    ///
    /// $$
    /// f(p, n, m) = q \bmod x^n, \quad p(q(x)) \equiv x \pmod{x^n, m}.
    /// $$
    fn mod_compositional_inverse_truncated(self, len: u64, m: M) -> Option<Self::Output>;
}

/// Divides one polynomial by another as power series modulo $m$, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length. The coefficients must already be reduced modulo $m$, and
/// the constant coefficient of the divisor must be invertible modulo $m$.
///
/// With $n$ equal to `len`, this is division in the ring of polynomials with coefficients modulo
/// $m$, taken modulo $x^n$, where a series is invertible exactly when its constant coefficient is.
pub trait ModDivTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Divides a polynomial by another as power series modulo $m$, truncating the quotient to its
    /// first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    fn mod_div_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Divides one polynomial by another as power series modulo $m$ in place, keeping only the
/// coefficients of $x^i$ for $i$ less than a given length. The coefficients must already be reduced
/// modulo $m$, and the constant coefficient of the divisor must be invertible modulo $m$.
pub trait ModDivTruncatedAssign<Rhs = Self, M = Self> {
    /// Divides `self` by another polynomial as power series modulo $m$, truncating the quotient to
    /// its first `len` coefficients.
    ///
    /// $$
    /// p \gets r, \quad \deg r < n, \quad qr \equiv p \pmod{x^n, m}.
    /// $$
    fn mod_div_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Divides one polynomial by another as power series modulo $m$, keeping only the coefficients of
/// $x^i$ for $i$ less than a given length, returning `None` if the constant coefficient of the
/// divisor is not invertible modulo $m$. The coefficients must already be reduced modulo $m$.
pub trait CheckedModDivTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Divides a polynomial by another as power series modulo $m$, truncating the quotient to its
    /// first `len` coefficients, or returns `None` if the divisor is not invertible.
    ///
    /// $$
    /// f(p, q, n, m) = \\begin{cases}
    ///     \operatorname{Some}(r) & \text{if } q_0 \text{ is invertible modulo } m, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $\deg r < n$ and $qr \equiv p \pmod{x^n, m}$.
    fn checked_mod_div_truncated(self, other: Rhs, len: u64, m: M) -> Option<Self::Output>;
}

/// Composes two polynomials, keeping only the coefficients of $x^i$ for $i$ less than a given
/// length: composition of power series. The inner polynomial must have constant coefficient 0.
///
/// With $n$ equal to `len`, this is $p(q(x)) \bmod x^n$. Since $q(0) = 0$, the coefficients of
/// $x^i$ for $i \geq n$ of $p$ and $q$ do not affect the result.
pub trait ComposeTruncated<Rhs = Self> {
    type Output;

    /// Composes two polynomials and truncates the result to its first `len` coefficients.
    ///
    /// $$
    /// f(p, q, n) = p(q(x)) \bmod x^n.
    /// $$
    fn compose_truncated(self, other: Rhs, len: u64) -> Self::Output;
}

/// Composes two polynomials in place, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length: composition of power series. The inner polynomial must have constant coefficient
/// 0.
pub trait ComposeTruncatedAssign<Rhs = Self> {
    /// Composes `self` with a polynomial and truncates the result to its first `len` coefficients.
    ///
    /// $$
    /// p \gets p(q(x)) \bmod x^n.
    /// $$
    fn compose_truncated_assign(&mut self, other: Rhs, len: u64);
}

/// Composes two polynomials modulo $m$, keeping only the coefficients of $x^i$ for $i$ less than a
/// given length: composition of power series with coefficients modulo $m$. The inner polynomial
/// must have constant coefficient 0, and the coefficients of both must already be reduced modulo
/// $m$.
///
/// With $n$ equal to `len`, this is $p(q(x)) \bmod x^n$ with coefficients modulo $m$. Since $q(0) =
/// 0$, the coefficients of $x^i$ for $i \geq n$ of $p$ and $q$ do not affect the result.
pub trait ModComposeTruncated<Rhs = Self, M = Self> {
    type Output;

    /// Composes two polynomials modulo $m$ and truncates the result to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \bmod x^n) \bmod m.
    /// $$
    fn mod_compose_truncated(self, other: Rhs, len: u64, m: M) -> Self::Output;
}

/// Composes two polynomials modulo $m$ in place, keeping only the coefficients of $x^i$ for $i$
/// less than a given length: composition of power series with coefficients modulo $m$. The inner
/// polynomial must have constant coefficient 0, and the coefficients of both must already be
/// reduced modulo $m$.
pub trait ModComposeTruncatedAssign<Rhs = Self, M = Self> {
    /// Composes `self` with a polynomial modulo $m$ and truncates the result to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (p(q(x)) \bmod x^n) \bmod m.
    /// $$
    fn mod_compose_truncated_assign(&mut self, other: Rhs, len: u64, m: M);
}

/// Composes two polynomials modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than
/// a given length: composition of power series with coefficients modulo $2^k$. The inner polynomial
/// must have constant coefficient 0, and the coefficients of both must already be reduced modulo
/// $2^k$.
///
/// With $n$ equal to `len`, this is $p(q(x)) \bmod x^n$ with coefficients modulo $2^k$. Since $q(0)
/// = 0$, the coefficients of $x^i$ for $i \geq n$ of $p$ and $q$ do not affect the result.
pub trait ModPowerOf2ComposeTruncated<Rhs = Self> {
    type Output;

    /// Composes two polynomials modulo $2^k$ and truncates the result to its first `len`
    /// coefficients.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_compose_truncated(self, other: Rhs, len: u64, pow: u64) -> Self::Output;
}

/// Composes two polynomials modulo $2^k$ in place, keeping only the coefficients of $x^i$ for $i$
/// less than a given length: composition of power series with coefficients modulo $2^k$. The inner
/// polynomial must have constant coefficient 0, and the coefficients of both must already be
/// reduced modulo $2^k$.
pub trait ModPowerOf2ComposeTruncatedAssign<Rhs = Self> {
    /// Composes `self` with a polynomial modulo $2^k$ and truncates the result to its first `len`
    /// coefficients.
    ///
    /// $$
    /// p \gets (p(q(x)) \bmod x^n) \bmod 2^k.
    /// $$
    fn mod_power_of_2_compose_truncated_assign(&mut self, other: Rhs, len: u64, pow: u64);
}
