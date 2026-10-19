// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ContentChained;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_content_chained);
    register_demo!(runner, demo_integer_matrix_content_chained_val_ref);
    register_demo!(runner, demo_integer_matrix_content_chained_ref_val);
    register_demo!(runner, demo_integer_matrix_content_chained_ref_ref);
    register_bench!(
        runner,
        benchmark_integer_matrix_content_chained_evaluation_strategy
    );
}

fn demo_integer_matrix_content_chained(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, x) in integer_matrix_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let x_old = x.clone();
        println!(
            "({a_old}).content_chained({x_old}) = {}",
            a.content_chained(x)
        );
    }
}

fn demo_integer_matrix_content_chained_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, x) in integer_matrix_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "({a_old}).content_chained(&{x}) = {}",
            a.content_chained(&x)
        );
    }
}

fn demo_integer_matrix_content_chained_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, x) in integer_matrix_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "(&{a}).content_chained({x_old}) = {}",
            (&a).content_chained(x)
        );
    }
}

fn demo_integer_matrix_content_chained_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, x) in integer_matrix_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).content_chained(&{x}) = {}",
            (&a).content_chained(&x)
        );
    }
}

fn benchmark_integer_matrix_content_chained_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.content_chained(Natural)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.content_chained(Natural)", &mut |(a, x)| {
                no_out!(a.content_chained(x));
            }),
            ("IntegerMatrix.content_chained(&Natural)", &mut |(a, x)| {
                no_out!(a.content_chained(&x));
            }),
            (
                "(&IntegerMatrix).content_chained(Natural)",
                &mut |(a, x)| {
                    no_out!((&a).content_chained(x));
                },
            ),
            (
                "(&IntegerMatrix).content_chained(&Natural)",
                &mut |(a, x)| {
                    no_out!((&a).content_chained(&x));
                },
            ),
        ],
    );
}
