#![allow(
    unstable_name_collisions,
    clippy::assertions_on_constants,
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    clippy::range_plus_one,
    clippy::suspicious_arithmetic_impl,
    clippy::suspicious_op_assign_impl,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms
)]
#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::large_digit_groups,
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
    clippy::unused_self
)]

extern crate itertools;
#[macro_use]
extern crate malachite_base;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(feature = "32_bit_limbs")]
pub use platform_32 as platform;
#[cfg(not(feature = "32_bit_limbs"))]
pub use platform_64 as platform;

#[cfg(feature = "fail_on_untested_path")]
#[inline]
pub fn fail_on_untested_path(message: &str) {
    panic!("Untested path. {}", message);
}

#[cfg(not(feature = "fail_on_untested_path"))]
#[inline]
pub const fn fail_on_untested_path(_message: &str) {}

#[cfg(feature = "32_bit_limbs")]
pub mod platform_32;
#[cfg(not(feature = "32_bit_limbs"))]
pub mod platform_64;
/// This module defines `Natural`s (arbitrarily large non-negative integers).
#[macro_use]
pub mod natural;
/// This module defines `Integer`s (which are arbitrarily large).
pub mod integer;
