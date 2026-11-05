// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use crate::polynomial::modular::traits::{
    CheckedModDivExact, CheckedModDivRem, CheckedModExtendedGcd, CheckedModGcd, CheckedModLcm,
    CheckedModPowerOf2DivExact, CheckedModPowerOf2DivRem, CheckedModPowerOf2ExtendedGcd,
    CheckedModPowerOf2Gcd, CheckedModPowerOf2Lcm, CheckedModPowerOf2Quotient,
    CheckedModPowerOf2Rem, CheckedModQuotient, CheckedModRem, ModCompose, ModComposeAssign,
    ModCoprimeWith, ModDivAssignRem, ModDivAssignRemPrecomputed, ModDivExact, ModDivExactAssign,
    ModDivRem, ModDivRemPrecomputed, ModDivRoot, ModDivRootAssign, ModDivisibleBy, ModEvaluate,
    ModEvaluateGeometric, ModEvaluateMany, ModEvaluateManyPrecomputed, ModExtendedGcd,
    ModFromRoots, ModGcd, ModGcdAssign, ModLcm, ModLcmAssign, ModMakeMonic, ModMakeMonicAssign,
    ModPowerOf2Compose, ModPowerOf2ComposeAssign, ModPowerOf2CoprimeWith, ModPowerOf2DivAssignRem,
    ModPowerOf2DivAssignRemPrecomputed, ModPowerOf2DivExact, ModPowerOf2DivExactAssign,
    ModPowerOf2DivRem, ModPowerOf2DivRemPrecomputed, ModPowerOf2DivRoot, ModPowerOf2DivRootAssign,
    ModPowerOf2DivisibleBy, ModPowerOf2Evaluate, ModPowerOf2EvaluateMany,
    ModPowerOf2EvaluateManyPrecomputed, ModPowerOf2FromRoots, ModPowerOf2Quotient,
    ModPowerOf2QuotientAssign, ModPowerOf2QuotientPrecomputed,
    ModPowerOf2QuotientPrecomputedAssign, ModPowerOf2Rem, ModPowerOf2RemAssign,
    ModPowerOf2RemPrecomputed, ModPowerOf2RemPrecomputedAssign, ModPowerOf2RemovePower,
    ModPowerOf2RemovePowerAssign, ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign,
    ModQuotient, ModQuotientAssign, ModQuotientPrecomputed, ModQuotientPrecomputedAssign, ModRem,
    ModRemAssign, ModRemPrecomputed, ModRemPrecomputedAssign, ModRemovePower, ModRemovePowerAssign,
    ModTaylorShift, ModTaylorShiftAssign,
};

