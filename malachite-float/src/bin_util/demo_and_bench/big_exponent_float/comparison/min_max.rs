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
use malachite_float::test_util::bench::bucketers::{
    pair_big_exponent_float_max_complexity_bucketer,
    quadruple_1_2_3_big_exponent_float_max_complexity_bucketer,
    quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer,
    triple_1_2_3_big_exponent_float_max_complexity_bucketer,
    triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_1,
    big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1, big_exponent_float_pair_gen,
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
    big_exponent_float_rational_unsigned_triple_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_min);
    register_demo!(runner, demo_big_exponent_float_min_prec_round);
    register_demo!(runner, demo_big_exponent_float_min_prec);
    register_demo!(runner, demo_big_exponent_float_min_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_min_rational_prec);
    register_demo!(runner, demo_big_exponent_float_max);
    register_demo!(runner, demo_big_exponent_float_max_prec_round);
    register_demo!(runner, demo_big_exponent_float_max_prec);
    register_demo!(runner, demo_big_exponent_float_max_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_max_rational_prec);

    register_bench!(runner, benchmark_big_exponent_float_min_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_big_exponent_float_min_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_min_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_min_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_min_rational_prec_evaluation_strategy
    );
    register_bench!(runner, benchmark_big_exponent_float_max_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_big_exponent_float_max_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_max_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_max_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_max_rational_prec_evaluation_strategy
    );
}

fn demo_big_exponent_float_min(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        println!("{x:#x}.min({y:#x}) = {:#x}", x.min_ref_ref(&y));
    }
}

fn demo_big_exponent_float_min_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.min_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.min_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_min_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.min_prec_ref_ref(&y, prec);
        println!("{x:#x}.min_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_min_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.min_rational_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.min_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_min_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.min_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.min_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_min_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.min(BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat.min(BigExponentFloat)", &mut |(x, y)| {
                no_out!(x.min(y));
            }),
            (
                "BigExponentFloat.min_val_ref(&BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.min_val_ref(&y));
                },
            ),
            (
                "BigExponentFloat.min_ref_val(BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.min_ref_val(y));
                },
            ),
            (
                "BigExponentFloat.min_ref_ref(&BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.min_ref_ref(&y));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_min_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.min_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_1()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.min_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.min_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.min_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.min_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_min_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.min_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.min_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.min_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.min_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.min_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_prec_ref_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_min_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.min_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.min_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.min_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.min_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.min_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_min_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.min_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.min_rational_prec(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_rational_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.min_rational_prec_val_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_rational_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.min_rational_prec_ref_val(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_rational_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.min_rational_prec_ref_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_rational_prec_ref_ref(&y, prec));
                },
            ),
        ],
    );
}

fn demo_big_exponent_float_max(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        println!("{x:#x}.max({y:#x}) = {:#x}", x.max_ref_ref(&y));
    }
}

fn demo_big_exponent_float_max_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.max_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.max_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_max_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.max_prec_ref_ref(&y, prec);
        println!("{x:#x}.max_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_max_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.max_rational_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.max_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_max_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.max_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.max_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_max_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.max(BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat.max(BigExponentFloat)", &mut |(x, y)| {
                no_out!(x.max(y));
            }),
            (
                "BigExponentFloat.max_val_ref(&BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.max_val_ref(&y));
                },
            ),
            (
                "BigExponentFloat.max_ref_val(BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.max_ref_val(y));
                },
            ),
            (
                "BigExponentFloat.max_ref_ref(&BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.max_ref_ref(&y));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_max_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.max_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_1()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.max_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.max_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.max_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.max_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_max_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.max_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.max_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.max_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.max_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.max_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_prec_ref_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_max_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.max_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.max_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.max_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.max_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.max_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_max_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.max_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.max_rational_prec(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_rational_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.max_rational_prec_val_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_rational_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.max_rational_prec_ref_val(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_rational_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.max_rational_prec_ref_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_rational_prec_ref_ref(&y, prec));
                },
            ),
        ],
    );
}
