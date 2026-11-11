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
use malachite_float::test_util::generators::integer_unsigned_rounding_mode_triple_gen_var_3;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, pair_integer_bit_u64_max_bucketer,
    triple_1_2_integer_bit_u64_max_bucketer,
};
use malachite_nz::test_util::generators::{integer_gen, integer_unsigned_pair_gen_var_6};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_from_integer);
    register_demo!(runner, demo_big_exponent_float_from_integer_prec_round);
    register_demo!(runner, demo_big_exponent_float_from_integer_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_from_integer_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_integer_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_integer_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_from_integer_prec);
}

fn demo_big_exponent_float_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({n}) = {:#x}",
            BigExponentFloat::from(&n)
        );
    }
}

fn demo_big_exponent_float_from_integer_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, prec, rm) in integer_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_integer_prec_round_ref(&n, prec, rm);
        println!("BigExponentFloat::from_integer_prec_round({n}, {prec}, {rm}) = ({x:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_from_integer_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, prec) in integer_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_integer_prec_ref(&n, prec);
        println!("BigExponentFloat::from_integer_prec({n}, {prec}) = ({x:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_from_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("BigExponentFloat::from(Integer)", &mut |n| {
                no_out!(BigExponentFloat::from(n));
            }),
            ("BigExponentFloat::from(&Integer)", &mut |n| {
                no_out!(BigExponentFloat::from(&n));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_integer_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("by value", &mut |(n, prec, rm)| {
                no_out!(BigExponentFloat::from_integer_prec_round(n, prec, rm));
            }),
            ("by reference", &mut |(n, prec, rm)| {
                no_out!(BigExponentFloat::from_integer_prec_round_ref(&n, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_integer_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_integer_prec_round_ref(&Integer, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(n, prec, rm)| {
                no_out!(BigExponentFloat::from_integer_prec_round_ref(&n, prec, rm));
            }),
            ("Float", &mut |(n, prec, rm)| {
                no_out!(Float::from_integer_prec_round_ref(&n, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_integer_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_integer_prec(Integer, u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [("Malachite", &mut |(n, prec)| {
            no_out!(BigExponentFloat::from_integer_prec(n, prec));
        })],
    );
}
