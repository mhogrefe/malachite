// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2Sqrt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use malachite_base::test_util::num::arithmetic::mod_power_of_2_sqrt::mod_power_of_2_sqrt_naive;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_sqrt);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_sqrt);
    register_bench!(runner, benchmark_mod_power_of_2_sqrt_algorithms_u8);
}

fn demo_mod_power_of_2_sqrt<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow) in unsigned_pair_gen_var_17::<T>().get(gm, config).take(limit) {
        println!(
            "{}.mod_power_of_2_sqrt({}) = {:?}",
            x,
            pow,
            x.mod_power_of_2_sqrt(pow)
        );
    }
}

fn benchmark_mod_power_of_2_sqrt<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_sqrt(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_17::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(x, pow)| {
            no_out!(x.mod_power_of_2_sqrt(pow));
        })],
    );
}

// The naive search tries up to 2 ^ pow residues, so it is only benchmarked for u8.
fn benchmark_mod_power_of_2_sqrt_algorithms_u8(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u8.mod_power_of_2_sqrt(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_17::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("default", &mut |(x, pow)| {
                no_out!(x.mod_power_of_2_sqrt(pow));
            }),
            ("naive", &mut |(x, pow)| {
                no_out!(mod_power_of_2_sqrt_naive(x, pow));
            }),
        ],
    );
}
