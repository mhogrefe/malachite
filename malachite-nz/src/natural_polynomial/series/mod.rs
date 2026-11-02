// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::polynomial::series::traits::{
    AddTruncated, AddTruncatedAssign, CheckedModDivTruncated, CheckedModPowerOf2DivTruncated,
    ComposeTruncated, ComposeTruncatedAssign, ModAddTruncated, ModAddTruncatedAssign,
    ModComposeTruncated, ModComposeTruncatedAssign, ModDivTruncated, ModDivTruncatedAssign,
    ModInverseTruncated, ModMulTruncated, ModMulTruncatedAssign, ModPowTruncated,
    ModPowTruncatedAssign, ModPowerOf2AddTruncated, ModPowerOf2AddTruncatedAssign,
    ModPowerOf2ComposeTruncated, ModPowerOf2ComposeTruncatedAssign, ModPowerOf2DivTruncated,
    ModPowerOf2DivTruncatedAssign, ModPowerOf2InverseTruncated, ModPowerOf2MulTruncated,
    ModPowerOf2MulTruncatedAssign, ModPowerOf2PowTruncated, ModPowerOf2PowTruncatedAssign,
    ModPowerOf2SquareTruncated, ModPowerOf2SquareTruncatedAssign, ModPowerOf2SubTruncated,
    ModPowerOf2SubTruncatedAssign, ModSquareTruncated, ModSquareTruncatedAssign, ModSubTruncated,
    ModSubTruncatedAssign, MulTruncated, MulTruncatedAssign, PowTruncated, PowTruncatedAssign,
    SquareTruncated, SquareTruncatedAssign,
};

/// Implementations of [`AddTruncated`] and [`AddTruncatedAssign`], for adding two polynomials and
/// keeping only their low coefficients.
pub mod add_truncated;
/// Implementations of [`ComposeTruncated`] and [`ComposeTruncatedAssign`], for composing two
/// polynomials as power series.
pub mod compose_truncated;
/// Implementations of [`ModAddTruncated`] and [`ModAddTruncatedAssign`], for adding two polynomials
/// modulo a [`Natural`](crate::natural::Natural) and keeping only their low coefficients.
pub mod mod_add_truncated;
/// Implementations of [`ModComposeTruncated`] and [`ModComposeTruncatedAssign`], for composing two
/// polynomials modulo $m$ as power series, keeping only the low coefficients of the result.
pub mod mod_compose_truncated;
/// Series reversion modulo a number: the compositional inverse of a polynomial as a power series.
pub mod mod_compositional_inverse_truncated;
/// Implementations of [`ModDivTruncated`], [`ModDivTruncatedAssign`], and
/// [`CheckedModDivTruncated`], for dividing polynomials as power series modulo a number, keeping
/// only the low coefficients.
pub mod mod_div_truncated;
/// Implementations of [`ModInverseTruncated`], for inverting a polynomial as a power series modulo
/// a number, keeping only the low coefficients.
pub mod mod_inverse_truncated;
/// Implementations of [`ModMulTruncated`] and [`ModMulTruncatedAssign`], for multiplying two
/// polynomials modulo a [`Natural`](crate::natural::Natural) and keeping only the low coefficients
/// of the product.
pub mod mod_mul_truncated;
/// Implementations of [`ModPowTruncated`] and [`ModPowTruncatedAssign`], for raising a polynomial
/// to a power modulo a number and keeping only the low coefficients.
pub mod mod_pow_truncated;
/// Implementations of [`ModPowerOf2AddTruncated`] and [`ModPowerOf2AddTruncatedAssign`], for adding
/// two polynomials modulo $2^k$ and keeping only their low coefficients.
pub mod mod_power_of_2_add_truncated;
/// Implementations of [`ModPowerOf2ComposeTruncated`] and [`ModPowerOf2ComposeTruncatedAssign`],
/// for composing two polynomials modulo $2^k$ as power series, keeping only the low coefficients of
/// the result.
pub mod mod_power_of_2_compose_truncated;
/// Series reversion modulo a power of 2: the compositional inverse of a polynomial as a power
/// series.
pub mod mod_power_of_2_compositional_inverse_truncated;
/// Implementations of [`ModPowerOf2DivTruncated`], [`ModPowerOf2DivTruncatedAssign`], and
/// [`CheckedModPowerOf2DivTruncated`], for dividing polynomials as power series modulo a power of
/// 2, keeping only the low coefficients.
pub mod mod_power_of_2_div_truncated;
/// Implementations of [`ModPowerOf2InverseTruncated`], for inverting a polynomial as a power series
/// modulo a power of 2, keeping only the low coefficients.
pub mod mod_power_of_2_inverse_truncated;
/// Implementations of [`ModPowerOf2MulTruncated`] and [`ModPowerOf2MulTruncatedAssign`], for
/// multiplying two polynomials modulo $2^k$ and keeping only the low coefficients of the product.
pub mod mod_power_of_2_mul_truncated;
/// Implementations of [`ModPowerOf2PowTruncated`] and [`ModPowerOf2PowTruncatedAssign`], for
/// raising a polynomial to a power modulo a power of 2 and keeping only the low coefficients.
pub mod mod_power_of_2_pow_truncated;
/// Implementations of [`ModPowerOf2SquareTruncated`] and [`ModPowerOf2SquareTruncatedAssign`], for
/// squaring a polynomial modulo $2^k$ and keeping only the low coefficients of the square.
pub mod mod_power_of_2_square_truncated;
/// Implementations of [`ModPowerOf2SubTruncated`] and [`ModPowerOf2SubTruncatedAssign`], for
/// subtracting one polynomial from another modulo $2^k$ and keeping only their low coefficients.
pub mod mod_power_of_2_sub_truncated;
/// Implementations of [`ModSquareTruncated`] and [`ModSquareTruncatedAssign`], for squaring a
/// polynomial modulo a [`Natural`](crate::natural::Natural) and keeping only the low coefficients
/// of the square.
pub mod mod_square_truncated;
/// Implementations of [`ModSubTruncated`] and [`ModSubTruncatedAssign`], for subtracting one
/// polynomial from another modulo a [`Natural`](crate::natural::Natural) and keeping only their low
/// coefficients.
pub mod mod_sub_truncated;
/// Implementations of [`MulTruncated`] and [`MulTruncatedAssign`], for multiplying two polynomials
/// and keeping only the low coefficients of the product.
pub mod mul_truncated;
/// Implementations of [`PowTruncated`] and [`PowTruncatedAssign`], for raising a polynomial to a
/// power and keeping only the low coefficients of the power.
pub mod pow_truncated;
/// Implementations of [`SquareTruncated`] and [`SquareTruncatedAssign`], for squaring a polynomial
/// and keeping only the low coefficients of the square.
pub mod square_truncated;
