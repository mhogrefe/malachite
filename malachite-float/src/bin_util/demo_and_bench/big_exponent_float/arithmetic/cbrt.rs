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
use malachite_float::big_exponent_float::arithmetic::root::root_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_cbrt_prec_round);
    register_demo!(runner, demo_big_exponent_float_cbrt_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_cbrt_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cbrt_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cbrt_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cbrt_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_cbrt_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cbrt_prec_round_ref(prec, rm);
        println!("{x:#x}.cbrt_prec_round({prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cbrt_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cbrt_prec_ref(prec);
        println!("{x:#x}.cbrt_prec({prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_cbrt_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cbrt_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.cbrt_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cbrt_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.cbrt_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cbrt_prec_round_ref(prec, rm));
                },
            ),
            (
                "BigExponentFloat.cbrt_prec_round_assign(u64, RoundingMode)",
                &mut |(mut x, prec, rm)| {
                    no_out!(x.cbrt_prec_round_assign(prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cbrt_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cbrt_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_pair_1_big_exponent_float_complexity_pair_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat.cbrt_prec(u64)", &mut |(x, prec)| {
                no_out!(x.cbrt_prec(prec));
            }),
            ("BigExponentFloat.cbrt_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.cbrt_prec_ref(prec));
            }),
            (
                "BigExponentFloat.cbrt_prec_assign(u64)",
                &mut |(mut x, prec)| {
                    no_out!(x.cbrt_prec_assign(prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cbrt_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cbrt_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("Float kernels", &mut |(x, prec, rm)| {
                no_out!(x.cbrt_prec_round_ref(prec, rm));
            }),
            ("generic", &mut |(x, prec, rm)| {
                no_out!(root_prec_round_generic(&x, 3, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_cbrt_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cbrt_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_6_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, prec, rm))| {
                no_out!(x.cbrt_prec_round_ref(prec, rm));
            }),
            ("Float", &mut |((x, prec, rm), _)| {
                no_out!(x.cbrt_prec_round_ref(prec, rm));
            }),
        ],
    );
}
