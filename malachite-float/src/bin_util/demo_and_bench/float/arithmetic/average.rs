// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Average, AverageAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::{
    pair_float_max_complexity_bucketer,
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_float_unsigned_rounding_mode_quadruple_gen_var_15, float_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_average);
    register_demo!(runner, demo_float_average_debug);
    register_demo!(runner, demo_float_average_assign);
    register_demo!(runner, demo_float_average_prec_round);
    register_demo!(runner, demo_float_average_prec_round_debug);
    register_demo!(runner, demo_float_average_prec_round_assign);

    register_bench!(runner, benchmark_float_average_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_float_average_prec_round_evaluation_strategy
    );
}

fn demo_float_average(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).average({}) = {}", x_old, y_old, x.average(y));
    }
}

fn demo_float_average_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({:#x}).average({:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.average(y))
        );
    }
}

fn demo_float_average_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.average_assign(&y);
        println!("x := {x_old}; x.average_assign(&{y}); x = {x}");
    }
}

fn demo_float_average_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).average_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.average_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_average_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (avg, o) = x.average_prec_round(y, prec, rm);
        println!(
            "({:#x}).average_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(avg),
            o
        );
    }
}

fn demo_float_average_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.average_prec_round_assign(y.clone(), prec, rm);
        println!("x := {x_old}; x.average_prec_round_assign({y}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_float_average_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.average(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.average(Float)", &mut |(x, y)| no_out!(x.average(y))),
            ("Float.average(&Float)", &mut |(x, y)| {
                no_out!(x.average(&y));
            }),
            ("(&Float).average(Float)", &mut |(x, y)| {
                no_out!((&x).average(y));
            }),
            ("(&Float).average(&Float)", &mut |(x, y)| {
                no_out!((&x).average(&y));
            }),
        ],
    );
}

fn benchmark_float_average_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.average_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.average_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.average_prec_round(y, prec, rm)),
            ),
            (
                "Float.average_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.average_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).average_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.average_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).average_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.average_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}
