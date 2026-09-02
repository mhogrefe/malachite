// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::test_util::bench::bucketers::{signed_abs_bucketer, unsigned_direct_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen_var_5, unsigned_gen_var_5};
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_power_of_2_u64);
    register_demo!(runner, demo_gaussian_rational_power_of_2_i64);

    register_bench!(runner, benchmark_gaussian_rational_power_of_2_u64);
    register_bench!(runner, benchmark_gaussian_rational_power_of_2_i64);
}

fn demo_gaussian_rational_power_of_2_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_5::<u64>().get(gm, config).take(limit) {
        println!(
            "GaussianRational::power_of_2({}) = {}",
            x,
            GaussianRational::power_of_2(x)
        );
    }
}

fn demo_gaussian_rational_power_of_2_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in signed_gen_var_5::<i64>().get(gm, config).take(limit) {
        println!(
            "GaussianRational::power_of_2({}) = {}",
            x,
            GaussianRational::power_of_2(x)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_power_of_2_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.power_of_2(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::power_of_2(x));
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_power_of_2_i64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.power_of_2(i64)",
        BenchmarkType::Single,
        signed_gen_var_5::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_abs_bucketer("pow"),
        &mut [("Malachite", &mut |x| {
            no_out!(GaussianRational::power_of_2(x));
        })],
    );
}
