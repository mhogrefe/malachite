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
use malachite_float::big_exponent_float::arithmetic::rational_div_big_exponent_float::*;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_rational_div_big_exponent_float_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_div_big_exponent_float_prec
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_div_big_exponent_float_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_div_big_exponent_float_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_div_big_exponent_float_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_div_big_exponent_float_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_rational_div_big_exponent_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) =
            BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(&x, &y, prec, rm);
        println!(
            "BigExponentFloat::rational_div_big_exponent_float_prec_round({x}, {y:#x}, {prec}, \
            {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_rational_div_big_exponent_float_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::rational_div_big_exponent_float_prec_ref_ref(&x, &y, prec);
        println!(
            "BigExponentFloat::rational_div_big_exponent_float_prec({x}, {y:#x}, {prec}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_rational_div_big_exponent_float_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_div_big_exponent_float_prec_round(Rational, BigExponentFloat, \
        u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_round(Rational, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_round(
                            x, y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(Rational, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_round_val_ref(
                            x, &y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(&Rational, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_val(
                            &x, y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(&Rational, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
                            &x, &y, prec, rm
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_div_big_exponent_float_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_div_big_exponent_float_prec(Rational, BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec(Rational, \
                BigExponentFloat, u64)",
                &mut |(y, x, prec)| {
                    no_out!(BigExponentFloat::rational_div_big_exponent_float_prec(
                        x, y, prec
                    ));
                },
            ),
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_val_ref(Rational, \
                &BigExponentFloat, u64)",
                &mut |(y, x, prec)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_val_ref(x, &y, prec)
                    );
                },
            ),
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_ref_val(&Rational, \
                BigExponentFloat, u64)",
                &mut |(y, x, prec)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_ref_val(&x, y, prec)
                    );
                },
            ),
            (
                "BigExponentFloat::rational_div_big_exponent_float_prec_ref_ref(&Rational, \
                &BigExponentFloat, u64)",
                &mut |(y, x, prec)| {
                    no_out!(
                        BigExponentFloat::rational_div_big_exponent_float_prec_ref_ref(
                            &x, &y, prec
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_div_big_exponent_float_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_div_big_exponent_float_prec_round(Rational, BigExponentFloat, \
        u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float kernels", &mut |(y, x, prec, rm)| {
                no_out!(
                    BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
                        &x, &y, prec, rm
                    )
                );
            }),
            ("generic", &mut |(y, x, prec, rm)| {
                no_out!(rational_div_big_exponent_float_prec_round_generic(
                    &x, &y, prec, rm
                ));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_rational_div_big_exponent_float_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_div_big_exponent_float_prec_round(Rational, BigExponentFloat, \
        u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_6_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(
                    BigExponentFloat::rational_div_big_exponent_float_prec_round_ref_ref(
                        &x, &y, prec, rm
                    )
                );
            }),
            ("Float", &mut |((y, x, prec, rm), _)| {
                no_out!(Float::rational_div_float_prec_round_ref_ref(
                    &x, &y, prec, rm
                ));
            }),
        ],
    );
}
