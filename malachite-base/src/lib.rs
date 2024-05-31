// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! This crate contains many utilities that are used by the
//! [`malachite-nz`](https://docs.rs/malachite-nz/latest/malachite_nz/) and
//! [`malachite-q`]((https://docs.rs/malachite-q/latest/malachite_q/)) crates. These utilities
//! include
//! - Traits that wrap functions from the standard library, like
//!   [`CheckedAdd`](num::arithmetic::traits::CheckedAdd).
//! - Traits that give extra functionality to primitive types, like
//!   [`Gcd`](num::arithmetic::traits::Gcd), [`FloorSqrt`](num::arithmetic::traits::FloorSqrt), and
//!   [`BitAccess`](num::logic::traits::BitAccess).
//! - Iterator-producing functions that let you generate values for testing. Here's an example of
//!   an iterator that produces all pairs of [`u32`]s:
//!   ```
//!   use malachite_base::num::exhaustive::exhaustive_unsigneds;
//!   use malachite_base::tuples::exhaustive::exhaustive_pairs_from_single;
//!
//!   let mut pairs = exhaustive_pairs_from_single(exhaustive_unsigneds::<u32>());
//!   assert_eq!(
//!       pairs.take(20).collect::<Vec<_>>(),
//!       &[
//!           (0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1),
//!           (3, 0), (3, 1), (2, 2), (2, 3), (3, 2), (3, 3), (0, 4), (0, 5), (1, 4), (1, 5)
//!       ]
//!   );
//!   ```
//! - The [`RoundingMode`](rounding_modes::RoundingMode) enum, which allows you to specify the
//!   rounding behavior of various functions.
//! - The [`NiceFloat`](num::float::NiceFloat) wrapper, which provides alternative implementations
//!   of [`Eq`], [`Ord`], and [`Display`](std::fmt::Display) for floating-point values which are in
//!   some ways nicer than the defaults.
//!
//! # Demos and benchmarks
//! This crate comes with a `bin` target that can be used for running demos and benchmarks.
//! - Almost all of the public functions in this crate have an associated demo. Running a demo
//!   shows you a function's behavior on a large number of inputs. For example, to demo the
//!   [`mod_pow`](num::arithmetic::traits::ModPow::mod_pow) function on [`u32`]s, you can use the
//!   following command:
//!   ```text
//!   cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_mod_pow_u32
//!   ```
//!   This command uses the `exhaustive` mode, which generates every possible input, generally
//!   starting with the simplest input and progressing to more complex ones. Another mode is
//!   `random`. The `-l` flag specifies how many inputs should be generated.
//! - You can use a similar command to run benchmarks. The following command benchmarks various
//!   GCD algorithms for [`u64`]s:
//!   ```text
//!   cargo run --features bin_build --release -- -l 1000000 -m random -b \
//!       benchmark_gcd_algorithms_u64 -o gcd-bench.gp
//!   ```
//!   This creates a file called gcd-bench.gp. You can use gnuplot to create an SVG from it like
//!   so:
//!   ```text
//!   gnuplot -e "set terminal svg; l \"gcd-bench.gp\"" > gcd-bench.svg
//!   ```
//!
//! The list of available demos and benchmarks is not documented anywhere; you must find them by
//! browsing through
//! [`bin_util/demo_and_bench`](https://github.com/mhogrefe/malachite/tree/master/malachite-base/src/bin_util/demo_and_bench).
//!
//! # Features
//! - `test_build`: A large proportion of the code in this crate is only used for testing. For a
//!   typical user, building this code would result in an unnecessarily long compilation time and
//!   an unnecessarily large binary. Much of it is also used for testing
//!   [`malachite-nz`](https://docs.rs/malachite-nz/latest/malachite_nz/) and
//!   [`malachite-q`](https://docs.rs/malachite-q/latest/malachite_q/), so it can't just be
//!   confined to the `tests` directory. My solution is to only build this code when the
//!   `test_build` feature is enabled. If you want to run unit tests, you must enable `test_build`.
//!   However, doctests don't require it, since they only test the public interface.
//! - `bin_build`: This feature is used to build the code for demos and benchmarks, which also
//!   takes a long time to build. Enabling this feature also enables `test_build`.

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
#![allow(
    clippy::cognitive_complexity,
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::upper_case_acronyms,
    unstable_name_collisions
)]
#![cfg_attr(not(any(feature = "test_build", feature = "random")), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "test_build")]
#[doc(hidden)]
#[inline]
pub fn fail_on_untested_path(message: &str) {
    panic!("Untested path. {message}");
}

