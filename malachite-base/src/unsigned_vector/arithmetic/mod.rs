// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`CanonicalPrimitivePart`](
/// crate::num::arithmetic::traits::CanonicalPrimitivePart), [`CanonicalPrimitivePartAssign`](
/// crate::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// crate::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a vector with
/// non-negative elements, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](crate::num::arithmetic::traits::Content),
/// [`PrimitivePart`](crate::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](crate::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](crate::num::arithmetic::traits::ContentAndPrimitivePart), which
/// compute the GCD of a vector's elements and the vector divided by it.
pub mod content;
/// Exact division of a vector by a scalar, by implementations of
/// [`DivExact`](crate::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](crate::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("()")
///         .unwrap()
///         .div_exact(5)
///         .to_string(),
///     "()"
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("(0, 6, 255)")
///         .unwrap()
///         .div_exact(1)
///         .to_string(),
///     "(0, 6, 255)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(0, 6, 255)").unwrap())
///         .div_exact(3)
///         .to_string(),
///     "(0, 2, 85)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u64>::from_str("(18446744073709551614, 4)").unwrap())
///         .div_exact(2)
///         .to_string(),
///     "(9223372036854775807, 2)"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(0, 6, 255)").unwrap();
/// v.div_exact_assign(3);
/// assert_eq!(v.to_string(), "(0, 2, 85)");
/// ```
pub mod div_exact;
/// Dividing every element of a vector by a scalar and rounding each quotient, by implementations of
/// [`EntrywiseDivRound`](crate::num::arithmetic::traits::EntrywiseDivRound) and
/// [`EntrywiseDivRoundAssign`](crate::num::arithmetic::traits::EntrywiseDivRoundAssign).
///
/// # entrywise_div_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("(1, 2, 3, 5)")
///         .unwrap()
///         .entrywise_div_round(2u8, Floor)
///         .to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap())
///         .entrywise_div_round(2u8, Ceiling)
///         .to_string(),
///     "(1, 1, 2, 3)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap())
///         .entrywise_div_round(2u8, Nearest)
///         .to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("(3, 6, 255)")
///         .unwrap()
///         .entrywise_div_round(3u8, Exact)
///         .to_string(),
///     "(1, 2, 85)"
/// );
/// ```
///
/// # entrywise_div_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseDivRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_div_round_assign(2u8, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
/// ```
pub mod entrywise_div_round;
/// Right-shifting a vector (dividing it by a power of 2) and rounding every element, by
/// implementations of [`EntrywiseShrRound`](crate::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](crate::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let v = UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap();
/// assert_eq!(
///     (&v).entrywise_shr_round(1u8, Floor).to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&v).entrywise_shr_round(1u16, Ceiling).to_string(),
///     "(1, 1, 2, 3)"
/// );
/// // Ties round to even.
/// assert_eq!(
///     (&v).entrywise_shr_round(1u32, Nearest).to_string(),
///     "(0, 1, 2, 2)"
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("(4, 8, 12)")
///         .unwrap()
///         .entrywise_shr_round(2u64, Exact)
///         .to_string(),
///     "(1, 2, 3)"
/// );
/// // Shifting by at least the width rounds every element to 0 or 1.
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("(255, 128, 127)")
///         .unwrap()
///         .entrywise_shr_round(8u8, Nearest)
///         .to_string(),
///     "(1, 0, 0)"
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::from_str("(255, 1, 0)")
///         .unwrap()
///         .entrywise_shr_round(100u128, Ceiling)
///         .to_string(),
///     "(1, 1, 0)"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1u8, Nearest);
/// assert_eq!(v.to_string(), "(0, 1, 2, 2)");
///
/// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap();
/// v.entrywise_shr_round_assign(1u32, Up);
/// assert_eq!(v.to_string(), "(1, 1, 2, 3)");
/// ```
pub mod entrywise_shr_round;
/// An implementation of [`Height`](crate::num::arithmetic::traits::Height), the largest height of
/// any element.
pub mod height;
/// Implementations of [`ModAdd`](crate::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::arithmetic::traits::ModAddAssign), for adding vectors modulo a
/// number.
pub mod mod_add;
// Dot products of vectors reduced modulo a word, accumulating the sum before reducing it once.
pub(crate) mod mod_dot;
/// An implementation of [`ModIsReduced`](crate::num::arithmetic::traits::ModIsReduced), which
/// checks whether every element of a vector is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMul`](crate::num::arithmetic::traits::ModMul) and
/// [`ModMulAssign`](crate::num::arithmetic::traits::ModMulAssign), for multiplying a vector by a
/// scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModNeg`](crate::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::arithmetic::traits::ModNegAssign), for negating a vector modulo a
/// number.
pub mod mod_neg;
/// Implementations of [`Mod`](crate::num::arithmetic::traits::Mod),
/// [`ModAssign`](crate::num::arithmetic::traits::ModAssign), [`Rem`](core::ops::Rem), and
/// [`RemAssign`](core::ops::RemAssign), traits for reducing every element of a vector modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](crate::num::arithmetic::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::arithmetic::traits::ModPowerOf2Assign), which reduce every
/// element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](crate::num::arithmetic::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](crate::num::arithmetic::traits::ModPowerOf2AddAssign), for adding
/// vectors modulo a power of 2.
pub mod mod_power_of_2_add;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced), which checks
/// whether every element of a vector is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::arithmetic::traits::ModPowerOf2MulAssign), for multiplying
/// a vector by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`](crate::num::arithmetic::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::arithmetic::traits::ModPowerOf2NegAssign), for negating a
/// vector modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Shl`](crate::num::arithmetic::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](crate::num::arithmetic::traits::ModPowerOf2ShlAssign), for
/// left-shifting a vector (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
/// assert_eq!(
///     v.clone().mod_power_of_2_shl(1u8, 3).to_string(),
///     "(2, 2, 6)"
/// );
/// // Shifting by at least the power zeroes every element, keeping the dimension.
/// assert_eq!(v.mod_power_of_2_shl(3u32, 3).to_string(), "(0, 0, 0)");
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(1, 128, 1)").unwrap())
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "(2, 0, 2)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u64>::from_str("(1, 1)").unwrap())
///         .mod_power_of_2_shl(63u128, 64)
///         .to_string(),
///     "(9223372036854775808, 9223372036854775808)"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
/// v.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(v.to_string(), "(2, 2, 6)");
///
/// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
/// v.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(v.to_string(), "(0, 0, 0)");
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Sub`](crate::num::arithmetic::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::arithmetic::traits::ModPowerOf2SubAssign), for subtracting
/// vectors modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModShl`](crate::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](crate::num::arithmetic::traits::ModShlAssign), for left-shifting a vector
/// (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShl;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
/// assert_eq!(v.clone().mod_shl(1u8, 7).to_string(), "(3, 2, 6)");
/// // The modulus need not be odd, so elements can become zero, keeping the dimension.
/// assert_eq!(v.mod_shl(3u32, 8).to_string(), "(0, 0, 0)");
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(1, 2)").unwrap())
///         .mod_shl(8u64, 255)
///         .to_string(),
///     "(1, 2)"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
/// v.mod_shl_assign(1u8, 7);
/// assert_eq!(v.to_string(), "(3, 2, 6)");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSub`](crate::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](crate::num::arithmetic::traits::ModSubAssign), for subtracting vectors modulo a
/// number.
pub mod mod_sub;
/// Implementations of [`Div`](core::ops::Div) and [`DivAssign`](core::ops::DivAssign), for dividing
/// a vector by a scalar.
///
/// # div
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// assert_eq!(
///     (UnsignedVector::<u8>::from_str("()").unwrap() / 5u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (UnsignedVector::<u8>::from_str("(0, 7, 255)").unwrap() / 1u8).to_string(),
///     "(0, 7, 255)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(0, 7, 255)").unwrap() / 2u8).to_string(),
///     "(0, 3, 127)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(0, 7, 255)").unwrap() / 255u8).to_string(),
///     "(0, 0, 1)"
/// );
/// ```
///
/// # div_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(0, 7, 255)").unwrap();
/// v /= 2u8;
/// assert_eq!(v.to_string(), "(0, 3, 127)");
/// ```
pub mod scalar_div;
/// Right-shifting a vector (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// assert_eq!(
///     (UnsignedVector::<u8>::from_str("()").unwrap() >> 10u8).to_string(),
///     "()"
/// );
/// assert_eq!(
///     (UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap() >> 0u16).to_string(),
///     "(1, 2, 3, 5)"
/// );
/// assert_eq!(
///     (UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap() >> 1u32).to_string(),
///     "(0, 1, 1, 2)"
/// );
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(255, 128)").unwrap() >> 7u64).to_string(),
///     "(1, 1)"
/// );
/// // Shifting by at least the width zeroes every element, keeping the dimension.
/// assert_eq!(
///     (&UnsignedVector::<u8>::from_str("(255, 128)").unwrap() >> 8u128).to_string(),
///     "(0, 0)"
/// );
/// assert_eq!(
///     (UnsignedVector::<u64>::from_str("(18446744073709551615, 1)").unwrap() >> 63usize)
///         .to_string(),
///     "(1, 0)"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3, 5)").unwrap();
/// v >>= 1u8;
/// assert_eq!(v.to_string(), "(0, 1, 1, 2)");
///
/// let mut v = UnsignedVector::<u8>::from_str("(255, 128)").unwrap();
/// v >>= 200u32;
/// assert_eq!(v.to_string(), "(0, 0)");
/// ```
pub mod shr;
