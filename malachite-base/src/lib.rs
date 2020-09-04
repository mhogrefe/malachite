#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
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
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]
#![allow(
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    unstable_name_collisions
)]

extern crate itertools;
extern crate rand;
extern crate rand_chacha;
extern crate sha3;

/// This module contains the `Named` trait, for getting a type's name.
#[macro_use]
pub mod named;

/// This module contains functions for working with `bool`s.
#[macro_use]
pub mod bools;
/// This module contains functions for working with `char`s.
#[macro_use]
pub mod chars;
/// This module contains macros and traits related to comparing values.
pub mod comparison;
/// This module contains functions and adaptors for iterators.
pub mod iterators;
/// This module contains functions for working with primitive integers and floats.
#[macro_use]
pub mod num;
/// This module contains functions for working with `Ordering`s.
pub mod orderings;
pub mod random;
pub mod rounding_modes;
#[macro_use]
pub mod slices;
pub mod nevers;
pub mod strings;
pub mod vecs;
