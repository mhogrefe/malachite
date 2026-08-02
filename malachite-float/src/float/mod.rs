// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(feature = "test_build")]
use crate::InnerFloat::Finite;
use alloc::string::String;
use core::cmp::Ordering::{self, *};
use core::ops::Deref;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
#[cfg(feature = "test_build")]
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

/// A floating-point number.
///
/// `Float`s are not yet feature-complete, but the functions that are implemented are thoroughly
/// tested and documented.
///
/// `Float`s are similar to the primitive floats defined by the IEEE 754 standard. They include NaN,
/// $\infty$ and $-\infty$, and positive and negative zero. There is only one NaN; there is no
/// concept of a NaN payload.
///
/// All the finite `Float`s are dyadic rationals (rational numbers whose denominator is a power of
/// 2). A finite `Float` consists of several fields:
/// - a sign, which denotes whether the `Float` is positive or negative;
/// - a significand, which is a [`Natural`] number whose value is equal to the `Float`'s absolute
///   value multiplied by a power of 2;
/// - an exponent, which is one more than the floor of the base-2 logarithm of the `Float`'s
///   absolute value;
/// - and finally, a precision, which is greater than zero and indicates the number of significant
///   bits. It is common to think of a `Float` as an approximation of some real number, and the
///   precision indicates how good the approximation is intended to be.
///
/// `Float`s inherit some odd behavior from the IEEE 754 standard regarding comparison. A `NaN` is
/// not equal to any `Float`, including itself. Positive and negative zero compare as equal, despite
/// being two distinct values. Additionally, (and this is not IEEE 754's fault), `Float`s with
/// different precisions compare as equal if they represent the same numeric value.
///
/// In many cases, the above behavior is unsatisfactory, so the [`ComparableFloat`] and
/// [`ComparableFloat`] wrappers are provided. See their documentation for a description of their
/// comparison behavior.
///
/// In documentation, we will use the '$=$' sign to mean that two `Float`s are identical, writing
/// things like $-\text{NaN}=\text{NaN}$ and $-(0.0) = -0.0$.
///
/// The `Float` type is designed to be very similar to the `mpfr_t` type in
/// [MPFR](https://www.mpfr.org/mpfr-current/mpfr.html#Nomenclature-and-Types), and all Malachite
/// functions produce exactly the same result as their counterparts in MPFR, unless otherwise noted.
///
/// Here are the structural difference between `Float` and `mpfr_t`:
/// - `Float` can only represent a single `NaN` value, with no sign or payload.
/// - Only finite, nonzero `Float`s have a significand, precision, and exponent. For other `Float`s,
///   these concepts are undefined. In particular, unlike `mpfr_t` zeros, `Float` zeros do not have
///   a precision.
/// - The types of `mpfr_t` components are configuration- and platform-dependent. The types of
///   `Float` components are platform-independent, although the `Limb` type is
///   configuration-dependent: it is `u64` by default, but may be changed to `u32` using the
///   `--32_bit_limbs` compiler flag. The type of the exponent is always `i32` and the type of the
///   precision is always `u64`. The `Limb` type only has a visible effect on the functions that
///   extract the raw significand. All other functions have the same interface when compiled with
///   either `Limb` type.
///
/// `Float`s whose precision is 64 bits or less can be represented without any memory allocation.
/// (Unless Malachite is compiled with `32_bit_limbs`, in which case the limit is 32).
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "SerdeFloat", into = "SerdeFloat"))]
pub struct Float(pub(crate) InnerFloat);

// A `Float` is serialized as the string `ComparableFloat`'s `Display` writes in base 16, for
// example `0x1.8#2`. Going through a string rather than the fields is what `Natural` and `Integer`
// do too, and here it also keeps the encoding independent of `Limb`'s width: the stored significand
// is padded out to a whole number of limbs, so its digits would differ between 32- and 64-bit
// builds, while the digits of the value itself do not. Reading it back parses, so a deserialized
// `Float` cannot violate the invariants that `is_valid` checks.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct SerdeFloat(String);

// We want to limit the visibility of the `NaN`, `Zero`, `Infinity`, and `Finite` constructors to
// within this crate. To do this, we wrap the `InnerFloat` enum in a struct that gets compiled away.
#[derive(Clone)]
pub(crate) enum InnerFloat {
    NaN,
    Infinity {
        sign: bool,
    },
    Zero {
        sign: bool,
    },
    Finite {
        sign: bool,
        exponent: i32,
        precision: u64,
        significand: Natural,
    },
}

