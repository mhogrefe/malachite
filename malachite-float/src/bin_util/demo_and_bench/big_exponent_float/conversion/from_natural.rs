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
use malachite_float::test_util::generators::natural_unsigned_rounding_mode_triple_gen_var_2;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_natural_bit_u64_max_bucketer,
    triple_1_2_natural_bit_u64_max_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_7};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_from_natural);
    register_demo!(runner, demo_big_exponent_float_from_natural_prec_round);
    register_demo!(runner, demo_big_exponent_float_from_natural_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_from_natural_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_natural_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_natural_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_from_natural_prec);
}

fn demo_big_exponent_float_from_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from({n}) = {:#x}",
            BigExponentFloat::from(&n)
        );
    }
}

fn demo_big_exponent_float_from_natural_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, prec, rm) in natural_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_natural_prec_round_ref(&n, prec, rm);
        println!("BigExponentFloat::from_natural_prec_round({n}, {prec}, {rm}) = ({x:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_from_natural_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, prec) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::from_natural_prec_ref(&n, prec);
        println!("BigExponentFloat::from_natural_prec({n}, {prec}) = ({x:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_from_natural_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("BigExponentFloat::from(Natural)", &mut |n| {
                no_out!(BigExponentFloat::from(n));
            }),
            ("BigExponentFloat::from(&Natural)", &mut |n| {
                no_out!(BigExponentFloat::from(&n));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_natural_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_natural_prec_round(Natural, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("by value", &mut |(n, prec, rm)| {
                no_out!(BigExponentFloat::from_natural_prec_round(n, prec, rm));
            }),
            ("by reference", &mut |(n, prec, rm)| {
                no_out!(BigExponentFloat::from_natural_prec_round_ref(&n, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_natural_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_natural_prec_round_ref(&Natural, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(n, prec, rm)| {
                no_out!(BigExponentFloat::from_natural_prec_round_ref(&n, prec, rm));
            }),
            ("Float", &mut |(n, prec, rm)| {
                no_out!(Float::from_natural_prec_round_ref(&n, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_natural_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_natural_prec(Natural, u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [("Malachite", &mut |(n, prec)| {
            no_out!(BigExponentFloat::from_natural_prec(n, prec));
        })],
    );
}
