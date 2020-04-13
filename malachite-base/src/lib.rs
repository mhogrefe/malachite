#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
    clippy::manual_mul_add,
    clippy::map_flatten,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::option_map_unwrap_or,
    clippy::option_map_unwrap_or_else,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::result_map_unwrap_or_else,
    clippy::single_match_else,
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]
#![allow(clippy::cognitive_complexity, clippy::many_single_char_names)]

pub mod bools;
pub mod chars;
pub mod comparison;
pub mod crement;
#[macro_use]
pub mod named;
pub mod num;
pub mod round;
pub mod slices;
pub mod strings;
pub mod vecs;
