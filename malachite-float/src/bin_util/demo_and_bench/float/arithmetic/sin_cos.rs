// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Cos, Sin, SinCos, SinCosAssign};
use malachite_base::num::basic::traits::NaN;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::sin_cos::{rug_sin_cos, rug_sin_cos_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sin_cos);
    register_demo!(runner, demo_float_sin_cos_debug);
    register_demo!(runner, demo_float_sin_cos_extreme);
    register_demo!(runner, demo_float_sin_cos_ref);
    register_demo!(runner, demo_float_sin_cos_assign);
    register_demo!(runner, demo_float_sin_cos_prec);
    register_demo!(runner, demo_float_sin_cos_prec_ref);
    register_demo!(runner, demo_float_sin_cos_prec_assign);
    register_demo!(runner, demo_float_sin_cos_round);
    register_demo!(runner, demo_float_sin_cos_round_ref);
    register_demo!(runner, demo_float_sin_cos_round_assign);
    register_demo!(runner, demo_float_sin_cos_prec_round);
    register_demo!(runner, demo_float_sin_cos_prec_round_debug);
    register_demo!(runner, demo_float_sin_cos_prec_round_ref);
    register_demo!(runner, demo_float_sin_cos_prec_round_assign);

    register_bench!(runner, benchmark_float_sin_cos_evaluation_strategy);
    register_bench!(runner, benchmark_float_sin_cos_library_comparison);
    register_bench!(runner, benchmark_float_sin_cos_algorithms);
    register_bench!(runner, benchmark_float_sin_cos_assign);
    register_bench!(
        runner,
        benchmark_float_sin_cos_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sin_cos_prec_round_library_comparison
    );
}

fn demo_float_sin_cos(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sin_cos() = {:?}", x_old, x.sin_cos());
    }
}

fn demo_float_sin_cos_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (s, c) = x.sin_cos();
        println!(
            "({:#x}).sin_cos() = ({:#x}, {:#x})",
            ComparableFloat(x_old),
            ComparableFloat(s),
            ComparableFloat(c)
        );
    }
}

fn demo_float_sin_cos_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sin_cos() = {:?}", x_old, x.sin_cos());
    }
}

fn demo_float_sin_cos_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).sin_cos() = {:?}", x, (&x).sin_cos());
    }
}

fn demo_float_sin_cos_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let mut c = Float::NAN;
        x.sin_cos_assign(&mut c);
        println!("x := {x_old}; x.sin_cos_assign(&mut c); x = {x}; c = {c}");
    }
}

fn demo_float_sin_cos_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).sin_cos_prec({}) = {:?}",
            x_old,
            prec,
            x.sin_cos_prec(prec)
        );
    }
}

fn demo_float_sin_cos_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).sin_cos_prec_ref({}) = {:?}",
            x,
            prec,
            x.sin_cos_prec_ref(prec)
        );
    }
}

fn demo_float_sin_cos_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let mut c = Float::NAN;
        let o = x.sin_cos_prec_assign(&mut c, prec);
        println!("x := {x_old}; x.sin_cos_prec_assign(&mut c, {prec}) = {o:?}; x = {x}; c = {c}");
    }
}

fn demo_float_sin_cos_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sin_cos_round({}) = {:?}",
            x_old,
            rm,
            x.sin_cos_round(rm)
        );
    }
}

fn demo_float_sin_cos_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sin_cos_round_ref({}) = {:?}",
            x,
            rm,
            x.sin_cos_round_ref(rm)
        );
    }
}

fn demo_float_sin_cos_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let mut c = Float::NAN;
        let o = x.sin_cos_round_assign(&mut c, rm);
        println!("x := {x_old}; x.sin_cos_round_assign(&mut c, {rm}) = {o:?}; x = {x}; c = {c}");
    }
}

fn demo_float_sin_cos_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sin_cos_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.sin_cos_prec_round(prec, rm)
        );
    }
}

fn demo_float_sin_cos_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (s, c, o_s, o_c) = x.sin_cos_prec_round(prec, rm);
        println!(
            "({:#x}).sin_cos_prec_round({}, {}) = ({:#x}, {:#x}, {:?}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(s),
            ComparableFloat(c),
            o_s,
            o_c
        );
    }
}

fn demo_float_sin_cos_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sin_cos_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.sin_cos_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_sin_cos_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let mut c = Float::NAN;
        let o = x.sin_cos_prec_round_assign(&mut c, prec, rm);
        println!(
            "x := {x_old}; x.sin_cos_prec_round_assign(&mut c, {prec}, {rm}) = {o:?}; x = {x}; \
             c = {c}"
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_float_sin_cos_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_cos()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.sin_cos()", &mut |x| no_out!(x.sin_cos())),
            ("(&Float).sin_cos()", &mut |x| no_out!((&x).sin_cos())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_cos_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_cos()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.sin_cos())),
            ("rug", &mut |(x, _)| no_out!(rug_sin_cos(&x))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_cos_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_cos()",
        BenchmarkType::Algorithms,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("together", &mut |x| no_out!(x.sin_cos())),
            ("separately", &mut |x| no_out!(((&x).sin(), x.cos()))),
        ],
    );
}

fn benchmark_float_sin_cos_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.sin_cos_assign(&mut Float)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| {
            let mut c = Float::NAN;
            x.sin_cos_assign(&mut c);
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_cos_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_cos_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.sin_cos_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sin_cos_prec_round(prec, rm)),
            ),
            (
                "(&Float).sin_cos_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sin_cos_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_cos_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_cos_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.sin_cos_prec_round(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_sin_cos_prec_round(&x, prec, rm));
            }),
        ],
    );
}
