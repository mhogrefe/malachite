// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_power_of_2);
    register_bench!(runner, benchmark_natural_power_of_2);
}

fn demo_natural_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for pow in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("2^{} = {}", pow, Natural::power_of_2(pow));
    }
}

fn benchmark_natural_power_of_2(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.power_of_2(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |pow| no_out!(Natural::power_of_2(pow)))],
    );
}
