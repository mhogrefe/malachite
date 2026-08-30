// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_abs_squared);
    register_demo!(runner, demo_float_abs_squared_ref);
    register_demo!(runner, demo_float_abs_squared_assign);

    register_bench!(runner, benchmark_float_abs_squared_evaluation_strategy);
}

fn demo_float_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("|{}|^2 = {}", x.clone(), x.abs_squared());
    }
}

fn demo_float_abs_squared_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("|&{}|^2 = {}", x, (&x).abs_squared());
    }
}

fn demo_float_abs_squared_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.abs_squared_assign();
        println!("x := {old_x}; x.abs_squared_assign(); x = {x}");
    }
}

fn benchmark_float_abs_squared_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.abs_squared()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.abs_squared()", &mut |x| no_out!(x.abs_squared())),
            ("(&Float).abs_squared()", &mut |x| {
                no_out!((&x).abs_squared());
            }),
        ],
    );
}
