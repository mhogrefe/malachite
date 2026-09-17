// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Acsc;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::acsc::{
    primitive_float_acsc, primitive_float_acsc_rational, primitive_float_acsc_with_period,
};
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_51, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_48,
    float_unsigned_rounding_mode_triple_gen_var_49, float_unsigned_rounding_mode_triple_gen_var_50,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_28,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_13,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    rational_bit_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_acsc_prec_round);
    register_demo!(runner, demo_float_acsc_prec_round_debug);
    register_demo!(runner, demo_float_acsc_prec_round_extreme);
    register_demo!(runner, demo_float_acsc_prec);
    register_demo!(runner, demo_float_acsc_round);
    register_demo!(runner, demo_float_acsc_prec_round_assign);
    register_demo!(runner, demo_float_acsc);
    register_demo!(runner, demo_float_acsc_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_acsc);
    register_bench!(runner, benchmark_float_acsc_prec_round_evaluation_strategy);
    register_primitive_float_benches!(runner, benchmark_primitive_float_acsc);
    register_demo!(runner, demo_float_acsc_rational_prec_round);
    register_demo!(runner, demo_float_acsc_rational_prec_round_debug);
    register_demo!(runner, demo_float_acsc_rational_prec);
    register_primitive_float_demos!(runner, demo_primitive_float_acsc_rational);
    register_bench!(
        runner,
        benchmark_float_acsc_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_acsc_rational);
    register_demo!(runner, demo_float_acsc_with_period_prec_round);
    register_demo!(runner, demo_float_acsc_with_period_prec_round_debug);
    register_demo!(runner, demo_float_acsc_with_period_prec_round_extreme);
    register_demo!(runner, demo_float_acsc_with_period_prec_round_assign);
    register_demo!(runner, demo_float_acsc_with_period_prec);
    register_demo!(runner, demo_float_acsc_with_period_round);
    register_demo!(runner, demo_float_acsc_with_period);
    register_demo!(runner, demo_float_acsc_with_period_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_acsc_with_period);
    register_bench!(
        runner,
        benchmark_float_acsc_with_period_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_acsc_with_period);
}

fn demo_float_acsc_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acsc_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.acsc_prec_round(prec, rm)
        );
    }
}

fn demo_float_acsc_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.acsc_prec_round(prec, rm);
        println!(
            "({:#x}).acsc_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acsc_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_49()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acsc_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.acsc_prec_round(prec, rm)
        );
    }
}

fn demo_float_acsc_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acsc_prec({}) = {:?}", x_old, prec, x.acsc_prec(prec));
    }
}

fn demo_float_acsc_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_51()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).acsc_round({}) = {:?}", x_old, rm, x.acsc_round(rm));
    }
}

fn demo_float_acsc_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acsc_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.acsc_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_acsc(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acsc() = {}", x_old, x.acsc());
    }
}

fn demo_float_acsc_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).acsc() = {}", x, (&x).acsc());
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acsc<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_acsc({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_acsc(x))
        );
    }
}

fn benchmark_float_acsc_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acsc_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_48().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.acsc_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acsc_prec_round(prec, rm)),
            ),
            (
                "(&Float).acsc_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acsc_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acsc<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acsc({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acsc(x));
        })],
    );
}

fn demo_float_acsc_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acsc_rational_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            Float::acsc_rational_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_acsc_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = Float::acsc_rational_prec_round(x.clone(), prec, rm);
        println!(
            "Float::acsc_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            x,
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acsc_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acsc_rational_prec({}, {}) = {:?}",
            x.clone(),
            prec,
            Float::acsc_rational_prec(x, prec)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acsc_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_acsc_rational({}) = {}",
            x,
            NiceFloat(primitive_float_acsc_rational::<T>(&x))
        );
    }
}

fn benchmark_float_acsc_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::acsc_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "Float::acsc_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(Float::acsc_rational_prec_round(x, prec, rm));
                },
            ),
            (
                "Float::acsc_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(Float::acsc_rational_prec_round_ref(&x, prec, rm));
                },
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acsc_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acsc_rational(&Rational) to {}", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acsc_rational::<T>(&x));
        })],
    );
}

fn demo_float_acsc_with_period_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).acsc_with_period_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            x.acsc_with_period_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_acsc_with_period_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().acsc_with_period_prec_round(u, prec, rm);
        println!(
            "({:#x}).acsc_with_period_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acsc_with_period_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_28()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).acsc_with_period_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            x.acsc_with_period_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_acsc_with_period_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acsc_with_period_prec_round_assign(u, prec, rm);
        println!(
            "x := {x_old}; x.acsc_with_period_prec_round_assign({u}, {prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_float_acsc_with_period_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).acsc_with_period_prec({}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            x.acsc_with_period_prec(u, prec)
        );
    }
}

fn demo_float_acsc_with_period_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_50()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).acsc_with_period_round({}, {}) = {:?}",
            x.clone(),
            u,
            rm,
            x.acsc_with_period_round(u, rm)
        );
    }
}

fn demo_float_acsc_with_period(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u) in float_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acsc_with_period({}) = {}",
            x_old,
            u,
            x.acsc_with_period(u)
        );
    }
}

fn demo_float_acsc_with_period_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u) in float_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).acsc_with_period_ref({}) = {}",
            x,
            u,
            x.acsc_with_period_ref(u)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acsc_with_period<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, u) in primitive_float_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_acsc_with_period({}, {}) = {}",
            NiceFloat(x),
            u,
            NiceFloat(primitive_float_acsc_with_period(x, u))
        );
    }
}

fn benchmark_float_acsc_with_period_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acsc_with_period_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.acsc_with_period_prec_round(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| no_out!(x.acsc_with_period_prec_round(u, prec, rm)),
            ),
            (
                "(&Float).acsc_with_period_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| no_out!(x.acsc_with_period_prec_round_ref(u, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acsc_with_period<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acsc_with_period({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("malachite", &mut |(x, u)| {
            no_out!(primitive_float_acsc_with_period(x, u));
        })],
    );
}
