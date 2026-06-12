// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Ln1PlusX, Ln1PlusXAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::ln_1_plus_x::{
    rug_ln_1_plus_x, rug_ln_1_plus_x_prec, rug_ln_1_plus_x_prec_round, rug_ln_1_plus_x_round,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_pair_1_float_complexity_bucketer,
    pair_2_pair_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    pair_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_36,
    float_rounding_mode_pair_gen_var_36_rm, float_rounding_mode_pair_gen_var_37,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_1_rm, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_21,
    float_unsigned_rounding_mode_triple_gen_var_21_rm,
    float_unsigned_rounding_mode_triple_gen_var_22,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_ln_1_plus_x);
    register_demo!(runner, demo_float_ln_1_plus_x_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_extreme);
    register_demo!(runner, demo_float_ln_1_plus_x_extreme_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_ref);
    register_demo!(runner, demo_float_ln_1_plus_x_ref_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_assign);
    register_demo!(runner, demo_float_ln_1_plus_x_assign_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_extreme);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_extreme_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_ref);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_ref_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_assign);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_assign_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_round);
    register_demo!(runner, demo_float_ln_1_plus_x_round_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_round_extreme);
    register_demo!(runner, demo_float_ln_1_plus_x_round_extreme_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_round_ref);
    register_demo!(runner, demo_float_ln_1_plus_x_round_ref_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_round_assign);
    register_demo!(runner, demo_float_ln_1_plus_x_round_assign_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_extreme);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_extreme_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_ref);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_ref_debug);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_assign);
    register_demo!(runner, demo_float_ln_1_plus_x_prec_round_assign_debug);

    register_bench!(runner, benchmark_float_ln_1_plus_x_evaluation_strategy);
    register_bench!(runner, benchmark_float_ln_1_plus_x_library_comparison);
    register_bench!(runner, benchmark_float_ln_1_plus_x_assign);
    register_bench!(runner, benchmark_float_ln_1_plus_x_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_ln_1_plus_x_prec_library_comparison);
    register_bench!(runner, benchmark_float_ln_1_plus_x_prec_assign);
    register_bench!(
        runner,
        benchmark_float_ln_1_plus_x_round_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_ln_1_plus_x_round_library_comparison);
    register_bench!(runner, benchmark_float_ln_1_plus_x_round_assign);
    register_bench!(
        runner,
        benchmark_float_ln_1_plus_x_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_ln_1_plus_x_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_ln_1_plus_x_prec_round_assign);
}

fn demo_float_ln_1_plus_x(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).ln_1_plus_x() = {}", x_old, x.ln_1_plus_x());
    }
}

fn demo_float_ln_1_plus_x_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).ln_1_plus_x() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.ln_1_plus_x())
        );
    }
}

fn demo_float_ln_1_plus_x_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).ln_1_plus_x() = {}", x_old, x.ln_1_plus_x());
    }
}

fn demo_float_ln_1_plus_x_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).ln_1_plus_x() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.ln_1_plus_x())
        );
    }
}

fn demo_float_ln_1_plus_x_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).ln_1_plus_x() = {}", x, (&x).ln_1_plus_x());
    }
}

fn demo_float_ln_1_plus_x_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).ln_1_plus_x() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).ln_1_plus_x())
        );
    }
}

fn demo_float_ln_1_plus_x_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ln_1_plus_x_assign();
        println!("x := {x_old}; x.ln_1_plus_x_assign(); x = {x}");
    }
}

fn demo_float_ln_1_plus_x_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ln_1_plus_x_assign();
        println!(
            "x := {:#x}; x.ln_1_plus_x_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_ln_1_plus_x_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).ln_1_plus_x_prec({}) = {:?}",
            x_old,
            prec,
            x.ln_1_plus_x_prec(prec)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.ln_1_plus_x_prec(prec);
        println!(
            "({:#x}).ln_1_plus_x_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).ln_1_plus_x_prec({}) = {:?}",
            x_old,
            prec,
            x.ln_1_plus_x_prec(prec)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.ln_1_plus_x_prec(prec);
        println!(
            "({:#x}).ln_1_plus_x_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).ln_1_plus_x_prec_ref({}) = {:?}",
            x,
            prec,
            x.ln_1_plus_x_prec_ref(prec)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (sum, o) = x.ln_1_plus_x_prec_ref(prec);
        println!(
            "(&{:#x}).ln_1_plus_x_prec_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ln_1_plus_x_prec_assign(prec);
        println!("x := {x_old}; x.ln_1_plus_x_prec_assign({prec}); x = {x}");
    }
}

fn demo_float_ln_1_plus_x_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.ln_1_plus_x_prec_assign(prec);
        println!(
            "x := {:#x}; x.ln_1_plus_x_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_ln_1_plus_x_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).ln_1_plus_x_round({}) = {:?}",
            x_old,
            rm,
            x.ln_1_plus_x_round(rm)
        );
    }
}

