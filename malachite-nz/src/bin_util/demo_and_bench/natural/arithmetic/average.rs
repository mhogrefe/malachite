// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Average, AverageAssign, AverageRound, AverageRoundAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_natural_max_bit_bucketer, triple_1_2_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_natural_rounding_mode_triple_gen_var_3, natural_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_average);
    register_demo!(runner, demo_natural_average_assign);
    register_demo!(runner, demo_natural_average_round);
    register_demo!(runner, demo_natural_average_round_assign);

    register_bench!(runner, benchmark_natural_average_evaluation_strategy);
    register_bench!(runner, benchmark_natural_average_round_evaluation_strategy);
}

fn demo_natural_average(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).average({}) = {}", x_old, y_old, x.average(y));
    }
}

fn demo_natural_average_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.average_assign(&y);
        println!("x := {x_old}; x.average_assign(&{y}); x = {x}");
    }
}

fn demo_natural_average_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in natural_natural_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).average_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.average_round(y, rm)
        );
    }
}

fn demo_natural_average_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in natural_natural_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.average_round_assign(&y, rm);
        println!("x := {x_old}; x.average_round_assign(&{y}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_natural_average_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.average(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.average(Natural)", &mut |(x, y)| {
                no_out!(x.average(y));
            }),
            ("Natural.average(&Natural)", &mut |(x, y)| {
                no_out!(x.average(&y));
            }),
            ("(&Natural).average(Natural)", &mut |(x, y)| {
                no_out!((&x).average(y));
            }),
            ("(&Natural).average(&Natural)", &mut |(x, y)| {
                no_out!((&x).average(&y));
            }),
        ],
    );
}

fn benchmark_natural_average_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.average_round(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.average_round(Natural, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.average_round(y, rm));
                },
            ),
            (
                "Natural.average_round(&Natural, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.average_round(&y, rm));
                },
            ),
            (
                "(&Natural).average_round(Natural, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!((&x).average_round(y, rm));
                },
            ),
            (
                "(&Natural).average_round(&Natural, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!((&x).average_round(&y, rm));
                },
            ),
        ],
    );
}
