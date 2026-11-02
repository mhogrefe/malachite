// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use crate::polynomial::modular::traits::{ModDivRemPrecomputed, ModPowerOf2DivRemPrecomputed};

/// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $m$. Both factors
/// must already be reduced modulo $f$, with degrees less than that of $f$, and their coefficients
/// and those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
/// modulo $m$.
pub trait ModMulModPolynomial<Rhs = Self, F = Self, M = Self> {
    type Output;

    /// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_mul_mod_polynomial(self, other: Rhs, f: F, m: M) -> Self::Output;
}

/// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $m$, in
/// place. Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and
/// their coefficients and those of $f$ must be reduced modulo $m$; the leading coefficient of $f$
/// must be invertible modulo $m$.
pub trait ModMulModPolynomialAssign<Rhs = Self, F = Self, M = Self> {
    /// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $m$, in
    /// place.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_mul_mod_polynomial_assign(&mut self, other: Rhs, f: F, m: M);
}

/// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $m$, using data
/// precomputed from $f$ and $m$. Both factors must already be reduced modulo $f$, with degrees less
/// than that of $f$, and their coefficients and those of $f$ must be reduced modulo $m$; the
/// leading coefficient of $f$ must be invertible modulo $m$.
///
/// If many products are taken modulo the same polynomial, it is quicker to precompute the data
/// once, with [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data),
/// and reuse it. The data is the same as for [`ModDivRemPrecomputed`].
pub trait ModMulModPolynomialPrecomputed<Rhs = Self, F = Self, M = Self> {
    type Output;
    type Data;

    /// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $m$, using data
    /// precomputed from $f$ and $m$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_mul_mod_polynomial_precomputed(
        self,
        other: Rhs,
        f: F,
        m: M,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $m$, in
/// place, using data precomputed from $f$ and $m$. Both factors must already be reduced modulo $f$,
/// with degrees less than that of $f$, and their coefficients and those of $f$ must be reduced
/// modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
///
/// For precomputing the data, use
/// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data).
pub trait ModMulModPolynomialPrecomputedAssign<Rhs = Self, F = Self, M = Self>:
    ModMulModPolynomialPrecomputed<Rhs, F, M>
{
    /// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $m$, in
    /// place, using data precomputed from $f$ and $m$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Rhs,
        f: F,
        m: M,
        data: &Self::Data,
    );
}

/// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $2^k$. Both factors
/// must already be reduced modulo $f$, with degrees less than that of $f$, and their coefficients
/// and those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
pub trait ModPowerOf2MulModPolynomial<Rhs = Self, F = Self> {
    type Output;

    /// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $2^k$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_mul_mod_polynomial(self, other: Rhs, f: F, pow: u64) -> Self::Output;
}

/// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $2^k$, in
/// place. Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and
/// their coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$
/// must be odd.
pub trait ModPowerOf2MulModPolynomialAssign<Rhs = Self, F = Self> {
    /// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $2^k$,
    /// in place.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: Rhs, f: F, pow: u64);
}

/// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $2^k$, using data
/// precomputed from $f$ and $k$. Both factors must already be reduced modulo $f$, with degrees less
/// than that of $f$, and their coefficients and those of $f$ must be reduced modulo $2^k$; the
/// leading coefficient of $f$ must be odd.
///
/// If many products are taken modulo the same polynomial, it is quicker to precompute the data
/// once, with [`precompute_mod_power_of_2_div_rem_data`](
/// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data), and reuse it. The data is
/// the same as for [`ModPowerOf2DivRemPrecomputed`].
pub trait ModPowerOf2MulModPolynomialPrecomputed<Rhs = Self, F = Self> {
    type Output;
    type Data;

