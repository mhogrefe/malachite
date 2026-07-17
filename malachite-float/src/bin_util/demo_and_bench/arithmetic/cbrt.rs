// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Cbrt, CbrtAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::arithmetic::cbrt::{primitive_float_cbrt, primitive_float_cbrt_rational};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
    pair_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_48, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_37,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_cbrt);
    register_demo!(runner, demo_float_cbrt_debug);
    register_demo!(runner, demo_float_cbrt_ref);
    register_demo!(runner, demo_float_cbrt_ref_debug);
    register_demo!(runner, demo_float_cbrt_assign);
    register_demo!(runner, demo_float_cbrt_assign_debug);
    register_demo!(runner, demo_float_cbrt_prec);
    register_demo!(runner, demo_float_cbrt_prec_debug);
    register_demo!(runner, demo_float_cbrt_prec_ref);
    register_demo!(runner, demo_float_cbrt_prec_ref_debug);
    register_demo!(runner, demo_float_cbrt_prec_assign);
    register_demo!(runner, demo_float_cbrt_prec_assign_debug);
    register_demo!(runner, demo_float_cbrt_round);
    register_demo!(runner, demo_float_cbrt_round_debug);
    register_demo!(runner, demo_float_cbrt_round_ref);
    register_demo!(runner, demo_float_cbrt_round_ref_debug);
    register_demo!(runner, demo_float_cbrt_round_assign);
    register_demo!(runner, demo_float_cbrt_round_assign_debug);
    register_demo!(runner, demo_float_cbrt_prec_round);
    register_demo!(runner, demo_float_cbrt_prec_round_debug);
    register_demo!(runner, demo_float_cbrt_prec_round_ref);
    register_demo!(runner, demo_float_cbrt_prec_round_ref_debug);
    register_demo!(runner, demo_float_cbrt_prec_round_assign);
    register_demo!(runner, demo_float_cbrt_prec_round_assign_debug);
    register_demo!(runner, demo_float_cbrt_rational_prec);
    register_demo!(runner, demo_float_cbrt_rational_prec_debug);
    register_demo!(runner, demo_float_cbrt_rational_prec_ref);
    register_demo!(runner, demo_float_cbrt_rational_prec_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_cbrt);
    register_primitive_float_demos!(runner, demo_primitive_float_cbrt_rational);

    register_bench!(runner, benchmark_float_cbrt_evaluation_strategy);
    register_bench!(runner, benchmark_float_cbrt_assign);
    register_bench!(runner, benchmark_float_cbrt_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_cbrt_prec_assign);
    register_bench!(runner, benchmark_float_cbrt_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_cbrt_prec_round_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_float_cbrt_rational_prec_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_cbrt);
    register_primitive_float_benches!(runner, benchmark_primitive_float_cbrt_rational);
}

fn demo_float_cbrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cbrt() = {}", x_old, x.cbrt());
    }
}

fn demo_float_cbrt_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).cbrt() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.cbrt())
        );
    }
}

fn demo_float_cbrt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).cbrt() = {}", x, (&x).cbrt());
    }
}

fn demo_float_cbrt_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).cbrt() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).cbrt())
        );
    }
}

fn demo_float_cbrt_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.cbrt_assign();
        println!("x := {x_old}; x.cbrt_assign(); x = {x}");
    }
}

fn demo_float_cbrt_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.cbrt_assign();
        println!(
            "x := {:#x}; x.cbrt_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_cbrt_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cbrt_prec({}) = {:?}", x_old, prec, x.cbrt_prec(prec));
    }
}

fn demo_float_cbrt_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (cbrt, o) = x.cbrt_prec(prec);
        println!(
            "({:#x}).cbrt_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).cbrt_prec_ref({}) = {:?}",
            x,
            prec,
            x.cbrt_prec_ref(prec)
        );
    }
}

fn demo_float_cbrt_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (cbrt, o) = x.cbrt_prec_ref(prec);
        println!(
            "(&{:#x}).cbrt_prec_ref({}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            prec,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.cbrt_prec_assign(prec);
        println!("x := {x_old}; x.cbrt_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_cbrt_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.cbrt_prec_assign(prec);
        println!(
            "x := {:#x}; x.cbrt_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_cbrt_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).cbrt_round({}) = {:?}", x_old, rm, x.cbrt_round(rm));
    }
}

