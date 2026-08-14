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
use malachite_nz::natural::arithmetic::landau_function::landau_function_prefix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_landau_function_prefix);
    register_bench!(runner, benchmark_natural_landau_function_prefix);
}

fn demo_natural_landau_function_prefix(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "landau_function_prefix({}) = {:?}",
            n,
            landau_function_prefix(u64::from(n))
        );
    }
}

fn benchmark_natural_landau_function_prefix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "landau_function_prefix(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(landau_function_prefix(u64::from(n)));
        })],
    );
}
