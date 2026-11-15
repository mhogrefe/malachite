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
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::trigonometric::sin_cos::{
    TrigFunction, trig_prec_round_generic, trig_with_period_prec_round_generic,
};
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_q::test_util::bench::bucketers::{
    quadruple_1_3_rational_bit_u64_max_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_cos_prec_round);
    register_demo!(runner, demo_big_exponent_float_cos_prec);
    register_demo!(runner, demo_big_exponent_float_cos_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_cos_rational_prec);
    register_demo!(runner, demo_big_exponent_float_cos_with_period_prec_round);
    register_demo!(runner, demo_big_exponent_float_cos_with_period_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_cos_with_period_rational_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_cos_with_period_rational_prec
    );
    register_demo!(runner, demo_big_exponent_float_cos_pi_prec_round);
    register_demo!(runner, demo_big_exponent_float_cos_pi_prec);
    register_demo!(runner, demo_big_exponent_float_cos_pi_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_cos_pi_rational_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_with_period_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_with_period_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_with_period_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_with_period_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_with_period_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_pi_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_pi_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_pi_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_pi_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_cos_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_cos_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cos_prec_round_ref(prec, rm);
        println!("{x:#x}.cos_prec_round({prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cos_prec_ref(prec);
        println!("{x:#x}.cos_prec({prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::cos_rational_prec_round_ref(&x, prec, rm);
        println!(
            "BigExponentFloat::cos_rational_prec_round({x}, {prec}, {rm}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_cos_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::cos_rational_prec_ref(&x, prec);
        println!("BigExponentFloat::cos_rational_prec({x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_with_period_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, rm) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cos_with_period_prec_round_ref(u, prec, rm);
        println!("{x:#x}.cos_with_period_prec_round({u}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_with_period_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in big_exponent_float_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cos_with_period_prec_ref(u, prec);
        println!("{x:#x}.cos_with_period_prec({u}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_with_period_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::cos_with_period_rational_prec_round_ref(&x, u, prec, rm);
        println!(
            "BigExponentFloat::cos_with_period_rational_prec_round({x}, {u}, {prec}, {rm}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_cos_with_period_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::cos_with_period_rational_prec_ref(&x, u, prec);
        println!(
            "BigExponentFloat::cos_with_period_rational_prec({x}, {u}, {prec}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_cos_pi_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cos_pi_prec_round_ref(prec, rm);
        println!("{x:#x}.cos_pi_prec_round({prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_pi_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.cos_pi_prec_ref(prec);
        println!("{x:#x}.cos_pi_prec({prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_cos_pi_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::cos_pi_rational_prec_round_ref(&x, prec, rm);
        println!(
            "BigExponentFloat::cos_pi_rational_prec_round({x}, {prec}, {rm}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_cos_pi_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::cos_pi_rational_prec_ref(&x, prec);
        println!("BigExponentFloat::cos_pi_rational_prec({x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_cos_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.cos_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cos_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.cos_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cos_prec_round_ref(prec, rm));
                },
            ),
            (
                "BigExponentFloat.cos_prec_round_assign(u64, RoundingMode)",
                &mut |(mut x, prec, rm)| {
                    no_out!(x.cos_prec_round_assign(prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat.cos_prec(u64)", &mut |(x, prec, _)| {
                no_out!(x.cos_prec(prec));
            }),
            ("BigExponentFloat.cos_prec_ref(u64)", &mut |(x, prec, _)| {
                no_out!(x.cos_prec_ref(prec));
            }),
            (
                "BigExponentFloat.cos_prec_assign(u64)",
                &mut |(mut x, prec, _)| {
                    no_out!(x.cos_prec_assign(prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("Float functions", &mut |(x, prec, rm)| {
                no_out!(x.cos_prec_round_ref(prec, rm));
            }),
            ("generic", &mut |(x, prec, rm)| {
                no_out!(trig_prec_round_generic(TrigFunction::Cos, &x, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_cos_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::cos_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::cos_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::cos_rational_prec_round(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat::cos_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::cos_rational_prec_round_ref(&x, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::cos_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::cos_rational_prec(Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::cos_rational_prec(x, prec));
                },
            ),
            (
                "BigExponentFloat::cos_rational_prec_ref(&Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::cos_rational_prec_ref(&x, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_with_period_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_with_period_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.cos_with_period_prec_round(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(x.cos_with_period_prec_round(u, prec, rm));
                },
            ),
            (
                "BigExponentFloat.cos_with_period_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(x.cos_with_period_prec_round_ref(u, prec, rm));
                },
            ),
            (
                "BigExponentFloat.cos_with_period_prec_round_assign(u64, u64, RoundingMode)",
                &mut |(mut x, u, prec, rm)| {
                    no_out!(x.cos_with_period_prec_round_assign(u, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_with_period_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_with_period_prec(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.cos_with_period_prec(u64, u64)",
                &mut |(x, u, prec, _)| {
                    no_out!(x.cos_with_period_prec(u, prec));
                },
            ),
            (
                "BigExponentFloat.cos_with_period_prec_ref(u64, u64)",
                &mut |(x, u, prec, _)| {
                    no_out!(x.cos_with_period_prec_ref(u, prec));
                },
            ),
            (
                "BigExponentFloat.cos_with_period_prec_assign(u64, u64)",
                &mut |(mut x, u, prec, _)| {
                    no_out!(x.cos_with_period_prec_assign(u, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_with_period_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_with_period_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("Float functions", &mut |(x, u, prec, rm)| {
                no_out!(x.cos_with_period_prec_round_ref(u, prec, rm));
            }),
            ("generic", &mut |(x, u, prec, rm)| {
                no_out!(trig_with_period_prec_round_generic(
                    TrigFunction::Cos,
                    &x,
                    u,
                    prec,
                    rm
                ));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_cos_with_period_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::cos_with_period_rational_prec_round(Rational, u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::cos_with_period_rational_prec_round(Rational, u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(BigExponentFloat::cos_with_period_rational_prec_round(
                        x, u, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::cos_with_period_rational_prec_round_ref(&Rational, u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(BigExponentFloat::cos_with_period_rational_prec_round_ref(
                        &x, u, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_with_period_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::cos_with_period_rational_prec(Rational, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::cos_with_period_rational_prec(Rational, u64, u64)",
                &mut |(x, u, prec, _)| {
                    no_out!(BigExponentFloat::cos_with_period_rational_prec(x, u, prec));
                },
            ),
            (
                "BigExponentFloat::cos_with_period_rational_prec_ref(&Rational, u64, u64)",
                &mut |(x, u, prec, _)| {
                    no_out!(BigExponentFloat::cos_with_period_rational_prec_ref(
                        &x, u, prec
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_pi_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_pi_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.cos_pi_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cos_pi_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.cos_pi_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cos_pi_prec_round_ref(prec, rm));
                },
            ),
            (
                "BigExponentFloat.cos_pi_prec_round_assign(u64, RoundingMode)",
                &mut |(mut x, prec, rm)| {
                    no_out!(x.cos_pi_prec_round_assign(prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_pi_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_pi_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat.cos_pi_prec(u64)", &mut |(x, prec, _)| {
                no_out!(x.cos_pi_prec(prec));
            }),
            (
                "BigExponentFloat.cos_pi_prec_ref(u64)",
                &mut |(x, prec, _)| {
                    no_out!(x.cos_pi_prec_ref(prec));
                },
            ),
            (
                "BigExponentFloat.cos_pi_prec_assign(u64)",
                &mut |(mut x, prec, _)| {
                    no_out!(x.cos_pi_prec_assign(prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_pi_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::cos_pi_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::cos_pi_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::cos_pi_rational_prec_round(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat::cos_pi_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(BigExponentFloat::cos_pi_rational_prec_round_ref(
                        &x, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_pi_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::cos_pi_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat::cos_pi_rational_prec(Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::cos_pi_rational_prec(x, prec));
                },
            ),
            (
                "BigExponentFloat::cos_pi_rational_prec_ref(&Rational, u64)",
                &mut |(x, prec, _)| {
                    no_out!(BigExponentFloat::cos_pi_rational_prec_ref(&x, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_cos_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.cos_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_21_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, prec, rm))| {
                no_out!(x.cos_prec_round_ref(prec, rm));
            }),
            ("Float", &mut |((x, prec, rm), _)| {
                no_out!(x.cos_prec_round_ref(prec, rm));
            }),
        ],
    );
}
