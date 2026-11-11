// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::generators::rational_unsigned_rounding_mode_triple_gen_var_1;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_from_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_from_rational_prec);
    register_demo!(runner, demo_big_exponent_float_try_from_rational);
    register_demo!(runner, demo_big_exponent_float_convertible_from_rational);

    register_bench!(
        runner,
        benchmark_big_exponent_float_from_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_rational_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_from_rational_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_try_from_rational_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_convertible_from_rational
    );
}

fn demo_big_exponent_float_from_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (q, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_rational_prec_round_ref(&q, prec, rm);
        println!("BigExponentFloat::from_rational_prec_round({q}, {prec}, {rm}) = ({x:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_from_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (q, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_rational_prec_ref(&q, prec);
        println!("BigExponentFloat::from_rational_prec({q}, {prec}) = ({x:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_try_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for q in rational_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::try_from({q}) = {:?}",
            BigExponentFloat::try_from(&q)
        );
    }
}

fn demo_big_exponent_float_convertible_from_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for q in rational_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::convertible_from({q}) = {}",
            BigExponentFloat::convertible_from(&q)
        );
    }
}

fn benchmark_big_exponent_float_from_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("q", "prec"),
        &mut [
            ("by value", &mut |(q, prec, rm)| {
                no_out!(BigExponentFloat::from_rational_prec_round(q, prec, rm));
            }),
            ("by reference", &mut |(q, prec, rm)| {
                no_out!(BigExponentFloat::from_rational_prec_round_ref(&q, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_rational_prec_round_ref(&Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        rational_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("q", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(q, prec, rm)| {
                no_out!(BigExponentFloat::from_rational_prec_round_ref(&q, prec, rm));
            }),
            ("Float", &mut |(q, prec, rm)| {
                no_out!(Float::from_rational_prec_round_ref(&q, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_rational_prec(Rational, u64)",
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("q", "prec"),
        &mut [("Malachite", &mut |(q, prec)| {
            no_out!(BigExponentFloat::from_rational_prec(q, prec));
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_big_exponent_float_try_from_rational_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::try_from(Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("q"),
        &mut [
            ("BigExponentFloat::try_from(Rational)", &mut |q| {
                no_out!(BigExponentFloat::try_from(q));
            }),
            ("BigExponentFloat::try_from(&Rational)", &mut |q| {
                no_out!(BigExponentFloat::try_from(&q));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_convertible_from_rational(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::convertible_from(&Rational)",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("q"),
        &mut [("Malachite", &mut |q| {
            no_out!(BigExponentFloat::convertible_from(&q));
        })],
    );
}
