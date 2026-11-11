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
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;
use malachite_float::{BigExponentFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_from_float);
    register_demo!(runner, demo_big_exponent_float_from_float_ref);

    register_bench!(
        runner,
        benchmark_big_exponent_float_from_float_evaluation_strategy
    );
}

fn demo_big_exponent_float_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({:#x}) = {:#x}",
            ComparableFloatRef(&f),
            BigExponentFloat::from(f.clone())
        );
    }
}

fn demo_big_exponent_float_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from(&{:#x}) = {:#x}",
            ComparableFloatRef(&f),
            BigExponentFloat::from(&f)
        );
    }
}

fn benchmark_big_exponent_float_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("f"),
        &mut [
            ("BigExponentFloat::from(Float)", &mut |f| {
                no_out!(BigExponentFloat::from(f));
            }),
            ("BigExponentFloat::from(&Float)", &mut |f| {
                no_out!(BigExponentFloat::from(&f));
            }),
        ],
    );
}
