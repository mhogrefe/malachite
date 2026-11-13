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

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_positive_difference_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_positive_difference_prec);
    register_demo!(runner, demo_big_exponent_float_positive_difference);

    register_bench!(
        runner,
        benchmark_big_exponent_float_positive_difference_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_positive_difference_prec_round_float_comparison
    );
    register_demo!(
        runner,
        demo_big_exponent_float_positive_difference_rational_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_positive_difference_rational_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_positive_difference_big_exponent_float_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_positive_difference_big_exponent_float_prec
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_positive_difference_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_positive_difference_rational_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_rational_positive_difference_big_exponent_float_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_big_exponent_float_rational_positive_difference_big_exponent_float_prec_round_float_comparison);
}

fn demo_big_exponent_float_positive_difference_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_8()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.positive_difference_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.positive_difference_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_positive_difference_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.positive_difference_prec_ref_ref(&y, prec);
        println!("{x:#x}.positive_difference_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_positive_difference_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.positive_difference_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_8()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.positive_difference_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.positive_difference_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.positive_difference_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_positive_difference_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.positive_difference_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_8_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.positive_difference_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.positive_difference_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_positive_difference(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{x:#x}.positive_difference({y:#x}) = {:#x}",
            x.positive_difference_ref_ref(&y)
        );
    }
}

fn demo_big_exponent_float_positive_difference_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.positive_difference_rational_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.positive_difference_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_positive_difference_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.positive_difference_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.positive_difference_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_positive_difference_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.positive_difference_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.positive_difference_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.positive_difference_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.positive_difference_rational_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.positive_difference_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.positive_difference_rational_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_positive_difference_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.positive_difference_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_7_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.positive_difference_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.positive_difference_rational_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_positive_difference_big_exponent_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) =
            BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(
                &x, &y, prec, rm,
            );
        println!(
            "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round({x}, {y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_rational_positive_difference_big_exponent_float_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::rational_positive_difference_big_exponent_float_prec_ref_ref(
            &x, &y, prec,
        );
        println!(
            "BigExponentFloat::rational_positive_difference_big_exponent_float_prec({x}, {y:#x}, {prec}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_rational_positive_difference_big_exponent_float_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_val_ref(x, &y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(&Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_val(&x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(&Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(&x, &y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_positive_difference_big_exponent_float_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_8_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(BigExponentFloat::rational_positive_difference_big_exponent_float_prec_round_ref_ref(&x, &y, prec, rm));
            }),
            ("Float", &mut |((y, x, prec, rm), _)| {
                no_out!(
                    Float::rational_positive_difference_float_prec_round_ref_ref(&x, &y, prec, rm)
                );
            }),
        ],
    );
}
