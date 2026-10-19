// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// matrices of the same shape.
pub mod add;
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::arithmetic::traits::BalancedModAssign), which reduce
/// every element of an [`IntegerMatrix`](super::IntegerMatrix) to the representative closest to
/// zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::arithmetic::traits::BalancedModPowerOf2) and
/// [`BalancedModPowerOf2Assign`](
/// malachite_base::num::arithmetic::traits::BalancedModPowerOf2Assign), which reduce every element
/// of an [`IntegerMatrix`](super::IntegerMatrix) to the representative closest to zero modulo a
/// power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a matrix with its first nonzero element, in row-major order, made positive.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a matrix's elements and the matrix divided by it.
pub mod content;
/// Implementations of [`ContentChained`](malachite_base::num::arithmetic::traits::ContentChained),
/// which computes the GCD of a number and a matrix's elements.
pub mod content_chained;
/// Implementations of [`BalancedCrt`](malachite_base::num::arithmetic::traits::BalancedCrt), which
/// combine a matrix of residues modulo any number with one modulo a word-sized number into balanced
/// representatives by the Chinese remainder theorem.
pub mod crt;
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod),
/// [`Rem`](core::ops::Rem), and [`RemAssign`](core::ops::RemAssign), for reducing every element of
/// an [`IntegerMatrix`](super::IntegerMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2),
/// [`RemPowerOf2`](malachite_base::num::arithmetic::traits::RemPowerOf2), and
/// [`RemPowerOf2Assign`](malachite_base::num::arithmetic::traits::RemPowerOf2Assign), for reducing
/// every element of an [`IntegerMatrix`](super::IntegerMatrix) modulo a power of 2.
pub mod mod_power_of_2;

/// Implementations of [`Mul`](core::ops::Mul) for multiplying an
/// [`IntegerMatrix`](super::IntegerMatrix) by an
/// [`IntegerVector`](crate::integer_vector::IntegerVector) on the right.
pub mod mul_vector;
/// [`IntegerMatrix::multi_balanced_crt`](super::IntegerMatrix::multi_balanced_crt), which combines
/// matrices of residues modulo many word-sized moduli into balanced representatives by the Chinese
/// remainder theorem.
pub mod multi_crt;
/// [`IntegerMatrix::multi_mod`](super::IntegerMatrix::multi_mod), which reduces a matrix modulo
/// many word-sized moduli.
pub mod multi_mod;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a matrix.
pub mod neg;

/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying an [`IntegerMatrix`](super::IntegerMatrix) by a scalar on either side.
pub mod scalar_mul;
/// Left-shifting a matrix (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[]_{0,0}").unwrap() << 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap() << 0u16).to_string(),
///     "[1, -2; 3, 4]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap() << 2u32).to_string(),
///     "[4, -8; 12, 16]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3; 0]").unwrap() << 100u64).to_string(),
///     "[-3802951800684688204490109616128; 0]"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
/// a <<= 2u32;
/// assert_eq!(a.to_string(), "[4, -8; 12, 16]");
/// ```
pub mod shl;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting matrices of the same shape.
pub mod sub;

/// Implementations of [`Mul`](core::ops::Mul) for multiplying an
/// [`IntegerVector`](crate::integer_vector::IntegerVector) by an
/// [`IntegerMatrix`](super::IntegerMatrix) on the right.
pub mod vector_mul;
