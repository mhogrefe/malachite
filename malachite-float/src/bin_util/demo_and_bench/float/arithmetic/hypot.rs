// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Hypot, HypotAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_max_primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::hypot::primitive_float_hypot;
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_float_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_max_complexity_bucketer,
    pair_2_triple_float_float_primitive_int_max_complexity_bucketer,
    pair_float_max_complexity_bucketer,
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_max_complexity_bucketer,
    triple_float_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::hypot::{
    rug_hypot, rug_hypot_prec, rug_hypot_prec_round, rug_hypot_round,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_43, float_float_rounding_mode_triple_gen_var_43_rm,
    float_float_rounding_mode_triple_gen_var_44,
    float_float_unsigned_rounding_mode_quadruple_gen_var_24,
    float_float_unsigned_rounding_mode_quadruple_gen_var_24_rm,
    float_float_unsigned_rounding_mode_quadruple_gen_var_25, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_1_rm, float_float_unsigned_triple_gen_var_2,
    float_pair_gen, float_pair_gen_rm, float_pair_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_hypot);
    register_demo!(runner, demo_float_hypot_debug);
    register_demo!(runner, demo_float_hypot_val_ref);
    register_demo!(runner, demo_float_hypot_val_ref_debug);
    register_demo!(runner, demo_float_hypot_ref_val);
    register_demo!(runner, demo_float_hypot_ref_val_debug);
    register_demo!(runner, demo_float_hypot_extreme);
    register_demo!(runner, demo_float_hypot_extreme_debug);
    register_demo!(runner, demo_float_hypot_ref_ref);
    register_demo!(runner, demo_float_hypot_ref_ref_debug);
    register_demo!(runner, demo_float_hypot_assign);
    register_demo!(runner, demo_float_hypot_assign_debug);
    register_demo!(runner, demo_float_hypot_assign_ref);
    register_demo!(runner, demo_float_hypot_assign_ref_debug);
    register_demo!(runner, demo_float_hypot_prec);
    register_demo!(runner, demo_float_hypot_prec_debug);
    register_demo!(runner, demo_float_hypot_prec_extreme);
    register_demo!(runner, demo_float_hypot_prec_extreme_debug);
    register_demo!(runner, demo_float_hypot_prec_val_ref);
    register_demo!(runner, demo_float_hypot_prec_val_ref_debug);
    register_demo!(runner, demo_float_hypot_prec_ref_val);
    register_demo!(runner, demo_float_hypot_prec_ref_val_debug);
    register_demo!(runner, demo_float_hypot_prec_assign);
    register_demo!(runner, demo_float_hypot_prec_assign_debug);
    register_demo!(runner, demo_float_hypot_prec_assign_ref);
    register_demo!(runner, demo_float_hypot_prec_assign_ref_debug);
    register_demo!(runner, demo_float_hypot_prec_ref_ref);
    register_demo!(runner, demo_float_hypot_prec_ref_ref_debug);
    register_demo!(runner, demo_float_hypot_round);
    register_demo!(runner, demo_float_hypot_round_debug);
    register_demo!(runner, demo_float_hypot_round_val_ref);
    register_demo!(runner, demo_float_hypot_round_val_ref_debug);
    register_demo!(runner, demo_float_hypot_round_ref_val);
    register_demo!(runner, demo_float_hypot_round_ref_val_debug);
    register_demo!(runner, demo_float_hypot_round_assign);
    register_demo!(runner, demo_float_hypot_round_assign_debug);
    register_demo!(runner, demo_float_hypot_round_assign_ref);
    register_demo!(runner, demo_float_hypot_round_assign_ref_debug);
    register_demo!(runner, demo_float_hypot_round_extreme);
    register_demo!(runner, demo_float_hypot_round_extreme_debug);
    register_demo!(runner, demo_float_hypot_prec_round);
    register_demo!(runner, demo_float_hypot_prec_round_debug);
    register_demo!(runner, demo_float_hypot_prec_round_extreme);
    register_demo!(runner, demo_float_hypot_prec_round_extreme_debug);
    register_demo!(runner, demo_float_hypot_prec_round_val_ref);
    register_demo!(runner, demo_float_hypot_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_hypot_prec_round_ref_val);
    register_demo!(runner, demo_float_hypot_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_hypot_prec_round_assign);
    register_demo!(runner, demo_float_hypot_prec_round_assign_debug);
    register_demo!(runner, demo_float_hypot_prec_round_assign_ref);
    register_demo!(runner, demo_float_hypot_prec_round_assign_ref_debug);
    register_demo!(runner, demo_float_hypot_prec_round_ref_ref);
    register_demo!(runner, demo_float_hypot_prec_round_ref_ref_debug);

    register_primitive_float_demos!(runner, demo_primitive_float_hypot);

    register_bench!(runner, benchmark_float_hypot_evaluation_strategy);
    register_bench!(runner, benchmark_float_hypot_library_comparison);
    register_bench!(runner, benchmark_float_hypot_assign_evaluation_strategy);
    register_bench!(runner, benchmark_float_hypot_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_hypot_prec_library_comparison);
    register_bench!(runner, benchmark_float_hypot_round_library_comparison);
    register_bench!(runner, benchmark_float_hypot_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_hypot_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_hypot_round_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_float_hypot_prec_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_hypot_round_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_hypot_prec_round_assign_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_hypot);
}

