// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_abs_squared);
    register_demo!(runner, demo_natural_abs_squared_ref);

    register_bench!(runner, benchmark_natural_abs_squared_evaluation_strategy);
}

fn demo_natural_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("|{}|^2 = {}", x.clone(), x.abs_squared());
    }
}

fn demo_natural_abs_squared_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("|&{}|^2 = {}", x, (&x).abs_squared());
    }
}

fn benchmark_natural_abs_squared_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.abs_squared()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Natural.abs_squared()", &mut |x| no_out!(x.abs_squared())),
            ("(&Natural).abs_squared()", &mut |x| {
                no_out!((&x).abs_squared());
            }),
        ],
    );
}
