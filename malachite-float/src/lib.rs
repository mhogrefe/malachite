// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! This crate defines [`Float`]s, which are arbitrary-precision floating-point numbers.
//!
//! [`Float`]s are currently experimental. They are missing many important functions. However, the
//! functions that are currently implemented are thoroughly tested and documented, with the
//! exception of string conversion functions. The current string conversions are incomplete and
//! will be changed in the future to match MPFR's behavior.
//!
//! # Demos and benchmarks
//! This crate comes with a `bin` target that can be used for running demos and benchmarks.
//! - Almost all of the public functions in this crate have an associated demo. Running a demo
//!   shows you a function's behavior on a large number of inputs. TODO
//! - You can use a similar command to run benchmarks. TODO
//!
//! The list of available demos and benchmarks is not documented anywhere; you must find them by
//! browsing through
//! [`bin_util/demo_and_bench`](https://github.com/mhogrefe/malachite/tree/master/malachite-float/src/bin_util/demo_and_bench).
//!
//! # Features
//! - `32_bit_limbs`: Sets the type of [`Limb`](malachite_nz#limbs) to [`u32`] instead of the
//!   default, [`u64`].
//! - `test_build`: A large proportion of the code in this crate is only used for testing. For a
//!   typical user, building this code would result in an unnecessarily long compilation time and
//!   an unnecessarily large binary. My solution is to only build this code when the `test_build`
//!   feature is enabled. If you want to run unit tests, you must enable `test_build`. However,
//!   doctests don't require it, since they only test the public interface.
//! - `bin_build`: This feature is used to build the code for demos and benchmarks, which also
//!   takes a long time to build. Enabling this feature also enables `test_build`.

#![allow(
    unstable_name_collisions,
    clippy::assertions_on_constants,
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    clippy::range_plus_one,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::upper_case_acronyms,
    clippy::multiple_bound_locations
)]
#![warn(
    clippy::cast_lossless,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map_next,
    clippy::large_digit_groups,
    clippy::manual_filter_map,
    clippy::manual_find_map,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::uninlined_format_args,
    clippy::unused_self,
    clippy::if_not_else,
    clippy::manual_assert,
    clippy::range_plus_one,
    clippy::redundant_else,
    clippy::semicolon_if_nothing_returned,
    clippy::cloned_instead_of_copied,
    clippy::flat_map_option,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::use_self,
    clippy::trivially_copy_pass_by_ref
)]
#![cfg_attr(
    not(any(feature = "test_build", feature = "random", feature = "std")),
    no_std
)]

extern crate alloc;

#[macro_use]
extern crate malachite_base;

#[cfg(feature = "test_build")]
extern crate itertools;

#[cfg(feature = "test_build")]
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use core::ops::Deref;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SciMantissaAndExponent};
#[cfg(feature = "test_build")]
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

/// A floating-point number.
///
/// `Float`s are currently experimental. They are missing many important functions. However, the
/// functions that are currently implemented are thoroughly tested and documented, with the
/// exception of string conversion functions. The current string conversions are incomplete and will
/// be changed in the future to match MPFR's behavior.
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
///   bits. It is common to think of a `Float` as an approximation to some real number, and the
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
pub struct Float(pub(crate) InnerFloat);

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
#[derive(Clone)]
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

#[allow(clippy::type_repetition_in_bounds)]
#[doc(hidden)]
pub fn emulate_float_to_float_fn<T: PrimitiveFloat, F: Fn(Float, u64) -> (Float, Ordering)>(
    f: F,
    x: T,
) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let x = Float::from(x);
    let (mut result, o) = f(x.clone(), T::MANTISSA_WIDTH + 1);
    if !result.is_normal() {
        return T::exact_from(&result);
    }
    let e = i64::from(<&Float as SciMantissaAndExponent<Float, i32, _>>::sci_exponent(&result));
    if e < T::MIN_NORMAL_EXPONENT {
        if e < T::MIN_EXPONENT {
            let rm =
                if e == T::MIN_EXPONENT - 1 && result.significand_ref().unwrap().is_power_of_2() {
                    let down = if result > T::ZERO { Less } else { Greater };
                    if o == down { Up } else { Down }
                } else {
                    Nearest
                };
            return T::rounding_from(&result, rm).0;
        }
        result = f(x, T::max_precision_for_sci_exponent(e)).0;
    }
    if result > T::MAX_FINITE {
        T::INFINITY
    } else if result < -T::MAX_FINITE {
        T::NEGATIVE_INFINITY
    } else {
        T::exact_from(&result)
    }
}

#[allow(clippy::type_repetition_in_bounds)]
#[doc(hidden)]
pub fn emulate_float_float_to_float_fn<
    T: PrimitiveFloat,
    F: Fn(Float, Float, u64) -> (Float, Ordering),
