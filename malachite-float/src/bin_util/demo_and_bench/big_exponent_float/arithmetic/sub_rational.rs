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
use malachite_float::big_exponent_float::arithmetic::sub_rational::sub_rational_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_sub_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_sub_rational_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_rational_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_rational_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_sub_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.sub_rational_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.sub_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_sub_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.sub_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.sub_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_sub_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.sub_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.sub_rational_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sub_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.sub_rational_prec(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_rational_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_val_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_rational_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_ref_val(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_rational_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_ref_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_rational_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_assign(Rational, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.sub_rational_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_rational_prec_assign_ref(&Rational, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.sub_rational_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sub_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float kernels", &mut |(x, y, prec, rm)| {
                no_out!(x.sub_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(sub_rational_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_sub_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_3_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.sub_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.sub_rational_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}
