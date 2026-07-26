// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! This crate defines [`Float`]s, which are arbitrary-precision floating-point numbers.
//!
//! [`Float`]s are not yet feature-complete, but the functions that are implemented are thoroughly
//! tested and documented.
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

#![forbid(unsafe_code)]
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
    clippy::comparison_chain,
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

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SciMantissaAndExponent};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_q::Rational;

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

#[allow(clippy::type_repetition_in_bounds)]
#[doc(hidden)]
pub fn emulate_rational_rational_to_float_fn<
    T: PrimitiveFloat,
    F: Fn(&Rational, &Rational, u64) -> (Float, Ordering),
>(
    f: F,
    x: &Rational,
    y: &Rational,
) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    let (mut result, o) = f(x, y, T::MANTISSA_WIDTH + 1);
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

/// [`Float`], the crate's floating-point type, and everything defined on it.
#[macro_use]
pub mod float;
pub use float::{ComparableFloat, ComparableFloatRef, Float};
pub(crate) use float::{
    InnerFloat, TWICE_WIDTH, WIDTH_MINUS_1, floor_and_ceiling, significand_bits,
};

#[cfg(feature = "test_build")]
pub mod test_util;
