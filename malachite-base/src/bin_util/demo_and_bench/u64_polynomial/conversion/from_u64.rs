// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::u64_polynomial::U64Polynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u64_polynomial_from_u64);
    register_bench!(runner, benchmark_u64_polynomial_from_u64);
}

fn demo_u64_polynomial_from_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "U64Polynomial::from({}) = {}",
            x.clone(),
            U64Polynomial::from(x)
        );
    }
}

fn benchmark_u64_polynomial_from_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "U64Polynomial::from(u64)",
        BenchmarkType::Single,
        unsigned_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| {
            let _ = U64Polynomial::from(x);
        })],
    );
}
