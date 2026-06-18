// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivAssignEuclidean, DivEuclidean};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen_var_5;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_div_euclidean);
    register_demo!(runner, demo_natural_div_assign_euclidean);

    register_bench!(runner, benchmark_natural_div_euclidean_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_natural_div_assign_euclidean_evaluation_strategy
    );
}

fn demo_natural_div_euclidean(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.div_euclidean({}) = {:?}",
            x_old,
            y_old,
            x.div_euclidean(y)
        );
    }
}

fn demo_natural_div_assign_euclidean(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let mut x = x;
        let r = x.div_assign_euclidean(&y);
        println!("x := {x_old}; x.div_assign_euclidean(&{y}) = {r}; x = {x}");
    }
}

fn benchmark_natural_div_euclidean_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_euclidean(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Natural.div_euclidean(Natural)", &mut |(x, y)| {
                no_out!(x.div_euclidean(y));
            }),
            ("Natural.div_euclidean(&Natural)", &mut |(x, y)| {
                no_out!(x.div_euclidean(&y));
            }),
            ("(&Natural).div_euclidean(Natural)", &mut |(x, y)| {
                no_out!((&x).div_euclidean(y));
            }),
            ("(&Natural).div_euclidean(&Natural)", &mut |(x, y)| {
                no_out!((&x).div_euclidean(&y));
            }),
        ],
    );
}

fn benchmark_natural_div_assign_euclidean_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_assign_euclidean(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Natural.div_assign_euclidean(Natural)",
                &mut |(mut x, y)| {
                    no_out!(x.div_assign_euclidean(y));
                },
            ),
            (
                "Natural.div_assign_euclidean(&Natural)",
                &mut |(mut x, y)| {
                    no_out!(x.div_assign_euclidean(&y));
                },
            ),
        ],
    );
}
