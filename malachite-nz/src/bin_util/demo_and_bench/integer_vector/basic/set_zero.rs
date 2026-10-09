// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::test_util::bench::bucketers::integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_set_zero);
    register_bench!(runner, benchmark_integer_vector_set_zero);
}

fn demo_integer_vector_set_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut v in integer_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        v.set_zero();
        println!("v := {v_old}; v.set_zero(); v = {v}");
    }
}

fn benchmark_integer_vector_set_zero(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.set_zero()",
        BenchmarkType::Single,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |mut v| {
            v.set_zero();
        })],
    );
}
