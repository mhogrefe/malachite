// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// vectors.
pub mod add;
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod), which
/// reduces every element of a vector to the representative closest to zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::arithmetic::traits::BalancedModPowerOf2), which
/// reduce every element of a vector to the representative closest to zero modulo a power of 2.
pub mod balanced_mod_power_of_2;
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
/// An implementation of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest height of any
/// element.
pub mod height;
/// An implementation of [`L1Norm`](malachite_base::num::arithmetic::traits::L1Norm), the sum of the
/// absolute values of the elements.
pub mod l1_norm;
/// Implementations of [`ModAdd`](malachite_base::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::arithmetic::traits::ModAddAssign), for adding vectors
/// modulo a number.
pub mod mod_add;
/// An implementation of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced),
/// which checks whether every element of a vector is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), for negating a vector
/// modulo a number.
pub mod mod_neg;
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod),
/// [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign), [`Rem`](core::ops::Rem), and
/// [`RemAssign`](core::ops::RemAssign), traits for reducing every element of a vector modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign), which reduce
/// every element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::arithmetic::traits::ModPowerOf2Add)
/// and [`ModPowerOf2AddAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign), for
/// adding vectors modulo a power of 2.
pub mod mod_power_of_2_add;
/// An implementation of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), which
/// checks whether every element of a vector is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign), for
/// negating a vector modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign), for
/// subtracting vectors modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting vectors
/// modulo a number.
pub mod mod_sub;
/// [`NaturalVector::multi_crt`](super::NaturalVector::multi_crt), which combines vectors of
/// residues modulo many word-sized moduli by the Chinese remainder theorem.
pub mod multi_crt;