    /// Multiplies two polynomials modulo a polynomial $f$, with coefficients modulo $2^k$, using
    /// data precomputed from $f$ and $k$.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_mul_mod_polynomial_precomputed(
        self,
        other: Rhs,
        f: F,
        pow: u64,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $2^k$, in
/// place, using data precomputed from $f$ and $k$. Both factors must already be reduced modulo $f$,
/// with degrees less than that of $f$, and their coefficients and those of $f$ must be reduced
/// modulo $2^k$; the leading coefficient of $f$ must be odd.
///
/// For precomputing the data, use [`precompute_mod_power_of_2_div_rem_data`](
/// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data).
pub trait ModPowerOf2MulModPolynomialPrecomputedAssign<Rhs = Self, F = Self>:
    ModPowerOf2MulModPolynomialPrecomputed<Rhs, F>
{
    /// Multiplies a polynomial by another modulo a polynomial $f$, with coefficients modulo $2^k$,
    /// in place, using data precomputed from $f$ and $k$.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_mul_mod_polynomial_precomputed_assign(
        &mut self,
        other: Rhs,
        f: F,
        pow: u64,
        data: &Self::Data,
    );
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$. The base
/// must already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients
/// and those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
/// modulo $m$.
pub trait ModPowModPolynomial<F = Self, M = Self> {
    type Output;

    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \\pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_pow_mod_polynomial(self, exp: u64, f: F, m: M) -> Self::Output;
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$, in place.
/// The base must already be reduced modulo $f$, with a degree less than that of $f$, and its
/// coefficients and those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be
/// invertible modulo $m$.
pub trait ModPowModPolynomialAssign<F = Self, M = Self> {
    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$, in
    /// place.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \\pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: F, m: M);
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$, using data
/// precomputed from $f$ and $m$. The base must already be reduced modulo $f$, with a degree less
/// than that of $f$, and its coefficients and those of $f$ must be reduced modulo $m$; the leading
/// coefficient of $f$ must be invertible modulo $m$.
///
/// If many powers are taken modulo the same polynomial, it is quicker to precompute the data once,
/// with [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data), and
/// reuse it: the data is the same as for division.
pub trait ModPowModPolynomialPrecomputed<F = Self, M = Self> {
    type Output;
    type Data;

    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$, using
    /// data precomputed from $f$ and $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \\pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: F,
        m: M,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$, in place,
/// using data precomputed from $f$ and $m$. The base must already be reduced modulo $f$, with a
/// degree less than that of $f$, and its coefficients and those of $f$ must be reduced modulo $m$;
/// the leading coefficient of $f$ must be invertible modulo $m$.
///
/// For precomputing the data, use
/// [`precompute_mod_div_rem_data`](ModDivRemPrecomputed::precompute_mod_div_rem_data).
pub trait ModPowModPolynomialPrecomputedAssign<F = Self, M = Self>:
    ModPowModPolynomialPrecomputed<F, M>
{
    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $m$, in
    /// place, using data precomputed from $f$ and $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \\pmod{m}, \quad \deg r < \deg f.
    /// $$
    fn mod_pow_mod_polynomial_precomputed_assign(
        &mut self,
        exp: u64,
        f: F,
        m: M,
        data: &Self::Data,
    );
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$. The base
/// must already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients
/// and those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
pub trait ModPowerOf2PowModPolynomial<F = Self> {
    type Output;

    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \\pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_pow_mod_polynomial(self, exp: u64, f: F, pow: u64) -> Self::Output;
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$, in
/// place. The base must already be reduced modulo $f$, with a degree less than that of $f$, and its
/// coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$ must
/// be odd.
pub trait ModPowerOf2PowModPolynomialAssign<F = Self> {
    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$, in
    /// place.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \\pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_pow_mod_polynomial_assign(&mut self, exp: u64, f: F, pow: u64);
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$, using
/// data precomputed from $f$ and $k$. The base must already be reduced modulo $f$, with a degree
/// less than that of $f$, and its coefficients and those of $f$ must be reduced modulo $2^k$; the
/// leading coefficient of $f$ must be odd.
///
/// If many powers are taken modulo the same polynomial, it is quicker to precompute the data once,
/// with [`precompute_mod_power_of_2_div_rem_data`](
/// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data), and reuse it: the data is
/// the same as for division.
pub trait ModPowerOf2PowModPolynomialPrecomputed<F = Self> {
    type Output;
    type Data;

    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$,
    /// using data precomputed from $f$ and $k$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \\pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: F,
        pow: u64,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$, in
/// place, using data precomputed from $f$ and $k$. The base must already be reduced modulo $f$,
/// with a degree less than that of $f$, and its coefficients and those of $f$ must be reduced
/// modulo $2^k$; the leading coefficient of $f$ must be odd.
///
/// For precomputing the data, use [`precompute_mod_power_of_2_div_rem_data`](
/// ModPowerOf2DivRemPrecomputed::precompute_mod_power_of_2_div_rem_data).
pub trait ModPowerOf2PowModPolynomialPrecomputedAssign<F = Self>:
    ModPowerOf2PowModPolynomialPrecomputed<F>
{
    /// Raises a polynomial to a power modulo a polynomial $f$, with coefficients modulo $2^k$, in
    /// place, using data precomputed from $f$ and $k$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \\pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    fn mod_power_of_2_pow_mod_polynomial_precomputed_assign(
        &mut self,
        exp: u64,
        f: F,
        pow: u64,
        data: &Self::Data,
    );
}

/// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $m$. The inner
/// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
/// coefficients of all three must be reduced modulo $m$; the leading coefficient of $h$ must be
/// invertible modulo $m$.
pub trait ModComposeModPolynomial<G = Self, H = Self, M = Self> {
    type Output;

    /// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $m$.
    ///
    /// $$
    /// f(f, g, h) = r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_compose_mod_polynomial(self, g: G, h: H, m: M) -> Self::Output;
}

/// Replaces a polynomial with its composition with another modulo a polynomial $h$, with
/// coefficients modulo $m$.
pub trait ModComposeModPolynomialAssign<G = Self, H = Self, M = Self> {
    /// Replaces `self` with its composition with another polynomial modulo a polynomial $h$, with
    /// coefficients modulo $m$.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_compose_mod_polynomial_assign(&mut self, g: G, h: H, m: M);
}

/// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $m$, using data
/// precomputed from $h$.
pub trait ModComposeModPolynomialPrecomputed<G = Self, H = Self, M = Self> {
    type Output;
    type Data;

    /// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $m$, using data
    /// precomputed from $h$.
    ///
    /// $$
    /// f(f, g, h) = r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: G,
        h: H,
        m: M,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Replaces a polynomial with its composition with another modulo a polynomial $h$, with
/// coefficients modulo $m$, using data precomputed from $h$.
pub trait ModComposeModPolynomialPrecomputedAssign<G = Self, H = Self, M = Self>:
    ModComposeModPolynomialPrecomputed<G, H, M>
{
    /// Replaces `self` with its composition with another polynomial modulo a polynomial $h$, with
    /// coefficients modulo $m$, using data precomputed from $h$.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: G,
        h: H,
        m: M,
        data: &Self::Data,
    );
}

/// Composes a polynomial with a fixed inner polynomial $g$ modulo a fixed polynomial $h$, with
/// coefficients modulo $m$, using the powers of $g$ modulo $h$, precomputed.
pub trait ModComposeModPolynomialPrecomputedPowers<H = Self, M = Self> {
    type Output;
    type Data;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$.
    fn precompute_mod_compose_mod_polynomial_powers_data(g: &H, h: &H, m: &M) -> Self::Data;

    /// Composes a polynomial with the inner polynomial that `data` was precomputed from, modulo a
    /// polynomial $h$, with coefficients modulo $m$.
    ///
    /// $$
    /// f(f, h) = r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: H,
        m: M,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Replaces a polynomial with its composition with a fixed inner polynomial $g$ modulo a fixed
/// polynomial $h$, with coefficients modulo $m$, using the powers of $g$ modulo $h$, precomputed.
pub trait ModComposeModPolynomialPrecomputedPowersAssign<H = Self, M = Self>:
    ModComposeModPolynomialPrecomputedPowers<H, M>
{
    /// Replaces `self` with its composition with the inner polynomial that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: H,
        m: M,
        data: &Self::Data,
    );
}

/// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $2^k$. The inner
/// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
/// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$ must be
/// invertible modulo $2^k$.
pub trait ModPowerOf2ComposeModPolynomial<G = Self, H = Self> {
    type Output;

    /// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $2^k$.
    ///
    /// $$
    /// f(f, g, h) = r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_power_of_2_compose_mod_polynomial(self, g: G, h: H, pow: u64) -> Self::Output;
}

/// Replaces a polynomial with its composition with another modulo a polynomial $h$, with
/// coefficients modulo $2^k$.
pub trait ModPowerOf2ComposeModPolynomialAssign<G = Self, H = Self> {
    /// Replaces `self` with its composition with another polynomial modulo a polynomial $h$, with
    /// coefficients modulo $2^k$.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: G, h: H, pow: u64);
}

/// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $2^k$, using data
/// precomputed from $h$.
pub trait ModPowerOf2ComposeModPolynomialPrecomputed<G = Self, H = Self> {
    type Output;
    type Data;

    /// Composes two polynomials modulo a polynomial $h$, with coefficients modulo $2^k$, using data
    /// precomputed from $h$.
    ///
    /// $$
    /// f(f, g, h) = r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: G,
        h: H,
        pow: u64,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Replaces a polynomial with its composition with another modulo a polynomial $h$, with
/// coefficients modulo $2^k$, using data precomputed from $h$.
pub trait ModPowerOf2ComposeModPolynomialPrecomputedAssign<G = Self, H = Self>:
    ModPowerOf2ComposeModPolynomialPrecomputed<G, H>
{
    /// Replaces `self` with its composition with another polynomial modulo a polynomial $h$, with
    /// coefficients modulo $2^k$, using data precomputed from $h$.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_power_of_2_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: G,
        h: H,
        pow: u64,
        data: &Self::Data,
    );
}

/// Composes a polynomial with a fixed inner polynomial $g$ modulo a fixed polynomial $h$, with
/// coefficients modulo $2^k$, using the powers of $g$ modulo $h$, precomputed.
pub trait ModPowerOf2ComposeModPolynomialPrecomputedPowers<H = Self> {
    type Output;
    type Data;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $2^k$.
    fn precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
        g: &H,
        h: &H,
        pow: &u64,
    ) -> Self::Data;

    /// Composes a polynomial with the inner polynomial that `data` was precomputed from, modulo a
    /// polynomial $h$, with coefficients modulo $2^k$.
    ///
    /// $$
    /// f(f, h) = r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers(
        self,
        h: H,
        pow: u64,
        data: &Self::Data,
    ) -> Self::Output;
}

/// Replaces a polynomial with its composition with a fixed inner polynomial $g$ modulo a fixed
/// polynomial $h$, with coefficients modulo $2^k$, using the powers of $g$ modulo $h$, precomputed.
pub trait ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign<H = Self>:
    ModPowerOf2ComposeModPolynomialPrecomputedPowers<H>
{
    /// Replaces `self` with its composition with the inner polynomial that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $2^k$.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r, \quad \deg r < \deg h.
    /// $$
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: H,
        pow: u64,
        data: &Self::Data,
    );
}