#[inline]
pub(crate) fn significand_bits(significand: &Natural) -> u64 {
    significand.limb_count() << Limb::LOG_WIDTH
}

// Given the `(Float, Ordering)` pair from a computation that rounded toward negative infinity (the
// `Float` is the rounded-down value and the `Ordering` compares it to the exact result, as the
// `*_prec_round` functions return), returns the `(floor, ceiling)` pair of `Float`s bracketing the
// exact result. When the value is inexact the ceiling is the next `Float` above the floor, so it is
// obtained by incrementing the floor rather than recomputing — much cheaper when the value comes
// from, for example, a transcendental function.
pub(crate) fn floor_and_ceiling((floor, o): (Float, Ordering)) -> (Float, Float) {
    let mut ceiling = floor.clone();
    if o != Equal {
        ceiling.increment();
    }
    (floor, ceiling)
}

// `Limb::WIDTH`-derived bit counts, shared across the crate so each is written out only once.
pub(crate) const WIDTH_MINUS_1: u64 = Limb::WIDTH - 1;
pub(crate) const TWICE_WIDTH: u64 = Limb::WIDTH << 1;

impl Float {
    /// The maximum raw exponent of any [`Float`], equal to $2^{30}-1$, or $1,073,741,823$. This is
    /// one more than the maximum scientific exponent. If we write a [`Float`] as $\pm m2^e$, with
    /// $1\leq m<2$ and $e$ an integer, we must have $e\leq 2^{30}-2$. If the result of a
    /// calculation would produce a [`Float`] with an exponent larger than this, then $\pm\infty$,
    /// the maximum finite float of the specified precision, or the minimum finite float of the
    /// specified pecision is returned instead, depending on the rounding mode.
    pub const MAX_EXPONENT: i32 = 0x3fff_ffff;
    /// The minimum raw exponent of any [`Float`], equal to $-(2^{30}-1)$, or $-1,073,741,823$. This
    /// is one more than the minimum scientific exponent. If we write a [`Float`] as $\pm m2^e$,
    /// with $1\leq m<2$ and $e$ an integer, we must have $e\geq -2^{30}$. If the result of a
    /// calculation would produce a [`Float`] with an exponent smaller than this, then $\pm0.0$, the
    /// minimum positive finite [`Float`], or the maximum negative finite [`Float`] is returned
    /// instead, depending on the rounding mode.
    pub const MIN_EXPONENT: i32 = -Self::MAX_EXPONENT;
    // Exponent bounds derived from `MIN_EXPONENT`/`MAX_EXPONENT`, written out once and shared by
    // the exponent-range checks throughout the crate.
    pub(crate) const MIN_EXPONENT_MINUS_1: i32 = Self::MIN_EXPONENT - 1;
    pub(crate) const MIN_EXPONENT_MINUS_1_I64: i64 = Self::MIN_EXPONENT_MINUS_1 as i64;
    pub(crate) const MIN_EXPONENT_PLUS_2: i32 = Self::MIN_EXPONENT + 2;
    pub(crate) const MIN_EXPONENT_I64: i64 = Self::MIN_EXPONENT as i64;
    pub(crate) const MIN_EXPONENT_MINUS_2_I64: i64 = (Self::MIN_EXPONENT - 2) as i64;
    pub(crate) const MAX_EXPONENT_I64: i64 = Self::MAX_EXPONENT as i64;
    pub(crate) const MAX_EXPONENT_U64: u64 = Self::MAX_EXPONENT as u64;
    pub(crate) const MIN_EXPONENT_PLUS_1_I64: i64 = Self::MIN_EXPONENT_I64 + 1;
    pub(crate) const MIN_EXPONENT_PLUS_2_I64: i64 = Self::MIN_EXPONENT_I64 + 2;
    pub(crate) const MIN_EXPONENT_PLUS_4_I64: i64 = Self::MIN_EXPONENT_I64 + 4;
    pub(crate) const MIN_EXPONENT_PLUS_8_I64: i64 = Self::MIN_EXPONENT_I64 + 8;
    pub(crate) const MAX_EXPONENT_MINUS_2_I64: i64 = Self::MAX_EXPONENT_I64 - 2;
    // The largest precision for which the near-one fast paths are safe: any more and the
    // intermediate exponent could fall below `MIN_EXPONENT`.
    pub(crate) const NEAR_ONE_MAX_PREC: u64 = (-Self::MIN_EXPONENT_I64 - 8) as u64;

    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        match self {
            Self(Finite {
                precision,
                significand,
                exponent,
                ..
            }) => {
                if *precision == 0
                    || !significand.is_valid()
                    || *exponent > Self::MAX_EXPONENT
                    || *exponent < Self::MIN_EXPONENT
                {
                    return false;
                }
                let bits = significand.significant_bits();
                bits != 0
                    && bits.divisible_by_power_of_2(Limb::LOG_WIDTH)
                    && *precision <= bits
                    && bits - precision < Limb::WIDTH
                    && significand.divisible_by_power_of_2(bits - precision)
            }
            _ => true,
        }
    }
}