fn demo_float_hypot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).hypot({}) = {}", x_old, y_old, x.hypot(y));
    }
}

fn demo_float_hypot_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({:#x}).hypot({:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.hypot(y))
        );
    }
}

fn demo_float_hypot_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).hypot({}) = {}", x_old, y_old, x.hypot(y));
    }
}

fn demo_float_hypot_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({:#x}).hypot({:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x.hypot(y))
        );
    }
}

fn demo_float_hypot_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!("(&{}).hypot(&{}) = {}", x, y, (&x).hypot(&y));
    }
}

fn demo_float_hypot_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).hypot(&{:#x}) = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloatRef(&y),
            ComparableFloat((&x).hypot(&y))
        );
    }
}

fn demo_float_hypot_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.hypot_assign(y);
        println!("x := {x_old}; x.hypot_assign({y_old}); x = {x}");
    }
}

fn demo_float_hypot_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.hypot_assign(y);
        println!(
            "x := {:#x}; x.hypot_assign({:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.hypot_assign(&y);
        println!("x := {x_old}; x.hypot_assign(&{y}); x = {x}");
    }
}

fn demo_float_hypot_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.hypot_assign(&y);
        println!(
            "x := {:#x}; x.hypot_assign(&{:#x}); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).hypot_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.hypot_prec(y, prec)
        );
    }
}

fn demo_float_hypot_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec(y, prec);
        println!(
            "({:#x}).hypot_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).hypot_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.hypot_prec(y, prec)
        );
    }
}

fn demo_float_hypot_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec(y, prec);
        println!(
            "({:#x}).hypot_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).hypot_prec_ref_ref(&{}, {}) = {:?}",
            x,
            y,
            prec,
            x.hypot_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_hypot_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (hypot, o) = x.hypot_prec_ref_ref(&y, prec);
        println!(
            "(&{:#x}).hypot_prec_ref_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            ComparableFloatRef(&y),
            prec,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).hypot_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.hypot_round(y, rm)
        );
    }
}

fn demo_float_hypot_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (hypot, o) = x.hypot_round(y, rm);
        println!(
            "({:#x}).hypot_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_44()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).hypot_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.hypot_round(y, rm)
        );
    }
}

