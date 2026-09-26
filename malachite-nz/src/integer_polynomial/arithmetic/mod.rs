// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::arithmetic::traits::BalancedModAssign), which reduce
/// every coefficient of a polynomial to the representative closest to zero.
pub mod balanced_mod;
/// Implementations of [`Evaluate`](malachite_base::polynomial::Evaluate), which evaluates a
/// polynomial at a value. Implementations of [`Content`](malachite_base::polynomial::Content),
/// [`PrimitivePart`](malachite_base::polynomial::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::polynomial::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::polynomial::ContentAndPrimitivePart), which compute
/// the GCD of a polynomial's coefficients and the polynomial divided by it.
pub mod content;
pub mod evaluate;
/// Implementations of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest of the magnitudes
/// of a polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
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
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a polynomial.
pub mod neg;
