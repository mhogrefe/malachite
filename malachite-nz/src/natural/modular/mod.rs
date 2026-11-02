// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(doc)]
use malachite_base::num::modular::traits::{
    BalancedMod, BalancedModPowerOf2, Crt, DivisibleBy, DivisibleByPowerOf2, EqMod, EqModPowerOf2,
    Mod, ModAbsSquared, ModAbsSquaredAssign, ModAdd, ModAddAssign, ModAddMul, ModAddMulAssign,
    ModAddMulShl, ModAddMulShlAssign, ModAssign, ModDiv, ModDivList, ModEuclidean,
    ModEuclideanAssign, ModInverse, ModIsReduced, ModMul, ModMulAssign, ModMulPrecomputed,
    ModMulPrecomputedAssign, ModNeg, ModNegAssign, ModPow, ModPowAssign, ModPowPrecomputed,
    ModPowerOf2, ModPowerOf2AbsSquared, ModPowerOf2AbsSquaredAssign, ModPowerOf2Add,
    ModPowerOf2AddAssign, ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2AddMulShl,
    ModPowerOf2AddMulShlAssign, ModPowerOf2Assign, ModPowerOf2Div, ModPowerOf2Inverse,
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Neg,
    ModPowerOf2NegAssign, ModPowerOf2Pow, ModPowerOf2PowAssign, ModPowerOf2Product, ModPowerOf2Shl,
    ModPowerOf2ShlAssign, ModPowerOf2Shr, ModPowerOf2ShrAssign, ModPowerOf2Sqrt, ModPowerOf2Square,
    ModPowerOf2SquareAssign, ModPowerOf2Sub, ModPowerOf2SubAssign, ModPowerOf2SubMul,
    ModPowerOf2SubMulAssign, ModPowerOf2SubMulShl, ModPowerOf2SubMulShlAssign, ModPowerOf2Sum,
    ModProduct, ModShl, ModShlAssign, ModShr, ModShrAssign, ModSqrt, ModSquare, ModSquareAssign,
    ModSquarePrecomputed, ModSub, ModSubAssign, ModSubMul, ModSubMulAssign, ModSubMulShl,
    ModSubMulShlAssign, ModSum, NegMod, NegModAssign, NegModPowerOf2, NegModPowerOf2Assign,
    RemPowerOf2, RemPowerOf2Assign,
};

