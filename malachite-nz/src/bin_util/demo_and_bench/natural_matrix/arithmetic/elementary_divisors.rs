// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ElementaryDivisors;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_elementary_divisors);
    register_demo!(runner, demo_natural_matrix_elementary_divisors_val);
    register_bench!(
        runner,
        benchmark_natural_matrix_elementary_divisors_evaluation_strategy
    );
}

fn demo_natural_matrix_elementary_divisors(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen().get(gm, config).take(limit) {
        println!(
            "elementary_divisors({a}) = {}",
            (&a).elementary_divisors().to_debug_string()
        );
    }
}

fn demo_natural_matrix_elementary_divisors_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "{a_old}.elementary_divisors() = {}",
            a.elementary_divisors().to_debug_string()
        );
    }
}

fn benchmark_natural_matrix_elementary_divisors_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.elementary_divisors()",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.elementary_divisors()", &mut |a| {
                no_out!(a.elementary_divisors());
            }),
            ("(&NaturalMatrix).elementary_divisors()", &mut |a| {
                no_out!((&a).elementary_divisors());
            }),
        ],
    );
}
