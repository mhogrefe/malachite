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
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::arithmetic::traits::BalancedModAssign), which reduce
/// every coefficient of a polynomial to the representative closest to zero.
pub mod balanced_mod;
/// Implementations of [`BitPack`](malachite_base::polynomial::BitPack), which packs a polynomial's
/// coefficients into fixed-width fields of a single integer.
pub mod bit_pack;
/// Implementations of [`BitUnpack`](malachite_base::polynomial::BitUnpack), which unpacks a
/// polynomial from the fixed-width fields of a single number.
pub mod bit_unpack;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// which bring a polynomial into canonical unit form.
pub mod canonicalize_unit;
#[doc(hidden)]
pub mod coefficient;
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
#[doc(hidden)]
pub mod content_chained;
/// Implementations of [`DeflatePowerOfX`](malachite_base::polynomial::DeflatePowerOfX) and
/// [`DeflatePowerOfXAssign`](malachite_base::polynomial::DeflatePowerOfXAssign), for undoing the
/// substitution of a power of the variable into a polynomial.
pub mod deflate_power_of_x;
/// Implementations of [`Derivative`](malachite_base::polynomial::Derivative) and
/// [`DerivativeAssign`](malachite_base::polynomial::DerivativeAssign), for differentiating a
/// polynomial.
pub mod derivative;
/// Implementations of [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign), for dividing a
/// polynomial by an [`Integer`](crate::integer::Integer) that divides every coefficient.
pub mod div_exact;
/// Implementations of [`DivPowerOfX`](malachite_base::polynomial::DivPowerOfX) and
/// [`DivPowerOfXAssign`](malachite_base::polynomial::DivPowerOfXAssign), for dividing a polynomial
/// by a power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// Implementations of [`Evaluate`](malachite_base::polynomial::Evaluate), which evaluates a
/// polynomial at a value.
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
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod), which reduces every
/// coefficient of a polynomial into $[0, m)$, producing a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial) or an
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial), and of
/// [`Rem`](core::ops::Rem) and [`RemAssign`](core::ops::RemAssign), which keep each remainder's
/// sign.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2), which
/// reduces every coefficient of a polynomial into $[0, 2^k)$, producing a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial), and of
/// [`RemPowerOf2`](malachite_base::num::arithmetic::traits::RemPowerOf2) and
/// [`RemPowerOf2Assign`](malachite_base::num::arithmetic::traits::RemPowerOf2Assign), which keep
/// each remainder's sign.
pub mod mod_power_of_2;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying two polynomials.
pub mod mul;
#[doc(hidden)]
pub mod mul_high;
#[doc(hidden)]
pub mod mul_middle;
/// Implementations of [`MulPowerOfX`](malachite_base::polynomial::MulPowerOfX) and
/// [`MulPowerOfXAssign`](malachite_base::polynomial::MulPowerOfXAssign), for multiplying a
/// polynomial by a power of its variable.
pub mod mul_power_of_x;
/// Implementations of [`MulTruncated`](malachite_base::polynomial::MulTruncated) and
/// [`MulTruncatedAssign`](malachite_base::polynomial::MulTruncatedAssign), for multiplying two
/// polynomials and keeping only the low coefficients of the product.
pub mod mul_truncated;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a polynomial.
pub mod neg;
/// Implementations of [`NthDerivative`](malachite_base::polynomial::NthDerivative) and
/// [`NthDerivativeAssign`](malachite_base::polynomial::NthDerivativeAssign), for differentiating a
/// polynomial any number of times.
pub mod nth_derivative;
#[doc(hidden)]
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), for raising a polynomial to a
/// power.
pub mod pow;
/// Implementations of [`PowTruncated`](malachite_base::polynomial::PowTruncated) and
/// [`PowTruncatedAssign`](malachite_base::polynomial::PowTruncatedAssign), for raising a polynomial
/// to a power and keeping only the low coefficients of the power.
pub mod pow_truncated;
pub mod scalar_add_mul;
#[doc(hidden)]
pub mod scalar_mul;
/// Left-shifting an [`IntegerPolynomial`](super::IntegerPolynomial) (multiplying it by a power of
/// 2), by shifting every coefficient.
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// assert_eq!((IntegerPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (IntegerPolynomial::from_str("x^2-3*x+5").unwrap() << 0u16).to_string(),
///     "x^2-3*x+5"
/// );
/// assert_eq!(
///     (IntegerPolynomial::from_str("x^2-3*x+5").unwrap() << 2u32).to_string(),
///     "4*x^2-12*x+20"
/// );
/// assert_eq!(
///     (IntegerPolynomial::from_str("x^2-3*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376*x^2-3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
///
/// assert_eq!((&IntegerPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (&IntegerPolynomial::from_str("x^2-3*x+5").unwrap() << 0u16).to_string(),
///     "x^2-3*x+5"
/// );
/// assert_eq!(
///     (&IntegerPolynomial::from_str("x^2-3*x+5").unwrap() << 2u32).to_string(),
///     "4*x^2-12*x+20"
/// );
/// assert_eq!(
///     (&IntegerPolynomial::from_str("x^2-3*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376*x^2-3802951800684688204490109616128*x+\
///     6338253001141147007483516026880"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// let mut p = IntegerPolynomial::ZERO;
/// p <<= 10u8;
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = IntegerPolynomial::from_str("x^2-3*x+5").unwrap();
/// p <<= 0u16;
/// assert_eq!(p.to_string(), "x^2-3*x+5");
///
/// let mut p = IntegerPolynomial::from_str("x^2-3*x+5").unwrap();
/// p <<= 2u32;
/// assert_eq!(p.to_string(), "4*x^2-12*x+20");
///
/// let mut p = IntegerPolynomial::from_str("x^2-3*x+5").unwrap();
/// p <<= 100u64;
/// assert_eq!(
///     p.to_string(),
///     "1267650600228229401496703205376*x^2-3802951800684688204490109616128*x+\
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
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting one polynomial from another.
pub mod sub;
/// Implementations of [`SubTruncated`](malachite_base::polynomial::SubTruncated) and
/// [`SubTruncatedAssign`](malachite_base::polynomial::SubTruncatedAssign), for subtracting one
/// polynomial from another and keeping only their low coefficients.
pub mod sub_truncated;
#[doc(hidden)]
pub mod vec;
