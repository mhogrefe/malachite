// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`AbsDiff`](malachite_base::num::arithmetic::traits::AbsDiff) and
/// [`AbsDiffAssign`](malachite_base::num::arithmetic::traits::AbsDiffAssign), traits for getting
/// the absolute value of the difference between two numbers.
pub mod abs_diff;
/// Addition of [`Natural`](super::Natural)s.
pub mod add;
/// Implementations of [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign), traits for adding a
/// number and the product of two other numbers.
pub mod add_mul;
/// Implementations of
/// [`BinomialCoefficient`](malachite_base::num::arithmetic::traits::BinomialCoefficient), a trait
/// for computing the binomial coefficient of two numbers.
pub mod binomial_coefficient;
/// Implementations of [`CheckedSub`](malachite_base::num::arithmetic::traits::CheckedSub), a trait
/// for subtracting two numbers and checking whether the result is representable.
pub mod checked_sub;
/// Implementations of [`CheckedSubMul`](malachite_base::num::arithmetic::traits::CheckedSubMul), a
/// trait for subtracting the product of two numbers from another number, and checking whether the
/// result is representable.
pub mod checked_sub_mul;
/// Implementations of [`CoprimeWith`](malachite_base::num::arithmetic::traits::CoprimeWith), a
/// trait for determining whether two numbers are coprime.
pub mod coprime_with;
/// Division of [`Natural`](super::Natural)s.
pub mod div;
/// Implementations of [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign), traits for dividing
/// two numbers when it's known that the division is exact.
pub mod div_exact;
/// Implementations of raits for simultaneously finding the quotient and remainder of two numbers,
/// subject to various rounding rules.
///
/// These are the traits:
///
/// | rounding     | by value or reference           | by mutable reference (assignment)      |
/// |--------------|---------------------------------|----------------------------------------|
/// | towards $-\infty$ | [`DivMod`](malachite_base::num::arithmetic::traits::DivMod) | [`DivAssignMod`](malachite_base::num::arithmetic::traits::DivAssignMod) |
/// | towards 0         | [`DivRem`](malachite_base::num::arithmetic::traits::DivRem) | [`DivAssignRem`](malachite_base::num::arithmetic::traits::DivAssignRem) |
/// | towards $\infty$  | [`CeilingDivNegMod`](malachite_base::num::arithmetic::traits::CeilingDivNegMod) | [`CeilingDivAssignNegMod`](malachite_base::num::arithmetic::traits::CeilingDivAssignNegMod) |
///
/// [`CeilingDivNegMod`](malachite_base::num::arithmetic::traits::CeilingDivNegMod) returns a
/// remainder greater than or equal to zero. This allows the remainder to have an unsigned type, but
/// modifies the usual relation $x = qy + r$ to $x = qy - r$.
pub mod div_mod;
/// Implementations of [`DivRound`](malachite_base::num::arithmetic::traits::DivRound) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivRoundAssign), traits for dividing
/// two numbers according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
pub mod div_round;
/// Implementations of [`DivisibleBy`](malachite_base::num::arithmetic::traits::DivisibleBy), a
/// trait for determining whether one number is divisible by another.
pub mod divisible_by;
/// Implementations of
/// [`DivisibleByPowerOf2`](malachite_base::num::arithmetic::traits::DivisibleByPowerOf2), a trait
/// for determining whether a number is divisible by $2^k$.
pub mod divisible_by_power_of_2;
/// Implementations of [`EqMod`](malachite_base::num::arithmetic::traits::EqMod), a trait for
/// determining whether one number is equal by another modulo a third.
pub mod eq_mod;
/// Implementations of [`EqModPowerOf2`](malachite_base::num::arithmetic::traits::EqModPowerOf2), a
/// trait for determining whether one number is equal to another modulo $2^k$.
pub mod eq_mod_power_of_2;
/// Implementations of [`Factorial`](malachite_base::num::arithmetic::traits::Factorial),
/// [`DoubleFactorial`](malachite_base::num::arithmetic::traits::DoubleFactorial),
/// [`Multifactorial`](malachite_base::num::arithmetic::traits::Multifactorial), and
/// [`Subfactorial`](malachite_base::num::arithmetic::traits::Subfactorial).
pub mod factorial;
#[cfg(feature = "float_helpers")]
pub mod float_add;
#[cfg(feature = "float_helpers")]
pub mod float_extras;
#[cfg(feature = "float_helpers")]
pub mod float_mul;
#[cfg(feature = "float_helpers")]
pub mod float_square;
#[cfg(feature = "float_helpers")]
pub mod float_sub;
/// Implementations of [`Gcd`](malachite_base::num::arithmetic::traits::Gcd) and
/// [`GcdAssign`](malachite_base::num::arithmetic::traits::GcdAssign), traits for computing the GCD
/// (greatest common divisor) of two numbers.
pub mod gcd;
/// Implementations of [`IsPowerOf2`](malachite_base::num::arithmetic::traits::IsPowerOf2), a trait
/// for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
/// Implementations of [`LegendreSymbol`](malachite_base::num::arithmetic::traits::LegendreSymbol),
/// [`JacobiSymbol`](malachite_base::num::arithmetic::traits::JacobiSymbol), and
/// [`KroneckerSymbol`](malachite_base::num::arithmetic::traits::KroneckerSymbol), traits for
/// computing the Legendre, Jacobi, and Kronecker symbols of two numbers.
pub mod kronecker_symbol;
/// Implementations of [`Lcm`](malachite_base::num::arithmetic::traits::Lcm),
/// [`LcmAssign`](malachite_base::num::arithmetic::traits::LcmAssign), and
/// [`CheckedLcm`](malachite_base::num::arithmetic::traits::CheckedLcm), traits for computing the
/// LCM (least common multiple) of two numbers.
pub mod lcm;
/// Implementations of traits for taking the base-$b$ logarithm of a number.
///
/// The traits are [`FloorLogBase`](malachite_base::num::arithmetic::traits::FloorLogBase),
/// [`CeilingLogBase`](malachite_base::num::arithmetic::traits::CeilingLogBase), and
/// [`CheckedLogBase`](malachite_base::num::arithmetic::traits::CheckedLogBase).
pub mod log_base;
/// Implementations of traits for taking the base-2 logarithm of a number.
///
/// The traits are [`FloorLogBase2`](malachite_base::num::arithmetic::traits::FloorLogBase2),
/// [`CeilingLogBase2`](malachite_base::num::arithmetic::traits::CeilingLogBase2), and
/// [`CheckedLogBase2`](malachite_base::num::arithmetic::traits::CheckedLogBase2).
pub mod log_base_2;
/// Implementations of traits for taking the base-$2^k$ logarithm of a number.
///
/// The traits are
/// [`FloorLogBasePowerOf2`](malachite_base::num::arithmetic::traits::FloorLogBasePowerOf2),
/// [`CeilingLogBasePowerOf2`](malachite_base::num::arithmetic::traits::CeilingLogBasePowerOf2), and
/// [`CheckedLogBasePowerOf2`](malachite_base::num::arithmetic::traits::CheckedLogBasePowerOf2).
pub mod log_base_power_of_2;
/// Implementations of [`ModAdd`](malachite_base::num::arithmetic::traits::ModAdd) and
/// [`ModAddAssign`](malachite_base::num::arithmetic::traits::ModAddAssign), traits for adding two
/// numbers modulo another number.
pub mod mod_add;
/// Implementations of [`ModInverse`](malachite_base::num::arithmetic::traits::ModInverse), a trait
/// for finding the multiplicative inverse of a number modulo another number.
pub mod mod_inverse;
/// Implementations of [`ModIsReduced`](malachite_base::num::arithmetic::traits::ModIsReduced), a
/// trait for checking whether a number is reduced modulo another number.
pub mod mod_is_reduced;
/// Implementations of traits for multiplying two numbers modulo another number.
///
/// The traits are [`ModMul`](malachite_base::num::arithmetic::traits::ModMul),
/// [`ModMulAssign`](malachite_base::num::arithmetic::traits::ModMulAssign),
/// [`ModMulPrecomputed`](malachite_base::num::arithmetic::traits::ModMulPrecomputed), and
/// [`ModMulPrecomputedAssign`](malachite_base::num::arithmetic::traits::ModMulPrecomputedAssign).
/// [`ModMulPrecomputed`](malachite_base::num::arithmetic::traits::ModMulPrecomputed) and
/// [`ModMulPrecomputedAssign`](malachite_base::num::arithmetic::traits::ModMulPrecomputedAssign)
/// are useful when having to make several multiplications modulo the same modulus.
pub mod mod_mul;
/// Implementations of [`ModNeg`](malachite_base::num::arithmetic::traits::ModNeg) and
/// [`ModNegAssign`](malachite_base::num::arithmetic::traits::ModNegAssign), traits for negating a
/// number modulo another number.
pub mod mod_neg;
/// Implementations of traits for finding the remainder of two numbers, subject to various rounding
/// rules.
///
/// These are the traits:
///
/// | rounding          | by value or reference      | by mutable reference (assignment)      |
/// |-------------------|----------------------------|----------------------------------------|
/// | towards $-\infty$ | [`Mod`](malachite_base::num::arithmetic::traits::Mod)       | [`ModAssign`](malachite_base::num::arithmetic::traits::ModAssign)       |
/// | towards $\infty$  | [`NegMod`](malachite_base::num::arithmetic::traits::NegMod) | [`NegModAssign`](malachite_base::num::arithmetic::traits::NegModAssign) |
///
/// [`NegMod`](malachite_base::num::arithmetic::traits::NegMod) returns a remainder greater than or
/// equal to zero. This allows the remainder to have an unsigned type, but modifies the usual
/// relation $x = qy + r$ to $x = qy - r$.
///
/// The [`Rem`](core::ops::Rem) trait in the standard library rounds towards 0.
pub mod mod_op;
/// Implementations of traits for raising a number to a power modulo another number.
///
/// The traits are [`ModPow`](malachite_base::num::arithmetic::traits::ModPow),
/// [`ModPowAssign`](malachite_base::num::arithmetic::traits::ModPowAssign), and
/// [`ModPowPrecomputed`](malachite_base::num::arithmetic::traits::ModPowPrecomputed).
/// [`ModPowPrecomputed`](malachite_base::num::arithmetic::traits::ModPowPrecomputed) is useful when
/// having to make several exponentiations modulo the same modulus.
pub mod mod_pow;
/// Implementations of traits for finding the remainder of a number divided by $2^k$, subject to
/// various rounding rules.
///
/// These are the traits:
///
/// | rounding | by value or reference | by mutable reference (assignment) |
/// |----------|-----------------------|-----------------------------------|
/// | towards $-\infty$ | [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2) | [`ModPowerOf2Assign`](malachite_base::num::arithmetic::traits::ModPowerOf2Assign)       |
/// | towards 0 | [`RemPowerOf2`](malachite_base::num::arithmetic::traits::RemPowerOf2) | [`RemPowerOf2Assign`](malachite_base::num::arithmetic::traits::RemPowerOf2Assign)       |
/// | towards $\infty$  | [`NegModPowerOf2`](malachite_base::num::arithmetic::traits::NegModPowerOf2) | [`NegModPowerOf2Assign`](malachite_base::num::arithmetic::traits::NegModPowerOf2Assign) |
///
/// [`NegModPowerOf2`](malachite_base::num::arithmetic::traits::NegModPowerOf2) returns a remainder
/// greater than or equal to zero. This allows the remainder to have an unsigned type, but modifies
/// the usual relation $x = q2^k + r$ to $x = q2^k - r$.
pub mod mod_power_of_2;
/// Implementations of [`ModPowerOf2Add`](malachite_base::num::arithmetic::traits::ModPowerOf2Add)
/// and [`ModPowerOf2AddAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign),
/// traits for adding two numbers modulo $2^k$.
pub mod mod_power_of_2_add;
/// Implementations of
/// [`ModPowerOf2Inverse`](malachite_base::num::arithmetic::traits::ModPowerOf2Inverse), a trait for
/// finding the multiplicative inverse of a number modulo $2^k$.
pub mod mod_power_of_2_inverse;
/// Implementations of
/// [`ModPowerOf2IsReduced`](malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced), a trait
/// for checking whether a number is reduced modulo $2^k$.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](malachite_base::num::arithmetic::traits::ModPowerOf2Mul)
/// and [`ModPowerOf2MulAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign),
/// traits for multiplying two numbers modulo $2^k$.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`](malachite_base::num::arithmetic::traits::ModPowerOf2Neg)
/// and [`ModPowerOf2NegAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2NegAssign),
/// traits for negating a number modulo $2^k$.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Pow`](malachite_base::num::arithmetic::traits::ModPowerOf2Pow)
/// and [`ModPowerOf2PowAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign),
/// traits for raising a number to a power modulo $2^k$.
pub mod mod_power_of_2_pow;
/// Implementations of [`ModPowerOf2Shl`](malachite_base::num::arithmetic::traits::ModPowerOf2Shl)
/// and [`ModPowerOf2ShlAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign),
/// traits for left-shifting a number modulo $2^k$.
///
/// # mod_power_of_2_shl
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shl;
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
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShlAssign;
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
/// Implementations of [`ModPowerOf2Shr`](malachite_base::num::arithmetic::traits::ModPowerOf2Shr)
/// and [`ModPowerOf2ShrAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2ShrAssign),
/// traits for right-shifting a number modulo $2^k$.
///
/// # mod_power_of_2_shr
/// ```
/// use malachite_base::num::arithmetic::traits::ModPowerOf2Shr;
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
/// use malachite_base::num::arithmetic::traits::ModPowerOf2ShrAssign;
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
/// Implementations of
/// `ModPowerOf2Square`](malachite_base::num::arithmetic::traits::ModPowerOf2Square) and
/// [`ModPowerOf2SquareAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign),
/// traits for squaring a number modulo $2^k$.
pub mod mod_power_of_2_square;
/// Implementations of [`ModPowerOf2Sub`](malachite_base::num::arithmetic::traits::ModPowerOf2Sub)
/// and [`ModPowerOf2SubAssign`](malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign),
/// traits for subtracting one number by another modulo $2^k$.
pub mod mod_power_of_2_sub;
/// Implementations of [`ModShl`](malachite_base::num::arithmetic::traits::ModShl) and
/// [`ModShlAssign`](malachite_base::num::arithmetic::traits::ModShlAssign), traits for
/// left-shifting a number modulo another number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShl;
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
/// use malachite_base::num::arithmetic::traits::ModShlAssign;
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
/// Implementations of [`ModShr`](malachite_base::num::arithmetic::traits::ModShr) and
/// [`ModShrAssign`](malachite_base::num::arithmetic::traits::ModShrAssign), traits for
/// right-shifting a number modulo another number.
///
/// # mod_shr
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::ModShr;
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
/// use malachite_base::num::arithmetic::traits::ModShrAssign;
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
/// Implementations of traits for squaring a number modulo another number.
///
/// The traits are [`ModSquare`](malachite_base::num::arithmetic::traits::ModSquare),
/// [`ModSquareAssign`](malachite_base::num::arithmetic::traits::ModSquareAssign), and
/// [`ModSquarePrecomputed`](malachite_base::num::arithmetic::traits::ModSquarePrecomputed).
/// [`ModSquarePrecomputed`](malachite_base::num::arithmetic::traits::ModSquarePrecomputed) is
/// useful when having to make several squarings modulo the same modulus.
pub mod mod_square;
/// Implementations of [`ModSub`](malachite_base::num::arithmetic::traits::ModSub) and
/// [`ModSubAssign`](malachite_base::num::arithmetic::traits::ModSubAssign), traits for subtracting
/// two numbers modulo another number.
pub mod mod_sub;
/// Multiplication of [`Natural`](super::Natural)s.
pub mod mul;
/// Negation of a [`Natural`](super::Natural), returning an [`Integer`](crate::integer::Integer).
pub mod neg;
/// Implementations of [`NextPowerOf2`](malachite_base::num::arithmetic::traits::NextPowerOf2) and
/// [`NextPowerOf2Assign`](malachite_base::num::arithmetic::traits::NextPowerOf2Assign), traits for
/// getting the next-highest power of 2.
pub mod next_power_of_2;
/// Implementations of [`Parity`](malachite_base::num::arithmetic::traits::Parity), a trait for
/// determining whether a number is even or odd.
pub mod parity;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), traits for raising a number
/// to a power.
pub mod pow;
/// Implementations of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait for
/// computing a power of 2.
pub mod power_of_2;
/// An implementation of [`Primorial`](malachite_base::num::arithmetic::traits::Primorial), a trait
/// for computing the primorial of a number.
pub mod primorial;
/// Implementations of traits for taking the $n$th root of a number.
///
/// The traits are [`FloorRoot`](malachite_base::num::arithmetic::traits::FloorRoot),
/// [`FloorRootAssign`](malachite_base::num::arithmetic::traits::FloorRootAssign),
/// [`CeilingRoot`](malachite_base::num::arithmetic::traits::CeilingRoot),
/// [`CeilingRootAssign`](malachite_base::num::arithmetic::traits::CeilingRootAssign),
/// [`CheckedRoot`](malachite_base::num::arithmetic::traits::CheckedRoot),
/// [`RootRem`](malachite_base::num::arithmetic::traits::RootRem), and
/// [`RootAssignRem`](malachite_base::num::arithmetic::traits::RootAssignRem).
pub mod root;
/// Implementations of [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple)
/// and [`RoundToMultipleAssign`](malachite_base::num::arithmetic::traits::RoundToMultipleAssign),
/// traits for rounding a number to a multiple of another number.
pub mod round_to_multiple;
/// Implementations of
/// [`RoundToMultipleOfPowerOf2`](malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2)
/// and
/// [`RoundToMultipleOfPowerOf2Assign`](malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign),
/// traits for rounding a number to a multiple of a power of 2.
pub mod round_to_multiple_of_power_of_2;
/// Implementations of [`SaturatingSub`](malachite_base::num::arithmetic::traits::SaturatingSub) and
/// [`SaturatingSubAssign`](malachite_base::num::arithmetic::traits::SaturatingSubAssign), traits
/// for subtracting two numbers and saturating at numeric bounds instead of overflowing.
pub mod saturating_sub;
/// Implementations of
/// [`SaturatingSubMul`](malachite_base::num::arithmetic::traits::SaturatingSubMul) and
/// [`SaturatingSubMulAssign`](malachite_base::num::arithmetic::traits::SaturatingSubMulAssign),
/// traits for subtracting a number by the product of two numbers and saturating at numeric bounds
/// instead of overflowing.
pub mod saturating_sub_mul;
/// Left-shifting a [`Natural`](super::Natural) (multiplying it by a power of 2).
///
/// # shl
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::ZERO << 10u8), 0);
/// assert_eq!((Natural::from(123u32) << 2u16), 492);
/// assert_eq!(
///     (Natural::from(123u32) << 100u64).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!((&Natural::ZERO << 10u8), 0);
/// assert_eq!((&Natural::from(123u32) << 2u16), 492);
/// assert_eq!(
///     (&Natural::from(123u32) << 100u64).to_string(),
///     "155921023828072216384094494261248"
/// );
///
/// assert_eq!((Natural::ZERO << 10i8), 0);
/// assert_eq!((Natural::from(123u32) << 2i16), 492);
/// assert_eq!(
///     (Natural::from(123u32) << 100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!((Natural::ZERO << -10i64), 0);
/// assert_eq!((Natural::from(10u32).pow(12) << -10i16), 976562500);
/// assert_eq!((&Natural::ZERO << 10i8), 0);
/// assert_eq!((&Natural::from(123u32) << 2i16), 492);
/// assert_eq!(
///     (&Natural::from(123u32) << 100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!((&Natural::ZERO << -10i64), 0);
/// assert_eq!((&Natural::from(492u32) << -2i8), 123);
/// assert_eq!((&Natural::from(10u32).pow(12) << -10i16), 976562500);
/// ```
///
/// # shl_assign
/// ```
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::ONE;
/// x <<= 1u8;
/// x <<= 2u16;
/// x <<= 3u32;
/// x <<= 4u64;
/// assert_eq!(x, 1024);
///
/// let mut x = Natural::ONE;
/// x <<= 1i8;
/// x <<= 2i16;
/// x <<= 3i32;
/// x <<= 4i64;
/// assert_eq!(x, 1024);
///
/// let mut x = Natural::from(1024u32);
/// x <<= -1i8;
/// x <<= -2i16;
/// x <<= -3i32;
/// x <<= -4i64;
/// assert_eq!(x, 1);
/// ```
pub mod shl;
/// Implementations of [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound) and
/// [`ShlRoundAssign`](malachite_base::num::arithmetic::traits::ShlRoundAssign), traits for
/// multiplying a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
///
/// # shl_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShlRound;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shl_round(-8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shl_round(-8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shl_round(-9i32, Down)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shl_round(-9i64, Up)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shl_round(-9i8, Nearest)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0xffu32)
///         .shl_round(-9i16, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x100u32)
///         .shl_round(-9i32, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x100u32)
///         .shl_round(-8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
/// assert_eq!(
///     Natural::ZERO.shl_round(10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Natural::from(123u32)
///         .shl_round(2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     Natural::from(123u32)
///         .shl_round(100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
///
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shl_round(-8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shl_round(-8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shl_round(-9i32, Down)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shl_round(-9i64, Up)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shl_round(-9i8, Nearest)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0xffu32))
///         .shl_round(-9i16, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x100u32))
///         .shl_round(-9i32, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x100u32))
///         .shl_round(-8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
/// assert_eq!(
///     (&Natural::ZERO).shl_round(10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     (&Natural::from(123u32))
///         .shl_round(2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     (&Natural::from(123u32))
///         .shl_round(100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
/// ```
///
/// # shl_round_assign
/// ```
/// use core::cmp::Ordering::*;
/// use malachite_base::num::arithmetic::traits::ShlRoundAssign;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shl_round_assign(-8i8, Down), Less);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shl_round_assign(-8i16, Up), Greater);
/// assert_eq!(n, 2);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shl_round_assign(-9i32, Down), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shl_round_assign(-9i64, Up), Greater);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shl_round_assign(-9i8, Nearest), Greater);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0xffu32);
/// assert_eq!(n.shl_round_assign(-9i16, Nearest), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x100u32);
/// assert_eq!(n.shl_round_assign(-9i32, Nearest), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x100u32);
/// assert_eq!(n.shl_round_assign(-8i64, Exact), Equal);
/// assert_eq!(n, 1);
///
/// let mut x = Natural::ONE;
/// assert_eq!(x.shl_round_assign(1i8, Exact), Equal);
/// assert_eq!(x.shl_round_assign(2i16, Exact), Equal);
/// assert_eq!(x.shl_round_assign(3i32, Exact), Equal);
/// assert_eq!(x.shl_round_assign(4i64, Exact), Equal);
/// assert_eq!(x, 1024);
/// ```
pub mod shl_round;
/// Right-shifting a [`Natural`](super::Natural) (dividing it by a power of 2).
///
/// # shr
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::ZERO >> 10u8), 0);
/// assert_eq!((Natural::from(492u32) >> 2u32), 123);
/// assert_eq!((Natural::from(10u32).pow(12) >> 10u64), 976562500);
/// assert_eq!((&Natural::ZERO >> 10u8), 0);
/// assert_eq!((&Natural::from(492u32) >> 2u32), 123);
/// assert_eq!((&Natural::from(10u32).pow(12) >> 10u64), 976562500);
///
/// assert_eq!((Natural::ZERO >> 10i8), 0);
/// assert_eq!((Natural::from(492u32) >> 2i16), 123);
/// assert_eq!((Natural::from(10u32).pow(12) >> 10i32), 976562500);
/// assert_eq!((Natural::ZERO >> -10i64), 0);
/// assert_eq!((Natural::from(123u32) >> -2i8), 492);
/// assert_eq!(
///     (Natural::from(123u32) >> -100i16).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!((&Natural::ZERO >> -10i8), 0);
/// assert_eq!((&Natural::from(123u32) >> -2i16), 492);
/// assert_eq!(
///     (&Natural::from(123u32) >> -100i32).to_string(),
///     "155921023828072216384094494261248"
/// );
/// assert_eq!((&Natural::ZERO >> 10i64), 0);
/// assert_eq!((&Natural::from(492u32) >> 2i8), 123);
/// assert_eq!((&Natural::from(10u32).pow(12) >> 10i16), 976562500);
/// ```
///
/// # shr_assign
/// ```
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::Natural;
///
/// let mut x = Natural::from(1024u32);
/// x >>= 1u8;
/// x >>= 2u16;
/// x >>= 3u32;
/// x >>= 4u64;
/// assert_eq!(x, 1);
///
/// let mut x = Natural::ONE;
/// x >>= -1i8;
/// x >>= -2i16;
/// x >>= -3i32;
/// x >>= -4i64;
/// assert_eq!(x, 1024);
///
/// let mut x = Natural::from(1024u32);
/// x >>= 1i8;
/// x >>= 2i16;
/// x >>= 3i32;
/// x >>= 4i64;
/// assert_eq!(x, 1);
/// ```
pub mod shr;
/// Implementations of [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound) and
/// [`ShrRoundAssign`](malachite_base::num::arithmetic::traits::ShrRoundAssign), traits for dividing
/// a number by a power of 2 and rounding according to a specified
/// [`RoundingMode`](malachite_base::rounding_modes::RoundingMode).
///
/// # shr_round
/// ```
/// use malachite_base::num::arithmetic::traits::ShrRound;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(8u8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(8u16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(9u32, Down)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(9u64, Up)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(9u8, Nearest)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0xffu32)
///         .shr_round(9u16, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x100u32)
///         .shr_round(9u32, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x100u32)
///         .shr_round(8u64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(8u8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(8u16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(9u32, Down)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(9u64, Up)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(9u8, Nearest)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0xffu32))
///         .shr_round(9u16, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x100u32))
///         .shr_round(9u32, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x100u32))
///         .shr_round(8u64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
///
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(9i32, Down)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(9i64, Up)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0x101u32)
///         .shr_round(9i8, Nearest)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     Natural::from(0xffu32)
///         .shr_round(9i16, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x100u32)
///         .shr_round(9i32, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     Natural::from(0x100u32)
///         .shr_round(8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
/// assert_eq!(
///     Natural::ZERO.shr_round(-10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Natural::from(123u32)
///         .shr_round(-2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     Natural::from(123u32)
///         .shr_round(-100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(8i8, Down)
///         .to_debug_string(),
///     "(1, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(8i16, Up)
///         .to_debug_string(),
///     "(2, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(9i32, Down)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(9i64, Up)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0x101u32))
///         .shr_round(9i8, Nearest)
///         .to_debug_string(),
///     "(1, Greater)"
/// );
/// assert_eq!(
///     (&Natural::from(0xffu32))
///         .shr_round(9i16, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x100u32))
///         .shr_round(9i32, Nearest)
///         .to_debug_string(),
///     "(0, Less)"
/// );
/// assert_eq!(
///     (&Natural::from(0x100u32))
///         .shr_round(8i64, Exact)
///         .to_debug_string(),
///     "(1, Equal)"
/// );
/// assert_eq!(
///     (&Natural::ZERO).shr_round(-10i8, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     (&Natural::from(123u32))
///         .shr_round(-2i16, Exact)
///         .to_debug_string(),
///     "(492, Equal)"
/// );
/// assert_eq!(
///     (&Natural::from(123u32))
///         .shr_round(-100i32, Exact)
///         .to_debug_string(),
///     "(155921023828072216384094494261248, Equal)"
/// );
/// ```
///
/// # shr_round_assign
/// ```
/// use core::cmp::Ordering::*;
/// use malachite_base::num::arithmetic::traits::ShrRoundAssign;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(8u8, Down), Less);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(8u16, Up), Greater);
/// assert_eq!(n, 2);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(9u32, Down), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(9u64, Up), Greater);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(9u8, Nearest), Greater);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0xffu32);
/// assert_eq!(n.shr_round_assign(9u16, Nearest), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x100u32);
/// assert_eq!(n.shr_round_assign(9u32, Nearest), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x100u32);
/// assert_eq!(n.shr_round_assign(8u64, Exact), Equal);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(8i8, Down), Less);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(8i16, Up), Greater);
/// assert_eq!(n, 2);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(9i32, Down), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(9i64, Up), Greater);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0x101u32);
/// assert_eq!(n.shr_round_assign(9i8, Nearest), Greater);
/// assert_eq!(n, 1);
///
/// let mut n = Natural::from(0xffu32);
/// assert_eq!(n.shr_round_assign(9i16, Nearest), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x100u32);
/// assert_eq!(n.shr_round_assign(9i32, Nearest), Less);
/// assert_eq!(n, 0);
///
/// let mut n = Natural::from(0x100u32);
/// assert_eq!(n.shr_round_assign(8i64, Exact), Equal);
/// assert_eq!(n, 1);
///
/// let mut x = Natural::ONE;
/// assert_eq!(x.shr_round_assign(-1i8, Exact), Equal);
/// assert_eq!(x.shr_round_assign(-2i16, Exact), Equal);
/// assert_eq!(x.shr_round_assign(-3i32, Exact), Equal);
/// assert_eq!(x.shr_round_assign(-4i64, Exact), Equal);
/// assert_eq!(x, 1024);
/// ```
pub mod shr_round;
/// Implementations of [`Sign`](malachite_base::num::arithmetic::traits::Sign), a trait for
/// determining the sign of a number.
pub mod sign;
/// Implementations of traits for taking the square root of a number.
///
/// The traits are [`FloorSqrt`](malachite_base::num::arithmetic::traits::FloorSqrt),
/// [`FloorSqrtAssign`](malachite_base::num::arithmetic::traits::FloorSqrtAssign),
/// [`CeilingSqrt`](malachite_base::num::arithmetic::traits::CeilingSqrt),
/// [`CeilingSqrtAssign`](malachite_base::num::arithmetic::traits::CeilingSqrtAssign),
/// [`CheckedSqrt`](malachite_base::num::arithmetic::traits::CheckedSqrt),
/// [`SqrtRem`](malachite_base::num::arithmetic::traits::SqrtRem), and
/// [`SqrtAssignRem`](malachite_base::num::arithmetic::traits::SqrtAssignRem).
pub mod sqrt;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), traits for squaring a
/// number.
pub mod square;
/// Subtraction of [`Natural`](super::Natural)s.
pub mod sub;
/// Implementations of [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign), traits for subtracting
/// the product of two numbers from a number.
pub mod sub_mul;
