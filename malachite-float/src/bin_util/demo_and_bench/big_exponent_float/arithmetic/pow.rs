// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::big_exponent_float::arithmetic::pow::{
    pow_prec_round_float_kernels, pow_prec_round_generic, pow_rational_prec_round_generic,
    rational_pow_prec_round_generic, rational_pow_rational_prec_round_generic,
};
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_pow_u_prec_round);
    register_demo!(runner, demo_big_exponent_float_pow_u_prec);
    register_demo!(runner, demo_big_exponent_float_pow_s_prec_round);
    register_demo!(runner, demo_big_exponent_float_pow_s_prec);
    register_demo!(runner, demo_big_exponent_float_pow_integer_prec_round);
    register_demo!(runner, demo_big_exponent_float_pow_integer_prec);
    register_demo!(runner, demo_big_exponent_float_pow);

    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_u_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_u_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_u_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_u_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_s_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_s_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_s_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_s_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_integer_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_integer_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_integer_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_integer_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_pow_evaluation_strategy);
    register_demo!(runner, demo_big_exponent_float_pow_prec_round);
    register_demo!(runner, demo_big_exponent_float_pow_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_prec_round_float_comparison
    );
    register_demo!(runner, demo_big_exponent_float_powr_prec_round);
    register_demo!(runner, demo_big_exponent_float_powr_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_powr_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_powr_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_powr_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_powr_prec_round_float_comparison
    );
    register_demo!(runner, demo_big_exponent_float_pow_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_pow_rational_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_rational_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_pow_rational_prec_round_float_comparison
    );
    register_demo!(runner, demo_big_exponent_float_rational_pow_prec_round);
    register_demo!(runner, demo_big_exponent_float_rational_pow_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_prec_round_float_comparison
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_pow_rational_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_rational_pow_rational_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_pow_rational_prec_round_algorithms
    );
    register_demo!(runner, demo_big_exponent_float_unsigned_pow_prec_round);
    register_demo!(runner, demo_big_exponent_float_unsigned_pow_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_unsigned_pow_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_unsigned_pow_prec_evaluation_strategy
    );
    register_demo!(
        runner,
        demo_big_exponent_float_unsigned_pow_rational_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_unsigned_pow_rational_prec);
    register_bench!(
        runner,
        benchmark_big_exponent_float_unsigned_pow_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_unsigned_pow_rational_prec_evaluation_strategy
    );
    register_demo!(
        runner,
        demo_big_exponent_float_unsigned_pow_unsigned_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_unsigned_pow_unsigned_prec);
}

