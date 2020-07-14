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

extern crate itertools;
extern crate rand;
extern crate rand_chacha;
extern crate sha3;

#[macro_export]
macro_rules! apply_to_unsigneds {
    ($m: tt) => {
        $m!(u8);
        $m!(u16);
        $m!(u32);
        $m!(u64);
        $m!(u128);
        $m!(usize);
    };
}

#[macro_export]
macro_rules! apply_to_signeds {
    ($m: tt) => {
        $m!(i8);
        $m!(i16);
        $m!(i32);
        $m!(i64);
        $m!(i128);
        $m!(isize);
    };
}

#[macro_export]
macro_rules! apply_to_primitive_ints {
    ($m: tt) => {
        apply_to_unsigneds!($m);
        apply_to_signeds!($m);
    };
}

#[macro_export]
macro_rules! apply_to_unsigned_signed_pair {
    ($m: tt) => {
        $m!(u8, i8);
        $m!(u16, i16);
        $m!(u32, i32);
        $m!(u64, i64);
        $m!(u128, i128);
        $m!(usize, isize);
    };
}

#[macro_use]
pub mod named;
#[macro_use]
pub mod bools;
#[macro_use]
pub mod chars;
pub mod comparison;
pub mod crement;
pub mod exhaustive;
pub mod iterators;
pub mod num;
pub mod orderings;
pub mod random;
pub mod rounding_modes;
#[macro_use]
pub mod slices;
pub mod strings;
pub mod vecs;
pub mod voids;
