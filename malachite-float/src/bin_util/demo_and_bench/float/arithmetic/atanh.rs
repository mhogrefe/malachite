// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Atanh, AtanhAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::atanh::primitive_float_atanh;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::atanh::{rug_atanh, rug_atanh_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_atanh);
    register_demo!(runner, demo_float_atanh_debug);
    register_demo!(runner, demo_float_atanh_extreme);
    register_demo!(runner, demo_float_atanh_extreme_debug);
    register_demo!(runner, demo_float_atanh_ref);
    register_demo!(runner, demo_float_atanh_ref_debug);
    register_demo!(runner, demo_float_atanh_assign);
    register_demo!(runner, demo_float_atanh_assign_debug);
    register_demo!(runner, demo_float_atanh_prec);
    register_demo!(runner, demo_float_atanh_prec_debug);
    register_demo!(runner, demo_float_atanh_prec_extreme);
    register_demo!(runner, demo_float_atanh_prec_ref);
    register_demo!(runner, demo_float_atanh_prec_assign);
    register_demo!(runner, demo_float_atanh_round);
    register_demo!(runner, demo_float_atanh_round_debug);
    register_demo!(runner, demo_float_atanh_round_ref);
    register_demo!(runner, demo_float_atanh_round_assign);
    register_demo!(runner, demo_float_atanh_prec_round);
    register_demo!(runner, demo_float_atanh_prec_round_debug);
    register_demo!(runner, demo_float_atanh_prec_round_ref);
    register_demo!(runner, demo_float_atanh_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_atanh);

    register_bench!(runner, benchmark_float_atanh_evaluation_strategy);
    register_bench!(runner, benchmark_float_atanh_library_comparison);
    register_bench!(runner, benchmark_float_atanh_assign);
    register_bench!(runner, benchmark_float_atanh_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_atanh_prec_round_library_comparison);
    register_primitive_float_benches!(runner, benchmark_primitive_float_atanh);
}

fn demo_float_atanh(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atanh() = {}", x_old, x.atanh());
    }
}

fn demo_float_atanh_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).atanh() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.atanh())
        );
    }
}

fn demo_float_atanh_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).atanh() = {}", x_old, x.atanh());
    }
}

fn demo_float_atanh_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).atanh() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.atanh())
        );
    }
}

fn demo_float_atanh_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).atanh() = {}", x, (&x).atanh());
    }
}

fn demo_float_atanh_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).atanh() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).atanh())
        );
    }
}

fn demo_float_atanh_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.atanh_assign();
        println!("x := {x_old}; x.atanh_assign(); x = {x}");
    }
}

fn demo_float_atanh_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.atanh_assign();
        println!(
            "x := {:#x}; x.atanh_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_atanh_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).atanh_prec({}) = {:?}",
            x_old,
            prec,
            x.atanh_prec(prec)
        );
    }
}

fn demo_float_atanh_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.atanh_prec(prec);
        println!(
            "({:#x}).atanh_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atanh_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).atanh_prec({}) = {:?}",
            x_old,
            prec,
            x.atanh_prec(prec)
        );
    }
}

fn demo_float_atanh_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).atanh_prec_ref({}) = {:?}",
            x,
            prec,
            x.atanh_prec_ref(prec)
        );
    }
}

fn demo_float_atanh_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.atanh_prec_assign(prec);
        println!("x := {x_old}; x.atanh_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_atanh_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).atanh_round({}) = {:?}", x_old, rm, x.atanh_round(rm));
    }
}

fn demo_float_atanh_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.atanh_round(rm);
        println!(
            "({:#x}).atanh_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atanh_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atanh_round_ref({}) = {:?}",
            x,
            rm,
            x.atanh_round_ref(rm)
        );
    }
}

fn demo_float_atanh_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atanh_round_assign(rm);
        println!("x := {x_old}; x.atanh_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_atanh_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).atanh_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.atanh_prec_round(prec, rm)
        );
    }
}

fn demo_float_atanh_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.atanh_prec_round(prec, rm);
        println!(
            "({:#x}).atanh_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_atanh_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).atanh_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.atanh_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_atanh_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.atanh_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.atanh_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_atanh_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atanh()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.atanh()", &mut |x| no_out!(x.atanh())),
            ("(&Float).atanh()", &mut |x| no_out!((&x).atanh())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_atanh_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atanh()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.atanh())),
            ("rug", &mut |(x, _)| no_out!(rug_atanh(&x))),
        ],
    );
}

fn benchmark_float_atanh_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.atanh_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.atanh_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_atanh_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atanh_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.atanh_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.atanh_prec_round(prec, rm)),
            ),
            (
                "(&Float).atanh_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.atanh_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_atanh_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.atanh_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.atanh_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_atanh_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_atanh<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_atanh({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_atanh(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_atanh<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_atanh({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_atanh(x));
        })],
    );
}