fn demo_float_cbrt_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (cbrt, o) = x.cbrt_round(rm);
        println!(
            "({:#x}).cbrt_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).cbrt_round_ref({}) = {:?}",
            x,
            rm,
            x.cbrt_round_ref(rm)
        );
    }
}

fn demo_float_cbrt_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let (cbrt, o) = x.cbrt_round_ref(rm);
        println!(
            "(&{:#x}).cbrt_round_ref({}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            rm,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cbrt_round_assign(rm);
        println!("x := {x_old}; x.cbrt_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_cbrt_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_48()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cbrt_round_assign(rm);
        println!(
            "x := {:#x}; x.cbrt_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_cbrt_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).cbrt_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.cbrt_prec_round(prec, rm)
        );
    }
}

fn demo_float_cbrt_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (cbrt, o) = x.cbrt_prec_round(prec, rm);
        println!(
            "({:#x}).cbrt_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).cbrt_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.cbrt_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_cbrt_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let (cbrt, o) = x.cbrt_prec_round_ref(prec, rm);
        println!(
            "(&{:#x}).cbrt_prec_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            prec,
            rm,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cbrt_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.cbrt_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_cbrt_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cbrt_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.cbrt_prec_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_cbrt_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::cbrt_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::cbrt_rational_prec(n, p)
        );
    }
}

fn demo_float_cbrt_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (cbrt, o) = Float::cbrt_rational_prec(n.clone(), p);
        println!(
            "Float::cbrt_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(cbrt),
            o
        );
    }
}

fn demo_float_cbrt_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::cbrt_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::cbrt_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_cbrt_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (cbrt, o) = Float::cbrt_rational_prec_ref(&n, p);
        println!(
            "Float::cbrt_rational_prec_ref(&{}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(cbrt),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_cbrt<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_cbrt({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_cbrt::<T>(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_cbrt_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_cbrt_rational({}) = {}",
            x,
            NiceFloat(primitive_float_cbrt_rational::<T>(&x))
        );
    }
}

fn benchmark_float_cbrt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cbrt()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.cbrt()", &mut |x| no_out!(x.cbrt())),
            ("(&Float).cbrt()", &mut |x| no_out!((&x).cbrt())),
        ],
    );
}

fn benchmark_float_cbrt_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.cbrt_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.cbrt_assign()", &mut |mut x| x.cbrt_assign())],
    );
}

fn benchmark_float_cbrt_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cbrt_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.cbrt_prec(u64)", &mut |(x, prec)| {
                no_out!(x.cbrt_prec(prec));
            }),
            ("(&Float).cbrt_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.cbrt_prec_ref(prec));
            }),
        ],
    );
}

fn benchmark_float_cbrt_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cbrt_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Float.cbrt_prec_assign(u64)", &mut |(mut x, prec)| {
            no_out!(x.cbrt_prec_assign(prec));
        })],
    );
}

fn benchmark_float_cbrt_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cbrt_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_48().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.cbrt_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.cbrt_round(rm));
            }),
            ("(&Float).cbrt_round_ref(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.cbrt_round_ref(rm));
            }),
        ],
    );
}

fn benchmark_float_cbrt_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cbrt_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_37().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.cbrt_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.cbrt_prec_round(prec, rm));
                },
            ),
            (
                "(&Float).cbrt_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.cbrt_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_cbrt_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cbrt_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("x", "prec"),
        &mut [
            ("Float.cbrt_rational_prec(Rational, u64)", &mut |(n, p)| {
                no_out!(Float::cbrt_rational_prec(n, p));
            }),
            (
                "Float.cbrt_rational_prec_ref(&Rational, u64)",
                &mut |(n, p)| no_out!(Float::cbrt_rational_prec_ref(&n, p)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_cbrt<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_cbrt({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| no_out!(primitive_float_cbrt::<T>(x)))],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_cbrt_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_cbrt_rational({})", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_cbrt_rational::<T>(&x));
        })],
    );
}
