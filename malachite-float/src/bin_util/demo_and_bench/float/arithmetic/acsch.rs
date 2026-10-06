// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Acsch, AcschAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::acsch::{
    primitive_float_acsch, primitive_float_acsch_rational,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_acsch_rational_prec);
    register_demo!(runner, demo_float_acsch_rational_prec_debug);
    register_demo!(runner, demo_float_acsch_rational_prec_ref);
    register_demo!(runner, demo_float_acsch_rational_prec_ref_debug);
    register_demo!(runner, demo_float_acsch_rational_prec_round);
    register_demo!(runner, demo_float_acsch_rational_prec_round_debug);
    register_demo!(runner, demo_float_acsch_rational_prec_round_ref);
    register_demo!(runner, demo_float_acsch_rational_prec_round_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_acsch_rational);
    register_bench!(
        runner,
        benchmark_float_acsch_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_acsch_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_acsch_rational);
    register_demo!(runner, demo_float_acsch);
    register_demo!(runner, demo_float_acsch_debug);
    register_demo!(runner, demo_float_acsch_extreme);
    register_demo!(runner, demo_float_acsch_extreme_debug);
    register_demo!(runner, demo_float_acsch_ref);
    register_demo!(runner, demo_float_acsch_ref_debug);
    register_demo!(runner, demo_float_acsch_assign);
    register_demo!(runner, demo_float_acsch_assign_debug);
    register_demo!(runner, demo_float_acsch_prec);
    register_demo!(runner, demo_float_acsch_prec_debug);
    register_demo!(runner, demo_float_acsch_prec_extreme);
    register_demo!(runner, demo_float_acsch_prec_ref);
    register_demo!(runner, demo_float_acsch_prec_assign);
    register_demo!(runner, demo_float_acsch_round);
    register_demo!(runner, demo_float_acsch_round_debug);
    register_demo!(runner, demo_float_acsch_round_ref);
    register_demo!(runner, demo_float_acsch_round_assign);
    register_demo!(runner, demo_float_acsch_prec_round);
    register_demo!(runner, demo_float_acsch_prec_round_debug);
    register_demo!(runner, demo_float_acsch_prec_round_ref);
    register_demo!(runner, demo_float_acsch_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_acsch);

    register_bench!(runner, benchmark_float_acsch_evaluation_strategy);
    register_bench!(runner, benchmark_float_acsch_assign);
    register_bench!(runner, benchmark_float_acsch_prec_round_evaluation_strategy);
    register_primitive_float_benches!(runner, benchmark_primitive_float_acsch);
}

fn demo_float_acsch(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acsch() = {}", x_old, x.acsch());
    }
}

fn demo_float_acsch_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).acsch() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.acsch())
        );
    }
}

fn demo_float_acsch_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).acsch() = {}", x_old, x.acsch());
    }
}

fn demo_float_acsch_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).acsch() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.acsch())
        );
    }
}

fn demo_float_acsch_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).acsch() = {}", x, (&x).acsch());
    }
}

fn demo_float_acsch_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).acsch() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).acsch())
        );
    }
}

fn demo_float_acsch_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.acsch_assign();
        println!("x := {x_old}; x.acsch_assign(); x = {x}");
    }
}

fn demo_float_acsch_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.acsch_assign();
        println!(
            "x := {:#x}; x.acsch_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_acsch_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).acsch_prec({}) = {:?}",
            x_old,
            prec,
            x.acsch_prec(prec)
        );
    }
}

fn demo_float_acsch_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.acsch_prec(prec);
        println!(
            "({:#x}).acsch_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acsch_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).acsch_prec({}) = {:?}",
            x_old,
            prec,
            x.acsch_prec(prec)
        );
    }
}

fn demo_float_acsch_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).acsch_prec_ref({}) = {:?}",
            x,
            prec,
            x.acsch_prec_ref(prec)
        );
    }
}

fn demo_float_acsch_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.acsch_prec_assign(prec);
        println!("x := {x_old}; x.acsch_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_acsch_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).acsch_round({}) = {:?}", x_old, rm, x.acsch_round(rm));
    }
}

fn demo_float_acsch_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.acsch_round(rm);
        println!(
            "({:#x}).acsch_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acsch_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).acsch_round_ref({}) = {:?}",
            x,
            rm,
            x.acsch_round_ref(rm)
        );
    }
}

fn demo_float_acsch_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acsch_round_assign(rm);
        println!("x := {x_old}; x.acsch_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_acsch_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).acsch_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.acsch_prec_round(prec, rm)
        );
    }
}

fn demo_float_acsch_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.acsch_prec_round(prec, rm);
        println!(
            "({:#x}).acsch_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_acsch_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).acsch_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.acsch_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_acsch_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.acsch_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.acsch_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_acsch_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acsch()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.acsch()", &mut |x| no_out!(x.acsch())),
            ("(&Float).acsch()", &mut |x| no_out!((&x).acsch())),
        ],
    );
}

fn benchmark_float_acsch_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.acsch_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.acsch_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_acsch_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.acsch_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.acsch_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acsch_prec_round(prec, rm)),
            ),
            (
                "(&Float).acsch_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.acsch_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acsch<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_acsch({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_acsch(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acsch<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acsch({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acsch(x));
        })],
    );
}

fn demo_float_acsch_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acsch_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::acsch_rational_prec(n, p)
        );
    }
}

fn demo_float_acsch_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acsch_rational_prec(n.clone(), p);
        println!(
            "Float::acsch_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_acsch_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acsch_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::acsch_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_acsch_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acsch_rational_prec_ref(&n, p);
        println!(
            "Float::acsch_rational_prec_ref(&{}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_acsch_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acsch_rational_prec_round({}, {}, {}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::acsch_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_acsch_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acsch_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::acsch_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_acsch_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::acsch_rational_prec_round_ref(&{}, {}, {}) = {:?}",
            n,
            p,
            rm,
            Float::acsch_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_acsch_rational_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::acsch_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::acsch_rational_prec_round_ref(&{}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn benchmark_float_acsch_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::acsch_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::acsch_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::acsch_rational_prec(n, prec)),
            ),
            (
                "Float::acsch_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::acsch_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_acsch_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::acsch_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::acsch_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::acsch_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::acsch_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::acsch_rational_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_acsch_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_acsch_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_acsch_rational::<T>(&x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_acsch_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_acsch_rational::<{}>(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_acsch_rational::<T>(&x));
        })],
    );
}
