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
use malachite_nz::natural::number_theory::tangent_numbers::tangent_numbers_prefix;
use malachite_nz::test_util::natural::number_theory::tangent_numbers::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_tangent_numbers_prefix);
    register_bench!(runner, benchmark_natural_tangent_numbers_prefix);
    register_bench!(runner, benchmark_natural_tangent_numbers_prefix_algorithms);
}

fn demo_natural_tangent_numbers_prefix(gm: GenMode, config: &GenConfig, limit: usize) {
    // Prefix lengths stay at u8 scale: a length-255 prefix already ends with a 3517-bit entry.
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "tangent_numbers_prefix({}) = {:?}",
            n,
            tangent_numbers_prefix(u64::from(n))
        );
    }
}

fn benchmark_natural_tangent_numbers_prefix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "tangent_numbers_prefix(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(tangent_numbers_prefix(u64::from(n)));
        })],
    );
}

fn benchmark_natural_tangent_numbers_prefix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "tangent_numbers_prefix(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(tangent_numbers_prefix(u64::from(n)));
            }),
            ("naive", &mut |n| {
                no_out!(tangent_numbers_prefix_naive(u64::from(n)));
            }),
        ],
    );
}
