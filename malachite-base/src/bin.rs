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
extern crate walkdir;

#[cfg(feature = "bin_build")]
use bin_util::demo_and_bench::register;
#[cfg(feature = "bin_build")]
use bin_util::generate::max_base::generate_max_base;
#[cfg(feature = "bin_build")]
use bin_util::generate::rle::generate_rle_encoding;
#[cfg(feature = "bin_build")]
use bin_util::generate::tuning_manager::{build_reference_data, test};
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::cmd::read_command_line_arguments;
#[cfg(feature = "bin_build")]
use malachite_base::test_util::runner::Runner;

// Examples:
//
// ```
// cargo run --features bin_build -- -g max_base
// cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_mod_pow_u32
// ```
#[cfg(feature = "bin_build")]
fn main() {
    let args = read_command_line_arguments("malachite-base");
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
            "max_base" => generate_max_base(),
            "rle_encode" => generate_rle_encoding(),
            "tm_build_reference_data" => build_reference_data(),
            "tm_test" => test(),
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
