// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! This crate defines [`Rational`]s. The name of this crate refers to the mathematical symbol for
//! rational numbers, $$\mathbb{Q}$$.
//! - There are many functions defined on [`Rational`]s.
//!   These include
//!   - All the ones you'd expect, like addition, subtraction, multiplication, and division;
//!   - Functions related to conversion between [`Rational`]s and other kinds of numbers, including
//!     primitive floats;
//!   - Functions for Diophantine approximation;
//!   - Functions for expressing [`Rational`]s in scientific notation.
//! - The numerators and denominators of [`Rational`]s are stored as [`Natural`]s, so [`Rational`]s
//!   with small numerators and denominators can be stored entirely on the stack.
//! - Most arithmetic involving [`Rational`]s requires (automatically) reducing the numerator and
//!   denominator. This is done very efficiently by using the high performance GCD and exact
//!   division algorithms implemented by [`Natural`]s.
//!
//! # Demos and benchmarks
//! This crate comes with a `bin` target that can be used for running demos and benchmarks.
//! - Almost all of the public functions in this crate have an associated demo. Running a demo
//!   shows you a function's behavior on a large number of inputs. For example, to demo
//!   [`Rational`] addition, you can use the following command:
//!   ```text
//!   cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_rational_add
//!   ```
//!   This command uses the `exhaustive` mode, which generates every possible input, generally
//!   starting with the simplest input and progressing to more complex ones. Another mode is
//!   `random`. The `-l` flag specifies how many inputs should be generated.
//! - You can use a similar command to run benchmarks. The following command benchmarks various
//!   addition algorithms:
//!   ```text
//!   cargo run --features bin_build --release -- -l 1000000 -m random -b \
//!       benchmark_rational_add_algorithms -o add-bench.gp
//!   ```
//!   or addition implementations of other libraries:
//!   ```text
//!   cargo run --features bin_build --release -- -l 1000000 -m random -b \
//!       benchmark_rational_add_assign_library_comparison -o add-bench.gp
//!   ```
//!   This creates a file called gcd-bench.gp. You can use gnuplot to create an SVG from it like
//!   so:
//!   ```text
//!   gnuplot -e "set terminal svg; l \"gcd-bench.gp\"" > gcd-bench.svg
//!   ```
//!
//! The list of available demos and benchmarks is not documented anywhere; you must find them by
//! browsing through
//! [`bin_util/demo_and_bench`](https://github.com/mhogrefe/malachite/tree/master/malachite-q/src/bin_util/demo_and_bench).
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
    clippy::upper_case_acronyms
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
    clippy::trivially_copy_pass_by_ref
)]
#![cfg_attr(not(any(feature = "test_build", feature = "random")), no_std)]

extern crate alloc;

#[macro_use]
extern crate malachite_base;
extern crate malachite_nz;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "test_build")]
extern crate itertools;
#[cfg(feature = "test_build")]
extern crate num;
#[cfg(feature = "test_build")]
extern crate rug;

use malachite_base::named::Named;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::CoprimeWith;
use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

/// A rational number.
///
/// `Rational`s whose numerator and denominator have 64 significant bits or fewer can be represented
/// without any memory allocation. (Unless Malachite is compiled with `32_bit_limbs`, in which case
/// the limit is 32).
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Rational {
    // whether the `Rational` is non-negative
    #[cfg_attr(feature = "serde", serde(rename = "s"))]
    pub(crate) sign: bool,
    #[cfg_attr(feature = "serde", serde(rename = "n"))]
    pub(crate) numerator: Natural,
    #[cfg_attr(feature = "serde", serde(rename = "d"))]
    pub(crate) denominator: Natural,
}

impl Rational {
    // Returns true iff `self` is valid.
    //
    // To be valid, its denominator must be nonzero, its numerator and denominator must be
    // relatively prime, and if its numerator is zero, then `sign` must be `true`. All `Rational`s
    // must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.denominator != 0
            && (self.sign || self.numerator != 0)
            && (&self.numerator).coprime_with(&self.denominator)
    }
}

impl SignificantBits for &Rational {
    /// Returns the sum of the bits needed to represent the numerator and denominator.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.significant_bits(), 1);
    /// assert_eq!(
    ///     Rational::from_str("-100/101").unwrap().significant_bits(),
    ///     14
    /// );
    /// ```
    fn significant_bits(self) -> u64 {
        self.numerator.significant_bits() + self.denominator.significant_bits()
    }
}

/// The constant 0.
impl Zero for Rational {
    const ZERO: Rational = Rational {
        sign: true,
        numerator: Natural::ZERO,
        denominator: Natural::ONE,
    };
}

/// The constant 1.
impl One for Rational {
    const ONE: Rational = Rational {
        sign: true,
        numerator: Natural::ONE,
        denominator: Natural::ONE,
    };
}

/// The constant 2.
impl Two for Rational {
    const TWO: Rational = Rational {
        sign: true,
        numerator: Natural::TWO,
        denominator: Natural::ONE,
    };
}

/// The constant -1.
impl NegativeOne for Rational {
    const NEGATIVE_ONE: Rational = Rational {
        sign: false,
        numerator: Natural::ONE,
        denominator: Natural::ONE,
    };
}

/// The constant 1/2.
impl OneHalf for Rational {
    const ONE_HALF: Rational = Rational {
        sign: true,
        numerator: Natural::ONE,
        denominator: Natural::TWO,
    };
}

impl Default for Rational {
    /// The default value of a [`Rational`], 0.
    fn default() -> Rational {
        Rational::ZERO
    }
}

// Implements `Named` for `Rational`.
impl_named!(Rational);

/// Traits for arithmetic.
pub mod arithmetic;
/// Traits for comparing [`Rational`]s for equality or order.
pub mod comparison;
/// Traits for converting to and from [`Rational`]s, converting to and from strings, and extracting
/// digits and continued fractions.
pub mod conversion;
/// Iterators that generate [`Rational`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Rational`]s randomly.
pub mod random;

#[cfg(feature = "test_build")]
pub mod test_util;
