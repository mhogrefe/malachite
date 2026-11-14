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
use malachite_float::{BigExponentFloat, Float};
use malachite_q::test_util::bench::bucketers::quadruple_1_3_rational_bit_u64_max_bucketer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_log_base_prec_round);
    register_demo!(runner, demo_big_exponent_float_log_base_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_prec_round_in_domain
    );
    register_demo!(runner, demo_big_exponent_float_log_base_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_log_base_rational_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_rational_prec_round_in_domain
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_log_base_rational_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_log_base_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, b, prec, rm) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.log_base_prec_round_ref(b, prec, rm);
        println!("{x:#x}.log_base_prec_round({b}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_log_base_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, b, prec, _) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.log_base_prec_ref(b, prec);
        println!("{x:#x}.log_base_prec({b}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_log_base_prec_round_in_domain(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, b, prec, rm) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.log_base_prec_round_ref(b, prec, rm);
        println!("{x:#x}.log_base_prec_round({b}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_log_base_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.log_base_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.log_base_prec_round(u64, u64, RoundingMode)",
                &mut |(x, b, prec, rm)| {
                    no_out!(x.log_base_prec_round(b, prec, rm));
                },
            ),
            (
                "BigExponentFloat.log_base_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, b, prec, rm)| {
                    no_out!(x.log_base_prec_round_ref(b, prec, rm));
                },
            ),
            (
                "BigExponentFloat.log_base_prec_round_assign(u64, u64, RoundingMode)",
                &mut |(mut x, b, prec, rm)| {
                    no_out!(x.log_base_prec_round_assign(b, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.log_base_prec(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.log_base_prec(u64, u64)",
                &mut |(x, b, prec, _)| {
                    no_out!(x.log_base_prec(b, prec));
                },
            ),
            (
                "BigExponentFloat.log_base_prec_ref(u64, u64)",
                &mut |(x, b, prec, _)| {
                    no_out!(x.log_base_prec_ref(b, prec));
                },
            ),
            (
                "BigExponentFloat.log_base_prec_assign(u64, u64)",
                &mut |(mut x, b, prec, _)| {
                    no_out!(x.log_base_prec_assign(b, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.log_base_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, b, prec, rm))| {
                no_out!(x.log_base_prec_round_ref(b, prec, rm));
            }),
            ("Float", &mut |((x, b, prec, rm), _)| {
                no_out!(x.log_base_prec_round_ref(b, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_log_base_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, b, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_rational_prec_round_ref(&x, b, prec, rm);
        println!(
            "BigExponentFloat::log_base_rational_prec_round({x}, {b}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_log_base_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, b, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_rational_prec_ref(&x, b, prec);
        println!("BigExponentFloat::log_base_rational_prec({x}, {b}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_log_base_rational_prec_round_in_domain(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, b, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_rational_prec_round_ref(&x, b, prec, rm);
        println!(
            "BigExponentFloat::log_base_rational_prec_round({x}, {b}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_log_base_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_rational_prec_round(Rational, u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::log_base_rational_prec_round(Rational, u64, u64, RoundingMode)",
                &mut |(x, b, prec, rm)| {
                    no_out!(BigExponentFloat::log_base_rational_prec_round(
                        x, b, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::log_base_rational_prec_round_ref(&Rational, &u64, u64, RoundingMode)",
                &mut |(x, b, prec, rm)| {
                    no_out!(BigExponentFloat::log_base_rational_prec_round_ref(
                        &x, b, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_rational_prec(Rational, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::log_base_rational_prec(Rational, u64, u64)",
                &mut |(x, b, prec, _)| {
                    no_out!(BigExponentFloat::log_base_rational_prec(x, b, prec));
                },
            ),
            (
                "BigExponentFloat::log_base_rational_prec_ref(&Rational, &u64, u64)",
                &mut |(x, b, prec, _)| {
                    no_out!(BigExponentFloat::log_base_rational_prec_ref(&x, b, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_rational_prec_round(&Rational, u64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(x, b, prec, rm)| {
                no_out!(BigExponentFloat::log_base_rational_prec_round_ref(
                    &x, b, prec, rm
                ));
            }),
            ("Float", &mut |(x, b, prec, rm)| {
                no_out!(Float::log_base_rational_prec_round_ref(&x, b, prec, rm));
            }),
        ],
    );
}
