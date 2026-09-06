// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Sin, SinAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::sin::{rug_sin, rug_sin_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sin);
    register_demo!(runner, demo_float_sin_debug);
    register_demo!(runner, demo_float_sin_extreme);
    register_demo!(runner, demo_float_sin_extreme_debug);
    register_demo!(runner, demo_float_sin_ref);
    register_demo!(runner, demo_float_sin_ref_debug);
    register_demo!(runner, demo_float_sin_assign);
    register_demo!(runner, demo_float_sin_assign_debug);
    register_demo!(runner, demo_float_sin_prec);
    register_demo!(runner, demo_float_sin_prec_debug);
    register_demo!(runner, demo_float_sin_prec_extreme);
    register_demo!(runner, demo_float_sin_prec_ref);
    register_demo!(runner, demo_float_sin_prec_assign);
    register_demo!(runner, demo_float_sin_round);
    register_demo!(runner, demo_float_sin_round_debug);
    register_demo!(runner, demo_float_sin_round_ref);
    register_demo!(runner, demo_float_sin_round_assign);
    register_demo!(runner, demo_float_sin_prec_round);
    register_demo!(runner, demo_float_sin_prec_round_debug);
    register_demo!(runner, demo_float_sin_prec_round_ref);
    register_demo!(runner, demo_float_sin_prec_round_assign);
    register_bench!(runner, benchmark_float_sin_evaluation_strategy);
    register_bench!(runner, benchmark_float_sin_library_comparison);
    register_bench!(runner, benchmark_float_sin_assign);
    register_bench!(runner, benchmark_float_sin_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_sin_prec_round_library_comparison);
}

fn demo_float_sin(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sin() = {}", x_old, x.sin());
    }
}

fn demo_float_sin_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).sin() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.sin())
        );
    }
}

fn demo_float_sin_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sin() = {}", x_old, x.sin());
    }
}

fn demo_float_sin_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).sin() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.sin())
        );
    }
}

fn demo_float_sin_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).sin() = {}", x, (&x).sin());
    }
}

fn demo_float_sin_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).sin() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).sin())
        );
    }
}

fn demo_float_sin_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.sin_assign();
        println!("x := {x_old}; x.sin_assign(); x = {x}");
    }
}

fn demo_float_sin_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.sin_assign();
        println!(
            "x := {:#x}; x.sin_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sin_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sin_prec({}) = {:?}", x_old, prec, x.sin_prec(prec));
    }
}

fn demo_float_sin_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (c, o) = x.sin_prec(prec);
        println!(
            "({:#x}).sin_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).sin_prec({}) = {:?}", x_old, prec, x.sin_prec(prec));
    }
}

fn demo_float_sin_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).sin_prec_ref({}) = {:?}",
            x,
            prec,
            x.sin_prec_ref(prec)
        );
    }
}

fn demo_float_sin_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.sin_prec_assign(prec);
        println!("x := {x_old}; x.sin_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_sin_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).sin_round({}) = {:?}", x_old, rm, x.sin_round(rm));
    }
}

fn demo_float_sin_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.sin_round(rm);
        println!(
            "({:#x}).sin_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).sin_round_ref({}) = {:?}", x, rm, x.sin_round_ref(rm));
    }
}

fn demo_float_sin_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sin_round_assign(rm);
        println!("x := {x_old}; x.sin_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_sin_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).sin_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.sin_prec_round(prec, rm)
        );
    }
}

fn demo_float_sin_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (c, o) = x.sin_prec_round(prec, rm);
        println!(
            "({:#x}).sin_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sin_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.sin_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_sin_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sin_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.sin_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_float_sin_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.sin()", &mut |x| no_out!(x.sin())),
            ("(&Float).sin()", &mut |x| no_out!((&x).sin())),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.sin())),
            ("rug", &mut |(x, _)| no_out!(rug_sin(&x))),
        ],
    );
}

fn benchmark_float_sin_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.sin_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.sin_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.sin_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sin_prec_round(prec, rm)),
            ),
            (
                "(&Float).sin_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sin_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_sin_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.sin_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_sin_prec_round(&x, prec, rm));
            }),
        ],
    );
}
