// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a number.
pub mod mod_is_reduced;
/// Implementations of [`ModNeg`](crate::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::arithmetic::traits::ModNegAssign), for negating every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_neg;
/// Implementations of [`Rem`](core::ops::Rem), [`RemAssign`](core::ops::RemAssign),
/// [`Mod`](crate::num::arithmetic::traits::Mod), and
/// [`ModAssign`](crate::num::arithmetic::traits::ModAssign), for reducing every element of an
/// [`UnsignedMatrix`](super::UnsignedMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](crate::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::arithmetic::traits::ModPowerOf2Assign), traits for reducing
/// every element of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a power of 2.
pub mod mod_power_of_2;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), a trait for
/// checking whether every element of an [`UnsignedMatrix`](super::UnsignedMatrix) is reduced modulo
/// a power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Neg`](crate::num::arithmetic::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::arithmetic::traits::ModPowerOf2NegAssign), for negating
/// every element of an [`UnsignedMatrix`](super::UnsignedMatrix) modulo a power of 2.
pub mod mod_power_of_2_neg;
