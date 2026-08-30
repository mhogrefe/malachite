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
use malachite_q::test_util::bench::bucketers::gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_abs_squared);
    register_demo!(runner, demo_gaussian_rational_abs_squared_ref);
    register_demo!(runner, demo_gaussian_rational_abs_squared_assign);

    register_bench!(
        runner,
        benchmark_gaussian_rational_abs_squared_evaluation_strategy
    );
}

fn demo_gaussian_rational_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!("|{}|^2 = {}", x.clone(), x.abs_squared());
    }
}

fn demo_gaussian_rational_abs_squared_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!("|&{}|^2 = {}", x, (&x).abs_squared());
    }
}

fn demo_gaussian_rational_abs_squared_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in gaussian_rational_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.abs_squared_assign();
        println!("x := {old_x}; x.abs_squared_assign(); x = {x}");
    }
}

fn benchmark_gaussian_rational_abs_squared_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.abs_squared()",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [
            ("GaussianRational.abs_squared()", &mut |x| {
                no_out!(x.abs_squared());
            }),
            ("(&GaussianRational).abs_squared()", &mut |x| {
                no_out!((&x).abs_squared());
            }),
        ],
    );
}