/// `ComparableFloat` is a wrapper around a [`Float`], taking the [`Float`] by value.
///
/// `CompatableFloat` has different comparison behavior than [`Float`]. See the [`Float`]
/// documentation for its comparison behavior, which is largely derived from the IEEE 754
/// specification; the `ComparableFloat` behavior, on the other hand, is more mathematically
/// well-behaved, and respects the principle that equality should be the finest equivalence
/// relation: that is, that two equal objects should not be different in any way.
///
/// To be more specific: when a [`Float`] is wrapped in a `ComparableFloat`,
/// - `NaN` is not equal to any other [`Float`], but equal to itself;
/// - Positive and negative zero are not equal to each other;
/// - Ordering is total. Negative zero is ordered to be smaller than positive zero, and `NaN` is
///   arbitrarily ordered to be between the two zeros;
/// - Two [`Float`]s with different precisions but representing the same value are unequal, and the
///   one with the greater precision is ordered to be larger;
/// - The hashing function is compatible with equality.
///
/// The analogous wrapper for primitive floats is
/// [`NiceFloat`](malachite_base::num::float::NiceFloat). However,
/// [`NiceFloat`](malachite_base::num::float::NiceFloat) also facilitates better string conversion,
/// something that isn't necessary for [`Float`]s
///
/// `ComparableFloat` owns its float. This is useful in many cases, for example if you want to use
/// [`Float`]s as keys in a hash map. In other situations, it is better to use
/// [`ComparableFloatRef`], which only has a reference to its float.
// Serialized as its inner `Float`, that is as the same hexadecimal string, since the wrapper adds
// no data of its own. That the string carries a precision is what makes the round trip preserve
// everything `ComparableFloat` compares by.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ComparableFloat(pub Float);

/// `ComparableFloatRef` is a wrapper around a [`Float`], taking the [`Float`] be reference.
///
/// See the [`ComparableFloat`] documentation for details.
#[derive(Clone)]
pub struct ComparableFloatRef<'a>(pub &'a Float);

impl ComparableFloat {
    pub const fn as_ref(&self) -> ComparableFloatRef<'_> {
        ComparableFloatRef(&self.0)
    }
}

impl Deref for ComparableFloat {
    type Target = Float;

    /// Allows a [`ComparableFloat`] to dereference to a [`Float`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// let x = ComparableFloat(Float::ONE);
    /// assert_eq!(*x, Float::ONE);
    /// ```
    fn deref(&self) -> &Float {
        &self.0
    }
}

impl Deref for ComparableFloatRef<'_> {
    type Target = Float;

    /// Allows a [`ComparableFloatRef`] to dereference to a [`Float`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// let x = Float::ONE;
    /// let y = ComparableFloatRef(&x);
    /// assert_eq!(*y, Float::ONE);
    /// ```
    fn deref(&self) -> &Float {
        self.0
    }
}

/// Traits for arithmetic.
pub mod arithmetic;
#[macro_use]
/// Basic traits for working with [`Float`]s.
pub mod basic;
/// Traits for comparing [`Float`]s for equality or order.
pub mod comparison;
/// Functions that produce [`Float`] approximations of mathematical constants, using a given
/// precision and rounding mode.
pub mod constants;
/// Traits for converting to and from [`Float`]s, including converting [`Float`]s to and from
/// strings.
pub mod conversion;
/// Iterators that generate [`Float`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Float`]s randomly.
pub mod random;
