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
use malachite_float::ComparableFloat;
use malachite_float::Float;
use malachite_float::test_util::generators::rational_rational_unsigned_rounding_mode_quadruple_gen_var_2;
use malachite_q::test_util::bench::bucketers::quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_log_base_rational_rational_base_prec);
    register_demo!(
        runner,
        demo_float_log_base_rational_rational_base_prec_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_rational_base_prec_round
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_rational_base_prec_round_debug
    );

    register_bench!(
        runner,
        benchmark_float_log_base_rational_rational_base_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_rational_rational_base_prec_round_evaluation_strategy
    );
}

fn demo_float_log_base_rational_rational_base_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "log_base_{}({}) = {:?} (prec {})",
            base.clone(),
            x.clone(),
            Float::log_base_rational_rational_base_prec(x, base, prec),
            prec
        );
    }
}

fn demo_float_log_base_rational_rational_base_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = Float::log_base_rational_rational_base_prec(x.clone(), base.clone(), prec);
        println!(
            "log_base_{}({}) = ({:#x}, {:?}) (prec {})",
            base,
            x,
            ComparableFloat(log),
            o,
            prec
        );
    }
}

fn demo_float_log_base_rational_rational_base_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "log_base_{}({}) = {:?} (prec {}, {})",
            base.clone(),
            x.clone(),
            Float::log_base_rational_rational_base_prec_round(x, base, prec, rm),
            prec,
            rm
        );
    }
}

fn demo_float_log_base_rational_rational_base_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) =
            Float::log_base_rational_rational_base_prec_round(x.clone(), base.clone(), prec, rm);
        println!(
            "log_base_{}({}) = ({:#x}, {:?}) (prec {}, {})",
            base,
            x,
            ComparableFloat(log),
            o,
            prec,
            rm
        );
    }
}

fn benchmark_float_log_base_rational_rational_base_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::log_base_rational_rational_base_prec(Rational, Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("x", "base", "prec"),
        &mut [
            (
                "Float::log_base_rational_rational_base_prec(Rational, Rational, u64)",
                &mut |(x, base, prec, _)| {
                    no_out!(Float::log_base_rational_rational_base_prec(x, base, prec));
                },
            ),
            (
                "Float::log_base_rational_rational_base_prec_ref(&Rational, &Rational, u64)",
                &mut |(x, base, prec, _)| {
                    no_out!(Float::log_base_rational_rational_base_prec_ref(
                        &x, &base, prec
                    ));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_rational_rational_base_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::log_base_rational_rational_base_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("x", "base", "prec"),
        &mut [
            (
                "Float::log_base_rational_rational_base_prec_round(Rational, Rational, u64, \
                 RoundingMode)",
                &mut |(x, base, prec, rm)| {
                    no_out!(Float::log_base_rational_rational_base_prec_round(
                        x, base, prec, rm
                    ));
                },
            ),
            (
                "Float::log_base_rational_rational_base_prec_round_ref(&Rational, &Rational, u64, \
                 RoundingMode)",
                &mut |(x, base, prec, rm)| {
                    no_out!(Float::log_base_rational_rational_base_prec_round_ref(
                        &x, &base, prec, rm
                    ));
                },
            ),
        ],
    );
}
