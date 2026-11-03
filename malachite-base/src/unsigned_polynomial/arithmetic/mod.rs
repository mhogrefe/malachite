// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of
/// [`CanonicalPrimitivePart`](crate::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](crate::num::arithmetic::traits::CanonicalPrimitivePartAssign),
/// and [`ContentAndCanonicalPrimitivePart`](
/// crate::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a polynomial with
/// non-negative coefficients, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of [`CanonicalizeUnit`](crate::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](crate::num::arithmetic::traits::CanonicalizeUnitAssign), which bring
/// a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`ComposePowerOfX`](crate::polynomial::arithmetic::traits::ComposePowerOfX)
/// and [`ComposePowerOfXAssign`](crate::polynomial::arithmetic::traits::ComposePowerOfXAssign), for
/// substituting a power of the variable into a polynomial.
pub mod compose_power_of_x;
/// Implementations of [`Content`](crate::num::arithmetic::traits::Content),
/// [`PrimitivePart`](crate::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](crate::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](crate::num::arithmetic::traits::ContentAndPrimitivePart), which
/// compute the GCD of a polynomial's coefficients and the polynomial divided by it.
pub mod content;
/// Implementations of [`DeflatePowerOfX`](crate::polynomial::arithmetic::traits::DeflatePowerOfX)
/// and [`DeflatePowerOfXAssign`](crate::polynomial::arithmetic::traits::DeflatePowerOfXAssign), for
/// undoing the substitution of a power of the variable into a polynomial.
pub mod deflate_power_of_x;
/// Implementations of [`DivPowerOfX`](crate::polynomial::arithmetic::traits::DivPowerOfX) and
/// [`DivPowerOfXAssign`](crate::polynomial::arithmetic::traits::DivPowerOfXAssign), for dividing a
/// polynomial by a power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// An implementation of [`ExponentGcd`](crate::polynomial::arithmetic::traits::ExponentGcd), the
/// greatest common divisor of the exponents at which a polynomial has nonzero coefficients.
pub mod exponent_gcd;
/// An implementation of [`Height`](crate::num::arithmetic::traits::Height), the largest of a
/// polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](crate::num::arithmetic::traits::IsUnit), a trait for determining
/// whether a number is a unit of its ring.
pub mod is_unit;
/// Implementations of [`MulPowerOfX`](crate::polynomial::arithmetic::traits::MulPowerOfX) and
/// [`MulPowerOfXAssign`](crate::polynomial::arithmetic::traits::MulPowerOfXAssign), for multiplying
/// a polynomial by a power of its variable.
pub mod mul_power_of_x;
#[doc(hidden)]
pub mod ntt;