fn demo_float_hypot_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_44()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (hypot, o) = x.hypot_round(y, rm);
        println!(
            "({:#x}).hypot_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).hypot_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.hypot_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_hypot_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec_round(y, prec, rm);
        println!(
            "({:#x}).hypot_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_25()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).hypot_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.hypot_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_hypot_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_25()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec_round(y, prec, rm);
        println!(
            "({:#x}).hypot_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).hypot_prec_round_ref_ref(&{}, {}, {}) = {:?}",
            x,
            y,
            prec,
            rm,
            x.hypot_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_hypot_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let (hypot, o) = x.hypot_prec_round_ref_ref(&y, prec, rm);
        println!(
            "(&{:#x}).hypot_prec_round_ref_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            ComparableFloatRef(&y),
            prec,
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn benchmark_float_hypot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.hypot(Float)", &mut |(x, y)| no_out!(x.hypot(y))),
            ("Float.hypot(&Float)", &mut |(x, y)| no_out!(x.hypot(&y))),
            ("(&Float).hypot(Float)", &mut |(x, y)| {
                no_out!((&x).hypot(y));
            }),
            ("(&Float).hypot(&Float)", &mut |(x, y)| {
                no_out!((&x).hypot(&y));
            }),
        ],
    );
}

fn benchmark_float_hypot_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot(Float)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.hypot(y))),
            ("rug", &mut |((x, y), _)| no_out!(rug_hypot(&x, &y))),
        ],
    );
}

fn benchmark_float_hypot_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_assign(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.hypot_assign(Float)", &mut |(mut x, y)| {
                x.hypot_assign(y);
            }),
            ("Float.hypot_assign(&Float)", &mut |(mut x, y)| {
                x.hypot_assign(&y);
            }),
        ],
    );
}

fn benchmark_float_hypot_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.hypot_prec(Float, u64)", &mut |(x, y, prec)| {
                no_out!(x.hypot_prec(y, prec));
            }),
            (
                "Float.hypot_prec_val_ref(&Float, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.hypot_prec_val_ref(&y, prec));
                },
            ),
            (
                "(&Float).hypot_prec_ref_val(Float, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.hypot_prec_ref_val(y, prec));
                },
            ),
            (
                "(&Float).hypot_prec_ref_ref(&Float, u64)",
                &mut |(x, y, prec)| no_out!(x.hypot_prec_ref_ref(&y, prec)),
            ),
        ],
    );
}

fn benchmark_float_hypot_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_prec(Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.hypot_prec(y, prec));
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_hypot_prec(&x, &y, prec));
            }),
        ],
    );
}

fn benchmark_float_hypot_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_round(Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_rounding_mode_triple_gen_var_43_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.hypot_round(y, rm));
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_hypot_round(&x, &y, rm));
            }),
        ],
    );
}

fn benchmark_float_hypot_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.hypot_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.hypot_prec_round(y, prec, rm)),
            ),
            (
                "Float.hypot_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.hypot_prec_round_val_ref(&y, prec, rm)),
            ),
            (
                "(&Float).hypot_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.hypot_prec_round_ref_val(y, prec, rm)),
            ),
            (
                "(&Float).hypot_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| no_out!(x.hypot_prec_round_ref_ref(&y, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_hypot_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_rounding_mode_quadruple_gen_var_24_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.hypot_prec_round_ref_ref(&y, prec, rm));
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_hypot_prec_round(&x, &y, prec, rm));
            }),
        ],
    );
}

fn demo_float_hypot_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).hypot(&{}) = {}", x_old, y, x.hypot(&y));
    }
}

fn demo_float_hypot_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).hypot(&{:#x}) = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            ComparableFloat(x.hypot(&y))
        );
    }
}

fn demo_float_hypot_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).hypot({}) = {}", x, y_old, (&x).hypot(y));
    }
}

fn demo_float_hypot_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{:#x}).hypot({:#x}) = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            ComparableFloat((&x).hypot(y))
        );
    }
}

