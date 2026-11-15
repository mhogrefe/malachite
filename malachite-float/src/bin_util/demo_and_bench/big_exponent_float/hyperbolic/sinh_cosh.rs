// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::NaN;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_q::test_util::bench::bucketers::triple_1_2_rational_bit_u64_max_bucketer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_sinh_cosh_prec_round);
    register_demo!(runner, demo_big_exponent_float_sinh_cosh_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_sinh_cosh_rational_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_sinh_cosh_rational_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_sinh_cosh_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sinh_cosh_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sinh_cosh_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sinh_cosh_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sinh_cosh_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_sinh_cosh_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (s, c, o_s, o_c) = x.sinh_cosh_prec_round_ref(prec, rm);
        println!(
            "{x:#x}.sinh_cosh_prec_round({prec}, {rm}) = \
            ({s:#x}, {c:#x}, {o_s:?}, {o_c:?})"
        );
    }
}

fn demo_big_exponent_float_sinh_cosh_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, _) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (s, c, o_s, o_c) = x.sinh_cosh_prec_ref(prec);
        println!("{x:#x}.sinh_cosh_prec({prec}) = ({s:#x}, {c:#x}, {o_s:?}, {o_c:?})");
    }
}

fn demo_big_exponent_float_sinh_cosh_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(&x, prec, rm);
        println!(
            "BigExponentFloat::sinh_cosh_rational_prec_round({x}, {prec}, {rm}) = \
            ({s:#x}, {c:#x}, {o_s:?}, {o_c:?})"
        );
    }
}

fn demo_big_exponent_float_sinh_cosh_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, _) in rational_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_ref(&x, prec);
        println!(
            "BigExponentFloat::sinh_cosh_rational_prec({x}, {prec}) = \
            ({s:#x}, {c:#x}, {o_s:?}, {o_c:?})"
        );
    }
}

fn benchmark_big_exponent_float_sinh_cosh_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sinh_cosh_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.sinh_cosh_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.sinh_cosh_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.sinh_cosh_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.sinh_cosh_prec_round_ref(prec, rm));
                },
            ),
            (
                "BigExponentFloat.sinh_cosh_prec_round_assign(&mut BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, prec, rm)| {
                    let mut c = BigExponentFloat::NAN;
                    no_out!(x.sinh_cosh_prec_round_assign(&mut c, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sinh_cosh_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sinh_cosh_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat.sinh_cosh_prec(u64)", &mut |(
                x,
                prec,
                _,
            )| {
                no_out!(x.sinh_cosh_prec(prec));
            }),
            (
                "BigExponentFloat.sinh_cosh_prec_ref(u64)",
                &mut |(x, prec, _)| {
                    no_out!(x.sinh_cosh_prec_ref(prec));
                },
            ),
            (
                "BigExponentFloat.sinh_cosh_prec_assign(&mut BigExponentFloat, u64)",
                &mut |(mut x, prec, _)| {
                    let mut c = BigExponentFloat::NAN;
                    no_out!(x.sinh_cosh_prec_assign(&mut c, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sinh_cosh_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::sinh_cosh_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::sinh_cosh_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::sinh_cosh_rational_prec_round(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat::sinh_cosh_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::sinh_cosh_rational_prec_round_ref(
                        &x, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sinh_cosh_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::sinh_cosh_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::sinh_cosh_rational_prec(Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::sinh_cosh_rational_prec(x, prec));
                },
            ),
            (
                "BigExponentFloat::sinh_cosh_rational_prec_ref(&Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::sinh_cosh_rational_prec_ref(&x, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sinh_cosh_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sinh_cosh_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_9_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, prec, rm))| {
                no_out!(x.sinh_cosh_prec_round_ref(prec, rm));
            }),
            ("Float", &mut |((x, prec, rm), _)| {
                no_out!(x.sinh_cosh_prec_round_ref(prec, rm));
            }),
        ],
    );
}