fn demo_float_ln_1_plus_x_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.ln_1_plus_x_round(rm);
        println!(
            "({:#x}).ln_1_plus_x_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).ln_1_plus_x_round({}) = {:?}",
            x_old,
            rm,
            x.ln_1_plus_x_round(rm)
        );
    }
}

fn demo_float_ln_1_plus_x_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_37()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (ln_1_plus_x, o) = x.ln_1_plus_x_round(rm);
        println!(
            "({:#x}).ln_1_plus_x_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(ln_1_plus_x),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).ln_1_plus_x_round_ref({}) = {:?}",
            x,
            rm,
            x.ln_1_plus_x_round_ref(rm)
        );
    }
}

fn demo_float_ln_1_plus_x_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.ln_1_plus_x_round_ref(rm);
        println!(
            "(&{:#x}).ln_1_plus_x_round_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.ln_1_plus_x_round_assign(rm);
        println!("x := {x_old}; x.ln_1_plus_x_round_assign({rm}); x = {x}");
    }
}

fn demo_float_ln_1_plus_x_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_1_plus_x_round_assign(rm);
        println!(
            "x := {:#x}; x.ln_1_plus_x_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).ln_1_plus_x_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.ln_1_plus_x_prec_round(prec, rm)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.ln_1_plus_x_prec_round(prec, rm);
        println!(
            "({:#x}).ln_1_plus_x_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).ln_1_plus_x_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.ln_1_plus_x_prec_round(prec, rm)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (ln_1_plus_x, o) = x.ln_1_plus_x_prec_round(prec, rm);
        println!(
            "({:#x}).ln_1_plus_x_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(ln_1_plus_x),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).ln_1_plus_x_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.ln_1_plus_x_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.ln_1_plus_x_prec_round_ref(prec, rm);
        println!(
            "({:#x}).ln_1_plus_x_prec_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_1_plus_x_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_1_plus_x_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.ln_1_plus_x_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_ln_1_plus_x_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_1_plus_x_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.ln_1_plus_x_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_ln_1_plus_x_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.ln_1_plus_x()", &mut |x| no_out!(x.ln_1_plus_x())),
            ("(&Float).ln_1_plus_x()", &mut |x| {
                no_out!((&x).ln_1_plus_x());
            }),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!((&x).ln_1_plus_x())),
            ("rug", &mut |(x, _)| no_out!(rug_ln_1_plus_x(&x))),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.ln_1_plus_x_assign()", &mut |mut x| {
            x.ln_1_plus_x_assign();
        })],
    );
}

fn benchmark_float_ln_1_plus_x_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.ln_1_plus_x_prec(u64)", &mut |(x, prec)| {
                no_out!(x.ln_1_plus_x_prec(prec));
            }),
            ("(&Float).ln_1_plus_x_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.ln_1_plus_x_prec_ref(prec));
            }),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.ln_1_plus_x_prec_ref(prec));
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_ln_1_plus_x_prec(&x, prec));
            }),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Float.ln_1_plus_x_prec_assign(u64)", &mut |(
            mut x,
            prec,
        )| {
            no_out!(x.ln_1_plus_x_prec_assign(prec));
        })],
    );
}

fn benchmark_float_ln_1_plus_x_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.ln_1_plus_x_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.ln_1_plus_x_round(rm));
            }),
            (
                "(&Float).ln_1_plus_x_round_ref(RoundingMode)",
                &mut |(x, rm)| no_out!(x.ln_1_plus_x_round_ref(rm)),
            ),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_round(RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.ln_1_plus_x_round_ref(rm));
            }),
            ("rug", &mut |((x, rm), _)| {
                no_out!(rug_ln_1_plus_x_round(&x, rm));
            }),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.ln_1_plus_x_round_assign(RoundingMode)",
            &mut |(mut x, rm)| no_out!(x.ln_1_plus_x_round_assign(rm)),
        )],
    );
}

fn benchmark_float_ln_1_plus_x_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.ln_1_plus_x_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.ln_1_plus_x_prec_round(prec, rm)),
            ),
            (
                "(&Float).ln_1_plus_x_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.ln_1_plus_x_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_21_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.ln_1_plus_x_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_ln_1_plus_x_prec_round(&x, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_ln_1_plus_x_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_1_plus_x_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.ln_1_plus_x_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.ln_1_plus_x_prec_round_assign(prec, rm)),
        )],
    );
}
