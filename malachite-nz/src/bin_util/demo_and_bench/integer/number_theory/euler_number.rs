// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer::number_theory::euler_number::*;
use malachite_nz::test_util::integer::number_theory::euler_number::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_euler_number);
    register_demo!(runner, demo_integer_euler_numbers_prefix);
    register_bench!(runner, benchmark_integer_euler_number_algorithms);
    register_bench!(runner, benchmark_integer_euler_numbers_prefix_algorithms);
}

fn demo_integer_euler_number(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "euler_number({}) = {}",
            n,
            Integer::euler_number(u64::from(n))
        );
    }
}

fn demo_integer_euler_numbers_prefix(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "euler_numbers_prefix({}) = {:?}",
            n,
            euler_numbers_prefix(u64::from(n))
        );
    }
}

fn benchmark_integer_euler_number_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::euler_number(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(Integer::euler_number(u64::from(n)));
            }),
            ("multimodular vector", &mut |n| {
                no_out!(euler_numbers_prefix(u64::from(n) + 1).pop());
            }),
        ],
    );
}

fn benchmark_integer_euler_numbers_prefix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "euler_numbers_prefix(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(euler_numbers_prefix(u64::from(n)));
            }),
            ("naive", &mut |n| {
                no_out!(euler_numbers_prefix_naive(u64::from(n)));
            }),
        ],
    );
}
