// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Acoth, AcothAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::acoth::{
    primitive_float_acoth, primitive_float_acoth_rational,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_55,
    rational_unsigned_rounding_mode_triple_gen_var_18,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    rational_bit_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::rational_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_acoth_rational_prec);
    register_demo!(runner, demo_float_acoth_rational_prec_debug);
    register_demo!(runner, demo_float_acoth_rational_prec_ref);
    register_demo!(runner, demo_float_acoth_rational_prec_ref_debug);
    register_demo!(runner, demo_float_acoth_rational_prec_round);
    register_demo!(runner, demo_float_acoth_rational_prec_round_debug);
    register_demo!(runner, demo_float_acoth_rational_prec_round_ref);
    register_demo!(runner, demo_float_acoth_rational_prec_round_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_acoth_rational);
    register_bench!(
        runner,
        benchmark_float_acoth_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_acoth_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_acoth_rational);
    register_demo!(runner, demo_float_acoth);
    register_demo!(runner, demo_float_acoth_debug);
    register_demo!(runner, demo_float_acoth_extreme);
    register_demo!(runner, demo_float_acoth_extreme_debug);
    register_demo!(runner, demo_float_acoth_ref);
    register_demo!(runner, demo_float_acoth_ref_debug);
    register_demo!(runner, demo_float_acoth_assign);
    register_demo!(runner, demo_float_acoth_assign_debug);
    register_demo!(runner, demo_float_acoth_prec);
    register_demo!(runner, demo_float_acoth_prec_debug);
    register_demo!(runner, demo_float_acoth_prec_extreme);
    register_demo!(runner, demo_float_acoth_prec_ref);
    register_demo!(runner, demo_float_acoth_prec_assign);
    register_demo!(runner, demo_float_acoth_round);
    register_demo!(runner, demo_float_acoth_round_debug);
    register_demo!(runner, demo_float_acoth_round_ref);
    register_demo!(runner, demo_float_acoth_round_assign);
    register_demo!(runner, demo_float_acoth_prec_round);
    register_demo!(runner, demo_float_acoth_prec_round_debug);
    register_demo!(runner, demo_float_acoth_prec_round_ref);
    register_demo!(runner, demo_float_acoth_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_acoth);

    register_bench!(runner, benchmark_float_acoth_evaluation_strategy);
    register_bench!(runner, benchmark_float_acoth_assign);
    register_bench!(runner, benchmark_float_acoth_prec_round_evaluation_strategy);
    register_primitive_float_benches!(runner, benchmark_primitive_float_acoth);
}

fn demo_float_acoth(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acoth() = {}", x_old, x.acoth());
    }
}

fn demo_float_acoth_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).acoth() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.acoth())
        );
    }
}

fn demo_float_acoth_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acoth() = {}", x_old, x.acoth());
    }
}

fn demo_float_acoth_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).acoth() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.acoth())
        );
    }
}

fn demo_float_acoth_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).acoth() = {}", x, (&x).acoth());
    }
}

fn demo_float_acoth_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).acoth() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).acoth())
        );
    }
}

fn demo_float_acoth_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.acoth_assign();
        println!("x := {x_old}; x.acoth_assign(); x = {x}");
    }
}

fn demo_float_acoth_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.acoth_assign();
        println!(
            "x := {:#x}; x.acoth_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_acoth_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).acoth_prec({}) = {:?}",
            x_old,
            prec,
            x.acoth_prec(prec)
        );
    }
}

fn demo_float_acoth_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.acoth_prec(prec);
        println!(
            "({:#x}).acoth_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acoth_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).acoth_prec({}) = {:?}",
            x_old,
            prec,
            x.acoth_prec(prec)
        );
    }
}

fn demo_float_acoth_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).acoth_prec_ref({}) = {:?}",
            x,
            prec,
            x.acoth_prec_ref(prec)
        );
    }
}

fn demo_float_acoth_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.acoth_prec_assign(prec);
        println!("x := {x_old}; x.acoth_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_acoth_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).acoth_round({}) = {:?}", x_old, rm, x.acoth_round(rm));
    }
}

fn demo_float_acoth_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.acoth_round(rm);
        println!(
            "({:#x}).acoth_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acoth_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).acoth_round_ref({}) = {:?}",
            x,
            rm,
            x.acoth_round_ref(rm)
        );
    }
}

fn demo_float_acoth_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acoth_round_assign(rm);
        println!("x := {x_old}; x.acoth_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_acoth_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_55()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acoth_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.acoth_prec_round(prec, rm)
        );
    }
}

fn demo_float_acoth_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_55()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.acoth_prec_round(prec, rm);
        println!(
            "({:#x}).acoth_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acoth_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_55()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).acoth_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.acoth_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_acoth_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_55()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acoth_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.acoth_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_acoth_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acoth()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.acoth()", &mut |x| no_out!(x.acoth())),
            ("(&Float).acoth()", &mut |x| no_out!((&x).acoth())),
        ],
    );
}

fn benchmark_float_acoth_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.acoth_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.acoth_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_acoth_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acoth_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_55().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.acoth_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acoth_prec_round(prec, rm)),
            ),
            (
                "(&Float).acoth_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acoth_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acoth<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_acoth({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_acoth(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acoth<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acoth({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acoth(x));
        })],
    );
}

fn demo_float_acoth_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, _) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acoth_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::acoth_rational_prec(n, p)
        );
    }
}

fn demo_float_acoth_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, _) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acoth_rational_prec(n.clone(), p);
        println!(
            "Float::acoth_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_acoth_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, _) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acoth_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::acoth_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_acoth_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, _) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acoth_rational_prec_ref(&n, p);
        println!(
            "Float::acoth_rational_prec_ref(&{}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_acoth_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acoth_rational_prec_round({}, {}, {}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::acoth_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_acoth_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acoth_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::acoth_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_acoth_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acoth_rational_prec_round_ref(&{}, {}, {}) = {:?}",
            n,
            p,
            rm,
            Float::acoth_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_acoth_rational_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acoth_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::acoth_rational_prec_round_ref(&{}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn benchmark_float_acoth_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::acoth_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::acoth_rational_prec(Rational, u64)",
                &mut |(n, prec, _)| no_out!(Float::acoth_rational_prec(n, prec)),
            ),
            (
                "Float::acoth_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec, _)| no_out!(Float::acoth_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_acoth_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::acoth_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::acoth_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::acoth_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::acoth_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::acoth_rational_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acoth_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_acoth_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_acoth_rational::<T>(&x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acoth_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acoth_rational::<{}>(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acoth_rational::<T>(&x));
        })],
    );
}
