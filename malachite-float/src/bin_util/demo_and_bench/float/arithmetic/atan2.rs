// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Atan2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_max_primitive_float_bucketer, quadruple_3_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::atan2::{
    primitive_float_atan2, primitive_float_atan2_rational,
};
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_43,
    float_float_unsigned_rounding_mode_quadruple_gen_var_24, float_float_unsigned_triple_gen_var_1,
    float_pair_gen, rational_rational_unsigned_rounding_mode_quadruple_gen_var_4,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_atan2_prec_round);
    register_demo!(runner, demo_float_atan2_prec_round_debug);
    register_demo!(runner, demo_float_atan2_prec);
    register_demo!(runner, demo_float_atan2_round);
    register_demo!(runner, demo_float_atan2_prec_round_assign);
    register_demo!(runner, demo_float_atan2);
    register_primitive_float_demos!(runner, demo_primitive_float_atan2);
    register_bench!(runner, benchmark_float_atan2_prec_round_evaluation_strategy);
    register_primitive_float_benches!(runner, benchmark_primitive_float_atan2);
    register_demo!(runner, demo_float_atan2_rational_prec_round);
    register_demo!(runner, demo_float_atan2_rational_prec_round_debug);
    register_demo!(runner, demo_float_atan2_rational_prec);
    register_demo!(runner, demo_float_atan2_rational_prec_ref);
    register_primitive_float_demos!(runner, demo_primitive_float_atan2_rational);
    register_bench!(
        runner,
        benchmark_float_atan2_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_atan2_rational);
}

fn demo_float_atan2_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).atan2_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.atan2_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_atan2_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (atan2, o) = x.atan2_prec_round(y, prec, rm);
        println!(
            "({:#x}).atan2_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(atan2),
            o
        );
    }
}

fn demo_float_atan2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).atan2_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.atan2_prec(y, prec)
        );
    }
}

fn demo_float_atan2_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).atan2_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.atan2_round(y, rm)
        );
    }
}

fn demo_float_atan2_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.atan2_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.atan2_prec_round_assign({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_atan2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).atan2({}) = {}", x_old, y_old, x.atan2(y));
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan2<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_atan2({}, {}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(primitive_float_atan2(x, y))
        );
    }
}

fn benchmark_float_atan2_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atan2_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.atan2_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.atan2_prec_round(y, prec, rm)),
            ),
            (
                "Float.atan2_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.atan2_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).atan2_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.atan2_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).atan2_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.atan2_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atan2<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_atan2({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_primitive_float_bucketer("x", "y"),
        &mut [("malachite", &mut |(x, y)| {
            no_out!(primitive_float_atan2(x, y));
        })],
    );
}

fn demo_float_atan2_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan2_rational_prec_round({}, {}, {}, {}) = {:?}",
            y.clone(),
            x.clone(),
            prec,
            rm,
            Float::atan2_rational_prec_round(y, x, prec, rm)
        );
    }
}

fn demo_float_atan2_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (t, o) = Float::atan2_rational_prec_round(y.clone(), x.clone(), prec, rm);
        println!(
            "Float::atan2_rational_prec_round({}, {}, {}, {}) = ({:#x}, {:?})",
            y,
            x,
            prec,
            rm,
            ComparableFloat(t),
            o
        );
    }
}

fn demo_float_atan2_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan2_rational_prec({}, {}, {}) = {:?}",
            y.clone(),
            x.clone(),
            prec,
            Float::atan2_rational_prec(y, x, prec)
        );
    }
}

fn demo_float_atan2_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::atan2_rational_prec_ref(&{}, &{}, {}) = {:?}",
            y,
            x,
            prec,
            Float::atan2_rational_prec_ref(&y, &x, prec)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atan2_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (y, x) in rational_pair_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_atan2_rational({}, {}) = {:?}",
            y,
            x,
            NiceFloat(primitive_float_atan2_rational::<T>(&y, &x))
        );
    }
}

fn benchmark_float_atan2_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::atan2_rational_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "Float::atan2_rational_prec_round(Rational, Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| no_out!(Float::atan2_rational_prec_round(y, x, prec, rm)),
            ),
            (
                "Float::atan2_rational_prec_round_ref(&Rational, &Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(Float::atan2_rational_prec_round_ref(&y, &x, prec, rm));
                },
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atan2_rational<T: PrimitiveFloat>(
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
            "primitive_float_atan2_rational::<{}>(&Rational, &Rational)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("y"),
        &mut [("malachite", &mut |(y, x)| {
            no_out!(primitive_float_atan2_rational::<T>(&y, &x));
        })],
    );
}
