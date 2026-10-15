// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_to_string);
    register_demo!(runner, demo_natural_matrix_to_debug_string);
    register_bench!(runner, benchmark_natural_matrix_to_string);
    register_bench!(runner, benchmark_natural_matrix_to_debug_string);
}

fn demo_natural_matrix_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        println!("{m}");
    }
}

fn demo_natural_matrix_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        println!("{m:?}");
    }
}

fn benchmark_natural_matrix_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.to_string()",
        BenchmarkType::Single,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            no_out!(m.to_string());
        })],
    );
}

fn benchmark_natural_matrix_to_debug_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.to_debug_string()",
        BenchmarkType::Single,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            no_out!(m.to_debug_string());
        })],
    );
}
