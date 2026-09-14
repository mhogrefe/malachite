// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Cot, CotAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::cot::primitive_float_cot;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::cot::{rug_cot, rug_cot_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_cot);
    register_demo!(runner, demo_float_cot_debug);
    register_demo!(runner, demo_float_cot_extreme);
    register_demo!(runner, demo_float_cot_extreme_debug);
    register_demo!(runner, demo_float_cot_ref);
    register_demo!(runner, demo_float_cot_ref_debug);
    register_demo!(runner, demo_float_cot_assign);
    register_demo!(runner, demo_float_cot_assign_debug);
    register_demo!(runner, demo_float_cot_prec);
    register_demo!(runner, demo_float_cot_prec_debug);
    register_demo!(runner, demo_float_cot_prec_extreme);
    register_demo!(runner, demo_float_cot_prec_ref);
    register_demo!(runner, demo_float_cot_prec_assign);
    register_demo!(runner, demo_float_cot_round);
    register_demo!(runner, demo_float_cot_round_debug);
    register_demo!(runner, demo_float_cot_round_ref);
    register_demo!(runner, demo_float_cot_round_assign);
    register_demo!(runner, demo_float_cot_prec_round);
    register_demo!(runner, demo_float_cot_prec_round_debug);
    register_demo!(runner, demo_float_cot_prec_round_ref);
    register_demo!(runner, demo_float_cot_prec_round_assign);
    register_bench!(runner, benchmark_float_cot_evaluation_strategy);
    register_bench!(runner, benchmark_float_cot_library_comparison);
    register_bench!(runner, benchmark_float_cot_assign);
    register_bench!(runner, benchmark_float_cot_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_cot_prec_round_library_comparison);
    register_primitive_float_demos!(runner, demo_primitive_float_cot);
    register_primitive_float_benches!(runner, benchmark_primitive_float_cot);
}

fn demo_float_cot(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cot() = {}", x_old, x.cot());
    }
}

fn demo_float_cot_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).cot() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.cot())
        );
    }
}

fn demo_float_cot_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cot() = {}", x_old, x.cot());
    }
}

fn demo_float_cot_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).cot() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.cot())
        );
    }
}

fn demo_float_cot_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).cot() = {}", x, (&x).cot());
    }
}

fn demo_float_cot_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).cot() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).cot())
        );
    }
}

fn demo_float_cot_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.cot_assign();
        println!("x := {x_old}; x.cot_assign(); x = {x}");
    }
}

fn demo_float_cot_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.cot_assign();
        println!(
            "x := {:#x}; x.cot_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_cot_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cot_prec({}) = {:?}", x_old, prec, x.cot_prec(prec));
    }
}

fn demo_float_cot_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.cot_prec(prec);
        println!(
            "({:#x}).cot_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_cot_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).cot_prec({}) = {:?}", x_old, prec, x.cot_prec(prec));
    }
}

fn demo_float_cot_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).cot_prec_ref({}) = {:?}",
            x,
            prec,
            x.cot_prec_ref(prec)
        );
    }
}

fn demo_float_cot_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.cot_prec_assign(prec);
        println!("x := {x_old}; x.cot_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_cot_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).cot_round({}) = {:?}", x_old, rm, x.cot_round(rm));
    }
}

fn demo_float_cot_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.cot_round(rm);
        println!(
            "({:#x}).cot_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_cot_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).cot_round_ref({}) = {:?}", x, rm, x.cot_round_ref(rm));
    }
}

fn demo_float_cot_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cot_round_assign(rm);
        println!("x := {x_old}; x.cot_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_cot_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).cot_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.cot_prec_round(prec, rm)
        );
    }
}

fn demo_float_cot_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.cot_prec_round(prec, rm);
        println!(
            "({:#x}).cot_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_cot_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).cot_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.cot_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_cot_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.cot_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.cot_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_cot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cot()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.cot()", &mut |x| no_out!(x.cot())),
            ("(&Float).cot()", &mut |x| no_out!((&x).cot())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_cot_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cot()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.cot())),
            ("rug", &mut |(x, _)| no_out!(rug_cot(&x))),
        ],
    );
}

fn benchmark_float_cot_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.cot_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.cot_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_cot_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cot_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.cot_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.cot_prec_round(prec, rm)),
            ),
            (
                "(&Float).cot_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.cot_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_cot_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.cot_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.cot_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_cot_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_cot<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_cot({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_cot(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_cot<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_cot({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_cot(x));
        })],
    );
}
