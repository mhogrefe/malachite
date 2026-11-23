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
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_q::test_util::bench::bucketers::triple_1_2_rational_bit_u64_max_bucketer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_lambert_w_minus_1_prec_round);
    register_demo!(
        runner,
        demo_big_exponent_float_lambert_w_minus_1_prec_round_debug
    );
    register_demo!(runner, demo_big_exponent_float_lambert_w_minus_1_prec);
    register_demo!(runner, demo_big_exponent_float_lambert_w_minus_1_prec_ref);
    register_demo!(
        runner,
        demo_big_exponent_float_lambert_w_minus_1_prec_assign
    );
    register_demo!(runner, demo_big_exponent_float_lambert_w_minus_1_prec_debug);
    register_demo!(
        runner,
        demo_big_exponent_float_lambert_w_minus_1_rational_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_lambert_w_minus_1_rational_prec_round_debug
    );
    register_demo!(
        runner,
        demo_big_exponent_float_lambert_w_minus_1_rational_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_lambert_w_minus_1_rational_prec_debug
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_lambert_w_minus_1_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_lambert_w_minus_1_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_lambert_w_minus_1_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_lambert_w_minus_1_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_lambert_w_minus_1_rational_prec_evaluation_strategy
    );
}

fn demo_big_exponent_float_lambert_w_minus_1_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.lambert_w_minus_1_prec_round_ref(prec, rm);
        println!("({x}).lambert_w_minus_1_prec_round({prec}, {rm}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.lambert_w_minus_1_prec_round_ref(prec, rm);
        println!("({x:#x}).lambert_w_minus_1_prec_round({prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.lambert_w_minus_1_prec_ref(prec);
        println!("({x}).lambert_w_minus_1_prec({prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.lambert_w_minus_1_prec_ref(prec);
        println!("({x:#x}).lambert_w_minus_1_prec({prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(&x, prec, rm);
        println!(
            "BigExponentFloat::lambert_w_minus_1_rational_prec_round({x}, {prec}, {rm}) = ({z}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(&x, prec, rm);
        println!(
            "BigExponentFloat::lambert_w_minus_1_rational_prec_round({x}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, _) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_ref(&x, prec);
        println!("BigExponentFloat::lambert_w_minus_1_rational_prec({x}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_rational_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, _) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::lambert_w_minus_1_rational_prec_ref(&x, prec);
        println!(
            "BigExponentFloat::lambert_w_minus_1_rational_prec({x}, {prec}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_prec_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).lambert_w_minus_1_prec_ref({prec}) = {:?}",
            x.lambert_w_minus_1_prec_ref(prec)
        );
    }
}

fn demo_big_exponent_float_lambert_w_minus_1_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.lambert_w_minus_1_prec_assign(prec);
        println!("x := {x_old}; x.lambert_w_minus_1_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn benchmark_big_exponent_float_lambert_w_minus_1_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.lambert_w_minus_1_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.lambert_w_minus_1_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.lambert_w_minus_1_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.lambert_w_minus_1_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.lambert_w_minus_1_prec_round_ref(prec, rm));
                },
            ),
            (
                "BigExponentFloat.lambert_w_minus_1_prec_round_assign(u64, RoundingMode)",
                &mut |(mut x, prec, rm)| {
                    no_out!(x.lambert_w_minus_1_prec_round_assign(prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_lambert_w_minus_1_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.lambert_w_minus_1_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.lambert_w_minus_1_prec(u64)",
                &mut |(x, prec, _)| {
                    no_out!(x.lambert_w_minus_1_prec(prec));
                },
            ),
            (
                "BigExponentFloat.lambert_w_minus_1_prec_ref(u64)",
                &mut |(x, prec, _)| {
                    no_out!(x.lambert_w_minus_1_prec_ref(prec));
                },
            ),
            (
                "BigExponentFloat.lambert_w_minus_1_prec_assign(u64)",
                &mut |(mut x, prec, _)| {
                    no_out!(x.lambert_w_minus_1_prec_assign(prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_lambert_w_minus_1_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.lambert_w_minus_1_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_24_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, prec, rm))| {
                no_out!(x.lambert_w_minus_1_prec_round_ref(prec, rm));
            }),
            ("Float", &mut |((x, prec, rm), _)| {
                no_out!(x.lambert_w_minus_1_prec_round_ref(prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_lambert_w_minus_1_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::lambert_w_minus_1_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::lambert_w_minus_1_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::lambert_w_minus_1_rational_prec_round(
                        x, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::lambert_w_minus_1_rational_prec_round_ref(
                        &x, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_lambert_w_minus_1_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::lambert_w_minus_1_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::lambert_w_minus_1_rational_prec(Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::lambert_w_minus_1_rational_prec(x, prec));
                },
            ),
            (
                "BigExponentFloat::lambert_w_minus_1_rational_prec_ref(&Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::lambert_w_minus_1_rational_prec_ref(
                        &x, prec
                    ));
                },
            ),
        ],
    );
}
