// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

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
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::uninlined_format_args,
    clippy::unused_self
)]

#[cfg(feature = "bin_build")]
extern crate itertools;
#[cfg(feature = "bin_build")]
#[macro_use]
extern crate malachite_base;
#[cfg(feature = "bin_build")]
extern crate malachite_nz;
#[cfg(feature = "bin_build")]
extern crate num;
#[cfg(feature = "bin_build")]
extern crate rug;
#[cfg(feature = "bin_build")]
extern crate serde;
#[cfg(feature = "bin_build")]
extern crate serde_json;

#[cfg(feature = "bin_build")]
use crate::bin_util::demo_and_bench::register;
#[cfg(feature = "bin_build")]
use crate::bin_util::generate::digits_data::generate_string_data;
#[cfg(feature = "bin_build")]
use crate::bin_util::generate::factorial_data::generate_factorial_data;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::cmd::read_command_line_arguments;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::Runner;

// Examples:
//
// ```
// cargo run --release --features bin_build -- -l 100000 -m special_random -d
//     demo_natural_from_unsigned_u128 -c "mean_run_length_n 4 mean_run_length_d 1"
//
// cargo run --release --features bin_build -- -l 100000 -m random -b
//     benchmark_limbs_to_digits_small_base_algorithms
//
// cargo run --features bin_build -- -g digits_data
// ```
#[cfg(feature = "bin_build")]
fn main() {
    let args = read_command_line_arguments("malachite-nz test utils");
    let mut runner = Runner::new();
    register(&mut runner);
    if let Some(demo_key) = args.demo_key {
        runner.run_demo(&demo_key, args.generation_mode, &args.config, args.limit);
    } else if let Some(bench_key) = args.bench_key {
        runner.run_bench(
            &bench_key,
            args.generation_mode,
            &args.config,
            args.limit,
            &args.out,
        );
    } else {
        let codegen_key = args.codegen_key.unwrap();
        match codegen_key.as_str() {
            "digits_data" => generate_string_data(),
            "factorial_data" => generate_factorial_data(),
            _ => panic!("Invalid codegen key: {codegen_key}"),
        }
    }
}

#[cfg(not(feature = "bin_build"))]
fn main() {}

#[cfg(feature = "bin_build")]
pub mod bin_util {
    pub mod demo_and_bench;
    pub mod generate;
}
