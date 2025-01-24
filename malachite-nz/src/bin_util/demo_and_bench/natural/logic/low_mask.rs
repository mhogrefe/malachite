// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::low_mask::limbs_low_mask;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_low_mask);
    register_demo!(runner, demo_natural_low_mask);

    register_bench!(runner, benchmark_limbs_low_mask);
    register_bench!(runner, benchmark_natural_low_mask_algorithms);
}

fn demo_limbs_low_mask(gm: GenMode, config: &GenConfig, limit: usize) {
    for bits in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("limbs_low_mask({}) = {:?}", bits, limbs_low_mask(bits));
    }
}

fn demo_natural_low_mask(gm: GenMode, config: &GenConfig, limit: usize) {
    for bits in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("Natural::low_mask({}) = {}", bits, Natural::low_mask(bits));
    }
}

fn benchmark_limbs_low_mask(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_low_mask(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |bits| no_out!(limbs_low_mask(bits)))],
    );
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_low_mask_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.low_mask(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Natural.low_mask(u64)", &mut |bits| {
                no_out!(Natural::low_mask(bits))
            }),
            ("Natural.power_of_2(u64) - 1", &mut |bits| {
                no_out!(Natural::power_of_2(bits) - Natural::ONE)
            }),
        ],
    );
}
