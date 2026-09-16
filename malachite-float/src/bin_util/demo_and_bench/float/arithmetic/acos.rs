// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Acos;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::acos::{
    primitive_float_acos, primitive_float_acos_rational,
};
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_49, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_42, float_unsigned_rounding_mode_triple_gen_var_43,
    rational_unsigned_rounding_mode_triple_gen_var_11,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    rational_bit_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_acos_prec_round);
    register_demo!(runner, demo_float_acos_prec_round_debug);
    register_demo!(runner, demo_float_acos_prec_round_extreme);
    register_demo!(runner, demo_float_acos_prec);
    register_demo!(runner, demo_float_acos_round);
    register_demo!(runner, demo_float_acos_prec_round_assign);
    register_demo!(runner, demo_float_acos);
    register_demo!(runner, demo_float_acos_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_acos);
    register_bench!(runner, benchmark_float_acos_prec_round_evaluation_strategy);
    register_primitive_float_benches!(runner, benchmark_primitive_float_acos);
    register_demo!(runner, demo_float_acos_rational_prec_round);
    register_demo!(runner, demo_float_acos_rational_prec_round_debug);
    register_demo!(runner, demo_float_acos_rational_prec);
    register_primitive_float_demos!(runner, demo_primitive_float_acos_rational);
    register_bench!(
        runner,
        benchmark_float_acos_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_acos_rational);
}

fn demo_float_acos_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_42()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acos_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.acos_prec_round(prec, rm)
        );
    }
}

fn demo_float_acos_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_42()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.acos_prec_round(prec, rm);
        println!(
            "({:#x}).acos_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acos_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acos_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.acos_prec_round(prec, rm)
        );
    }
}

fn demo_float_acos_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acos_prec({}) = {:?}", x_old, prec, x.acos_prec(prec));
    }
}

fn demo_float_acos_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_49()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).acos_round({}) = {:?}", x_old, rm, x.acos_round(rm));
    }
}

fn demo_float_acos_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_42()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acos_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.acos_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_acos(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acos() = {}", x_old, x.acos());
    }
}

fn demo_float_acos_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).acos() = {}", x, (&x).acos());
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acos<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_acos({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_acos(x))
        );
    }
}

fn benchmark_float_acos_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acos_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_42().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.acos_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acos_prec_round(prec, rm)),
            ),
            (
                "(&Float).acos_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acos_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acos<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acos({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acos(x));
        })],
    );
}

fn demo_float_acos_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acos_rational_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            Float::acos_rational_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_acos_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = Float::acos_rational_prec_round(x.clone(), prec, rm);
        println!(
            "Float::acos_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            x,
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acos_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acos_rational_prec({}, {}) = {:?}",
            x.clone(),
            prec,
            Float::acos_rational_prec(x, prec)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acos_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_acos_rational({}) = {}",
            x,
            NiceFloat(primitive_float_acos_rational::<T>(&x))
        );
    }
}

fn benchmark_float_acos_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::acos_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            (
                "Float::acos_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(Float::acos_rational_prec_round(x, prec, rm));
                },
            ),
            (
                "Float::acos_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(Float::acos_rational_prec_round_ref(&x, prec, rm));
                },
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acos_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acos_rational(&Rational) to {}", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acos_rational::<T>(&x));
        })],
    );
}
