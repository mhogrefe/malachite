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
#[doc(hidden)]
pub mod content_chained;
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
/// Implementations of [`MulPowerOfX`](malachite_base::polynomial::MulPowerOfX) and
/// [`MulPowerOfXAssign`](malachite_base::polynomial::MulPowerOfXAssign), for multiplying a
/// polynomial by a power of its variable.
pub mod mul_power_of_x;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a polynomial.
pub mod neg;
#[doc(hidden)]
pub mod scalar_add_mul;
#[doc(hidden)]
pub mod scalar_mul;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting one polynomial from another.
pub mod sub;
/// Implementations of [`SubTruncated`](malachite_base::polynomial::SubTruncated) and
/// [`SubTruncatedAssign`](malachite_base::polynomial::SubTruncatedAssign), for subtracting one
/// polynomial from another and keeping only their low coefficients.
pub mod sub_truncated;
