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
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_round);
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_round_ref);
    register_demo!(
        runner,
        demo_big_exponent_float_ln_1_plus_x_prec_round_assign
    );
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_round_debug);
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec);
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_ref);
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_assign);
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_debug);
    register_demo!(runner, demo_big_exponent_float_ln_1_plus_x_prec_ref_debug);
    register_demo!(
        runner,
        demo_big_exponent_float_ln_1_plus_x_prec_assign_debug
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_ln_1_plus_x_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ln_1_plus_x_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ln_1_plus_x_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_ln_1_plus_x_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.ln_1_plus_x_prec_round_ref(prec, rm);
        println!("({x}).ln_1_plus_x_prec_round({prec}, {rm}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.ln_1_plus_x_prec_round_ref(prec, rm);
        println!("({x:#x}).ln_1_plus_x_prec_round({prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.ln_1_plus_x_prec_ref(prec);
        println!("({x}).ln_1_plus_x_prec({prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.ln_1_plus_x_prec_ref(prec);
        println!("({x:#x}).ln_1_plus_x_prec({prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).ln_1_plus_x_prec_ref({prec}) = {:?}",
            x.ln_1_plus_x_prec_ref(prec)
        );
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.ln_1_plus_x_prec_ref(prec);
        println!("(&{x:#x}).ln_1_plus_x_prec_ref({prec}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.ln_1_plus_x_prec_assign(prec);
        println!("x := {x_old}; x.ln_1_plus_x_prec_assign({prec}); x = {x}");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_1_plus_x_prec_assign(prec);
        println!("x := {x_old:#x}; x.ln_1_plus_x_prec_assign({prec}) = {o:?}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_round_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({x}).ln_1_plus_x_prec_round_ref({prec}, {rm}) = {:?}",
            x.ln_1_plus_x_prec_round_ref(prec, rm)
        );
    }
}

fn demo_big_exponent_float_ln_1_plus_x_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_1_plus_x_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.ln_1_plus_x_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_big_exponent_float_ln_1_plus_x_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ln_1_plus_x_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.ln_1_plus_x_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.ln_1_plus_x_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.ln_1_plus_x_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.ln_1_plus_x_prec_round_ref(prec, rm));
                },
            ),
            (
                "BigExponentFloat.ln_1_plus_x_prec_round_assign(u64, RoundingMode)",
                &mut |(mut x, prec, rm)| {
                    no_out!(x.ln_1_plus_x_prec_round_assign(prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_ln_1_plus_x_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ln_1_plus_x_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.ln_1_plus_x_prec(u64)",
                &mut |(x, prec, _)| {
                    no_out!(x.ln_1_plus_x_prec(prec));
                },
            ),
            (
                "BigExponentFloat.ln_1_plus_x_prec_ref(u64)",
                &mut |(x, prec, _)| {
                    no_out!(x.ln_1_plus_x_prec_ref(prec));
                },
            ),
            (
                "BigExponentFloat.ln_1_plus_x_prec_assign(u64)",
                &mut |(mut x, prec, _)| {
                    no_out!(x.ln_1_plus_x_prec_assign(prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_ln_1_plus_x_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ln_1_plus_x_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_16_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, prec, rm))| {
                no_out!(x.ln_1_plus_x_prec_round_ref(prec, rm));
            }),
            ("Float", &mut |((x, prec, rm), _)| {
                no_out!(x.ln_1_plus_x_prec_round_ref(prec, rm));
            }),
        ],
    );
}
