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
/// An implementation of [`ExponentGcd`](crate::polynomial::ExponentGcd), the greatest common
/// divisor of the exponents at which a polynomial has nonzero coefficients.
pub mod deflate_power_of_x;
/// Implementations of [`DivPowerOfX`](crate::polynomial::DivPowerOfX) and
/// [`DivPowerOfXAssign`](crate::polynomial::DivPowerOfXAssign), for dividing a polynomial by a
/// power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// Implementations of [`ModEvaluate`](crate::polynomial::ModEvaluate) and
/// [`ModPowerOf2Evaluate`](crate::polynomial::ModPowerOf2Evaluate), which evaluate a polynomial at
/// a value modulo a value or a power of 2.
pub mod evaluate;
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
/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), which
/// checks whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMakeMonic`](crate::polynomial::ModMakeMonic) and
/// [`ModMakeMonicAssign`](crate::polynomial::ModMakeMonicAssign), which make a polynomial monic
/// modulo a value.
pub mod mod_make_monic;
/// Implementations of [`ModNeg`](crate::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::arithmetic::traits::ModNegAssign), which negate a polynomial modulo
/// a number.
pub mod mod_neg;
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
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), which checks
/// whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Neg`](crate::num::arithmetic::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::arithmetic::traits::ModPowerOf2NegAssign), which negate a
/// polynomial modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Sub`](crate::num::arithmetic::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::arithmetic::traits::ModPowerOf2SubAssign), for subtracting
/// one polynomial from another modulo $2^k$.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModPowerOf2SubTruncated`](crate::polynomial::ModPowerOf2SubTruncated) and
/// [`ModPowerOf2SubTruncatedAssign`](crate::polynomial::ModPowerOf2SubTruncatedAssign), for
/// subtracting one polynomial from another modulo $2^k$ and keeping only their low coefficients.
pub mod mod_power_of_2_sub_truncated;
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
