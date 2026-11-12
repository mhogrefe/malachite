// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_abs);
    register_bench!(runner, benchmark_big_exponent_float_abs_evaluation_strategy);
}

fn demo_big_exponent_float_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.abs() = {:#x}", (&x).abs());
    }
}

fn benchmark_big_exponent_float_abs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.abs()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat.abs()", &mut |x| {
                no_out!(x.abs());
            }),
            ("(&BigExponentFloat).abs()", &mut |x| {
                no_out!((&x).abs());
            }),
            ("BigExponentFloat.abs_assign()", &mut |mut x| {
                x.abs_assign();
            }),
        ],
    );
}
