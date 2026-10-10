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
/// Exact division of a vector by a scalar, by implementations of
/// [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     NaturalVector::from_str("()")
///         .unwrap()
///         .div_exact(Natural::from(5u32))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(0, 6, 255)")
///         .unwrap()
///         .div_exact(Natural::ONE)
///         .to_string(),
///     "(0, 6, 255)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(0, 6, 255)").unwrap())
///         .div_exact(Natural::from(3u32))
///         .to_string(),
///     "(0, 2, 85)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3802951800684688204490109616128, 6)").unwrap())
///         .div_exact(Natural::from(3u32))
///         .to_string(),
///     "(1267650600228229401496703205376, 2)"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(0, 6, 255)").unwrap();
/// v.div_exact_assign(Natural::from(3u32));
/// assert_eq!(v.to_string(), "(0, 2, 85)");
/// ```
pub mod div_exact;
/// Dividing every element of a vector by a scalar and rounding each quotient, by implementations of
/// [`EntrywiseDivRound`](malachite_base::num::arithmetic::traits::EntrywiseDivRound) and
/// [`EntrywiseDivRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseDivRoundAssign).
///
/// # entrywise_div_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRound;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_div_round(Natural::TWO, Floor)
///         .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)").unwrap())
///         .entrywise_div_round(Natural::TWO, Ceiling)
///         .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)").unwrap())
///         .entrywise_div_round(Natural::TWO, Nearest)
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     NaturalVector::from_str("(3, 6, 255)")
///         .unwrap()
///         .entrywise_div_round(Natural::from(3u32), Exact)
///         .to_string(),
///     "(1, 2, 85)"
/// );
/// ```
///
/// # entrywise_div_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRoundAssign;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_div_round_assign(Natural::TWO, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// ```
pub mod entrywise_div_round;
/// Left-shifting a vector (multiplying it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShlRound`](malachite_base::num::arithmetic::traits::EntrywiseShlRound) and
/// [`EntrywiseShlRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign).
///
/// # entrywise_shl_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i8, Floor))
///     .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i16, Ceiling))
///     .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i32, Nearest))
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(4, 8, 12)")
///         .unwrap()
///         .entrywise_shl_round(-2i64, Exact))
///     .to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3)")
///         .unwrap()
///         .entrywise_shl_round(2i8, Exact))
///         .to_string(),
///     "(4, 8, 12)"
/// );
/// ```
///
/// # entrywise_shl_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shl_round_assign(-1i8, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v.entrywise_shl_round_assign(2i32, Exact);
/// assert_eq!(v.to_string(), "(4, 8, 12)");
/// ```
pub mod entrywise_shl_round;
/// Right-shifting a vector (dividing it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShrRound`](malachite_base::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u8, Floor))
///     .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u16, Ceiling))
///     .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u32, Nearest))
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(4, 8, 12)")
///         .unwrap()
///         .entrywise_shr_round(2u64, Exact))
///     .to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1i8, Down))
///     .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3)")
///         .unwrap()
///         .entrywise_shr_round(-2i16, Exact))
///         .to_string(),
///     "(4, 8, 12)"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1u8, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1i32, Up);
/// assert_eq!(v.to_string(), "(1, 1, 2, 3)");
/// ```
pub mod entrywise_shr_round;
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
/// Implementations of [`ModMul`](malachite_base::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign), for multiplying a
/// vector by a scalar modulo a number.
pub mod mod_mul;
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
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign), for
/// multiplying a vector by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign), for
/// negating a vector modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Shl`](malachite_base::num::arithmetic::traits::ModPowerOf2Shl)
/// and [`ModPowerOf2ShlAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign), for
/// left-shifting a vector (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// assert_eq!(
///     v.clone().mod_power_of_2_shl(1u8, 3).to_string(),
///     "(2, 2, 6)"
/// );
/// // Shifting by at least the power zeroes every element, keeping the dimension.
/// assert_eq!(v.mod_power_of_2_shl(3u32, 3).to_string(), "(0, 0, 0)");
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 128, 1)").unwrap())
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "(2, 0, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 1)").unwrap())
///         .mod_power_of_2_shl(100u128, 101)
///         .to_string(),
///     "(1267650600228229401496703205376, 1267650600228229401496703205376)"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// v.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(v.to_string(), "(2, 2, 6)");
///
/// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// v.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(v.to_string(), "(0, 0, 0)");
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign), for
/// subtracting vectors modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModShl`](malachite_base::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](malachite_base::num::arithmetic::traits::ModShlAssign), for left-shifting a
/// vector (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShl;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// assert_eq!(
///     v.clone().mod_shl(1u8, Natural::from(7u32)).to_string(),
///     "(3, 2, 6)"
/// );
/// // The modulus need not be odd, so elements can become zero, keeping the dimension.
/// assert_eq!(
///     v.mod_shl(3u32, Natural::from(8u32)).to_string(),
///     "(0, 0, 0)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 1)").unwrap())
///         .mod_shl(
///             100u64,
///             Natural::from_str("1000000000000000000000000000000").unwrap()
///         )
///         .to_string(),
///     "(267650600228229401496703205376, 267650600228229401496703205376)"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// v.mod_shl_assign(1u8, Natural::from(7u32));
/// assert_eq!(v.to_string(), "(3, 2, 6)");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), for subtracting vectors
/// modulo a number.
pub mod mod_sub;
/// [`NaturalVector::multi_crt`](super::NaturalVector::multi_crt), which combines vectors of
/// residues modulo many word-sized moduli by the Chinese remainder theorem.
pub mod multi_crt;
/// Implementations of [`Div`](core::ops::Div) and [`DivAssign`](core::ops::DivAssign), for dividing
/// a vector by a scalar.
///
/// # div
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("()").unwrap() / Natural::from(5u32)).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(0, 7, 255)").unwrap() / Natural::ONE).to_string(),
///     "(0, 7, 255)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(0, 7, 255)").unwrap() / Natural::TWO).to_string(),
///     "(0, 3, 127)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3802951800684688204490109616128, 7)").unwrap()
///         / Natural::from(3u32))
///     .to_string(),
///     "(1267650600228229401496703205376, 2)"
/// );
/// ```
///
/// # div_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(0, 7, 255)").unwrap();
/// v /= Natural::TWO;
/// assert_eq!(v.to_string(), "(0, 3, 127)");
/// ```
pub mod scalar_div;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying a vector by a scalar, with the scalar on either side.
pub mod scalar_mul;
/// Left-shifting a vector (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("()").unwrap() << 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() << 0u16).to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() << 2u32).to_string(),
///     "(4, 8, 12)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3, 0)").unwrap() << 100u64).to_string(),
///     "(3802951800684688204490109616128, 0)"
/// );
///
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() << 2i8).to_string(),
///     "(4, 8, 12)"
/// );
///
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 2, 3, 5)").unwrap() << -1i64).to_string(),
///     "(0, 1, 1, 2)"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v <<= 2u32;
/// assert_eq!(v.to_string(), "(4, 8, 12)");
///
/// let mut v = NaturalVector::from_str("(1, 2, 3, 5)").unwrap();
/// v <<= -1i32;
/// assert_eq!(v.to_string(), "(0, 1, 1, 2)");
/// ```
pub mod shl;
/// Right-shifting a vector (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     (NaturalVector::from_str("()").unwrap() >> 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() >> 0u16).to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(4, 8, 12)").unwrap() >> 2u32).to_string(),
///     "(1, 2, 3)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3, 5)").unwrap() >> 1u8).to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(3802951800684688204490109616128, 0)").unwrap() >> 100u64)
///         .to_string(),
///     "(3, 0)"
/// );
/// assert_eq!(
///     (NaturalVector::from_str("(1, 2, 3)").unwrap() >> -2i8).to_string(),
///     "(4, 8, 12)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(4, 8, 12)").unwrap() >> 2i64).to_string(),
///     "(1, 2, 3)"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(4, 8, 12)").unwrap();
/// v >>= 2u32;
/// assert_eq!(v.to_string(), "(1, 2, 3)");
/// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
/// v >>= -2i32;
/// assert_eq!(v.to_string(), "(4, 8, 12)");
/// ```
pub mod shr;
