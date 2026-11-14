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
use malachite_float::test_util::generators::*;
use malachite_q::test_util::bench::bucketers::quadruple_2_3_rational_bit_u64_max_bucketer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_rational_big_exponent_float_base_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_rational_big_exponent_float_base_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_log_base_rational_big_exponent_float_base_prec_round_in_domain
    );

    register_bench!(runner, benchmark_big_exponent_float_log_base_rational_big_exponent_float_base_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_big_exponent_float_log_base_rational_big_exponent_float_base_prec_evaluation_strategy);
}

fn demo_big_exponent_float_log_base_rational_big_exponent_float_base_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (b, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_15()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
            &x, &b, prec, rm,
        );
        println!(
            "BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round({x}, {b:#x}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_log_base_rational_big_exponent_float_base_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (b, x, prec, _) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) =
            BigExponentFloat::log_base_rational_big_exponent_float_base_prec_ref(&x, &b, prec);
        println!(
            "BigExponentFloat::log_base_rational_big_exponent_float_base_prec({x}, {b:#x}, {prec}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_log_base_rational_big_exponent_float_base_prec_round_in_domain(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (b, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_18()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
            &x, &b, prec, rm,
        );
        println!(
            "BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round({x}, {b:#x}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_log_base_rational_big_exponent_float_base_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(b, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round(
                            x, b, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(&Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(b, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::log_base_rational_big_exponent_float_base_prec_round_ref(
                            &x, &b, prec, rm
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_log_base_rational_big_exponent_float_base_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::log_base_rational_big_exponent_float_base_prec(Rational, BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::log_base_rational_big_exponent_float_base_prec(Rational, BigExponentFloat, u64)",
                &mut |(b, x, prec, _)| {
                    no_out!(
                        BigExponentFloat::log_base_rational_big_exponent_float_base_prec(
                            x, b, prec
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::log_base_rational_big_exponent_float_base_prec_ref(&Rational, &BigExponentFloat, u64)",
                &mut |(b, x, prec, _)| {
                    no_out!(
                        BigExponentFloat::log_base_rational_big_exponent_float_base_prec_ref(
                            &x, &b, prec
                        )
                    );
                },
            ),
        ],
    );
}
