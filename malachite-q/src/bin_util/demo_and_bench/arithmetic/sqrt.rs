// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSqrt;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::rational_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_checked_sqrt);
    register_demo!(runner, demo_rational_checked_sqrt_ref);

    register_bench!(runner, benchmark_rational_checked_sqrt_evaluation_strategy);
}

fn demo_rational_checked_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_3().get(gm, config).take(limit) {
        println!("({}).checked_sqrt() = {:?}", x, x.clone().checked_sqrt());
    }
}

fn demo_rational_checked_sqrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_3().get(gm, config).take(limit) {
        println!("(&{}).checked_sqrt() = {:?}", x, (&x).checked_sqrt());
    }
}

fn benchmark_rational_checked_sqrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.checked_sqrt()",
        BenchmarkType::EvaluationStrategy,
        rational_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            (
                "Rational.checked_sqrt()",
                &mut |x| no_out!(x.checked_sqrt()),
            ),
            ("(&Rational).checked_sqrt()", &mut |x| {
                no_out!((&x).checked_sqrt())
            }),
        ],
    );
}