/// [`BalancedMod`], a trait for finding the representative of a number modulo another number that
/// is closest to zero.
pub mod balanced_mod;
/// [`BalancedModPowerOf2`], a trait for finding the representative of a number modulo $2^k$ that is
/// closest to zero.
pub mod balanced_mod_power_of_2;
/// Implementations of [`Crt`], a trait for combining two congruences by the Chinese remainder
/// theorem.
pub mod crt;
crate_test_mod! {
    /// `CrtComb`, a precomputed context for reducing a number modulo many word-sized moduli at once
    /// and combining word residues back, by the Chinese remainder theorem. Internal machinery for
    /// future multimodular algorithms, public only under `test_build`.
    crt_comb
}
/// Implementations of [`DivisibleBy`], a trait for determining whether one number is divisible by
/// another.
pub mod divisible_by;
/// Implementations of [`DivisibleByPowerOf2`], a trait for determining whether a number is
/// divisible by $2^k$.
pub mod divisible_by_power_of_2;
/// Implementations of [`EqMod`], a trait for determining whether one number is equal by another
/// modulo a third.
pub mod eq_mod;
/// Implementations of [`EqModPowerOf2`], a trait for determining whether one number is equal to
/// another modulo $2^k$.
pub mod eq_mod_power_of_2;
/// Implementations of [`ModAbsSquared`] and [`ModAbsSquaredAssign`], traits for computing the
/// squared absolute value of a number modulo another number.
pub mod mod_abs_squared;
/// Implementations of [`ModAdd`] and [`ModAddAssign`], traits for adding two numbers modulo another
/// number.
pub mod mod_add;
/// Implementations of [`ModAddMul`] and [`ModAddMulAssign`], traits for adding a number and the
/// product of two other numbers modulo another number.
pub mod mod_add_mul;
/// Implementations of [`ModAddMulShl`] and [`ModAddMulShlAssign`], traits for adding a number and
/// the product of two other numbers, shifted left, modulo another number.
pub mod mod_add_mul_shl;
/// Implementations of [`ModDiv`], a trait for dividing two numbers modulo another number.
pub mod mod_div;
/// Implementations of [`ModDivList`], a trait for finding all quotients of two numbers modulo
/// another number.
pub mod mod_div_list;
/// Implementations of [`ModEuclidean`] and [`ModEuclideanAssign`], traits for finding the remainder
/// of two numbers, where the remainder is nonnegative.
pub mod mod_euclidean;
/// Implementations of [`ModInverse`], a trait for finding the multiplicative inverse of a number
/// modulo another number.
pub mod mod_inverse;
/// Implementations of [`ModIsReduced`], a trait for checking whether a number is reduced modulo
/// another number.
pub mod mod_is_reduced;
/// Implementations of traits for multiplying two numbers modulo another number.
///
/// The traits are [`ModMul`], [`ModMulAssign`], [`ModMulPrecomputed`], and
/// [`ModMulPrecomputedAssign`]. [`ModMulPrecomputed`] and [`ModMulPrecomputedAssign`] are useful
/// when having to make several multiplications modulo the same modulus.
pub mod mod_mul;
/// Implementations of [`ModNeg`] and [`ModNegAssign`], traits for negating a number modulo another
/// number.
pub mod mod_neg;
/// Implementations of traits for finding the remainder of two numbers, subject to various rounding
/// rules.
///
/// These are the traits:
///
/// | rounding          | by value or reference      | by mutable reference (assignment)      |
/// |-------------------|----------------------------|----------------------------------------|
/// | towards $-\infty$ | [`Mod`]       | [`ModAssign`]       |
/// | towards $\infty$  | [`NegMod`] | [`NegModAssign`] |
///
/// [`NegMod`] returns a remainder greater than or equal to zero. This allows the remainder to have
/// an unsigned type, but modifies the usual relation $x = qy + r$ to $x = qy - r$.
///
/// The [`Rem`](core::ops::Rem) trait in the standard library rounds towards 0.
pub mod mod_op;
/// Implementations of traits for raising a number to a power modulo another number.
///
/// The traits are [`ModPow`], [`ModPowAssign`], and [`ModPowPrecomputed`]. [`ModPowPrecomputed`] is
/// useful when having to make several exponentiations modulo the same modulus.
pub mod mod_pow;
/// Implementations of traits for finding the remainder of a number divided by $2^k$, subject to
/// various rounding rules.
///
/// These are the traits:
///
/// | rounding | by value or reference | by mutable reference (assignment) |
/// |----------|-----------------------|-----------------------------------|
/// | towards $-\infty$ | [`ModPowerOf2`] | [`ModPowerOf2Assign`]       |
/// | towards 0 | [`RemPowerOf2`] | [`RemPowerOf2Assign`]       |
/// | towards $\infty$  | [`NegModPowerOf2`] | [`NegModPowerOf2Assign`] |
///
/// [`NegModPowerOf2`] returns a remainder greater than or equal to zero. This allows the remainder
/// to have an unsigned type, but modifies the usual relation $x = q2^k + r$ to $x = q2^k - r$.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2AbsSquared`] and [`ModPowerOf2AbsSquaredAssign`], traits for
/// computing the squared absolute value of a number modulo $2^k$.
pub mod mod_power_of_2_abs_squared;
/// Implementations of [`ModPowerOf2Add`] and [`ModPowerOf2AddAssign`], traits for adding two
/// numbers modulo $2^k$.
pub mod mod_power_of_2_add;
/// Implementations of [`ModPowerOf2AddMul`] and [`ModPowerOf2AddMulAssign`], traits for adding a
/// number and the product of two other numbers modulo $2^k$.
pub mod mod_power_of_2_add_mul;
/// Implementations of [`ModPowerOf2AddMulShl`] and [`ModPowerOf2AddMulShlAssign`], traits for
/// adding a number and the product of two other numbers, shifted left, modulo $2^k$.
pub mod mod_power_of_2_add_mul_shl;
/// Implementations of [`ModPowerOf2Div`], a trait for dividing two numbers modulo $2^k$.
pub mod mod_power_of_2_div;
/// Implementations of [`ModPowerOf2Inverse`], a trait for finding the multiplicative inverse of a
/// number modulo $2^k$.
pub mod mod_power_of_2_inverse;
/// Implementations of [`ModPowerOf2IsReduced`], a trait for checking whether a number is reduced
/// modulo $2^k$.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`] and [`ModPowerOf2MulAssign`], traits for multiplying two
/// numbers modulo $2^k$.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`] and [`ModPowerOf2NegAssign`], traits for negating a number
/// modulo $2^k$.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Pow`] and [`ModPowerOf2PowAssign`], traits for raising a number
/// to a power modulo $2^k$.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2Product`], a trait for multiplying together the numbers in an
/// iterator modulo $2^k$.
pub mod mod_power_of_2_product;
/// Implementations of [`ModPowerOf2Shl`] and [`ModPowerOf2ShlAssign`], traits for left-shifting a
/// number modulo $2^k$.
///
/// # mod_power_of_2_shl
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Shl;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shl(5u16, 8), 96);
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shl(100u64, 80), 0);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shl(5u16, 8), 96);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shl(100u64, 80), 0);
///
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shl(5i16, 8), 96);
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shl(100i64, 80), 0);
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shl(-2i8, 8), 30);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shl(5i16, 8), 96);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shl(100i64, 80), 0);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shl(-2i8, 8), 30);
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2ShlAssign;
/// use malachite_nz::natural::Natural;
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shl_assign(5u16, 8);
/// assert_eq!(n, 96);
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shl_assign(100u64, 80);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shl_assign(5i16, 8);
/// assert_eq!(n, 96);
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shl_assign(100i64, 80);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shl_assign(-2i8, 8);
/// assert_eq!(n, 30);
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Shr`] and [`ModPowerOf2ShrAssign`], traits for right-shifting a
/// number modulo $2^k$.
///
/// # mod_power_of_2_shr
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2Shr;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shr(-5i16, 8), 96);
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shr(-100i64, 80), 0);
/// assert_eq!(Natural::from(123u32).mod_power_of_2_shr(2i8, 8), 30);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shr(-5i16, 8), 96);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shr(-100i64, 80), 0);
/// assert_eq!((&Natural::from(123u32)).mod_power_of_2_shr(2i8, 8), 30);
/// ```
///
/// # mod_power_of_2_shr_assign
/// ```
/// use malachite_base::num::modular::traits::ModPowerOf2ShrAssign;
/// use malachite_nz::natural::Natural;
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shr_assign(-5i16, 8);
/// assert_eq!(n, 96);
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shr_assign(-100i64, 80);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(123u32);
/// n.mod_power_of_2_shr_assign(2i8, 8);
/// assert_eq!(n, 30);
/// ```
pub mod mod_power_of_2_shr;
/// An implementation of [`ModPowerOf2Sqrt`], a trait for finding the least square root of a number
/// modulo $2^k$.
///
/// # mod_power_of_2_sqrt
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::modular::traits::ModPowerOf2Sqrt;
/// use malachite_nz::natural::Natural;
///
/// // The roots of 17 modulo 32 are 7, 9, 23, and 25.
/// assert_eq!(
///     Natural::from(17u32).mod_power_of_2_sqrt(5),
///     Some(Natural::from(7u32))
/// );
/// assert_eq!(
///     (&Natural::from(17u32)).mod_power_of_2_sqrt(64),
///     Some(Natural::from(405959429219100393u64))
/// );
/// assert_eq!(
///     Natural::from(17u32 << 20).mod_power_of_2_sqrt(64),
///     Some(Natural::from(2195515552539648u64))
/// );
/// // 3 is not 1 mod 8, and 8 = 2 ^ 3 has an odd power of 2.
/// assert_eq!(Natural::from(3u32).mod_power_of_2_sqrt(4), None);
/// assert_eq!(Natural::from(8u32).mod_power_of_2_sqrt(4), None);
/// assert_eq!(Natural::ZERO.mod_power_of_2_sqrt(10), Some(Natural::ZERO));
/// ```
pub mod mod_power_of_2_sqrt;
/// Implementations of `ModPowerOf2Square`](ModPowerOf2Square) and [`ModPowerOf2SquareAssign`],
/// traits for squaring a number modulo $2^k$.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2Sub`] and [`ModPowerOf2SubAssign`], traits for subtracting one
/// number by another modulo $2^k$.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModPowerOf2SubMul`] and [`ModPowerOf2SubMulAssign`], traits for subtracting
/// the product of two numbers from a number modulo $2^k$.
pub mod mod_power_of_2_sub_mul;
/// Implementations of [`ModPowerOf2SubMulShl`] and [`ModPowerOf2SubMulShlAssign`], traits for
/// subtracting the product of two numbers, shifted left, from a number modulo $2^k$.
pub mod mod_power_of_2_sub_mul_shl;
/// Implementations of [`ModPowerOf2Sum`], a trait for adding up the numbers in an iterator modulo
/// $2^k$.
pub mod mod_power_of_2_sum;
/// Implementations of [`ModProduct`], a trait for multiplying together the numbers in an iterator
/// modulo another number.
pub mod mod_product;
/// Implementations of [`ModShl`] and [`ModShlAssign`], traits for left-shifting a number modulo
/// another number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShl;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(8u32).mod_shl(2u16, Natural::from(10u32)), 2);
/// assert_eq!(
///     Natural::from(123456u32).mod_shl(100u64, Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(Natural::from(8u32).mod_shl(2u16, &Natural::from(10u32)), 2);
/// assert_eq!(
///     Natural::from(123456u32).mod_shl(100u64, &Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(
///     (&Natural::from(8u32)).mod_shl(2u16, Natural::from(10u32)),
///     2
/// );
/// assert_eq!(
///     (&Natural::from(123456u32))
///         .mod_shl(100u64, Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(
///     (&Natural::from(8u32)).mod_shl(2u16, &Natural::from(10u32)),
///     2
/// );
/// assert_eq!(
///     (&Natural::from(123456u32))
///         .mod_shl(100u64, &Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
///
/// assert_eq!(Natural::from(8u32).mod_shl(2i8, Natural::from(10u32)), 2);
/// assert_eq!(
///     Natural::from(5u32).mod_shl(-100i32, Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     Natural::from(123456u32).mod_shl(100i64, Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(Natural::from(8u32).mod_shl(2i8, &Natural::from(10u32)), 2);
/// assert_eq!(
///     Natural::from(5u32).mod_shl(-100i32, &Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     Natural::from(123456u32).mod_shl(100i64, &Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!((&Natural::from(8u32)).mod_shl(2i8, Natural::from(10u32)), 2);
/// assert_eq!(
///     (&Natural::from(5u32)).mod_shl(-100i32, Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     (&Natural::from(123456u32))
///         .mod_shl(100i64, Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(
///     (&Natural::from(8u32)).mod_shl(2i8, &Natural::from(10u32)),
///     2
/// );
/// assert_eq!(
///     (&Natural::from(5u32)).mod_shl(-100i32, &Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     (&Natural::from(123456u32))
///         .mod_shl(100i64, &Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShlAssign;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(8u32);
/// x.mod_shl_assign(2u16, Natural::from(10u32));
/// assert_eq!(x, 2);
///
/// let mut x = Natural::from(123456u32);
/// x.mod_shl_assign(100u64, Natural::from_str("12345678987654321").unwrap());
/// assert_eq!(x, 7436663564915145u64);
///
/// let mut x = Natural::from(8u32);
/// x.mod_shl_assign(2u16, &Natural::from(10u32));
/// assert_eq!(x, 2);
///
/// let mut x = Natural::from(123456u32);
/// x.mod_shl_assign(100u64, &Natural::from_str("12345678987654321").unwrap());
/// assert_eq!(x, 7436663564915145u64);
///
/// let mut x = Natural::from(8u32);
/// x.mod_shl_assign(2i8, Natural::from(10u32));
/// assert_eq!(x, 2);
///
/// let mut x = Natural::from(5u32);
/// x.mod_shl_assign(-100i32, Natural::from(10u32));
/// assert_eq!(x, 0);
///
/// let mut x = Natural::from(123456u32);
/// x.mod_shl_assign(100i64, Natural::from_str("12345678987654321").unwrap());
/// assert_eq!(x, 7436663564915145u64);
///
/// let mut x = Natural::from(8u32);
/// x.mod_shl_assign(2i8, &Natural::from(10u32));
/// assert_eq!(x, 2);
///
/// let mut x = Natural::from(5u32);
/// x.mod_shl_assign(-100i32, &Natural::from(10u32));
/// assert_eq!(x, 0);
///
/// let mut x = Natural::from(123456u32);
/// x.mod_shl_assign(100i64, &Natural::from_str("12345678987654321").unwrap());
/// assert_eq!(x, 7436663564915145u64);
/// ```
pub mod mod_shl;
/// Implementations of [`ModShr`] and [`ModShrAssign`], traits for right-shifting a number modulo
/// another number.
///
/// # mod_shr
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShr;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(8u32).mod_shr(-2i8, Natural::from(10u32)), 2);
/// assert_eq!(Natural::from(5u32).mod_shr(100i32, Natural::from(10u32)), 0);
/// assert_eq!(
///     Natural::from(123456u32).mod_shr(-100i64, Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(Natural::from(8u32).mod_shr(-2i8, &Natural::from(10u32)), 2);
/// assert_eq!(
///     Natural::from(5u32).mod_shr(100i32, &Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     Natural::from(123456u32).mod_shr(-100i64, &Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(
///     (&Natural::from(8u32)).mod_shr(-2i8, Natural::from(10u32)),
///     2
/// );
/// assert_eq!(
///     (&Natural::from(5u32)).mod_shr(100i32, Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     (&Natural::from(123456u32))
///         .mod_shr(-100i64, Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// assert_eq!(
///     (&Natural::from(8u32)).mod_shr(-2i8, &Natural::from(10u32)),
///     2
/// );
/// assert_eq!(
///     (&Natural::from(5u32)).mod_shr(100i32, &Natural::from(10u32)),
///     0
/// );
/// assert_eq!(
///     (&Natural::from(123456u32))
///         .mod_shr(-100i64, &Natural::from_str("12345678987654321").unwrap()),
///     7436663564915145u64
/// );
/// ```
///
/// # mod_shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::modular::traits::ModShrAssign;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(8u32);
/// x.mod_shr_assign(-2i8, Natural::from(10u32));
/// assert_eq!(x, 2);
///
/// let mut x = Natural::from(5u32);
/// x.mod_shr_assign(100i32, Natural::from(10u32));
/// assert_eq!(x, 0);
///
/// let mut x = Natural::from(123456u32);
/// x.mod_shr_assign(-100i64, Natural::from_str("12345678987654321").unwrap());
/// assert_eq!(x, 7436663564915145u64);
///
/// let mut x = Natural::from(8u32);
/// x.mod_shr_assign(-2i8, &Natural::from(10u32));
/// assert_eq!(x, 2);
///
/// let mut x = Natural::from(5u32);
/// x.mod_shr_assign(100i32, &Natural::from(10u32));
/// assert_eq!(x, 0);
///
/// let mut x = Natural::from(123456u32);
/// x.mod_shr_assign(-100i64, &Natural::from_str("12345678987654321").unwrap());
/// assert_eq!(x, 7436663564915145u64);
/// ```
pub mod mod_shr;
/// Implementations of [`ModSqrt`], a trait for computing a square root of a number modulo another
/// number.
pub mod mod_sqrt;
/// Implementations of traits for squaring a number modulo another number.
///
/// The traits are [`ModSquare`], [`ModSquareAssign`], and [`ModSquarePrecomputed`].
/// [`ModSquarePrecomputed`] is useful when having to make several squarings modulo the same
/// modulus.
pub mod mod_square;
/// Implementations of [`ModSub`] and [`ModSubAssign`], traits for subtracting two numbers modulo
/// another number.
pub mod mod_sub;
/// Implementations of [`ModSubMul`] and [`ModSubMulAssign`], traits for subtracting the product of
/// two numbers from a number modulo another number.
pub mod mod_sub_mul;
/// Implementations of [`ModSubMulShl`] and [`ModSubMulShlAssign`], traits for subtracting the
/// product of two numbers, shifted left, from a number modulo another number.
pub mod mod_sub_mul_shl;
/// Implementations of [`ModSum`], a trait for adding up the numbers in an iterator modulo another
/// number.
pub mod mod_sum;
crate_test_mod! {
    /// `MultiCrt`, a precomputed context for combining many congruences by the Chinese remainder
    /// theorem, and the home of the public one-shot
    /// [`Natural::multi_crt`](super::Natural::multi_crt). The context itself is internal machinery
    /// for future multimodular algorithms, public only under `test_build`.
    multi_crt
}
/// The kernel of rational reconstruction, adopted from FLINT, used by malachite-q's
/// `Rational::reconstruct` and by the integer matrix solvers.
#[doc(hidden)]
pub mod reconstruct;
