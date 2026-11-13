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
    register_demo!(runner, demo_big_exponent_float_root_u_prec_round);
    register_demo!(runner, demo_big_exponent_float_root_u_prec);
    register_demo!(runner, demo_big_exponent_float_root_s_prec_round);
    register_demo!(runner, demo_big_exponent_float_root_s_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_root_u_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_u_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_u_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_u_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_s_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_s_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_s_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_root_s_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_root_u_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, rm) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.root_u_prec_round_ref(k, prec, rm);
        println!("{x:#x}.root_u_prec_round({k}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_root_u_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec) in big_exponent_float_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.root_u_prec_ref(k, prec);
        println!("{x:#x}.root_u_prec({k}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_root_u_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.root_u_prec_round(u64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(x.root_u_prec_round(k, prec, rm));
                },
            ),
            (
                "BigExponentFloat.root_u_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(x.root_u_prec_round_ref(k, prec, rm));
                },
            ),
            (
                "BigExponentFloat.root_u_prec_round_assign(u64, u64, RoundingMode)",
                &mut |(mut x, k, prec, rm)| {
                    no_out!(x.root_u_prec_round_assign(k, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_root_u_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_u_prec(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.root_u_prec(u64, u64)",
                &mut |(x, k, prec)| {
                    no_out!(x.root_u_prec(k, prec));
                },
            ),
            (
                "BigExponentFloat.root_u_prec_ref(u64, u64)",
                &mut |(x, k, prec)| {
                    no_out!(x.root_u_prec_ref(k, prec));
                },
            ),
            (
                "BigExponentFloat.root_u_prec_assign(u64, u64)",
                &mut |(mut x, k, prec)| {
                    no_out!(x.root_u_prec_assign(k, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_root_u_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("Float kernels", &mut |(x, k, prec, rm)| {
                no_out!(x.root_u_prec_round_ref(k, prec, rm));
            }),
            ("generic", &mut |(x, k, prec, rm)| {
                no_out!(root_prec_round_generic(&x, i128::from(k), prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_root_u_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, k, prec, rm))| {
                no_out!(x.root_u_prec_round_ref(k, prec, rm));
            }),
            ("Float", &mut |((x, k, prec, rm), _)| {
                no_out!(x.root_u_prec_round_ref(k, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_root_s_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, rm) in big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.root_s_prec_round_ref(k, prec, rm);
        println!("{x:#x}.root_s_prec_round({k}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_root_s_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec) in big_exponent_float_signed_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.root_s_prec_ref(k, prec);
        println!("{x:#x}.root_s_prec({k}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_root_s_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.root_s_prec_round(i64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(x.root_s_prec_round(k, prec, rm));
                },
            ),
            (
                "BigExponentFloat.root_s_prec_round_ref(i64, u64, RoundingMode)",
                &mut |(x, k, prec, rm)| {
                    no_out!(x.root_s_prec_round_ref(k, prec, rm));
                },
            ),
            (
                "BigExponentFloat.root_s_prec_round_assign(i64, u64, RoundingMode)",
                &mut |(mut x, k, prec, rm)| {
                    no_out!(x.root_s_prec_round_assign(k, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_root_s_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_s_prec(i64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.root_s_prec(i64, u64)",
                &mut |(x, k, prec)| {
                    no_out!(x.root_s_prec(k, prec));
                },
            ),
            (
                "BigExponentFloat.root_s_prec_ref(i64, u64)",
                &mut |(x, k, prec)| {
                    no_out!(x.root_s_prec_ref(k, prec));
                },
            ),
            (
                "BigExponentFloat.root_s_prec_assign(i64, u64)",
                &mut |(mut x, k, prec)| {
                    no_out!(x.root_s_prec_assign(k, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_root_s_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("Float kernels", &mut |(x, k, prec, rm)| {
                no_out!(x.root_s_prec_round_ref(k, prec, rm));
            }),
            ("generic", &mut |(x, k, prec, rm)| {
                no_out!(root_prec_round_generic(&x, i128::from(k), prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_root_s_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.root_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_1_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, k, prec, rm))| {
                no_out!(x.root_s_prec_round_ref(k, prec, rm));
            }),
            ("Float", &mut |((x, k, prec, rm), _)| {
                no_out!(x.root_s_prec_round_ref(k, prec, rm));
            }),
        ],
    );
}
