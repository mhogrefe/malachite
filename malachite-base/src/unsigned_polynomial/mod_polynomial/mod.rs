// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use crate::polynomial::mod_polynomial::traits::{
    ModMulModPolynomial, ModMulModPolynomialAssign, ModMulModPolynomialPrecomputed,
    ModMulModPolynomialPrecomputedAssign, ModPowModPolynomial, ModPowModPolynomialAssign,
    ModPowModPolynomialPrecomputed, ModPowModPolynomialPrecomputedAssign,
    ModPowerOf2MulModPolynomial, ModPowerOf2MulModPolynomialAssign,
    ModPowerOf2MulModPolynomialPrecomputed, ModPowerOf2MulModPolynomialPrecomputedAssign,
    ModPowerOf2PowModPolynomial, ModPowerOf2PowModPolynomialAssign,
    ModPowerOf2PowModPolynomialPrecomputed, ModPowerOf2PowModPolynomialPrecomputedAssign,
};

/// Modular composition of polynomials modulo a polynomial.
pub mod mod_compose_mod_polynomial;
/// Modular composition of several polynomials with the same inner polynomial modulo a polynomial.
pub mod mod_compose_mod_polynomial_many;
/// Modular composition of polynomials modulo a polynomial, using precomputed data for division.
pub mod mod_compose_mod_polynomial_precomputed;
/// Modular composition of several polynomials with the same inner polynomial modulo a polynomial,
/// using precomputed data for division.
pub mod mod_compose_mod_polynomial_precomputed_many;
/// Modular composition of polynomials modulo a polynomial, using precomputed powers of the inner
/// polynomial.
pub mod mod_compose_mod_polynomial_precomputed_powers;
/// Implementations of [`ModMulModPolynomial`] and [`ModMulModPolynomialAssign`], for multiplying
/// polynomials modulo a polynomial, with coefficients modulo a number.
pub mod mod_mul_mod_polynomial;
/// Implementations of [`ModMulModPolynomialPrecomputed`] and
/// [`ModMulModPolynomialPrecomputedAssign`], for multiplying polynomials modulo a polynomial, with
/// coefficients modulo a number, with data precomputed from the modulus.
pub mod mod_mul_mod_polynomial_precomputed;
/// Implementations of [`ModPowModPolynomial`] and [`ModPowModPolynomialAssign`], for raising
/// polynomials to powers modulo a polynomial, with coefficients modulo a number.
pub mod mod_pow_mod_polynomial;
/// Implementations of [`ModPowModPolynomialPrecomputed`] and
/// [`ModPowModPolynomialPrecomputedAssign`], for raising polynomials to powers modulo a polynomial,
/// with coefficients modulo a number, with data precomputed from the modulus.
pub mod mod_pow_mod_polynomial_precomputed;
/// Modular composition of polynomials modulo a polynomial, with coefficients modulo a power of 2.
pub mod mod_power_of_2_compose_mod_polynomial;
/// Modular composition of several polynomials with the same inner polynomial modulo a polynomial,
/// with coefficients modulo a power of 2.
pub mod mod_power_of_2_compose_mod_polynomial_many;
/// Modular composition of polynomials modulo a polynomial, with coefficients modulo a power of 2,
/// using precomputed data for division.
pub mod mod_power_of_2_compose_mod_polynomial_precomputed;
/// Modular composition of several polynomials with the same inner polynomial modulo a polynomial,
/// with coefficients modulo a power of 2, using precomputed data for division.
pub mod mod_power_of_2_compose_mod_polynomial_precomputed_many;
/// Modular composition of polynomials modulo a polynomial, with coefficients modulo a power of 2,
/// using precomputed powers of the inner polynomial.
pub mod mod_power_of_2_compose_mod_polynomial_precomputed_powers;
/// Implementations of [`ModPowerOf2MulModPolynomial`] and [`ModPowerOf2MulModPolynomialAssign`],
/// for multiplying polynomials modulo a polynomial, with coefficients modulo a power of 2.
pub mod mod_power_of_2_mul_mod_polynomial;
/// Implementations of [`ModPowerOf2MulModPolynomialPrecomputed`] and
/// [`ModPowerOf2MulModPolynomialPrecomputedAssign`], for multiplying polynomials modulo a
/// polynomial, with coefficients modulo a power of 2, with data precomputed from the modulus.
pub mod mod_power_of_2_mul_mod_polynomial_precomputed;
/// Implementations of [`ModPowerOf2PowModPolynomial`] and [`ModPowerOf2PowModPolynomialAssign`],
/// for raising polynomials to powers modulo a polynomial, with coefficients modulo a power of 2.
pub mod mod_power_of_2_pow_mod_polynomial;
/// Implementations of [`ModPowerOf2PowModPolynomialPrecomputed`] and
/// [`ModPowerOf2PowModPolynomialPrecomputedAssign`], for raising polynomials to powers modulo a
/// polynomial, with coefficients modulo a power of 2, with data precomputed from the modulus.
pub mod mod_power_of_2_pow_mod_polynomial_precomputed;