fn demo_float_hypot_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).hypot_prec_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            prec,
            x.hypot_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_hypot_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (hypot, o) = x.hypot_prec_val_ref(&y, prec);
        println!(
            "({:#x}).hypot_prec_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            prec,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).hypot_prec_ref_val({}, {}) = {:?}",
            x,
            y_old,
            prec,
            x.hypot_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_hypot_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec_ref_val(y, prec);
        println!(
            "(&{:#x}).hypot_prec_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_assign(y, prec);
        println!("x := {x_old}; x.hypot_prec_assign({y_old}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_hypot_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_assign(y, prec);
        println!(
            "x := {:#x}; x.hypot_prec_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.hypot_prec_assign_ref(&{y}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_hypot_prec_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_assign_ref(&y, prec);
        println!(
            "x := {:#x}; x.hypot_prec_assign_ref(&{:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).hypot_round_val_ref(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.hypot_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_hypot_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (hypot, o) = x.hypot_round_val_ref(&y, rm);
        println!(
            "({:#x}).hypot_round_val_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).hypot_round_ref_val({}, {}) = {:?}",
            x,
            y_old,
            rm,
            x.hypot_round_ref_val(y, rm)
        );
    }
}

fn demo_float_hypot_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (hypot, o) = x.hypot_round_ref_val(y, rm);
        println!(
            "(&{:#x}).hypot_round_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_round_assign(y, rm);
        println!("x := {x_old}; x.hypot_round_assign({y_old}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_hypot_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_round_assign(y, rm);
        println!(
            "x := {:#x}; x.hypot_round_assign({:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_round_assign_ref(&y, rm);
        println!("x := {x_old}; x.hypot_round_assign_ref(&{y}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_hypot_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in float_float_rounding_mode_triple_gen_var_43()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_round_assign_ref(&y, rm);
        println!(
            "x := {:#x}; x.hypot_round_assign_ref(&{:#x}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).hypot_prec_round_val_ref(&{}, {}, {}) = {:?}",
            x_old,
            y,
            prec,
            rm,
            x.hypot_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_hypot_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (hypot, o) = x.hypot_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).hypot_prec_round_val_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            prec,
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).hypot_prec_round_ref_val({}, {}, {}) = {:?}",
            x,
            y_old,
            prec,
            rm,
            x.hypot_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_hypot_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec_round_ref_val(y, prec, rm);
        println!(
            "(&{:#x}).hypot_prec_round_ref_val({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloatRef(&x),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(hypot),
            o
        );
    }
}

fn demo_float_hypot_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.hypot_prec_round_assign({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_hypot_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_round_assign(y, prec, rm);
        println!(
            "x := {:#x}; x.hypot_prec_round_assign({:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_hypot_prec_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {x_old}; x.hypot_prec_round_assign_ref(&{y}, {prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_float_hypot_prec_round_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_24()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {:#x}; x.hypot_prec_round_assign_ref(&{:#x}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloatRef(&y),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_hypot<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_hypot({}, {}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(primitive_float_hypot(x, y))
        );
    }
}

fn benchmark_float_hypot_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_43().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.hypot_round(Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.hypot_round(y, rm));
                },
            ),
            (
                "Float.hypot_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.hypot_round_val_ref(&y, rm));
                },
            ),
            (
                "(&Float).hypot_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.hypot_round_ref_val(y, rm));
                },
            ),
            (
                "(&Float).hypot_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.hypot_round_ref_ref(&y, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_hypot_prec_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_prec_assign(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.hypot_prec_assign(Float, u64)", &mut |(
                mut x,
                y,
                prec,
            )| {
                no_out!(x.hypot_prec_assign(y, prec));
            }),
            (
                "Float.hypot_prec_assign_ref(&Float, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.hypot_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_float_hypot_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_round_assign(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_43().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "Float.hypot_round_assign(Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.hypot_round_assign(y, rm)),
            ),
            (
                "Float.hypot_round_assign_ref(&Float, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.hypot_round_assign_ref(&y, rm)),
            ),
        ],
    );
}

fn benchmark_float_hypot_prec_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.hypot_prec_round_assign(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.hypot_prec_round_assign(Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.hypot_prec_round_assign(y, prec, rm)),
            ),
            (
                "Float.hypot_prec_round_assign_ref(&Float, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| no_out!(x.hypot_prec_round_assign_ref(&y, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_hypot<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_hypot({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_primitive_float_bucketer("x", "y"),
        &mut [("malachite", &mut |(x, y)| {
            no_out!(primitive_float_hypot(x, y));
        })],
    );
}
