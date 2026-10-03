// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`CanonicalizeUnit`](crate::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](crate::num::arithmetic::traits::CanonicalizeUnitAssign), which bring
/// a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`ComposePowerOfX`](crate::polynomial::ComposePowerOfX) and
/// [`ComposePowerOfXAssign`](crate::polynomial::ComposePowerOfXAssign), for substituting a power of
/// the variable into a polynomial.
pub mod compose_power_of_x;
/// Implementations of [`Content`](crate::polynomial::Content),
/// [`PrimitivePart`](crate::polynomial::PrimitivePart),
/// [`PrimitivePartAssign`](crate::polynomial::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](crate::polynomial::ContentAndPrimitivePart), which compute the GCD
/// of a polynomial's coefficients and the polynomial divided by it.
pub mod content;
/// Implementations of [`DeflatePowerOfX`](crate::polynomial::DeflatePowerOfX) and
/// [`DeflatePowerOfXAssign`](crate::polynomial::DeflatePowerOfXAssign), for undoing the
/// substitution of a power of the variable into a polynomial.
pub mod deflate_power_of_x;
/// Implementations of [`DivPowerOfX`](crate::polynomial::DivPowerOfX) and
/// [`DivPowerOfXAssign`](crate::polynomial::DivPowerOfXAssign), for dividing a polynomial by a
/// power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// Implementations of [`ModEvaluate`](crate::polynomial::ModEvaluate) and
/// [`ModPowerOf2Evaluate`](crate::polynomial::ModPowerOf2Evaluate), which evaluate a polynomial at
/// a value modulo a value or a power of 2.
pub mod evaluate;
/// An implementation of [`ExponentGcd`](crate::polynomial::ExponentGcd), the greatest common
/// divisor of the exponents at which a polynomial has nonzero coefficients.
pub mod exponent_gcd;
/// An implementation of [`Height`](crate::num::arithmetic::traits::Height), the largest of a
/// polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](crate::num::arithmetic::traits::IsUnit), a trait for determining
/// whether a number is a unit of its ring.
pub mod is_unit;
/// Implementations of [`ModAdd`](crate::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::arithmetic::traits::ModAddAssign), for adding two polynomials
/// modulo a value.
pub mod mod_add;
/// Implementations of [`ModAddTruncated`](crate::polynomial::ModAddTruncated) and
/// [`ModAddTruncatedAssign`](crate::polynomial::ModAddTruncatedAssign), for adding two polynomials
/// modulo a value and keeping only their low coefficients.
pub mod mod_add_truncated;
/// Implementations of [`ModDerivative`](crate::polynomial::ModDerivative) and
/// [`ModDerivativeAssign`](crate::polynomial::ModDerivativeAssign), for differentiating a
/// polynomial modulo a number.
pub mod mod_derivative;
/// Implementations of [`ModIntegral`](crate::polynomial::ModIntegral) and
/// [`ModIntegralAssign`](crate::polynomial::ModIntegralAssign), for integrating a polynomial modulo
/// a value.
pub mod mod_integral;
/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), which
/// checks whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMakeMonic`](crate::polynomial::ModMakeMonic) and
/// [`ModMakeMonicAssign`](crate::polynomial::ModMakeMonicAssign), which make a polynomial monic
/// modulo a value.
pub mod mod_make_monic;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](crate::num::arithmetic::traits::ModMulAssign), for multiplying two polynomials
/// modulo a value.
pub mod mod_mul;
#[doc(hidden)]
pub mod mod_mul_middle;
/// Implementations of [`ModMulTruncated`](crate::polynomial::ModMulTruncated) and
/// [`ModMulTruncatedAssign`](crate::polynomial::ModMulTruncatedAssign), for multiplying two
/// polynomials modulo a value and keeping the low coefficients of the product.
pub mod mod_mul_truncated;
/// Implementations of [`ModNeg`](crate::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::arithmetic::traits::ModNegAssign), which negate a polynomial modulo
/// a number.
pub mod mod_neg;
/// Implementations of [`ModNthDerivative`](crate::polynomial::ModNthDerivative) and
/// [`ModNthDerivativeAssign`](crate::polynomial::ModNthDerivativeAssign), for differentiating a
/// polynomial any number of times modulo a number.
pub mod mod_nth_derivative;
/// Implementations of [`Mod`](crate::num::arithmetic::traits::Mod),
/// [`ModAssign`](crate::num::arithmetic::traits::ModAssign), [`Rem`](core::ops::Rem) and
/// [`RemAssign`](core::ops::RemAssign), which reduce every coefficient of a polynomial modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](crate::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::arithmetic::traits::ModPowerOf2Assign), which reduce every
/// coefficient of a polynomial modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](crate::num::arithmetic::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](crate::num::arithmetic::traits::ModPowerOf2AddAssign), for adding two
/// polynomials modulo $2^k$.
pub mod mod_power_of_2_add;
/// Implementations of [`ModPowerOf2AddTruncated`](crate::polynomial::ModPowerOf2AddTruncated) and
/// [`ModPowerOf2AddTruncatedAssign`](crate::polynomial::ModPowerOf2AddTruncatedAssign), for adding
/// two polynomials modulo $2^k$ and keeping only their low coefficients.
pub mod mod_power_of_2_add_truncated;
/// Implementations of [`ModPowerOf2Derivative`](crate::polynomial::ModPowerOf2Derivative) and
/// [`ModPowerOf2DerivativeAssign`](crate::polynomial::ModPowerOf2DerivativeAssign), for
/// differentiating a polynomial modulo a power of 2.
pub mod mod_power_of_2_derivative;
/// Implementations of [`ModPowerOf2Integral`](crate::polynomial::ModPowerOf2Integral) and
/// [`ModPowerOf2IntegralAssign`](crate::polynomial::ModPowerOf2IntegralAssign), for integrating a
/// polynomial modulo $2^k$.
pub mod mod_power_of_2_integral;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), which checks
/// whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::arithmetic::traits::ModPowerOf2MulAssign), for multiplying
/// two polynomials modulo $2^k$.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2MulTruncated`](crate::polynomial::ModPowerOf2MulTruncated) and
/// [`ModPowerOf2MulTruncatedAssign`](crate::polynomial::ModPowerOf2MulTruncatedAssign), for
/// multiplying two polynomials modulo $2^k$ and keeping the low coefficients of the product.
pub mod mod_power_of_2_mul_truncated;
/// Implementations of [`ModPowerOf2Neg`](crate::num::arithmetic::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::arithmetic::traits::ModPowerOf2NegAssign), which negate a
/// polynomial modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2NthDerivative`](crate::polynomial::ModPowerOf2NthDerivative) and
/// [`ModPowerOf2NthDerivativeAssign`](crate::polynomial::ModPowerOf2NthDerivativeAssign), for
/// differentiating a polynomial any number of times modulo a power of 2.
pub mod mod_power_of_2_nth_derivative;
/// Implementations of [`ModPowerOf2Pow`](crate::num::arithmetic::traits::ModPowerOf2Pow) and
/// [`ModPowerOf2PowAssign`](crate::num::arithmetic::traits::ModPowerOf2PowAssign), for raising a
/// polynomial to a power modulo a power of 2.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2PowTruncated`](crate::polynomial::ModPowerOf2PowTruncated) and
/// [`ModPowerOf2PowTruncatedAssign`](crate::polynomial::ModPowerOf2PowTruncatedAssign), for raising
/// a polynomial to a power modulo a power of 2 and keeping only the low coefficients.
pub mod mod_power_of_2_pow_truncated;
/// Implementations of [`ModPowerOf2Shl`](crate::num::arithmetic::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](crate::num::arithmetic::traits::ModPowerOf2ShlAssign), for
/// left-shifting a polynomial modulo a power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
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
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
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
/// Implementations of [`ModPowerOf2Square`](crate::num::arithmetic::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](crate::num::arithmetic::traits::ModPowerOf2SquareAssign), for
/// squaring a polynomial modulo $2^k$.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2SquareTruncated`](crate::polynomial::ModPowerOf2SquareTruncated)
/// and [`ModPowerOf2SquareTruncatedAssign`](crate::polynomial::ModPowerOf2SquareTruncatedAssign),
/// for squaring a polynomial modulo $2^k$ and keeping the low coefficients of the square.
pub mod mod_power_of_2_square_truncated;
/// Implementations of [`ModPowerOf2Sub`](crate::num::arithmetic::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::arithmetic::traits::ModPowerOf2SubAssign), for subtracting
/// one polynomial from another modulo $2^k$.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModPowerOf2SubTruncated`](crate::polynomial::ModPowerOf2SubTruncated) and
/// [`ModPowerOf2SubTruncatedAssign`](crate::polynomial::ModPowerOf2SubTruncatedAssign), for
/// subtracting one polynomial from another modulo $2^k$ and keeping only their low coefficients.
pub mod mod_power_of_2_sub_truncated;
/// Implementations of [`ModShl`](crate::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](crate::num::arithmetic::traits::ModShlAssign), for left-shifting a polynomial
/// modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShl;
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
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
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
/// Implementations of [`ModSquare`](crate::num::arithmetic::traits::ModSquare) and
/// [`ModSquareAssign`](crate::num::arithmetic::traits::ModSquareAssign), for squaring a polynomial
/// modulo a value.
pub mod mod_square;
/// Implementations of [`ModSquareTruncated`](crate::polynomial::ModSquareTruncated) and
/// [`ModSquareTruncatedAssign`](crate::polynomial::ModSquareTruncatedAssign), for squaring a
/// polynomial modulo a value and keeping the low coefficients of the square.
pub mod mod_square_truncated;
/// Implementations of [`ModSub`](crate::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](crate::num::arithmetic::traits::ModSubAssign), for subtracting one polynomial
/// from another modulo a value.
pub mod mod_sub;
/// Implementations of [`ModSubTruncated`](crate::polynomial::ModSubTruncated) and
/// [`ModSubTruncatedAssign`](crate::polynomial::ModSubTruncatedAssign), for subtracting one
/// polynomial from another modulo a value and keeping only their low coefficients.
pub mod mod_sub_truncated;
/// Implementations of [`MulPowerOfX`](crate::polynomial::MulPowerOfX) and
/// [`MulPowerOfXAssign`](crate::polynomial::MulPowerOfXAssign), for multiplying a polynomial by a
/// power of its variable.
pub mod mul_power_of_x;
