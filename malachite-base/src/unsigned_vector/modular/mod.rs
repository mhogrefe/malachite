// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`ModAbsSquared`](crate::num::modular::traits::ModAbsSquared), the squared
/// norm of a vector modulo a number.
pub mod mod_abs_squared;
/// Implementations of [`ModAdd`](crate::num::modular::traits::ModAdd) and
/// [`ModAddAssign`](crate::num::modular::traits::ModAddAssign), for adding vectors modulo a number.
pub mod mod_add;
/// Implementations of [`ModAddDot`](crate::num::modular::traits::ModAddDot) and
/// [`ModAddDotAssign`](crate::num::modular::traits::ModAddDotAssign), for adding the dot product of
/// two vectors to a number modulo a number.
pub mod mod_add_dot;
/// Implementations of [`ModAddDotReversed`](crate::num::modular::traits::ModAddDotReversed) and
/// [`ModAddDotReversedAssign`](crate::num::modular::traits::ModAddDotReversedAssign), for adding
/// the dot product of a vector and the reverse of another to a number modulo a number.
pub mod mod_add_dot_reversed;
/// Implementations of [`ModAddMul`](crate::num::modular::traits::ModAddMul) and
/// [`ModAddMulAssign`](crate::num::modular::traits::ModAddMulAssign), for adding a scalar multiple
/// of one vector to another modulo a number.
pub mod mod_add_mul;
/// Implementations of [`ModAddMulShl`](crate::num::modular::traits::ModAddMulShl) and
/// [`ModAddMulShlAssign`](crate::num::modular::traits::ModAddMulShlAssign), for adding a scalar
/// multiple of one vector, shifted left, to another modulo a number.
pub mod mod_add_mul_shl;
/// Implementations of [`ModCross`](crate::num::modular::traits::ModCross) and
/// [`ModCrossAssign`](crate::num::modular::traits::ModCrossAssign), for computing the cross product
/// of two 3-dimensional vectors modulo a number.
pub mod mod_cross;
/// Dividing every element of a vector by a scalar modulo $m$, by implementations of
/// [`ModDiv`](crate::num::modular::traits::ModDiv).
pub mod mod_div;
/// Implementations of [`ModDot`](crate::num::modular::traits::ModDot), the dot product of two
/// vectors modulo a number, accumulating the sum before reducing it once.
pub mod mod_dot;
/// Implementations of [`ModDotReversed`](crate::num::modular::traits::ModDotReversed), the dot
/// product of a vector and the reverse of another modulo a number.
pub mod mod_dot_reversed;
/// Implementations of [`ModElementProduct`](crate::num::modular::traits::ModElementProduct), the
/// product of the elements modulo a number.
pub mod mod_element_product;
/// Implementations of [`ModElementSum`](crate::num::modular::traits::ModElementSum), the sum of the
/// elements modulo a number.
pub mod mod_element_sum;
/// Inverting every element of a vector modulo $m$, by implementations of
/// [`ModEntrywiseInverse`](crate::num::modular::traits::ModEntrywiseInverse).
pub mod mod_entrywise_inverse;
/// Multiplying two vectors element by element modulo $m$, by implementations of
/// [`ModEntrywiseMul`](crate::num::modular::traits::ModEntrywiseMul) and
/// [`ModEntrywiseMulAssign`](crate::num::modular::traits::ModEntrywiseMulAssign).
pub mod mod_entrywise_mul;
/// Implementations of [`ModGeneralizedCross`](crate::num::modular::traits::ModGeneralizedCross),
/// for computing the generalized cross product of $n - 1$ vectors of dimension $n$ modulo a number.
pub mod mod_generalized_cross;
/// An implementation of [`ModIsReduced`](crate::num::modular::traits::ModIsReduced), which checks
/// whether every element of a vector is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMul`](crate::num::modular::traits::ModMul) and
/// [`ModMulAssign`](crate::num::modular::traits::ModMulAssign), for multiplying a vector by a
/// scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModNeg`](crate::num::modular::traits::ModNeg) and
/// [`ModNegAssign`](crate::num::modular::traits::ModNegAssign), for negating a vector modulo a
/// number.
pub mod mod_neg;
/// Implementations of [`Mod`](crate::num::modular::traits::Mod),
/// [`ModAssign`](crate::num::modular::traits::ModAssign), [`Rem`](core::ops::Rem), and
/// [`RemAssign`](core::ops::RemAssign), traits for reducing every element of a vector modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](crate::num::modular::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](crate::num::modular::traits::ModPowerOf2Assign), which reduce every
/// element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of
/// [`ModPowerOf2AbsSquared`](crate::num::modular::traits::ModPowerOf2AbsSquared), the squared norm
/// of a vector modulo a power of 2.
pub mod mod_power_of_2_abs_squared;
/// Implementations of [`ModPowerOf2Add`](crate::num::modular::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](crate::num::modular::traits::ModPowerOf2AddAssign), for adding vectors
/// modulo a power of 2.
pub mod mod_power_of_2_add;
/// Implementations of [`ModPowerOf2AddDot`](crate::num::modular::traits::ModPowerOf2AddDot) and
/// [`ModPowerOf2AddDotAssign`](crate::num::modular::traits::ModPowerOf2AddDotAssign), for adding
/// the dot product of two vectors to a number modulo a power of 2.
pub mod mod_power_of_2_add_dot;
/// Implementations of
/// [`ModPowerOf2AddDotReversed`](crate::num::modular::traits::ModPowerOf2AddDotReversed) and
/// [`ModPowerOf2AddDotReversedAssign`](
/// crate::num::modular::traits::ModPowerOf2AddDotReversedAssign), for adding the dot product of a
/// vector and the reverse of another to a number modulo a power of 2.
pub mod mod_power_of_2_add_dot_reversed;
/// Implementations of [`ModPowerOf2AddMul`](crate::num::modular::traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](crate::num::modular::traits::ModPowerOf2AddMulAssign), for adding a
/// scalar multiple of one vector to another modulo a power of 2.
pub mod mod_power_of_2_add_mul;
/// Implementations of [`ModPowerOf2AddMulShl`](crate::num::modular::traits::ModPowerOf2AddMulShl)
/// and [`ModPowerOf2AddMulShlAssign`](crate::num::modular::traits::ModPowerOf2AddMulShlAssign), for
/// adding a scalar multiple of one vector, shifted left, to another modulo a power of 2.
pub mod mod_power_of_2_add_mul_shl;
/// Implementations of [`ModPowerOf2Cross`](crate::num::modular::traits::ModPowerOf2Cross) and
/// [`ModPowerOf2CrossAssign`](crate::num::modular::traits::ModPowerOf2CrossAssign), for computing
/// the cross product of two 3-dimensional vectors modulo $2^k$.
pub mod mod_power_of_2_cross;
/// Dividing every element of a vector by a scalar modulo $2^k$, by implementations of
/// [`ModPowerOf2Div`](crate::num::modular::traits::ModPowerOf2Div).
pub mod mod_power_of_2_div;
/// Implementations of [`ModPowerOf2Dot`](crate::num::modular::traits::ModPowerOf2Dot), the dot
/// product of two vectors modulo a power of 2.
pub mod mod_power_of_2_dot;
/// Implementations of
/// [`ModPowerOf2DotReversed`](crate::num::modular::traits::ModPowerOf2DotReversed), the dot product
/// of a vector and the reverse of another modulo a power of 2.
pub mod mod_power_of_2_dot_reversed;
/// An implementation of
/// [`ModPowerOf2ElementProduct`](crate::num::modular::traits::ModPowerOf2ElementProduct), the
/// product of the elements modulo a power of 2.
pub mod mod_power_of_2_element_product;
/// An implementation of
/// [`ModPowerOf2ElementSum`](crate::num::modular::traits::ModPowerOf2ElementSum), the sum of the
/// elements modulo a power of 2.
pub mod mod_power_of_2_element_sum;
/// Inverting every element of a vector modulo $2^k$, by implementations of
/// [`ModPowerOf2EntrywiseInverse`](crate::num::modular::traits::ModPowerOf2EntrywiseInverse).
pub mod mod_power_of_2_entrywise_inverse;
/// Multiplying two vectors element by element modulo $2^k$, by implementations of
/// [`ModPowerOf2EntrywiseMul`](crate::num::modular::traits::ModPowerOf2EntrywiseMul) and
/// [`ModPowerOf2EntrywiseMulAssign`]( crate::num::modular::traits::ModPowerOf2EntrywiseMulAssign).
pub mod mod_power_of_2_entrywise_mul;
/// Implementations of
/// [`ModPowerOf2GeneralizedCross`](crate::num::modular::traits::ModPowerOf2GeneralizedCross), for
/// computing the generalized cross product of $n - 1$ vectors of dimension $n$ modulo $2^k$.
pub mod mod_power_of_2_generalized_cross;
/// An implementation of
/// [`ModPowerOf2IsReduced`](crate::num::modular::traits::ModPowerOf2IsReduced), which checks
/// whether every element of a vector is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](crate::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](crate::num::modular::traits::ModPowerOf2MulAssign), for multiplying a
/// vector by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`](crate::num::modular::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](crate::num::modular::traits::ModPowerOf2NegAssign), for negating a
/// vector modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Shl`](crate::num::modular::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](crate::num::modular::traits::ModPowerOf2ShlAssign), for left-shifting a
/// vector (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModPowerOf2Shl;
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
/// use malachite_base::num::modular::traits::ModPowerOf2ShlAssign;
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
/// Implementations of [`ModPowerOf2Sub`](crate::num::modular::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](crate::num::modular::traits::ModPowerOf2SubAssign), for subtracting
/// vectors modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModPowerOf2SubDot`](crate::num::modular::traits::ModPowerOf2SubDot) and
/// [`ModPowerOf2SubDotAssign`](crate::num::modular::traits::ModPowerOf2SubDotAssign), for
/// subtracting the dot product of two vectors from a number modulo a power of 2.
pub mod mod_power_of_2_sub_dot;
/// Implementations of
/// [`ModPowerOf2SubDotReversed`](crate::num::modular::traits::ModPowerOf2SubDotReversed) and
/// [`ModPowerOf2SubDotReversedAssign`](
/// crate::num::modular::traits::ModPowerOf2SubDotReversedAssign), for subtracting the dot product
/// of a vector and the reverse of another from a number modulo a power of 2.
pub mod mod_power_of_2_sub_dot_reversed;
/// Implementations of [`ModPowerOf2SubMul`](crate::num::modular::traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](crate::num::modular::traits::ModPowerOf2SubMulAssign), for
/// subtracting a scalar multiple of one vector from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul;
/// Implementations of [`ModPowerOf2SubMulShl`](crate::num::modular::traits::ModPowerOf2SubMulShl)
/// and [`ModPowerOf2SubMulShlAssign`](crate::num::modular::traits::ModPowerOf2SubMulShlAssign), for
/// subtracting a scalar multiple of one vector, shifted left, from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul_shl;
/// Implementations of [`ModPowerOf2Sum`](crate::num::modular::traits::ModPowerOf2Sum), for adding
/// up vectors entry by entry modulo a power of 2.
pub mod mod_power_of_2_sum;
/// Implementations of [`ModShl`](crate::num::modular::traits::ModShl) and
/// [`ModShlAssign`](crate::num::modular::traits::ModShlAssign), for left-shifting a vector
/// (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShl;
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
/// use malachite_base::num::modular::traits::ModShlAssign;
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
/// v.mod_shl_assign(1u8, 7);
/// assert_eq!(v.to_string(), "(3, 2, 6)");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSub`](crate::num::modular::traits::ModSub) and
/// [`ModSubAssign`](crate::num::modular::traits::ModSubAssign), for subtracting vectors modulo a
/// number.
pub mod mod_sub;
/// Implementations of [`ModSubDot`](crate::num::modular::traits::ModSubDot) and
/// [`ModSubDotAssign`](crate::num::modular::traits::ModSubDotAssign), for subtracting the dot
/// product of two vectors from a number modulo a number.
pub mod mod_sub_dot;
/// Implementations of [`ModSubDotReversed`](crate::num::modular::traits::ModSubDotReversed) and
/// [`ModSubDotReversedAssign`](crate::num::modular::traits::ModSubDotReversedAssign), for
/// subtracting the dot product of a vector and the reverse of another from a number modulo a
/// number.
pub mod mod_sub_dot_reversed;
/// Implementations of [`ModSubMul`](crate::num::modular::traits::ModSubMul) and
/// [`ModSubMulAssign`](crate::num::modular::traits::ModSubMulAssign), for subtracting a scalar
/// multiple of one vector from another modulo a number.
pub mod mod_sub_mul;
/// Implementations of [`ModSubMulShl`](crate::num::modular::traits::ModSubMulShl) and
/// [`ModSubMulShlAssign`](crate::num::modular::traits::ModSubMulShlAssign), for subtracting a
/// scalar multiple of one vector, shifted left, from another modulo a number.
pub mod mod_sub_mul_shl;
/// Implementations of [`ModSum`](crate::num::modular::traits::ModSum), for adding up vectors entry
/// by entry modulo a number.
pub mod mod_sum;