fn demo_big_exponent_float_pow_u_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.pow_u_prec_round_ref(n, prec, rm);
        println!("{x:#x}.pow_u_prec_round({n}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_pow_u_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in big_exponent_float_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.pow_u_prec_ref(n, prec);
        println!("{x:#x}.pow_u_prec({n}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_pow_u_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_u_prec_round(u64, u64, RoundingMode)",
                &mut |(x, n, prec, rm)| {
                    no_out!(x.pow_u_prec_round(n, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_u_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, n, prec, rm)| {
                    no_out!(x.pow_u_prec_round_ref(n, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_u_prec_round_assign(u64, u64, RoundingMode)",
                &mut |(mut x, n, prec, rm)| {
                    no_out!(x.pow_u_prec_round_assign(n, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_u_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_u_prec(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_u_prec(u64, u64)",
                &mut |(x, n, prec)| {
                    no_out!(x.pow_u_prec(n, prec));
                },
            ),
            (
                "BigExponentFloat.pow_u_prec_ref(u64, u64)",
                &mut |(x, n, prec)| {
                    no_out!(x.pow_u_prec_ref(n, prec));
                },
            ),
            (
                "BigExponentFloat.pow_u_prec_assign(u64, u64)",
                &mut |(mut x, n, prec)| {
                    no_out!(x.pow_u_prec_assign(n, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_u_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("binary exponentiation", &mut |(x, n, prec, rm)| {
                no_out!(x.pow_u_prec_round_ref(n, prec, rm));
            }),
            ("Float kernels", &mut |(x, n, prec, rm)| {
                no_out!(pow_prec_round_float_kernels(
                    &x,
                    &Integer::from(n),
                    prec,
                    rm
                ));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_pow_u_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, n, prec, rm))| {
                no_out!(x.pow_u_prec_round_ref(n, prec, rm));
            }),
            ("Float", &mut |((x, n, prec, rm), _)| {
                no_out!(x.pow_u_prec_round_ref(n, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_pow_s_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.pow_s_prec_round_ref(n, prec, rm);
        println!("{x:#x}.pow_s_prec_round({n}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_pow_s_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in big_exponent_float_signed_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.pow_s_prec_ref(n, prec);
        println!("{x:#x}.pow_s_prec({n}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_pow_s_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_s_prec_round(i64, u64, RoundingMode)",
                &mut |(x, n, prec, rm)| {
                    no_out!(x.pow_s_prec_round(n, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_s_prec_round_ref(i64, u64, RoundingMode)",
                &mut |(x, n, prec, rm)| {
                    no_out!(x.pow_s_prec_round_ref(n, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_s_prec_round_assign(i64, u64, RoundingMode)",
                &mut |(mut x, n, prec, rm)| {
                    no_out!(x.pow_s_prec_round_assign(n, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_s_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_s_prec(i64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_s_prec(i64, u64)",
                &mut |(x, n, prec)| {
                    no_out!(x.pow_s_prec(n, prec));
                },
            ),
            (
                "BigExponentFloat.pow_s_prec_ref(i64, u64)",
                &mut |(x, n, prec)| {
                    no_out!(x.pow_s_prec_ref(n, prec));
                },
            ),
            (
                "BigExponentFloat.pow_s_prec_assign(i64, u64)",
                &mut |(mut x, n, prec)| {
                    no_out!(x.pow_s_prec_assign(n, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_s_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("binary exponentiation", &mut |(x, n, prec, rm)| {
                no_out!(x.pow_s_prec_round_ref(n, prec, rm));
            }),
            ("Float kernels", &mut |(x, n, prec, rm)| {
                no_out!(pow_prec_round_float_kernels(
                    &x,
                    &Integer::from(n),
                    prec,
                    rm
                ));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_pow_s_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_2_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, n, prec, rm))| {
                no_out!(x.pow_s_prec_round_ref(n, prec, rm));
            }),
            ("Float", &mut |((x, n, prec, rm), _)| {
                no_out!(x.pow_s_prec_round_ref(n, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_pow_integer_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.pow_integer_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.pow_integer_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_pow_integer_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_integer_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.pow_integer_prec_ref_ref(&y, prec);
        println!("{x:#x}.pow_integer_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_pow_integer_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_integer_prec_round(Integer, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_integer_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_round_val_ref(&Integer, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_integer_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_round_ref_val(Integer, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_integer_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_round_ref_ref(&Integer, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_integer_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_round_assign(Integer, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.pow_integer_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_round_assign_ref(&Integer, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.pow_integer_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_integer_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_integer_prec(Integer, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_integer_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_integer_prec(Integer, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.pow_integer_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_val_ref(&Integer, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.pow_integer_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_ref_val(Integer, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.pow_integer_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_ref_ref(&Integer, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.pow_integer_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_assign(Integer, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.pow_integer_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_integer_prec_assign_ref(&Integer, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.pow_integer_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_integer_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("binary exponentiation", &mut |(x, y, prec, rm)| {
                no_out!(x.pow_integer_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float kernels", &mut |(x, y, prec, rm)| {
                no_out!(pow_prec_round_float_kernels(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_pow_integer_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_integer_unsigned_rounding_mode_quadruple_gen_var_1_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.pow_integer_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.pow_integer_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in big_exponent_float_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{x:#x}.pow({n}) = {:#x}", (&x).pow(n));
    }
}

fn benchmark_big_exponent_float_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat.pow(u64)", &mut |(x, n)| {
                no_out!(x.pow(n));
            }),
            ("(&BigExponentFloat).pow(u64)", &mut |(x, n)| {
                no_out!((&x).pow(n));
            }),
            ("BigExponentFloat.pow_assign(u64)", &mut |(mut x, n)| {
                no_out!(x.pow_assign(n));
            }),
        ],
    );
}

fn demo_big_exponent_float_pow_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = x.pow_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.pow_prec_round({y:#x}, {prec}, {rm}) = ({r:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_pow_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, _) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = x.pow_prec_ref_ref(&y, prec);
        println!("{x:#x}.pow_prec({y:#x}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_pow_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).pow_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).pow_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.pow_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.pow_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_prec_val_ref(&y, prec));
                },
            ),
            (
                "(&BigExponentFloat).pow_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_prec_ref_val(y, prec));
                },
            ),
            (
                "(&BigExponentFloat).pow_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_prec_assign(BigExponentFloat, u64)",
                &mut |(mut x, y, prec, _)| {
                    no_out!(x.pow_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_prec_assign_ref(&BigExponentFloat, u64)",
                &mut |(mut x, y, prec, _)| {
                    no_out!(x.pow_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float functions", &mut |(x, y, prec, rm)| {
                no_out!(x.pow_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(pow_prec_round_generic(&x, &y, false, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_pow_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_18_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.pow_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((xf, yf, prec, rm), _)| {
                no_out!(xf.pow_prec_round_ref_ref(&yf, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_powr_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = x.powr_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.powr_prec_round({y:#x}, {prec}, {rm}) = ({r:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_powr_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, _) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = x.powr_prec_ref_ref(&y, prec);
        println!("{x:#x}.powr_prec({y:#x}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_powr_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.powr_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.powr_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.powr_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.powr_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.powr_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).powr_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.powr_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).powr_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.powr_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.powr_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.powr_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.powr_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.powr_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_powr_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.powr_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.powr_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.powr_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.powr_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.powr_prec_val_ref(&y, prec));
                },
            ),
            (
                "(&BigExponentFloat).powr_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.powr_prec_ref_val(y, prec));
                },
            ),
            (
                "(&BigExponentFloat).powr_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.powr_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.powr_prec_assign(BigExponentFloat, u64)",
                &mut |(mut x, y, prec, _)| {
                    no_out!(x.powr_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.powr_prec_assign_ref(&BigExponentFloat, u64)",
                &mut |(mut x, y, prec, _)| {
                    no_out!(x.powr_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_powr_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.powr_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float functions", &mut |(x, y, prec, rm)| {
                no_out!(x.powr_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(pow_prec_round_generic(&x, &y, true, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_powr_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.powr_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_19_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.powr_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((xf, yf, prec, rm), _)| {
                no_out!(xf.powr_prec_round_ref_ref(&yf, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_pow_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = x.pow_rational_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.pow_rational_prec_round({y}, {prec}, {rm}) = ({r:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_pow_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, _) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = x.pow_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.pow_rational_prec({y}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_pow_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).pow_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).pow_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.pow_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.pow_rational_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.pow_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.pow_rational_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.pow_rational_prec(Rational, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_rational_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_rational_prec_val_ref(&Rational, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_rational_prec_val_ref(&y, prec));
                },
            ),
            (
                "(&BigExponentFloat).pow_rational_prec_ref_val(Rational, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_rational_prec_ref_val(y, prec));
                },
            ),
            (
                "(&BigExponentFloat).pow_rational_prec_ref_ref(&Rational, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(x.pow_rational_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_rational_prec_assign(Rational, u64)",
                &mut |(mut x, y, prec, _)| {
                    no_out!(x.pow_rational_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.pow_rational_prec_assign_ref(&Rational, u64)",
                &mut |(mut x, y, prec, _)| {
                    no_out!(x.pow_rational_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_pow_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float functions", &mut |(x, y, prec, rm)| {
                no_out!(x.pow_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(pow_rational_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_pow_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.pow_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_21_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.pow_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((xf, y, prec, rm), _)| {
                no_out!(xf.pow_rational_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_pow_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = BigExponentFloat::rational_pow_prec_round_ref_ref(&x, &y, prec, rm);
        println!(
            "BigExponentFloat::rational_pow_prec_round({x}, {y:#x}, {prec}, {rm}) = ({r:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_rational_pow_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, _) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::rational_pow_prec_ref_ref(&x, &y, prec);
        println!("BigExponentFloat::rational_pow_prec({x}, {y:#x}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_rational_pow_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_pow_prec_round(Rational, BigExponentFloat, u64, \
                RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_pow_prec_round(x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_pow_prec_round_val_ref(Rational, &BigExponentFloat, \
                u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_pow_prec_round_val_ref(
                        x, &y, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::rational_pow_prec_round_ref_val(&Rational, BigExponentFloat, \
                u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_pow_prec_round_ref_val(
                        &x, y, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::rational_pow_prec_round_ref_ref(&Rational, &BigExponentFloat, \
                u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_pow_prec_round_ref_ref(
                        &x, &y, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_pow_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_prec(Rational, BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_pow_prec(Rational, BigExponentFloat, u64)",
                &mut |(y, x, prec, _)| {
                    no_out!(BigExponentFloat::rational_pow_prec(x, y, prec));
                },
            ),
            (
                "BigExponentFloat::rational_pow_prec_val_ref(Rational, &BigExponentFloat, u64)",
                &mut |(y, x, prec, _)| {
                    no_out!(BigExponentFloat::rational_pow_prec_val_ref(x, &y, prec));
                },
            ),
            (
                "BigExponentFloat::rational_pow_prec_ref_val(&Rational, BigExponentFloat, u64)",
                &mut |(y, x, prec, _)| {
                    no_out!(BigExponentFloat::rational_pow_prec_ref_val(&x, y, prec));
                },
            ),
            (
                "BigExponentFloat::rational_pow_prec_ref_ref(&Rational, &BigExponentFloat, u64)",
                &mut |(y, x, prec, _)| {
                    no_out!(BigExponentFloat::rational_pow_prec_ref_ref(&x, &y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_pow_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            ("Float functions", &mut |(y, x, prec, rm)| {
                no_out!(BigExponentFloat::rational_pow_prec_round_ref_ref(
                    &x, &y, prec, rm
                ));
            }),
            ("generic", &mut |(y, x, prec, rm)| {
                no_out!(rational_pow_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_rational_pow_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_22_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "y", "x", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(BigExponentFloat::rational_pow_prec_round_ref_ref(
                    &x, &y, prec, rm
                ));
            }),
            ("Float", &mut |((yf, x, prec, rm), _)| {
                no_out!(Float::rational_pow_prec_round_ref_ref(&x, &yf, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_pow_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::rational_pow_rational_prec_round_ref(&x, &y, prec, rm);
        println!(
            "BigExponentFloat::rational_pow_rational_prec_round({x}, {y}, {prec}, {rm}) = ({r:#x}, \
            {o:?})"
        );
    }
}

fn demo_big_exponent_float_rational_pow_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::rational_pow_rational_prec_ref(&x, &y, prec);
        println!(
            "BigExponentFloat::rational_pow_rational_prec({x}, {y}, {prec}) = ({r:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_rational_pow_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_rational_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat::rational_pow_rational_prec_round(Rational, Rational, u64, \
                RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(BigExponentFloat::rational_pow_rational_prec_round(
                        x, y, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::rational_pow_rational_prec_round_ref(&Rational, &Rational, u64, \
                RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(BigExponentFloat::rational_pow_rational_prec_round_ref(
                        &x, &y, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_pow_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_rational_prec(Rational, Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat::rational_pow_rational_prec(Rational, Rational, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(BigExponentFloat::rational_pow_rational_prec(x, y, prec));
                },
            ),
            (
                "BigExponentFloat::rational_pow_rational_prec_ref(&Rational, &Rational, u64)",
                &mut |(x, y, prec, _)| {
                    no_out!(BigExponentFloat::rational_pow_rational_prec_ref(
                        &x, &y, prec
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_pow_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_pow_rational_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            ("Float functions", &mut |(x, y, prec, rm)| {
                no_out!(BigExponentFloat::rational_pow_rational_prec_round_ref(
                    &x, &y, prec, rm
                ));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(rational_pow_rational_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_unsigned_pow_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let y = float_to_big_exponent_float(&y);
        let (r, o) = BigExponentFloat::unsigned_pow_prec_round_ref(x, &y, prec, rm);
        println!(
            "BigExponentFloat::unsigned_pow_prec_round({x}, {y:#x}, {prec}, {rm}) = ({r:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_unsigned_pow_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let y = float_to_big_exponent_float(&y);
        let (r, o) = BigExponentFloat::unsigned_pow_prec_ref(x, &y, prec);
        println!("BigExponentFloat::unsigned_pow_prec({x}, {y:#x}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_unsigned_pow_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::unsigned_pow_prec_round(u64, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat::unsigned_pow_prec_round(u64, BigExponentFloat, u64, \
                RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    let y = float_to_big_exponent_float(&y);
                    no_out!(BigExponentFloat::unsigned_pow_prec_round(x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::unsigned_pow_prec_round_ref(u64, &BigExponentFloat, u64, \
                RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    let y = float_to_big_exponent_float(&y);
                    no_out!(BigExponentFloat::unsigned_pow_prec_round_ref(
                        x, &y, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_unsigned_pow_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::unsigned_pow_prec(u64, BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat::unsigned_pow_prec(u64, BigExponentFloat, u64)",
                &mut |(y, x, prec, _)| {
                    let y = float_to_big_exponent_float(&y);
                    no_out!(BigExponentFloat::unsigned_pow_prec(x, y, prec));
                },
            ),
            (
                "BigExponentFloat::unsigned_pow_prec_ref(u64, &BigExponentFloat, u64)",
                &mut |(y, x, prec, _)| {
                    let y = float_to_big_exponent_float(&y);
                    no_out!(BigExponentFloat::unsigned_pow_prec_ref(x, &y, prec));
                },
            ),
        ],
    );
}

fn demo_big_exponent_float_unsigned_pow_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::unsigned_pow_rational_prec_round_ref(x, &y, prec, rm);
        println!(
            "BigExponentFloat::unsigned_pow_rational_prec_round({x}, {y}, {prec}, {rm}) = ({r:#x}, \
            {o:?})"
        );
    }
}

fn demo_big_exponent_float_unsigned_pow_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::unsigned_pow_rational_prec_ref(x, &y, prec);
        println!(
            "BigExponentFloat::unsigned_pow_rational_prec({x}, {y}, {prec}) = ({r:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_unsigned_pow_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::unsigned_pow_rational_prec_round(u64, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat::unsigned_pow_rational_prec_round(u64, Rational, u64, \
                RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::unsigned_pow_rational_prec_round(
                        x, y, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::unsigned_pow_rational_prec_round_ref(u64, &Rational, u64, \
                RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::unsigned_pow_rational_prec_round_ref(
                        x, &y, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_unsigned_pow_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::unsigned_pow_rational_prec(u64, Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat::unsigned_pow_rational_prec(u64, Rational, u64)",
                &mut |(y, x, prec, _)| {
                    no_out!(BigExponentFloat::unsigned_pow_rational_prec(x, y, prec));
                },
            ),
            (
                "BigExponentFloat::unsigned_pow_rational_prec_ref(u64, &Rational, u64)",
                &mut |(y, x, prec, _)| {
                    no_out!(BigExponentFloat::unsigned_pow_rational_prec_ref(
                        x, &y, prec
                    ));
                },
            ),
        ],
    );
}

fn demo_big_exponent_float_unsigned_pow_unsigned_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::unsigned_pow_unsigned_prec_round(x, y, prec, rm);
        println!(
            "BigExponentFloat::unsigned_pow_unsigned_prec_round({x}, {y}, {prec}, {rm}) = ({r:#x}, \
            {o:?})"
        );
    }
}

fn demo_big_exponent_float_unsigned_pow_unsigned_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, _) in unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::unsigned_pow_unsigned_prec(x, y, prec);
        println!(
            "BigExponentFloat::unsigned_pow_unsigned_prec({x}, {y}, {prec}) = ({r:#x}, {o:?})"
        );
    }
}
