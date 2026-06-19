// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{LogBaseOf1PlusX, LogBaseOf1PlusXAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::arithmetic::log_base_rational_base_1_plus_x::*;
use malachite_float::test_util::bench::bucketers::{
    quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer,
    triple_1_2_float_rational_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_rational_rounding_mode_triple_gen_var_14, float_rational_rounding_mode_triple_gen_var_15,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_13,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_14,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_primitive_float_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_log_base_rational_base_1_plus_x);
    register_demo!(runner, demo_float_log_base_rational_base_1_plus_x_debug);
    register_demo!(runner, demo_float_log_base_rational_base_1_plus_x_ref);
    register_demo!(runner, demo_float_log_base_rational_base_1_plus_x_assign);
    register_demo!(runner, demo_float_log_base_rational_base_1_plus_x_prec);
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_prec_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_prec_extreme
    );
    register_demo!(runner, demo_float_log_base_rational_base_1_plus_x_round);
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_round_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_round_extreme
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_prec_round
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_prec_round_debug
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_prec_round_extreme
    );
    register_demo!(
        runner,
        demo_float_log_base_rational_base_1_plus_x_prec_round_assign
    );

    register_bench!(
        runner,
        benchmark_float_log_base_rational_base_1_plus_x_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_rational_base_1_plus_x_assign
    );
    register_bench!(
        runner,
        benchmark_float_log_base_rational_base_1_plus_x_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_rational_base_1_plus_x_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_rational_base_1_plus_x_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_rational_base_1_plus_x_prec_round_assign
    );

    register_primitive_float_demos!(runner, demo_primitive_float_log_base_rational_base_1_plus_x);
    register_primitive_float_benches!(
        runner,
        benchmark_primitive_float_log_base_rational_base_1_plus_x
    );
}

