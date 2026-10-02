// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// two polynomials.
pub mod add;
/// Implementations of [`AddTruncated`](malachite_base::polynomial::AddTruncated) and
/// [`AddTruncatedAssign`](malachite_base::polynomial::AddTruncatedAssign), for adding two
/// polynomials and keeping only their low coefficients.
pub mod add_truncated;
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod), which
/// reduces every coefficient of a polynomial to the representative closest to zero, producing an
/// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial).
pub mod balanced_mod;
/// Implementations of [`BitPack`](malachite_base::polynomial::BitPack), which packs a polynomial's
/// coefficients into fixed-width fields of a single number.
pub mod bit_pack;
/// Implementations of [`BitUnpack`](malachite_base::polynomial::BitUnpack), which unpacks a
/// polynomial from the fixed-width fields of a single number.
pub mod bit_unpack;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// which bring a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`ComposePowerOfX`](malachite_base::polynomial::ComposePowerOfX) and
/// [`ComposePowerOfXAssign`](malachite_base::polynomial::ComposePowerOfXAssign), for substituting a
/// power of the variable into a polynomial.
pub mod compose_power_of_x;
/// Implementations of [`Content`](malachite_base::polynomial::Content),
/// [`PrimitivePart`](malachite_base::polynomial::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::polynomial::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::polynomial::ContentAndPrimitivePart), which compute
/// the GCD of a polynomial's coefficients and the polynomial divided by it.
pub mod content;
/// Implementations of [`DeflatePowerOfX`](malachite_base::polynomial::DeflatePowerOfX) and
/// [`DeflatePowerOfXAssign`](malachite_base::polynomial::DeflatePowerOfXAssign), for undoing the
/// substitution of a power of the variable into a polynomial.
pub mod deflate_power_of_x;
/// Implementations of [`Derivative`](malachite_base::polynomial::Derivative) and
/// [`DerivativeAssign`](malachite_base::polynomial::DerivativeAssign), for differentiating a
/// polynomial.
pub mod derivative;
/// Implementations of [`DivPowerOfX`](malachite_base::polynomial::DivPowerOfX) and
/// [`DivPowerOfXAssign`](malachite_base::polynomial::DivPowerOfXAssign), for dividing a polynomial
/// by a power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// Implementations of [`Evaluate`](malachite_base::polynomial::Evaluate), which evaluates a
/// polynomial at a value, and of
/// [`ModPowerOf2Evaluate`](malachite_base::polynomial::ModPowerOf2Evaluate), which does so modulo a
/// power of 2.
pub mod evaluate;
/// An implementation of [`ExponentGcd`](malachite_base::polynomial::ExponentGcd), the greatest
/// common divisor of the exponents at which a polynomial has nonzero coefficients.
pub mod exponent_gcd;
/// An implementation of [`FloorL2Norm`](malachite_base::polynomial::FloorL2Norm), the floor of a
/// polynomial's $L^2$ norm.
pub mod floor_l2_norm;
/// Implementations of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest of the magnitudes
/// of a polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// An implementation of [`L2NormSquared`](malachite_base::polynomial::L2NormSquared), the sum of
/// the squares of a polynomial's coefficients.
pub mod l2_norm_squared;
/// Implementations of [`ModAdd`](malachite_base::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::arithmetic::traits::ModAddAssign), for adding two
/// polynomials modulo a [`Natural`](crate::natural::Natural).
pub mod mod_add;
/// Implementations of [`ModAddTruncated`](malachite_base::polynomial::ModAddTruncated) and
/// [`ModAddTruncatedAssign`](malachite_base::polynomial::ModAddTruncatedAssign), for adding two
/// polynomials modulo a [`Natural`](crate::natural::Natural) and keeping only their low
/// coefficients.
pub mod mod_add_truncated;
/// Implementations of [`ModDerivative`](malachite_base::polynomial::ModDerivative) and
/// [`ModDerivativeAssign`](malachite_base::polynomial::ModDerivativeAssign), for differentiating a
/// polynomial modulo a number.
pub mod mod_derivative;
/// Implementations of [`ModIntegral`](malachite_base::polynomial::ModIntegral) and
/// [`ModIntegralAssign`](malachite_base::polynomial::ModIntegralAssign), for integrating a
/// polynomial modulo a value.
pub mod mod_integral;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced),
/// which checks whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMakeMonic`](malachite_base::polynomial::ModMakeMonic) and
/// [`ModMakeMonicAssign`](malachite_base::polynomial::ModMakeMonicAssign), which make a polynomial
/// monic modulo a value.
pub mod mod_make_monic;
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign), for multiplying two
/// polynomials modulo a [`Natural`](crate::natural::Natural).
pub mod mod_mul;
/// Implementations of [`ModMulTruncated`](malachite_base::polynomial::ModMulTruncated) and
/// [`ModMulTruncatedAssign`](malachite_base::polynomial::ModMulTruncatedAssign), for multiplying
/// two polynomials modulo a [`Natural`](crate::natural::Natural) and keeping only the low
/// coefficients of the product.
pub mod mod_mul_truncated;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), which negate a
/// polynomial modulo a number.
pub mod mod_neg;
/// Implementations of [`ModNthDerivative`](malachite_base::polynomial::ModNthDerivative) and
/// [`ModNthDerivativeAssign`](malachite_base::polynomial::ModNthDerivativeAssign), for
/// differentiating a polynomial any number of times modulo a number.
pub mod mod_nth_derivative;
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod),
/// [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign), [`Rem`](core::ops::Rem), and
/// [`RemAssign`](core::ops::RemAssign), traits for reducing every coefficient of a polynomial
/// modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), which reduce
/// every coefficient of a polynomial modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::arithmetic::traits::ModPowerOf2Add)
/// and [`ModPowerOf2AddAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign), for
/// adding two polynomials modulo $2^k$.
pub mod mod_power_of_2_add;
/// Implementations of
/// [`ModPowerOf2AddTruncated`](malachite_base::polynomial::ModPowerOf2AddTruncated) and
/// [`ModPowerOf2AddTruncatedAssign`](malachite_base::polynomial::ModPowerOf2AddTruncatedAssign),
/// for adding two polynomials modulo $2^k$ and keeping only their low coefficients.
pub mod mod_power_of_2_add_truncated;
/// Implementations of [`ModPowerOf2Derivative`](malachite_base::polynomial::ModPowerOf2Derivative)
/// and [`ModPowerOf2DerivativeAssign`](malachite_base::polynomial::ModPowerOf2DerivativeAssign),
/// for differentiating a polynomial modulo a power of 2.
pub mod mod_power_of_2_derivative;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), which
/// checks whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign), for
/// multiplying two polynomials modulo $2^k$.
pub mod mod_power_of_2_mul;
/// Implementations of
/// [`ModPowerOf2MulTruncated`](malachite_base::polynomial::ModPowerOf2MulTruncated) and
/// [`ModPowerOf2MulTruncatedAssign`](malachite_base::polynomial::ModPowerOf2MulTruncatedAssign),
/// for multiplying two polynomials modulo $2^k$ and keeping only the low coefficients of the
/// product.
pub mod mod_power_of_2_mul_truncated;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign),
/// which negate a polynomial modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of
/// [`ModPowerOf2NthDerivative`](malachite_base::polynomial::ModPowerOf2NthDerivative) and
/// [`ModPowerOf2NthDerivativeAssign`](malachite_base::polynomial::ModPowerOf2NthDerivativeAssign),
/// for differentiating a polynomial any number of times modulo a power of 2.
pub mod mod_power_of_2_nth_derivative;
/// Implementations of [`ModPowerOf2Shl`](malachite_base::num::arithmetic::traits::ModPowerOf2Shl)
/// and [`ModPowerOf2ShlAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign), for
/// left-shifting a polynomial modulo a power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
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
///     NaturalPolynomial::from_str("4*x^2+3")
///         .unwrap()
///         .mod_power_of_2_shl(1u64, 3)
///         .to_string(),
///     "6"
/// );
/// assert_eq!(
///     NaturalPolynomial::from_str("x+1")
///         .unwrap()
///         .mod_power_of_2_shl(99u128, 100)
///         .to_string(),
///     "633825300114114700748351602688*x+633825300114114700748351602688"
/// );
///
/// let p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
/// assert_eq!((&p).mod_power_of_2_shl(1u8, 3).to_string(), "2*x^2+2*x+6");
/// assert_eq!((&p).mod_power_of_2_shl(2u16, 3).to_string(), "4*x^2+4*x+4");
/// assert_eq!((&p).mod_power_of_2_shl(3u32, 3).to_string(), "0");
/// assert_eq!(
///     (&NaturalPolynomial::from_str("4*x^2+3").unwrap())
///         .mod_power_of_2_shl(1u64, 3)
///         .to_string(),
///     "6"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x+1").unwrap())
///         .mod_power_of_2_shl(99u128, 100)
///         .to_string(),
///     "633825300114114700748351602688*x+633825300114114700748351602688"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
/// p.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(p.to_string(), "2*x^2+2*x+6");
///
/// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
/// p.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
/// p.mod_power_of_2_shl_assign(1u64, 3);
/// assert_eq!(p.to_string(), "6");
///
/// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
/// p.mod_power_of_2_shl_assign(99u128, 100);
/// assert_eq!(
///     p.to_string(),
///     "633825300114114700748351602688*x+633825300114114700748351602688"
/// );
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of
/// [`ModPowerOf2Square`](malachite_base::num::arithmetic::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign),
/// for squaring a polynomial modulo $2^k$.
pub mod mod_power_of_2_square;
/// Implementations of
/// [`ModPowerOf2SquareTruncated`](malachite_base::polynomial::ModPowerOf2SquareTruncated) and
/// [`ModPowerOf2SquareTruncatedAssign`](
/// malachite_base::polynomial::ModPowerOf2SquareTruncatedAssign), for squaring a polynomial modulo
/// $2^k$ and keeping only the low coefficients of the square.
pub mod mod_power_of_2_square_truncated;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign), for
/// subtracting one polynomial from another modulo $2^k$.
pub mod mod_power_of_2_sub;
/// Implementations of
/// [`ModPowerOf2SubTruncated`](malachite_base::polynomial::ModPowerOf2SubTruncated) and
/// [`ModPowerOf2SubTruncatedAssign`](malachite_base::polynomial::ModPowerOf2SubTruncatedAssign),
/// for subtracting one polynomial from another modulo $2^k$ and keeping only their low
/// coefficients.
pub mod mod_power_of_2_sub_truncated;
/// Implementations of [`ModShl`](malachite_base::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](malachite_base::num::arithmetic::traits::ModShlAssign), for left-shifting a
/// polynomial modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::{ModShl, Pow};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let p = NaturalPolynomial::from_str("3*x^2+x+7").unwrap();
/// assert_eq!(
///     p.clone().mod_shl(1u8, Natural::from(10u32)).to_string(),
///     "6*x^2+2*x+4"
/// );
/// assert_eq!(
///     p.mod_shl(3u16, Natural::from(10u32)).to_string(),
///     "4*x^2+8*x+6"
/// );
/// assert_eq!(
///     NaturalPolynomial::from_str("3*x^2+5")
///         .unwrap()
///         .mod_shl(2u32, Natural::from(12u32))
///         .to_string(),
///     "8"
/// );
/// assert_eq!(
///     NaturalPolynomial::from_str("x+1")
///         .unwrap()
///         .mod_shl(100u64, Natural::from(10u32).pow(30))
///         .to_string(),
///     "267650600228229401496703205376*x+267650600228229401496703205376"
/// );
///
/// let p = NaturalPolynomial::from_str("3*x^2+x+7").unwrap();
/// assert_eq!(
///     p.clone().mod_shl(1u8, &Natural::from(10u32)).to_string(),
///     "6*x^2+2*x+4"
/// );
/// assert_eq!(
///     p.mod_shl(3u16, &Natural::from(10u32)).to_string(),
///     "4*x^2+8*x+6"
/// );
/// assert_eq!(
///     NaturalPolynomial::from_str("3*x^2+5")
///         .unwrap()
///         .mod_shl(2u32, &Natural::from(12u32))
///         .to_string(),
///     "8"
/// );
/// assert_eq!(
///     NaturalPolynomial::from_str("x+1")
///         .unwrap()
///         .mod_shl(100u64, &Natural::from(10u32).pow(30))
///         .to_string(),
///     "267650600228229401496703205376*x+267650600228229401496703205376"
/// );
///
/// let p = NaturalPolynomial::from_str("3*x^2+x+7").unwrap();
/// assert_eq!(
///     (&p).mod_shl(1u8, Natural::from(10u32)).to_string(),
///     "6*x^2+2*x+4"
/// );
/// assert_eq!(
///     (&p).mod_shl(3u16, Natural::from(10u32)).to_string(),
///     "4*x^2+8*x+6"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("3*x^2+5").unwrap())
///         .mod_shl(2u32, Natural::from(12u32))
///         .to_string(),
///     "8"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x+1").unwrap())
///         .mod_shl(100u64, Natural::from(10u32).pow(30))
///         .to_string(),
///     "267650600228229401496703205376*x+267650600228229401496703205376"
/// );
///
/// let p = NaturalPolynomial::from_str("3*x^2+x+7").unwrap();
/// assert_eq!(
///     (&p).mod_shl(1u8, &Natural::from(10u32)).to_string(),
///     "6*x^2+2*x+4"
/// );
/// assert_eq!(
///     (&p).mod_shl(3u16, &Natural::from(10u32)).to_string(),
///     "4*x^2+8*x+6"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("3*x^2+5").unwrap())
///         .mod_shl(2u32, &Natural::from(12u32))
///         .to_string(),
///     "8"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x+1").unwrap())
///         .mod_shl(100u64, &Natural::from(10u32).pow(30))
///         .to_string(),
///     "267650600228229401496703205376*x+267650600228229401496703205376"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::{ModShlAssign, Pow};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let mut p = NaturalPolynomial::from_str("3*x^2+x+7").unwrap();
/// p.mod_shl_assign(1u8, Natural::from(10u32));
/// assert_eq!(p.to_string(), "6*x^2+2*x+4");
///
/// let mut p = NaturalPolynomial::from_str("3*x^2+x+7").unwrap();
/// p.mod_shl_assign(3u16, &Natural::from(10u32));
/// assert_eq!(p.to_string(), "4*x^2+8*x+6");
///
/// let mut p = NaturalPolynomial::from_str("3*x^2+5").unwrap();
/// p.mod_shl_assign(2u32, Natural::from(12u32));
/// assert_eq!(p.to_string(), "8");
///
/// let mut p = NaturalPolynomial::from_str("x+1").unwrap();
/// p.mod_shl_assign(100u64, &Natural::from(10u32).pow(30));
/// assert_eq!(
///     p.to_string(),
///     "267650600228229401496703205376*x+267650600228229401496703205376"
/// );
/// ```
pub mod mod_shl;
/// Implementations of [`ModSquare`](malachite_base::num::arithmetic::traits::ModSquare) and
/// [`ModSquareAssign`](malachite_base::num::arithmetic::traits::ModSquareAssign), for squaring a
/// polynomial modulo a [`Natural`](crate::natural::Natural).
pub mod mod_square;
/// Implementations of [`ModSquareTruncated`](malachite_base::polynomial::ModSquareTruncated) and
/// [`ModSquareTruncatedAssign`](malachite_base::polynomial::ModSquareTruncatedAssign), for squaring
/// a polynomial modulo a [`Natural`](crate::natural::Natural) and keeping only the low coefficients
/// of the square.
pub mod mod_square_truncated;
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting one
/// polynomial from another modulo a [`Natural`](crate::natural::Natural).
pub mod mod_sub;
/// Implementations of [`ModSubTruncated`](malachite_base::polynomial::ModSubTruncated) and
/// [`ModSubTruncatedAssign`](malachite_base::polynomial::ModSubTruncatedAssign), for subtracting
/// one polynomial from another modulo a [`Natural`](crate::natural::Natural) and keeping only their
/// low coefficients.
pub mod mod_sub_truncated;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying two polynomials.
pub mod mul;
/// Implementations of [`MulPowerOfX`](malachite_base::polynomial::MulPowerOfX) and
/// [`MulPowerOfXAssign`](malachite_base::polynomial::MulPowerOfXAssign), for multiplying a
/// polynomial by a power of its variable.
pub mod mul_power_of_x;
/// Implementations of [`MulTruncated`](malachite_base::polynomial::MulTruncated) and
/// [`MulTruncatedAssign`](malachite_base::polynomial::MulTruncatedAssign), for multiplying two
/// polynomials and keeping only the low coefficients of the product.
pub mod mul_truncated;
/// Implementations of [`NthDerivative`](malachite_base::polynomial::NthDerivative) and
/// [`NthDerivativeAssign`](malachite_base::polynomial::NthDerivativeAssign), for differentiating a
/// polynomial any number of times.
pub mod nth_derivative;
/// Left-shifting a [`NaturalPolynomial`](super::NaturalPolynomial) (multiplying it by a power of
/// 2), by shifting every coefficient.
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert_eq!((NaturalPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 0u16).to_string(),
///     "x^2+3*x+5"
/// );
/// assert_eq!(
///     (NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 2u32).to_string(),
///     "4*x^2+12*x+20"
/// );
/// assert_eq!(
///     (NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376*x^2+3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
///
/// assert_eq!((&NaturalPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 0u16).to_string(),
///     "x^2+3*x+5"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 2u32).to_string(),
///     "4*x^2+12*x+20"
/// );
/// assert_eq!(
///     (&NaturalPolynomial::from_str("x^2+3*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376*x^2+3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let mut p = NaturalPolynomial::ZERO;
/// p <<= 10u8;
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = NaturalPolynomial::from_str("x^2+3*x+5").unwrap();
/// p <<= 0u16;
/// assert_eq!(p.to_string(), "x^2+3*x+5");
///
/// let mut p = NaturalPolynomial::from_str("x^2+3*x+5").unwrap();
/// p <<= 2u32;
/// assert_eq!(p.to_string(), "4*x^2+12*x+20");
///
/// let mut p = NaturalPolynomial::from_str("x^2+3*x+5").unwrap();
/// p <<= 100u64;
/// assert_eq!(
///     p.to_string(),
///     "1267650600228229401496703205376*x^2+3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
/// ```
pub mod shl;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), for squaring a
/// polynomial.
pub mod square;
/// Implementations of [`SquareTruncated`](malachite_base::polynomial::SquareTruncated) and
/// [`SquareTruncatedAssign`](malachite_base::polynomial::SquareTruncatedAssign), for squaring a
/// polynomial and keeping only the low coefficients of the square.
pub mod square_truncated;
