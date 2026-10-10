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
/// Adding a multiple of one vector to another, by implementations of
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign).
///
/// # add_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     IntegerVector::from_str("()")
///         .unwrap()
///         .add_mul(IntegerVector::from_str("()").unwrap(), Integer::from(5))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .add_mul(
///             IntegerVector::from_str("(4, 5, -6)").unwrap(),
///             Integer::ZERO
///         )
///         .to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .add_mul(IntegerVector::from_str("(4, 5, -6)").unwrap(), Integer::ONE)
///         .to_string(),
///     "(5, 3, -3)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .add_mul(
///             IntegerVector::from_str("(4, 5, -6)").unwrap(),
///             Integer::NEGATIVE_ONE
///         )
///         .to_string(),
///     "(-3, -7, 9)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(1, -2, 3)").unwrap())
///         .add_mul(
///             &IntegerVector::from_str("(4, 5, -6)").unwrap(),
///             &Integer::from(10)
///         )
///         .to_string(),
///     "(41, 48, -57)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, 0)")
///         .unwrap()
///         .add_mul(
///             &IntegerVector::from_str("(18446744073709551615, -1)").unwrap(),
///             &Integer::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "(340282366920938463463374607431768211456, -18446744073709551617)"
/// );
/// ```
///
/// # add_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
/// v.add_mul_assign(
///     IntegerVector::from_str("(4, 5, -6)").unwrap(),
///     Integer::from(10),
/// );
/// assert_eq!(v.to_string(), "(41, 48, -57)");
/// ```
pub mod add_mul;
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::arithmetic::traits::BalancedModAssign), which reduce
/// every element of a vector to the representative closest to zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::arithmetic::traits::BalancedModPowerOf2) and
/// [`BalancedModPowerOf2Assign`](
/// malachite_base::num::arithmetic::traits::BalancedModPowerOf2Assign), which reduce every element
/// of a vector to the representative closest to zero modulo a power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a vector with its first nonzero element made positive.
pub mod canonical_primitive_part;
/// Implementations of
/// [`CanonicalizeSign`](malachite_base::num::arithmetic::traits::CanonicalizeSign) and
/// [`CanonicalizeSignAssign`](malachite_base::num::arithmetic::traits::CanonicalizeSignAssign),
/// which negate a vector if its pivot is negative.
pub mod canonicalize_sign;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a vector's elements and the vector divided by it.
pub mod content;
#[doc(hidden)]
pub mod content_chained;
/// Exact division of a vector by a scalar, by implementations of
/// [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::num::basic::traits::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     IntegerVector::from_str("()")
///         .unwrap()
///         .div_exact(Integer::from(5))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(0, -6, 255)")
///         .unwrap()
///         .div_exact(Integer::ONE)
///         .to_string(),
///     "(0, -6, 255)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(0, -6, 255)")
///         .unwrap()
///         .div_exact(Integer::NEGATIVE_ONE)
///         .to_string(),
///     "(0, 6, -255)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(0, -6, 255)").unwrap())
///         .div_exact(Integer::from(-3))
///         .to_string(),
///     "(0, 2, -85)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-3802951800684688204490109616128, 6)").unwrap())
///         .div_exact(Integer::from(3))
///         .to_string(),
///     "(-1267650600228229401496703205376, 2)"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(0, -6, 255)").unwrap();
/// v.div_exact_assign(Integer::from(-3));
/// assert_eq!(v.to_string(), "(0, 2, -85)");
/// ```
pub mod div_exact;
#[doc(hidden)]
pub mod dot_general;
/// Implementations of [`EntrywiseAbs`](malachite_base::num::arithmetic::traits::EntrywiseAbs),
/// [`EntrywiseAbsAssign`](malachite_base::num::arithmetic::traits::EntrywiseAbsAssign), and
/// [`EntrywiseUnsignedAbs`](malachite_base::num::arithmetic::traits::EntrywiseUnsignedAbs), which
/// replace every element of a vector by its absolute value.
pub mod entrywise_abs;
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
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     IntegerVector::from_str("(-3, -1, 1, 3)")
///         .unwrap()
///         .entrywise_div_round(Integer::TWO, Floor)
///         .to_string(),
///     "(-2, -1, 0, 1)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-3, -1, 1, 3)").unwrap())
///         .entrywise_div_round(Integer::TWO, Down)
///         .to_string(),
///     "(-1, 0, 0, 1)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-3, -1, 1, 3)").unwrap())
///         .entrywise_div_round(Integer::TWO, Ceiling)
///         .to_string(),
///     "(-1, 0, 1, 2)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(-3, -1, 1, 3)")
///         .unwrap()
///         .entrywise_div_round(Integer::TWO, Nearest)
///         .to_string(),
///     "(-2, 0, 0, 2)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(3, -6, 255)").unwrap())
///         .entrywise_div_round(Integer::from(-3), Exact)
///         .to_string(),
///     "(-1, 2, -85)"
/// );
/// ```
///
/// # entrywise_div_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRoundAssign;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(-3, -1, 1, 3)").unwrap();
/// v.entrywise_div_round_assign(Integer::TWO, Floor);
/// assert_eq!(v.to_string(), "(-2, -1, 0, 1)");
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
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i8, Floor))
///     .to_string(),
///     "(-1, 1, -2, 2)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i16, Down))
///     .to_string(),
///     "(0, 1, -1, 2)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shl_round(-1i32, Nearest))
///         .to_string(),
///     "(0, 1, -2, 2)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(4, -8, 12)")
///         .unwrap()
///         .entrywise_shl_round(-2i64, Exact))
///     .to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .entrywise_shl_round(2i8, Exact))
///         .to_string(),
///     "(4, -8, 12)"
/// );
/// ```
///
/// # entrywise_shl_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(-1, 2, -3, 5)").unwrap();
/// v.entrywise_shl_round_assign(-1i8, Down);
/// assert_eq!(v.to_string(), "(0, 1, -1, 2)");
/// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
/// v.entrywise_shl_round_assign(2i32, Exact);
/// assert_eq!(v.to_string(), "(4, -8, 12)");
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
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u8, Floor))
///     .to_string(),
///     "(-1, 1, -2, 2)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u16, Down))
///     .to_string(),
///     "(0, 1, -1, 2)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u32, Ceiling))
///     .to_string(),
///     "(0, 1, -1, 3)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u64, Up))
///         .to_string(),
///     "(-1, 1, -2, 3)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1u8, Nearest))
///     .to_string(),
///     "(0, 1, -2, 2)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(4, -8, 12)")
///         .unwrap()
///         .entrywise_shr_round(2u16, Exact))
///     .to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)")
///         .unwrap()
///         .entrywise_shr_round(1i8, Down))
///     .to_string(),
///     "(0, 1, -1, 2)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .entrywise_shr_round(-2i16, Exact))
///         .to_string(),
///     "(4, -8, 12)"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(-1, 2, -3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1u8, Down);
/// assert_eq!(v.to_string(), "(0, 1, -1, 2)");
/// let mut v = IntegerVector::from_str("(-1, 2, -3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1i32, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, -2, 2)");
/// ```
pub mod entrywise_shr_round;
/// An implementation of [`Height`](malachite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef), the largest height of any
/// element.
pub mod height;
/// An implementation of [`L1Norm`](malachite_base::num::arithmetic::traits::L1Norm), the sum of the
/// absolute values of the elements.
pub mod l1_norm;
#[doc(hidden)]
pub mod max_bits;
#[doc(hidden)]
pub mod max_limbs;
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod),
/// [`Rem`](core::ops::Rem), and [`RemAssign`](core::ops::RemAssign), for reducing every element of
/// a vector modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2),
/// [`RemPowerOf2`](malachite_base::num::arithmetic::traits::RemPowerOf2), and
/// [`RemPowerOf2Assign`](malachite_base::num::arithmetic::traits::RemPowerOf2Assign), for reducing
/// every element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// [`IntegerVector::multi_balanced_crt`](super::IntegerVector::multi_balanced_crt), which combines
/// vectors of residues modulo many word-sized moduli into balanced representatives by the Chinese
/// remainder theorem.
pub mod multi_crt;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a vector.
pub mod neg;
/// Implementations of [`Div`](core::ops::Div) and [`DivAssign`](core::ops::DivAssign), for dividing
/// a vector by a scalar.
///
/// # div
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Two};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     (IntegerVector::from_str("()").unwrap() / Integer::from(5)).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(0, -7, 255)").unwrap() / Integer::ONE).to_string(),
///     "(0, -7, 255)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(0, -7, 255)").unwrap() / Integer::NEGATIVE_ONE).to_string(),
///     "(0, 7, -255)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(0, -7, 255)").unwrap() / Integer::TWO).to_string(),
///     "(0, -3, 127)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(0, -7, 255)").unwrap() / Integer::from(-2)).to_string(),
///     "(0, 3, -127)"
/// );
/// ```
///
/// # div_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Two;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(0, -7, 255)").unwrap();
/// v /= Integer::TWO;
/// assert_eq!(v.to_string(), "(0, -3, 127)");
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
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     (IntegerVector::from_str("()").unwrap() << 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(1, -2, 3)").unwrap() << 0u16).to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(1, -2, 3)").unwrap() << 2u32).to_string(),
///     "(4, -8, 12)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-3, 0)").unwrap() << 100u64).to_string(),
///     "(-3802951800684688204490109616128, 0)"
/// );
///
/// assert_eq!(
///     (IntegerVector::from_str("(1, -2, 3)").unwrap() << 2i8).to_string(),
///     "(4, -8, 12)"
/// );
///
/// assert_eq!(
///     (&IntegerVector::from_str("(-1, 2, -3, 5)").unwrap() << -1i64).to_string(),
///     "(-1, 1, -2, 2)"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
/// v <<= 2u32;
/// assert_eq!(v.to_string(), "(4, -8, 12)");
///
/// let mut v = IntegerVector::from_str("(-1, 2, -3, 5)").unwrap();
/// v <<= -1i32;
/// assert_eq!(v.to_string(), "(-1, 1, -2, 2)");
/// ```
pub mod shl;
/// Right-shifting a vector (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     (IntegerVector::from_str("()").unwrap() >> 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(1, -2, 3)").unwrap() >> 0u16).to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(4, -8, 12)").unwrap() >> 2u32).to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(-1, 2, -3, 5)").unwrap() >> 1u8).to_string(),
///     "(-1, 1, -2, 2)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-3802951800684688204490109616128, 0)").unwrap() >> 100u64)
///         .to_string(),
///     "(-3, 0)"
/// );
/// assert_eq!(
///     (IntegerVector::from_str("(1, -2, 3)").unwrap() >> -2i8).to_string(),
///     "(4, -8, 12)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(-1, 2, -3, 5)").unwrap() >> 1i64).to_string(),
///     "(-1, 1, -2, 2)"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(-1, 2, -3, 5)").unwrap();
/// v >>= 1u32;
/// assert_eq!(v.to_string(), "(-1, 1, -2, 2)");
/// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
/// v >>= -2i32;
/// assert_eq!(v.to_string(), "(4, -8, 12)");
/// ```
pub mod shr;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting vectors.
pub mod sub;
/// Subtracting a multiple of one vector from another, by implementations of
/// [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign).
///
/// # sub_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// assert_eq!(
///     IntegerVector::from_str("()")
///         .unwrap()
///         .sub_mul(IntegerVector::from_str("()").unwrap(), Integer::from(5))
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .sub_mul(
///             IntegerVector::from_str("(4, 5, -6)").unwrap(),
///             Integer::ZERO
///         )
///         .to_string(),
///     "(1, -2, 3)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .sub_mul(IntegerVector::from_str("(4, 5, -6)").unwrap(), Integer::ONE)
///         .to_string(),
///     "(-3, -7, 9)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, -2, 3)")
///         .unwrap()
///         .sub_mul(
///             IntegerVector::from_str("(4, 5, -6)").unwrap(),
///             Integer::NEGATIVE_ONE
///         )
///         .to_string(),
///     "(5, 3, -3)"
/// );
/// assert_eq!(
///     (&IntegerVector::from_str("(1, -2, 3)").unwrap())
///         .sub_mul(
///             &IntegerVector::from_str("(4, 5, -6)").unwrap(),
///             &Integer::from(10)
///         )
///         .to_string(),
///     "(-39, -52, 63)"
/// );
/// assert_eq!(
///     IntegerVector::from_str("(1, 0)")
///         .unwrap()
///         .sub_mul(
///             &IntegerVector::from_str("(18446744073709551615, -1)").unwrap(),
///             &Integer::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "(-340282366920938463463374607431768211454, 18446744073709551617)"
/// );
/// ```
///
/// # sub_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
/// v.sub_mul_assign(
///     IntegerVector::from_str("(4, 5, -6)").unwrap(),
///     Integer::from(10),
/// );
/// assert_eq!(v.to_string(), "(-39, -52, 63)");
/// ```
pub mod sub_mul;
#[doc(hidden)]
pub mod sum_max_bits;