fn demo_float_log_base_rational_base_1_plus_x(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_1_plus_x({}) = {}",
            x.clone(),
            base.clone(),
            x.log_base_1_plus_x(base)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({:#x}).log_base_1_plus_x({}) = {:#x}",
            ComparableFloat(x.clone()),
            base.clone(),
            ComparableFloat(x.log_base_1_plus_x(base))
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_1_plus_x(&{}) = {}",
            x,
            base,
            (&x).log_base_1_plus_x(&base)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, base, _, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_1_plus_x_assign(&base);
        println!("x := {x_old}; x.log_base_1_plus_x_assign(&{base}); x = {x}");
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_rational_base_1_plus_x_prec(&{}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            x.log_base_rational_base_1_plus_x_prec(&base, prec)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.clone().log_base_rational_base_1_plus_x_prec(&base, prec);
        println!(
            "({:#x}).log_base_rational_base_1_plus_x_prec(&{}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            base,
            prec,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, _) in float_rational_unsigned_rounding_mode_quadruple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_rational_base_1_plus_x_prec(&{}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            x.log_base_rational_base_1_plus_x_prec(&base, prec)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_rational_rounding_mode_triple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_rational_base_1_plus_x_round(&{}, {}) = {:?}",
            x.clone(),
            base,
            rm,
            x.log_base_rational_base_1_plus_x_round(&base, rm)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, rm) in float_rational_rounding_mode_triple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.clone().log_base_rational_base_1_plus_x_round(&base, rm);
        println!(
            "({:#x}).log_base_rational_base_1_plus_x_round(&{}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            base,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_round_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, rm) in float_rational_rounding_mode_triple_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_rational_base_1_plus_x_round(&{}, {}) = {:?}",
            x.clone(),
            base,
            rm,
            x.log_base_rational_base_1_plus_x_round(&base, rm)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_rational_base_1_plus_x_prec_round(&{}, {}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            rm,
            x.log_base_rational_base_1_plus_x_prec_round(&base, prec, rm)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x
            .clone()
            .log_base_rational_base_1_plus_x_prec_round(&base, prec, rm);
        println!(
            "({:#x}).log_base_rational_base_1_plus_x_prec_round(&{}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            base,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec_round_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_rational_base_1_plus_x_prec_round(&{}, {}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            rm,
            x.log_base_rational_base_1_plus_x_prec_round(&base, prec, rm)
        );
    }
}

fn demo_float_log_base_rational_base_1_plus_x_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, base, prec, rm) in float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_rational_base_1_plus_x_prec_round_assign(&base, prec, rm);
        println!(
            "x := {x_old}; x.log_base_rational_base_1_plus_x_prec_round_assign(&{base}, {prec}, \
             {rm}) = {o:?}; x = {x}"
        );
    }
}

fn benchmark_float_log_base_rational_base_1_plus_x_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_1_plus_x(Rational)",
        BenchmarkType::EvaluationStrategy,
        float_rational_rounding_mode_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_rational_max_complexity_bucketer("x", "base"),
        &mut [
            ("Float.log_base_1_plus_x(Rational)", &mut |(x, base, _)| {
                no_out!(x.log_base_1_plus_x(base));
            }),
            (
                "(&Float).log_base_1_plus_x(&Rational)",
                &mut |(x, base, _)| {
                    no_out!((&x).log_base_1_plus_x(&base));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_rational_base_1_plus_x_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_1_plus_x_assign(&Rational)",
        BenchmarkType::Single,
        float_rational_rounding_mode_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_rational_max_complexity_bucketer("x", "base"),
        &mut [(
            "Float.log_base_1_plus_x_assign(&Rational)",
            &mut |(mut x, base, _)| {
                x.log_base_1_plus_x_assign(&base);
            },
        )],
    );
}

fn benchmark_float_log_base_rational_base_1_plus_x_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_rational_base_1_plus_x_prec(&Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer("x", "base", "prec"),
        &mut [
            (
                "Float.log_base_rational_base_1_plus_x_prec(&Rational, u64)",
                &mut |(x, base, prec, _)| {
                    no_out!(x.log_base_rational_base_1_plus_x_prec(&base, prec));
                },
            ),
            (
                "(&Float).log_base_rational_base_1_plus_x_prec_ref(&Rational, u64)",
                &mut |(x, base, prec, _)| {
                    no_out!(x.log_base_rational_base_1_plus_x_prec_ref(&base, prec));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_rational_base_1_plus_x_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_rational_base_1_plus_x_round(&Rational, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rational_rounding_mode_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_rational_max_complexity_bucketer("x", "base"),
        &mut [
            (
                "Float.log_base_rational_base_1_plus_x_round(&Rational, RoundingMode)",
                &mut |(x, base, rm)| no_out!(x.log_base_rational_base_1_plus_x_round(&base, rm)),
            ),
            (
                "(&Float).log_base_rational_base_1_plus_x_round_ref(&Rational, RoundingMode)",
                &mut |(x, base, rm)| {
                    no_out!(x.log_base_rational_base_1_plus_x_round_ref(&base, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_rational_base_1_plus_x_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_rational_base_1_plus_x_prec_round(&Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer("x", "base", "prec"),
        &mut [
            (
                "Float.log_base_rational_base_1_plus_x_prec_round(&Rational, u64, RoundingMode)",
                &mut |(x, base, prec, rm)| {
                    no_out!(x.log_base_rational_base_1_plus_x_prec_round(&base, prec, rm));
                },
            ),
            (
                "(&Float).log_base_rational_base_1_plus_x_prec_round_ref(&Rational, u64, \
                 RoundingMode)",
                &mut |(x, base, prec, rm)| {
                    no_out!(x.log_base_rational_base_1_plus_x_prec_round_ref(&base, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_rational_base_1_plus_x_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_rational_base_1_plus_x_prec_round_assign(&Rational, u64, RoundingMode)",
        BenchmarkType::Single,
        float_rational_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer("x", "base", "prec"),
        &mut [(
            "Float.log_base_rational_base_1_plus_x_prec_round_assign(&Rational, u64, RoundingMode)",
            &mut |(mut x, base, prec, rm)| {
                no_out!(x.log_base_rational_base_1_plus_x_prec_round_assign(&base, prec, rm));
            },
        )],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_log_base_rational_base_1_plus_x<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (base, x) in rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .filter(|(base, _)| *base > 1u32)
        .take(limit)
    {
        println!(
            "primitive_float_log_base_rational_base_1_plus_x({}, {}) = {}",
            NiceFloat(x),
            base.clone(),
            NiceFloat(primitive_float_log_base_rational_base_1_plus_x(x, &base))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_log_base_rational_base_1_plus_x<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!(
            "primitive_float_log_base_rational_base_1_plus_x({}, Rational)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_primitive_float_pair_gen::<T>()
            .get(gm, config)
            .filter(|(base, _)| *base > 1u32),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("base"),
        &mut [("malachite", &mut |(base, x)| {
            no_out!(primitive_float_log_base_rational_base_1_plus_x(x, &base));
        })],
    );
}
