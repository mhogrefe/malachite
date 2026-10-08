// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a vector
/// with non-negative elements, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a vector's elements and the vector divided by it.
pub mod content;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced),
/// which checks whether every element of a vector is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod),
/// [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign), [`Rem`](core::ops::Rem), and
/// [`RemAssign`](core::ops::RemAssign), traits for reducing every element of a vector modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), which reduce
/// every element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), which
/// checks whether every element of a vector is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign), for
/// negating a vector modulo a power of 2.
pub mod mod_power_of_2_neg;