>(
    f: F,
    x: T,
    y: T,
) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let x = Float::from(x);
    let y = Float::from(y);
    let (mut result, o) = f(x.clone(), y.clone(), T::MANTISSA_WIDTH + 1);
    if !result.is_normal() {
        return T::exact_from(&result);
    }
    let e = i64::from(<&Float as SciMantissaAndExponent<Float, i32, _>>::sci_exponent(&result));
    if e < T::MIN_NORMAL_EXPONENT {
        if e < T::MIN_EXPONENT {
            let rm =
                if e == T::MIN_EXPONENT - 1 && result.significand_ref().unwrap().is_power_of_2() {
                    let down = if result > T::ZERO { Less } else { Greater };
                    if o == down { Up } else { Down }
                } else {
                    Nearest
                };
            return T::rounding_from(&result, rm).0;
        }
        result = f(x, y, T::max_precision_for_sci_exponent(e)).0;
    }
    if result > T::MAX_FINITE {
        T::INFINITY
    } else if result < -T::MAX_FINITE {
        T::NEGATIVE_INFINITY
    } else {
        T::exact_from(&result)
    }
}

#[allow(clippy::type_repetition_in_bounds)]
#[doc(hidden)]
pub fn emulate_rational_to_float_fn<T: PrimitiveFloat, F: Fn(&Rational, u64) -> (Float, Ordering)>(
    f: F,
    x: &Rational,
) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let (mut result, o) = f(x, T::MANTISSA_WIDTH + 1);
    if !result.is_normal() {
        return T::exact_from(&result);
    }
    let e = i64::from(<&Float as SciMantissaAndExponent<Float, i32, _>>::sci_exponent(&result));
    if e < T::MIN_NORMAL_EXPONENT {
        if e < T::MIN_EXPONENT {
            let rm =
                if e == T::MIN_EXPONENT - 1 && result.significand_ref().unwrap().is_power_of_2() {
                    let down = if result > T::ZERO { Less } else { Greater };
                    if o == down { Up } else { Down }
                } else {
                    Nearest
                };
            return T::rounding_from(&result, rm).0;
        }
        result = f(x, T::max_precision_for_sci_exponent(e)).0;
    }
    if result > T::MAX_FINITE {
        T::INFINITY
    } else if result < -T::MAX_FINITE {
        T::NEGATIVE_INFINITY
    } else {
        T::exact_from(&result)
    }
}

/// Given the `(Float, Ordering)` result of an operation, determines whether an overflow occurred.
///
/// We're defining an overflow to occur whenever the actual result is outside the representable
/// finite range, and is rounded to either infinity or to the maximum or minimum representable
/// finite value. An overflow can present itself in four ways:
/// - The result is $\infty$ and the `Ordering` is `Greater`
/// - The result is $-\infty$ and the `Ordering` is `Less`
/// - The result is the largest finite value (of any `Float` with its precision) and the `Ordering`
///   is `Less`
/// - The result is the smallest (most negative) finite value (of any `Float` with its precision)
///   and the `Ordering` is `Greater`
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, One};
/// use malachite_float::{test_overflow, Float};
/// use std::cmp::Ordering::*;
///
/// assert!(test_overflow(&Float::INFINITY, Greater));
/// assert!(test_overflow(&Float::NEGATIVE_INFINITY, Less));
/// assert!(test_overflow(&Float::max_finite_value_with_prec(10), Less));
/// assert!(test_overflow(
///     &-Float::max_finite_value_with_prec(10),
///     Greater
/// ));
///
/// assert!(!test_overflow(&Float::INFINITY, Equal));
/// assert!(!test_overflow(&Float::ONE, Less));
/// ```
pub fn test_overflow(result: &Float, o: Ordering) -> bool {
    if o == Equal {
        return false;
    }
    *result == Float::INFINITY && o == Greater
        || *result == Float::NEGATIVE_INFINITY && o == Less
        || *result > 0u32 && result.abs_is_max_finite_value_with_prec() && o == Less
        || *result < 0u32 && result.abs_is_max_finite_value_with_prec() && o == Greater
}

/// Given the `(Float, Ordering)` result of an operation, determines whether an underflow occurred.
///
/// We're defining an underflow to occur whenever the actual result is outside the representable
/// finite range, and is rounded to zero, to the minimum positive value, or to the maximum negative
/// value. An underflow can present itself in four ways:
/// - The result is $0.0$ or $-0.0$ and the `Ordering` is `Less`
/// - The result is $0.0$ or $-0.0$ and the `Ordering` is `Greater`
/// - The result is the smallest positive value and the `Ordering` is `Greater`
/// - The result is the largest (least negative) negative value and the `Ordering` is `Less`
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_float::{test_underflow, Float};
/// use std::cmp::Ordering::*;
///
/// assert!(test_underflow(&Float::ZERO, Less));
/// assert!(test_underflow(&Float::ZERO, Greater));
/// assert!(test_underflow(&Float::min_positive_value_prec(10), Greater));
/// assert!(test_underflow(&-Float::min_positive_value_prec(10), Less));
///
/// assert!(!test_underflow(&Float::ZERO, Equal));
/// assert!(!test_underflow(&Float::ONE, Less));
/// ```
pub fn test_underflow(result: &Float, o: Ordering) -> bool {
    if o == Equal {
        return false;
    }
    *result == 0u32
        || *result > 0u32 && result.abs_is_min_positive_value() && o == Greater
        || *result < 0u32 && result.abs_is_min_positive_value() && o == Less
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

#[cfg(feature = "test_build")]
pub mod test_util;
