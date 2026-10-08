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
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_zero);
    register_bench!(runner, benchmark_integer_vector_zero);
}

fn demo_integer_vector_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for dimension in unsigned_gen_var_5::<u64>().get(gm, config).take(limit) {
        println!(
            "IntegerVector::zero({dimension}) = {}",
            IntegerVector::zero(dimension)
        );
    }
}

fn benchmark_integer_vector_zero(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "IntegerVector::zero(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |dimension| {
            no_out!(IntegerVector::zero(dimension));
        })],
    );
}
