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
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// which bring a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Content`](malachite_base::polynomial::Content),
/// [`PrimitivePart`](malachite_base::polynomial::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::polynomial::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::polynomial::ContentAndPrimitivePart), which compute
/// the GCD of a polynomial's coefficients and the polynomial divided by it.
pub mod content;
/// Implementations of [`Evaluate`](malachite_base::polynomial::Evaluate), which evaluates a
/// polynomial at a value, and of
/// [`ModPowerOf2Evaluate`](malachite_base::polynomial::ModPowerOf2Evaluate), which does so modulo a
/// power of 2.
pub mod evaluate;
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
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced),
/// which checks whether every coefficient of a polynomial is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMakeMonic`](malachite_base::polynomial::ModMakeMonic) and
/// [`ModMakeMonicAssign`](malachite_base::polynomial::ModMakeMonicAssign), which make a polynomial
/// monic modulo a value.
pub mod mod_make_monic;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), which negate a
/// polynomial modulo a number.
pub mod mod_neg;
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
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), which
/// checks whether every coefficient of a polynomial is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign),
/// which negate a polynomial modulo a power of 2.
pub mod mod_power_of_2_neg;
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
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting one
/// polynomial from another modulo a [`Natural`](crate::natural::Natural).
pub mod mod_sub;
/// Implementations of [`ModSubTruncated`](malachite_base::polynomial::ModSubTruncated) and
/// [`ModSubTruncatedAssign`](malachite_base::polynomial::ModSubTruncatedAssign), for subtracting
/// one polynomial from another modulo a [`Natural`](crate::natural::Natural) and keeping only their
/// low coefficients.
pub mod mod_sub_truncated;
/// Implementations of [`MulPowerOfX`](malachite_base::polynomial::MulPowerOfX) and
/// [`MulPowerOfXAssign`](malachite_base::polynomial::MulPowerOfXAssign), for multiplying a
/// polynomial by a power of its variable.
pub mod mul_power_of_x;
