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

#[macro_export]
macro_rules! assert_panic {
    ($e: expr) => {
        let result = catch_unwind(|| $e);
        assert!(result.is_err());
    };
}

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

#[macro_export]
macro_rules! apply_fn_to_unsigneds {
    ($f: ident) => {
        $f::<u8>();
        $f::<u16>();
        $f::<u32>();
        $f::<u64>();
        $f::<u128>();
        $f::<usize>();
    };
}

#[macro_export]
macro_rules! apply_fn_to_signeds {
    ($f: ident) => {
        $f::<i8>();
        $f::<i16>();
        $f::<i32>();
        $f::<i64>();
        $f::<i128>();
        $f::<isize>();
    };
}

#[macro_export]
macro_rules! apply_fn_to_primitive_ints {
    ($f: ident) => {
        apply_fn_to_unsigneds!($f);
        apply_fn_to_signeds!($f);
    };
}

#[macro_export]
macro_rules! apply_fn_to_unsigned_signed_pair {
    ($f: ident) => {
        $f::<u8, i8>();
        $f::<u16, i16>();
        $f::<u32, i32>();
        $f::<u64, i64>();
        $f::<u128, i128>();
        $f::<usize, isize>();
    };
}

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
pub mod num;
pub mod orderings;
pub mod random;
pub mod rounding_modes;
#[macro_use]
pub mod slices;
pub mod strings;
pub mod vecs;
pub mod voids;
