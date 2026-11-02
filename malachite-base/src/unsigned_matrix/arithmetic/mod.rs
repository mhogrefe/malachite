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
/// crate::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which, for a matrix with
/// non-negative elements, are the same as the primitive part.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](crate::num::arithmetic::traits::Content),
/// [`PrimitivePart`](crate::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](crate::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](crate::num::arithmetic::traits::ContentAndPrimitivePart), which
/// compute the GCD of a matrix's elements and the matrix divided by it.
pub mod content;
/// Implementations of [`ContentChained`](crate::num::arithmetic::traits::ContentChained), which
/// computes the GCD of a number and a matrix's elements.
pub mod content_chained;
/// Exact division of a matrix by a scalar, by implementations of
/// [`DivExact`](crate::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](crate::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[]_{2,0}")
///         .unwrap()
///         .div_exact(5)
///         .to_string(),
///     "[]_{2,0}"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[0, 6; 255, 3]")
///         .unwrap()
///         .div_exact(1)
///         .to_string(),
///     "[0, 6; 255, 3]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[0, 6; 255, 3]").unwrap())
///         .div_exact(3)
///         .to_string(),
///     "[0, 2; 85, 1]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u64>::from_str("[18446744073709551614, 4]").unwrap())
///         .div_exact(2)
///         .to_string(),
///     "[9223372036854775807, 2]"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[0, 6; 255, 3]").unwrap();
/// a.div_exact_assign(3);
/// assert_eq!(a.to_string(), "[0, 2; 85, 1]");
/// ```
pub mod div_exact;
/// Right-shifting a matrix (dividing it by a power of 2) and rounding every element, by
/// implementations of [`EntrywiseShrRound`](crate::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](crate::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let v = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// assert_eq!(
///     (&v).entrywise_shr_round(1u8, Floor).to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (&v).entrywise_shr_round(1u16, Ceiling).to_string(),
///     "[1, 1; 2, 3]"
/// );
/// // Ties round to even.
/// assert_eq!(
///     (&v).entrywise_shr_round(1u32, Nearest).to_string(),
///     "[0, 1; 2, 2]"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[4, 8, 12]")
///         .unwrap()
///         .entrywise_shr_round(2u64, Exact)
///         .to_string(),
///     "[1, 2, 3]"
/// );
/// // Shifting by at least the width rounds every element to 0 or 1.
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[255, 128, 127]")
///         .unwrap()
///         .entrywise_shr_round(8u8, Nearest)
///         .to_string(),
///     "[1, 0, 0]"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::from_str("[255, 1, 0]")
///         .unwrap()
///         .entrywise_shr_round(100u128, Ceiling)
///         .to_string(),
///     "[1, 1, 0]"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u8, Nearest);
/// assert_eq!(a.to_string(), "[0, 1; 2, 2]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u32, Up);
/// assert_eq!(a.to_string(), "[1, 1; 2, 3]");
/// ```
pub mod entrywise_shr_round;
/// Right-shifting a matrix (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// assert_eq!(
///     (UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap() >> 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap() >> 0u16).to_string(),
///     "[1, 2; 3, 5]"
/// );
/// assert_eq!(
///     (UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap() >> 1u32).to_string(),
///     "[0, 1; 1, 2]"
/// );
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[255, 128]").unwrap() >> 7u64).to_string(),
///     "[1, 1]"
/// );
/// // Shifting by at least the width zeroes every element, keeping the shape.
/// assert_eq!(
///     (&UnsignedMatrix::<u8>::from_str("[255, 128]").unwrap() >> 8u128).to_string(),
///     "[0, 0]"
/// );
/// assert_eq!(
///     (UnsignedMatrix::<u64>::from_str("[18446744073709551615, 1]").unwrap() >> 63usize)
///         .to_string(),
///     "[1, 0]"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
/// a >>= 1u8;
/// assert_eq!(a.to_string(), "[0, 1; 1, 2]");
///
/// let mut a = UnsignedMatrix::<u8>::from_str("[255, 128]").unwrap();
/// a >>= 200u32;
/// assert_eq!(a.to_string(), "[0, 0]");
/// ```
pub mod shr;