#[cfg(not(feature = "test_build"))]
#[doc(hidden)]
#[inline]
pub const fn fail_on_untested_path(_message: &str) {}

// TODO links for malachite-nz and malachite-q

/// The [`Named`](named::Named) trait, for getting a type's name.
#[macro_use]
pub mod named;

#[doc(hidden)]
#[macro_use]
pub mod macros;

/// Functions for working with [`bool`]s.
#[macro_use]
pub mod bools;
/// Functions for working with [`char`]s.
#[macro_use]
pub mod chars;
/// Macros and traits related to comparing values.
pub mod comparison;
/// Functions and adaptors for iterators.
pub mod iterators;
/// [`Never`](nevers::Never), a type that cannot be instantiated.
pub mod nevers;
/// Functions for working with primitive integers and floats.
#[macro_use]
pub mod num;
/// Functions for working with [`Ordering`](std::cmp::Ordering)s.
pub mod options;
/// Functions for working with [`Option`]s.
pub mod orderings;
#[cfg(feature = "random")]
/// Functions for generating random values.
pub mod random;
/// [`RationalSequence`](rational_sequences::RationalSequence), a type representing a sequence that
/// is finite or eventually repeating, just like the digits of a rational number.
pub mod rational_sequences;
/// [`RoundingMode`](rounding_modes::RoundingMode), an enum used to specify rounding behavior.
pub mod rounding_modes;
/// Functions for working with [`HashSet`](std::collections::HashSet)s and
/// [`BTreeSet`](std::collections::BTreeSet)s.
pub mod sets;
/// Functions for working with slices.
#[macro_use]
pub mod slices;
/// Functions for working with [`String`]s.
pub mod strings;
/// Functions for working with tuples.
pub mod tuples;
/// Unions (sum types). These are essentially generic enums.
///
/// # unwrap
/// ```
/// use malachite_base::union_struct;
/// use malachite_base::unions::UnionFromStrError;
/// use std::fmt::{self, Display, Formatter};
/// use std::str::FromStr;
///
/// union_struct!(
///     (pub(crate)),
///     Union3,
///     Union3<T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c]
/// );
///
/// let mut u: Union3<char, char, char>;
///
/// u = Union3::A('a');
/// assert_eq!(u.unwrap(), 'a');
///
/// u = Union3::B('b');
/// assert_eq!(u.unwrap(), 'b');
///
/// u = Union3::C('c');
/// assert_eq!(u.unwrap(), 'c');
/// ```
///
/// # fmt
/// ```
/// use malachite_base::union_struct;
/// use malachite_base::unions::UnionFromStrError;
/// use std::fmt::{self, Display, Formatter};
/// use std::str::FromStr;
///
/// union_struct!(
///     (pub(crate)),
///     Union3,
///     Union3<T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c]
/// );
///
/// let mut u: Union3<char, u32, bool>;
///
/// u = Union3::A('a');
/// assert_eq!(u.to_string(), "A(a)");
///
/// u = Union3::B(5);
/// assert_eq!(u.to_string(), "B(5)");
///
/// u = Union3::C(false);
/// assert_eq!(u.to_string(), "C(false)");
/// ```
///
/// # from_str
/// ```
/// use malachite_base::union_struct;
/// use malachite_base::unions::UnionFromStrError;
/// use std::fmt::{self, Display, Formatter};
/// use std::str::FromStr;
///
/// union_struct!(
///     (pub(crate)),
///     Union3,
///     Union3<T, T, T>,
///     [A, A, 'A', a],
///     [B, B, 'B', b],
///     [C, C, 'C', c]
/// );
///
/// let u3: Union3<bool, u32, char> = Union3::from_str("B(5)").unwrap();
/// assert_eq!(u3, Union3::B(5));
///
/// let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("xyz");
/// assert_eq!(result, Err(UnionFromStrError::Generic("xyz".to_string())));
///
/// let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("A(ab)");
/// if let Err(UnionFromStrError::Specific(Union3::A(_e))) = result {
/// } else {
///     panic!("wrong error variant")
/// }
/// ```
pub mod unions;
/// Functions for working with [`Vec`]s.
pub mod vecs;

#[cfg(feature = "test_build")]
pub mod test_util;
