// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Csc, CscAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::csc::primitive_float_csc;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::csc::{rug_csc, rug_csc_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_csc);
    register_demo!(runner, demo_float_csc_debug);
    register_demo!(runner, demo_float_csc_extreme);
    register_demo!(runner, demo_float_csc_extreme_debug);
    register_demo!(runner, demo_float_csc_ref);
    register_demo!(runner, demo_float_csc_ref_debug);
    register_demo!(runner, demo_float_csc_assign);
    register_demo!(runner, demo_float_csc_assign_debug);
    register_demo!(runner, demo_float_csc_prec);
    register_demo!(runner, demo_float_csc_prec_debug);
    register_demo!(runner, demo_float_csc_prec_extreme);
    register_demo!(runner, demo_float_csc_prec_ref);
    register_demo!(runner, demo_float_csc_prec_assign);
    register_demo!(runner, demo_float_csc_round);
    register_demo!(runner, demo_float_csc_round_debug);
    register_demo!(runner, demo_float_csc_round_ref);
    register_demo!(runner, demo_float_csc_round_assign);
    register_demo!(runner, demo_float_csc_prec_round);
    register_demo!(runner, demo_float_csc_prec_round_debug);
    register_demo!(runner, demo_float_csc_prec_round_ref);
    register_demo!(runner, demo_float_csc_prec_round_assign);
    register_bench!(runner, benchmark_float_csc_evaluation_strategy);
    register_bench!(runner, benchmark_float_csc_library_comparison);
    register_bench!(runner, benchmark_float_csc_assign);
    register_bench!(runner, benchmark_float_csc_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_csc_prec_round_library_comparison);
    register_primitive_float_demos!(runner, demo_primitive_float_csc);
    register_primitive_float_benches!(runner, benchmark_primitive_float_csc);
}

fn demo_float_csc(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).csc() = {}", x_old, x.csc());
    }
}

fn demo_float_csc_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).csc() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.csc())
        );
    }
}

fn demo_float_csc_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).csc() = {}", x_old, x.csc());
    }
}

fn demo_float_csc_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).csc() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.csc())
        );
    }
}

fn demo_float_csc_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).csc() = {}", x, (&x).csc());
    }
}

fn demo_float_csc_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).csc() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).csc())
        );
    }
}

fn demo_float_csc_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.csc_assign();
        println!("x := {x_old}; x.csc_assign(); x = {x}");
    }
}

fn demo_float_csc_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.csc_assign();
        println!(
            "x := {:#x}; x.csc_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_csc_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).csc_prec({}) = {:?}", x_old, prec, x.csc_prec(prec));
    }
}

fn demo_float_csc_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.csc_prec(prec);
        println!(
            "({:#x}).csc_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_csc_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).csc_prec({}) = {:?}", x_old, prec, x.csc_prec(prec));
    }
}

fn demo_float_csc_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).csc_prec_ref({}) = {:?}",
            x,
            prec,
            x.csc_prec_ref(prec)
        );
    }
}

fn demo_float_csc_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.csc_prec_assign(prec);
        println!("x := {x_old}; x.csc_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_csc_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).csc_round({}) = {:?}", x_old, rm, x.csc_round(rm));
    }
}

fn demo_float_csc_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.csc_round(rm);
        println!(
            "({:#x}).csc_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_csc_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).csc_round_ref({}) = {:?}", x, rm, x.csc_round_ref(rm));
    }
}

fn demo_float_csc_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.csc_round_assign(rm);
        println!("x := {x_old}; x.csc_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_csc_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).csc_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.csc_prec_round(prec, rm)
        );
    }
}

fn demo_float_csc_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.csc_prec_round(prec, rm);
        println!(
            "({:#x}).csc_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_csc_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).csc_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.csc_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_csc_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.csc_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.csc_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_csc_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.csc()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.csc()", &mut |x| no_out!(x.csc())),
            ("(&Float).csc()", &mut |x| no_out!((&x).csc())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_csc_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.csc()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.csc())),
            ("rug", &mut |(x, _)| no_out!(rug_csc(&x))),
        ],
    );
}

fn benchmark_float_csc_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.csc_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.csc_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_csc_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.csc_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.csc_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.csc_prec_round(prec, rm)),
            ),
            (
                "(&Float).csc_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.csc_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_csc_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.csc_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.csc_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_csc_prec_round(&x, prec, rm));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_csc<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_csc({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_csc(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_csc<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_csc({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_csc(x));
        })],
    );
}
