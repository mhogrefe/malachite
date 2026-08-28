// Copyright © 2026 Mikhail Hogrefe
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
//! - The numerators and denominators of [`Rational`]s are stored as
//!   [`Natural`](malachite_nz::natural::Natural)s, so [`Rational`]s with small numerators and
//!   denominators can be stored entirely on the stack.
//! - Most arithmetic involving [`Rational`]s requires (automatically) reducing the numerator and
//!   denominator. This is done very efficiently by using the high performance GCD and exact
//!   division algorithms implemented by [`Natural`](malachite_nz::natural::Natural)s.
//!
//! # Complexity conventions
//! Functions in this crate are documented with worst-case time and additional-memory bounds,
//! following the conventions described in the `malachite-base`
//! [docs](https://docs.rs/malachite-base/latest/malachite_base/#complexity-conventions).
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
extern crate malachite_nz;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
#[macro_use]
mod macros;

#[cfg(feature = "test_build")]
extern crate itertools;
#[cfg(feature = "test_build")]
extern crate num;
#[cfg(feature = "test_build")]
extern crate rug;

/// [`Rational`], the crate's rational-number type, and everything defined on it.
/// [`GaussianRational`](gaussian_rational::GaussianRational), a type representing complex
/// numbers whose real and imaginary parts are both rational.
pub mod gaussian_rational;
pub mod rational;
pub use rational::Rational;

#[cfg(feature = "test_build")]
pub mod test_util;