/// The half-GCD algorithm for polynomials modulo a number or a power of 2.
#[doc(hidden)]
pub mod hgcd;
/// Implementations of [`ModAdd`](crate::num::modular::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::modular::traits::ModAddAssign), for adding two polynomials modulo a
/// value.
pub mod mod_add;
/// Implementations of [`ModCompose`] and [`ModComposeAssign`], for composing two polynomials modulo
/// a number.
pub mod mod_compose;
/// Implementations of [`ModCoprimeWith`], which determines whether two polynomials are coprime
/// modulo a number.
pub mod mod_coprime_with;
/// Implementations of [`ModDivExact`], [`ModDivExactAssign`], and [`CheckedModDivExact`], for
/// dividing polynomials modulo a number when the division is exact.
pub mod mod_div_exact;
/// Implementations of [`ModDivRem`], [`ModDivAssignRem`], and [`CheckedModDivRem`], for dividing
/// polynomials modulo a number, returning the quotient and remainder.
pub mod mod_div_rem;
/// Implementations of [`ModDivRemPrecomputed`] and [`ModDivAssignRemPrecomputed`], for dividing
/// polynomials modulo a number with data precomputed from the divisor, returning the quotient and
/// remainder.
pub mod mod_div_rem_precomputed;
/// Implementations of [`ModDivRoot`] and [`ModDivRootAssign`], for dividing a polynomial by $x - c$
/// modulo a number.
pub mod mod_div_root;
/// An implementation of [`ModDivisibleBy`], which determines whether one polynomial is divisible by
/// another modulo a number.
pub mod mod_divisible_by;
/// Implementations of [`ModEvaluate`], for evaluating a polynomial at a value modulo a number.
pub mod mod_evaluate;
/// Implementations of [`ModEvaluate`] at a square
/// [`UnsignedMatrix`](crate::unsigned_matrix::UnsignedMatrix), for evaluating a polynomial at a
/// matrix modulo a number.
pub mod mod_evaluate_at_matrix;
/// An implementation of [`ModEvaluateGeometric`], for evaluating a polynomial modulo a number at
/// the powers of a value.
pub mod mod_evaluate_geometric;
/// An implementation of [`ModEvaluateMany`], for evaluating a polynomial at many points modulo a
/// number.
pub mod mod_evaluate_many;
/// An implementation of [`ModEvaluateManyPrecomputed`], for evaluating a polynomial at many points
/// modulo a number, with a subproduct tree precomputed from the points.
pub mod mod_evaluate_many_precomputed;
/// Implementations of [`ModExtendedGcd`] and [`CheckedModExtendedGcd`], for computing extended GCDs
/// of polynomials modulo a number.
pub mod mod_extended_gcd;
/// An implementation of [`ModFromRoots`], which builds the monic polynomial with given roots modulo
/// a number.
pub mod mod_from_roots;
/// Implementations of [`ModGcd`], [`ModGcdAssign`], and [`CheckedModGcd`], for the monic GCD of two
/// polynomials modulo a number.
pub mod mod_gcd;
/// An implementation of [`ModIsReduced`](crate::num::modular::traits::ModIsReduced), which checks
/// whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModLcm`], [`ModLcmAssign`], and [`CheckedModLcm`], for computing LCMs of
/// polynomials modulo a number.
pub mod mod_lcm;
/// Implementations of [`ModMakeMonic`] and [`ModMakeMonicAssign`], which make a polynomial monic
/// modulo a value.
pub mod mod_make_monic;
/// Implementations of [`ModMul`](crate::num::modular::traits::ModMul) and
/// [`ModMulAssign`](crate::num::modular::traits::ModMulAssign), for multiplying two polynomials
/// modulo a value.
pub mod mod_mul;
#[doc(hidden)]
pub mod mod_mul_middle;
/// Implementations of [`ModNeg`](crate::num::modular::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::modular::traits::ModNegAssign), which negate a polynomial modulo a
/// number.
pub mod mod_neg;
/// Implementations of [`Mod`](crate::num::modular::traits::Mod),
/// [`ModAssign`](crate::num::modular::traits::ModAssign), [`Rem`](core::ops::Rem) and
/// [`RemAssign`](core::ops::RemAssign), which reduce every coefficient of a polynomial modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPow`](crate::num::modular::traits::ModPow) and
/// [`ModPowAssign`](crate::num::modular::traits::ModPowAssign), for raising a polynomial to a power
/// modulo a number.
pub mod mod_pow;
/// Implementations of [`ModPowerOf2`](crate::num::modular::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::modular::traits::ModPowerOf2Assign), which reduce every
/// coefficient of a polynomial modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](crate::num::modular::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](crate::num::modular::traits::ModPowerOf2AddAssign), for adding two
/// polynomials modulo $2^k$.
pub mod mod_power_of_2_add;
/// Implementations of [`ModPowerOf2Compose`] and [`ModPowerOf2ComposeAssign`], for composing two
/// polynomials modulo $2^k$.
pub mod mod_power_of_2_compose;
/// Implementations of [`ModPowerOf2CoprimeWith`], which determines whether two polynomials are
/// coprime modulo a power of 2.
pub mod mod_power_of_2_coprime_with;
/// Implementations of [`ModPowerOf2DivExact`], [`ModPowerOf2DivExactAssign`], and
/// [`CheckedModPowerOf2DivExact`], for dividing polynomials modulo a power of 2 when the division
/// is exact.
pub mod mod_power_of_2_div_exact;
/// Implementations of [`ModPowerOf2DivRem`], [`ModPowerOf2DivAssignRem`], and
/// [`CheckedModPowerOf2DivRem`], for dividing polynomials modulo a power of 2, returning the
/// quotient and remainder.
pub mod mod_power_of_2_div_rem;
/// Implementations of [`ModPowerOf2DivRemPrecomputed`] and [`ModPowerOf2DivAssignRemPrecomputed`],
/// for dividing polynomials modulo a power of 2 with data precomputed from the divisor, returning
/// the quotient and remainder.
pub mod mod_power_of_2_div_rem_precomputed;
/// Implementations of [`ModPowerOf2DivRoot`] and [`ModPowerOf2DivRootAssign`], for dividing a
/// polynomial by $x - c$ modulo a power of 2.
pub mod mod_power_of_2_div_root;
/// An implementation of [`ModPowerOf2DivisibleBy`], which determines whether one polynomial is
/// divisible by another modulo a power of 2.
pub mod mod_power_of_2_divisible_by;
/// Implementations of [`ModPowerOf2Evaluate`], for evaluating a polynomial at a value modulo a
/// power of 2.
pub mod mod_power_of_2_evaluate;
/// Implementations of [`ModPowerOf2Evaluate`] at a square
/// [`UnsignedMatrix`](crate::unsigned_matrix::UnsignedMatrix), for evaluating a polynomial at a
/// matrix modulo a power of 2.
pub mod mod_power_of_2_evaluate_at_matrix;
/// An implementation of [`ModPowerOf2EvaluateMany`], for evaluating a polynomial at many points
/// modulo a power of 2.
pub mod mod_power_of_2_evaluate_many;
/// An implementation of [`ModPowerOf2EvaluateManyPrecomputed`], for evaluating a polynomial at many
/// points modulo a power of 2, with a subproduct tree precomputed from the points.
pub mod mod_power_of_2_evaluate_many_precomputed;
/// Implementations of [`CheckedModPowerOf2ExtendedGcd`], for computing extended GCDs of polynomials
/// modulo a power of 2.
pub mod mod_power_of_2_extended_gcd;
/// An implementation of [`ModPowerOf2FromRoots`], which builds the monic polynomial with given
/// roots modulo a power of 2.
pub mod mod_power_of_2_from_roots;
/// Implementations of [`CheckedModPowerOf2Gcd`], for the monic GCD of two polynomials modulo a
/// power of 2.
pub mod mod_power_of_2_gcd;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::modular::traits::ModPowerOf2IsReduced), which checks
/// whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`CheckedModPowerOf2Lcm`], for computing LCMs of polynomials modulo a power
/// of 2.
pub mod mod_power_of_2_lcm;
/// Implementations of [`ModPowerOf2Mul`](crate::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::modular::traits::ModPowerOf2MulAssign), for multiplying two
/// polynomials modulo $2^k$.
pub mod mod_power_of_2_mul;
/// Middle products of polynomials modulo a power of 2, in wrapping arithmetic. This is not part of
/// the public API.
#[doc(hidden)]
pub mod mod_power_of_2_mul_middle;
/// Implementations of [`ModPowerOf2Neg`](crate::num::modular::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::modular::traits::ModPowerOf2NegAssign), which negate a
/// polynomial modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Pow`](crate::num::modular::traits::ModPowerOf2Pow) and
/// [`ModPowerOf2PowAssign`](crate::num::modular::traits::ModPowerOf2PowAssign), for raising a
/// polynomial to a power modulo a power of 2.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2Product`](crate::num::modular::traits::ModPowerOf2Product), for
/// multiplying polynomials together modulo a power of 2.
pub mod mod_power_of_2_product;
/// Implementations of [`ModPowerOf2Quotient`], [`ModPowerOf2QuotientAssign`], and
/// [`CheckedModPowerOf2Quotient`], for finding the quotient of polynomials modulo a power of 2.
pub mod mod_power_of_2_quotient;
/// Implementations of [`ModPowerOf2QuotientPrecomputed`] and
/// [`ModPowerOf2QuotientPrecomputedAssign`], for finding the quotient of polynomials modulo a power
/// of 2 with data precomputed from the divisor.
pub mod mod_power_of_2_quotient_precomputed;
/// Implementations of [`ModPowerOf2Rem`], [`ModPowerOf2RemAssign`], and [`CheckedModPowerOf2Rem`],
/// for finding the remainder of polynomials modulo a power of 2.
pub mod mod_power_of_2_rem;
/// Implementations of [`ModPowerOf2RemPrecomputed`] and [`ModPowerOf2RemPrecomputedAssign`], for
/// finding the remainder of polynomials modulo a power of 2 with data precomputed from the divisor.
pub mod mod_power_of_2_rem_precomputed;
/// Implementations of [`ModPowerOf2RemovePower`] and [`ModPowerOf2RemovePowerAssign`], for removing
/// the largest power of a polynomial that divides another modulo a power of 2.
pub mod mod_power_of_2_remove_power;
/// Implementations of [`ModPowerOf2Shl`](crate::num::modular::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](crate::num::modular::traits::ModPowerOf2ShlAssign), for left-shifting a
/// polynomial modulo a power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModPowerOf2Shl;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
///
/// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
/// assert_eq!(
///     p.clone().mod_power_of_2_shl(1u8, 3).to_string(),
///     "2*x^2+2*x+6"
/// );
/// assert_eq!(
///     p.clone().mod_power_of_2_shl(2u16, 3).to_string(),
///     "4*x^2+4*x+4"
/// );
/// assert_eq!(p.mod_power_of_2_shl(3u32, 3).to_string(), "0");
/// assert_eq!(
///     UnsignedPolynomial::<u8>::from_str("x^2+128*x+1")
///         .unwrap()
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "2*x^2+2"
/// );
/// assert_eq!(
///     UnsignedPolynomial::<u64>::from_str("x+1")
///         .unwrap()
///         .mod_power_of_2_shl(63u128, 64)
///         .to_string(),
///     "9223372036854775808*x+9223372036854775808"
/// );
///
/// let p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
/// assert_eq!((&p).mod_power_of_2_shl(1u8, 3).to_string(), "2*x^2+2*x+6");
/// assert_eq!((&p).mod_power_of_2_shl(2u16, 3).to_string(), "4*x^2+4*x+4");
/// assert_eq!((&p).mod_power_of_2_shl(3u32, 3).to_string(), "0");
/// assert_eq!(
///     (&UnsignedPolynomial::<u8>::from_str("x^2+128*x+1").unwrap())
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "2*x^2+2"
/// );
/// assert_eq!(
///     (&UnsignedPolynomial::<u64>::from_str("x+1").unwrap())
///         .mod_power_of_2_shl(63u128, 64)
///         .to_string(),
///     "9223372036854775808*x+9223372036854775808"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModPowerOf2ShlAssign;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
///
/// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
/// p.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(p.to_string(), "2*x^2+2*x+6");
///
/// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
/// p.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+128*x+1").unwrap();
/// p.mod_power_of_2_shl_assign(1u64, 8);
/// assert_eq!(p.to_string(), "2*x^2+2");
///
/// let mut p = UnsignedPolynomial::<u64>::from_str("x+1").unwrap();
/// p.mod_power_of_2_shl_assign(63u128, 64);
/// assert_eq!(p.to_string(), "9223372036854775808*x+9223372036854775808");
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Square`](crate::num::modular::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](crate::num::modular::traits::ModPowerOf2SquareAssign), for squaring
/// a polynomial modulo $2^k$.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2Sub`](crate::num::modular::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::modular::traits::ModPowerOf2SubAssign), for subtracting one
/// polynomial from another modulo $2^k$.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModPowerOf2Sum`](crate::num::modular::traits::ModPowerOf2Sum), for adding
/// up polynomials modulo a power of 2.
pub mod mod_power_of_2_sum;
/// Implementations of [`ModPowerOf2TaylorShift`] and [`ModPowerOf2TaylorShiftAssign`], for shifting
/// a polynomial by a constant modulo $2^k$.
pub mod mod_power_of_2_taylor_shift;
/// Implementations of [`ModProduct`](crate::num::modular::traits::ModProduct), for multiplying
/// polynomials together modulo a number.
pub mod mod_product;
/// Implementations of [`ModQuotient`], [`ModQuotientAssign`], and [`CheckedModQuotient`], for
/// finding the quotient of polynomials modulo a number.
pub mod mod_quotient;
/// Implementations of [`ModQuotientPrecomputed`] and [`ModQuotientPrecomputedAssign`], for finding
/// the quotient of polynomials modulo a number with data precomputed from the divisor.
pub mod mod_quotient_precomputed;
/// Implementations of [`ModRem`], [`ModRemAssign`], and [`CheckedModRem`], for finding the
/// remainder of polynomials modulo a number.
pub mod mod_rem;
/// Implementations of [`ModRemPrecomputed`] and [`ModRemPrecomputedAssign`], for finding the
/// remainder of polynomials modulo a number with data precomputed from the divisor.
pub mod mod_rem_precomputed;
/// Implementations of [`ModRemovePower`] and [`ModRemovePowerAssign`], for removing the largest
/// power of a polynomial that divides another modulo a number.
pub mod mod_remove_power;
/// Resultants of polynomials modulo a number or a power of 2.
#[doc(hidden)]
pub mod mod_resultant;
/// Implementations of [`ModShl`](crate::num::modular::traits::ModShl) and
/// [`ModShlAssign`](crate::num::modular::traits::ModShlAssign), for left-shifting a polynomial
/// modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShl;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
///
/// let p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+7").unwrap();
/// assert_eq!(p.clone().mod_shl(1u8, 10).to_string(), "6*x^2+2*x+4");
/// assert_eq!(p.mod_shl(3u16, 10).to_string(), "4*x^2+8*x+6");
/// assert_eq!(
///     UnsignedPolynomial::<u8>::from_str("3*x^2+5")
///         .unwrap()
///         .mod_shl(2u32, 12)
///         .to_string(),
///     "8"
/// );
/// assert_eq!(
///     UnsignedPolynomial::<u64>::from_str("x+1")
///         .unwrap()
///         .mod_shl(100u64, 1000000000000000000)
///         .to_string(),
///     "229401496703205376*x+229401496703205376"
/// );
///
/// let p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+7").unwrap();
/// assert_eq!((&p).mod_shl(1u8, 10).to_string(), "6*x^2+2*x+4");
/// assert_eq!((&p).mod_shl(3u16, 10).to_string(), "4*x^2+8*x+6");
/// assert_eq!(
///     (&UnsignedPolynomial::<u8>::from_str("3*x^2+5").unwrap())
///         .mod_shl(2u32, 12)
///         .to_string(),
///     "8"
/// );
/// assert_eq!(
///     (&UnsignedPolynomial::<u64>::from_str("x+1").unwrap())
///         .mod_shl(100u64, 1000000000000000000)
///         .to_string(),
///     "229401496703205376*x+229401496703205376"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShlAssign;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
///
/// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+7").unwrap();
/// p.mod_shl_assign(1u8, 10);
/// assert_eq!(p.to_string(), "6*x^2+2*x+4");
///
/// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+x+7").unwrap();
/// p.mod_shl_assign(3u16, 10);
/// assert_eq!(p.to_string(), "4*x^2+8*x+6");
///
/// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+5").unwrap();
/// p.mod_shl_assign(2u32, 12);
/// assert_eq!(p.to_string(), "8");
///
/// let mut p = UnsignedPolynomial::<u64>::from_str("x+1").unwrap();
/// p.mod_shl_assign(100u64, 1000000000000000000);
/// assert_eq!(p.to_string(), "229401496703205376*x+229401496703205376");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSquare`](crate::num::modular::traits::ModSquare) and
/// [`ModSquareAssign`](crate::num::modular::traits::ModSquareAssign), for squaring a polynomial
/// modulo a value.
pub mod mod_square;
/// Implementations of [`ModSub`](crate::num::modular::traits::ModSub) and
/// [`ModSubAssign`](crate::num::modular::traits::ModSubAssign), for subtracting one polynomial from
/// another modulo a value.
pub mod mod_sub;
/// Implementations of [`ModSum`](crate::num::modular::traits::ModSum), for adding up polynomials
/// modulo a number.
pub mod mod_sum;
/// Implementations of [`ModTaylorShift`] and [`ModTaylorShiftAssign`], for shifting a polynomial by
/// a constant modulo a number.
pub mod mod_taylor_shift;
/// Subproduct trees of linear factors modulo a number or a power of 2, for evaluation at many
/// points.
#[doc(hidden)]
pub mod subproduct_tree;
