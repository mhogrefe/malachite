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
use malachite_q::test_util::bench::bucketers::quadruple_1_2_3_rational_bit_i64_u64_max_bucketer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_power_of_2_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_log_base_power_of_2_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_power_of_2_rational_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_power_of_2_rational_prec
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_power_of_2_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_power_of_2_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_power_of_2_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_power_of_2_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_power_of_2_rational_prec_evaluation_strategy
    );
}

fn demo_big_exponent_float_log_base_power_of_2_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, k, prec, rm) in big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.log_base_power_of_2_prec_round_ref(k, prec, rm);
        println!("{x:#x}.log_base_power_of_2_prec_round({k}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_log_base_power_of_2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, _) in big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.log_base_power_of_2_prec_ref(k, prec);
        println!("{x:#x}.log_base_power_of_2_prec({k}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_log_base_power_of_2_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, k, prec, rm) in rational_signed_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_power_of_2_rational_prec_round_ref(&x, k, prec, rm);
        println!(
            "BigExponentFloat::log_base_power_of_2_rational_prec_round({x}, {k}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_log_base_power_of_2_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, k, prec, _) in rational_signed_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_power_of_2_rational_prec_ref(&x, k, prec);
        println!(
            "BigExponentFloat::log_base_power_of_2_rational_prec({x}, {k}, {prec}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_log_base_power_of_2_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.log_base_power_of_2_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.log_base_power_of_2_prec_round(i64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(x.log_base_power_of_2_prec_round(k, prec, rm));
                },
            ),
            (
                "BigExponentFloat.log_base_power_of_2_prec_round_ref(i64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(x.log_base_power_of_2_prec_round_ref(k, prec, rm));
                },
            ),
            (
                "BigExponentFloat.log_base_power_of_2_prec_round_assign(i64, u64, RoundingMode)",
                &mut |(mut x, k, prec, rm)| {
                    no_out!(x.log_base_power_of_2_prec_round_assign(k, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_power_of_2_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.log_base_power_of_2_prec(i64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.log_base_power_of_2_prec(i64, u64)",
                &mut |(x, k, prec, _)| {
                    no_out!(x.log_base_power_of_2_prec(k, prec));
                },
            ),
            (
                "BigExponentFloat.log_base_power_of_2_prec_ref(i64, u64)",
                &mut |(x, k, prec, _)| {
                    no_out!(x.log_base_power_of_2_prec_ref(k, prec));
                },
            ),
            (
                "BigExponentFloat.log_base_power_of_2_prec_assign(i64, u64)",
                &mut |(mut x, k, prec, _)| {
                    no_out!(x.log_base_power_of_2_prec_assign(k, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_power_of_2_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.log_base_power_of_2_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, k, prec, rm))| {
                no_out!(x.log_base_power_of_2_prec_round_ref(k, prec, rm));
            }),
            ("Float", &mut |((x, k, prec, rm), _)| {
                no_out!(x.log_base_power_of_2_prec_round_ref(k, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_power_of_2_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_power_of_2_rational_prec_round(Rational, i64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_signed_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_bit_i64_u64_max_bucketer("x", "k", "prec"),
        &mut [
            (
                "BigExponentFloat::log_base_power_of_2_rational_prec_round(Rational, i64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(BigExponentFloat::log_base_power_of_2_rational_prec_round(
                        x, k, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::log_base_power_of_2_rational_prec_round_ref(&Rational, i64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(
                        BigExponentFloat::log_base_power_of_2_rational_prec_round_ref(
                            &x, k, prec, rm
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_power_of_2_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_power_of_2_rational_prec(Rational, i64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_signed_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_bit_i64_u64_max_bucketer("x", "k", "prec"),
        &mut [
            (
                "BigExponentFloat::log_base_power_of_2_rational_prec(Rational, i64, u64)",
                &mut |(x, k, prec, _)| {
                    no_out!(BigExponentFloat::log_base_power_of_2_rational_prec(
                        x, k, prec
                    ));
                },
            ),
            (
                "BigExponentFloat::log_base_power_of_2_rational_prec_ref(&Rational, i64, u64)",
                &mut |(x, k, prec, _)| {
                    no_out!(BigExponentFloat::log_base_power_of_2_rational_prec_ref(
                        &x, k, prec
                    ));
                },
            ),
        ],
    );
}
