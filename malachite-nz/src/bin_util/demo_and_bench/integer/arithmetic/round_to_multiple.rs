// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_integer_rounding_mode_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_round_to_multiple_assign);
    register_demo!(runner, demo_integer_round_to_multiple_assign_ref);
    register_demo!(runner, demo_integer_round_to_multiple);
    register_demo!(runner, demo_integer_round_to_multiple_val_ref);
    register_demo!(runner, demo_integer_round_to_multiple_ref_val);
    register_demo!(runner, demo_integer_round_to_multiple_ref_ref);

    register_bench!(
        runner,
        benchmark_integer_round_to_multiple_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_round_to_multiple_evaluation_strategy
    );
}

fn demo_integer_round_to_multiple_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in integer_integer_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.round_to_multiple_assign(y, rm);
        println!("x := {x_old}; x.round_to_multiple_assign({y_old}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_integer_round_to_multiple_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in integer_integer_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.round_to_multiple_assign(&y, rm);
        println!("x := {x_old}; x.round_to_multiple_assign(&{y}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_integer_round_to_multiple(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).round_to_multiple({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_integer_round_to_multiple_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).round_to_multiple(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.round_to_multiple(&y, rm)
        );
    }
}

fn demo_integer_round_to_multiple_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).round_to_multiple({}, {}) = {:?}",
            x,
            y_old,
            rm,
            (&x).round_to_multiple(y, rm)
        );
    }
}

fn demo_integer_round_to_multiple_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).round_to_multiple(&{}, {}) = {:?}",
            x,
            y,
            rm,
            (&x).round_to_multiple(&y, rm)
        );
    }
}

fn benchmark_integer_round_to_multiple_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.round_to_multiple_assign(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_integer_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("n"),
        &mut [
            (
                "Integer.round_to_multiple_assign(Integer, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.round_to_multiple_assign(y, rm)),
            ),
            (
                "Integer.round_to_multiple_assign(&Integer, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.round_to_multiple_assign(&y, rm)),
            ),
        ],
    );
}

fn benchmark_integer_round_to_multiple_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.round_to_multiple(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_integer_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("n"),
        &mut [
            (
                "Integer.round_to_multiple(Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.round_to_multiple(y, rm)),
            ),
            (
                "Integer.round_to_multiple(&Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.round_to_multiple(&y, rm)),
            ),
            (
                "(&Integer).round_to_multiple(Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).round_to_multiple(y, rm)),
            ),
            (
                "(&Integer).round_to_multiple(&Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).round_to_multiple(&y, rm)),
            ),
        ],
    );
}
