// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Cosh, CoshAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::cosh::primitive_float_cosh;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::cosh::{rug_cosh, rug_cosh_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_cosh);
    register_demo!(runner, demo_float_cosh_debug);
    register_demo!(runner, demo_float_cosh_extreme);
    register_demo!(runner, demo_float_cosh_extreme_debug);
    register_demo!(runner, demo_float_cosh_ref);
    register_demo!(runner, demo_float_cosh_ref_debug);
    register_demo!(runner, demo_float_cosh_assign);
    register_demo!(runner, demo_float_cosh_assign_debug);
    register_demo!(runner, demo_float_cosh_prec);
    register_demo!(runner, demo_float_cosh_prec_debug);
    register_demo!(runner, demo_float_cosh_prec_extreme);
    register_demo!(runner, demo_float_cosh_prec_ref);
    register_demo!(runner, demo_float_cosh_prec_assign);
    register_demo!(runner, demo_float_cosh_round);
    register_demo!(runner, demo_float_cosh_round_debug);
    register_demo!(runner, demo_float_cosh_round_ref);
    register_demo!(runner, demo_float_cosh_round_assign);
    register_demo!(runner, demo_float_cosh_prec_round);
    register_demo!(runner, demo_float_cosh_prec_round_debug);
    register_demo!(runner, demo_float_cosh_prec_round_ref);
    register_demo!(runner, demo_float_cosh_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_cosh);

    register_bench!(runner, benchmark_float_cosh_evaluation_strategy);
    register_bench!(runner, benchmark_float_cosh_library_comparison);
    register_bench!(runner, benchmark_float_cosh_assign);
    register_bench!(runner, benchmark_float_cosh_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_cosh_prec_round_library_comparison);
    register_primitive_float_benches!(runner, benchmark_primitive_float_cosh);
}

fn demo_float_cosh(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cosh() = {}", x_old, x.cosh());
    }
}

fn demo_float_cosh_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).cosh() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.cosh())
        );
    }
}

fn demo_float_cosh_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cosh() = {}", x_old, x.cosh());
    }
}

fn demo_float_cosh_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).cosh() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.cosh())
        );
    }
}

fn demo_float_cosh_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).cosh() = {}", x, (&x).cosh());
    }
}

fn demo_float_cosh_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).cosh() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).cosh())
        );
    }
}

fn demo_float_cosh_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.cosh_assign();
        println!("x := {x_old}; x.cosh_assign(); x = {x}");
    }
}

fn demo_float_cosh_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.cosh_assign();
        println!(
            "x := {:#x}; x.cosh_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_cosh_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cosh_prec({}) = {:?}", x_old, prec, x.cosh_prec(prec));
    }
}

fn demo_float_cosh_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.cosh_prec(prec);
        println!(
            "({:#x}).cosh_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_cosh_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cosh_prec({}) = {:?}", x_old, prec, x.cosh_prec(prec));
    }
}

fn demo_float_cosh_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).cosh_prec_ref({}) = {:?}",
            x,
            prec,
            x.cosh_prec_ref(prec)
        );
    }
}

fn demo_float_cosh_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.cosh_prec_assign(prec);
        println!("x := {x_old}; x.cosh_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_cosh_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).cosh_round({}) = {:?}", x_old, rm, x.cosh_round(rm));
    }
}

fn demo_float_cosh_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.cosh_round(rm);
        println!(
            "({:#x}).cosh_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_cosh_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).cosh_round_ref({}) = {:?}",
            x,
            rm,
            x.cosh_round_ref(rm)
        );
    }
}

fn demo_float_cosh_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cosh_round_assign(rm);
        println!("x := {x_old}; x.cosh_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_cosh_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).cosh_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.cosh_prec_round(prec, rm)
        );
    }
}

fn demo_float_cosh_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.cosh_prec_round(prec, rm);
        println!(
            "({:#x}).cosh_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_cosh_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).cosh_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.cosh_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_cosh_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cosh_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.cosh_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_cosh_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cosh()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.cosh()", &mut |x| no_out!(x.cosh())),
            ("(&Float).cosh()", &mut |x| no_out!((&x).cosh())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_cosh_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cosh()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.cosh())),
            ("rug", &mut |(x, _)| no_out!(rug_cosh(&x))),
        ],
    );
}

fn benchmark_float_cosh_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.cosh_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.cosh_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_cosh_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cosh_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.cosh_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.cosh_prec_round(prec, rm)),
            ),
            (
                "(&Float).cosh_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.cosh_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_cosh_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cosh_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.cosh_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_cosh_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_cosh<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_cosh({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_cosh(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_cosh<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_cosh({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_cosh(x));
        })],
    );
}
