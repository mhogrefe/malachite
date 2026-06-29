// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{PowerOf2, PowerOf2Assign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
    pair_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_power_of_2_of_float_prec_round);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_round_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_round_ref);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_round_ref_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_round_assign);
    register_demo!(
        runner,
        demo_float_power_of_2_of_float_prec_round_assign_debug
    );
    register_demo!(runner, demo_float_power_of_2_of_float_prec);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_ref);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_ref_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_assign);
    register_demo!(runner, demo_float_power_of_2_of_float_prec_assign_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_round);
    register_demo!(runner, demo_float_power_of_2_of_float_round_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_round_ref);
    register_demo!(runner, demo_float_power_of_2_of_float_round_ref_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_round_assign);
    register_demo!(runner, demo_float_power_of_2_of_float_round_assign_debug);
    register_demo!(runner, demo_float_power_of_2_of_float);
    register_demo!(runner, demo_float_power_of_2_of_float_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_ref);
    register_demo!(runner, demo_float_power_of_2_of_float_ref_debug);
    register_demo!(runner, demo_float_power_of_2_of_float_assign);
    register_demo!(runner, demo_float_power_of_2_of_float_assign_debug);

    register_bench!(
        runner,
        benchmark_float_power_of_2_of_float_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_of_float_prec_round_assign
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_of_float_prec_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_power_of_2_of_float_prec_assign);
    register_bench!(
        runner,
        benchmark_float_power_of_2_of_float_round_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_power_of_2_of_float_round_assign);
    register_bench!(
        runner,
        benchmark_float_power_of_2_of_float_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_power_of_2_of_float_assign);
}

// -------- prec_round --------

fn demo_float_power_of_2_of_float_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_of_float_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            Float::power_of_2_of_float_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_power_of_2_of_float_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::power_of_2_of_float_prec_round(x.clone(), prec, rm);
        println!(
            "power_of_2_of_float_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_of_float_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_of_float_prec_round_ref(&{}, {}, {}) = {:?}",
            x,
            prec,
            rm,
            Float::power_of_2_of_float_prec_round_ref(&x, prec, rm)
        );
    }
}

fn demo_float_power_of_2_of_float_prec_round_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::power_of_2_of_float_prec_round_ref(&x, prec, rm);
        println!(
            "power_of_2_of_float_prec_round_ref(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_of_float_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.power_of_2_of_float_prec_round_assign(prec, rm);
        println!(
            "x := {x_old}; x.power_of_2_of_float_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_float_power_of_2_of_float_prec_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.power_of_2_of_float_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.power_of_2_of_float_prec_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

// -------- prec --------

fn demo_float_power_of_2_of_float_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "power_of_2_of_float_prec({}, {}) = {:?}",
            x.clone(),
            prec,
            Float::power_of_2_of_float_prec(x, prec)
        );
    }
}

fn demo_float_power_of_2_of_float_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (p, o) = Float::power_of_2_of_float_prec(x.clone(), prec);
        println!(
            "power_of_2_of_float_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_of_float_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "power_of_2_of_float_prec_ref(&{}, {}) = {:?}",
            x,
            prec,
            Float::power_of_2_of_float_prec_ref(&x, prec)
        );
    }
}

fn demo_float_power_of_2_of_float_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (p, o) = Float::power_of_2_of_float_prec_ref(&x, prec);
        println!(
            "power_of_2_of_float_prec_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_of_float_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.power_of_2_of_float_prec_assign(prec);
        println!("x := {x_old}; x.power_of_2_of_float_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_power_of_2_of_float_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.power_of_2_of_float_prec_assign(prec);
        println!(
            "x := {:#x}; x.power_of_2_of_float_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

// -------- round --------

fn demo_float_power_of_2_of_float_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_of_float_round({}, {}) = {:?}",
            x.clone(),
            rm,
            Float::power_of_2_of_float_round(x, rm)
        );
    }
}

fn demo_float_power_of_2_of_float_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::power_of_2_of_float_round(x.clone(), rm);
        println!(
            "power_of_2_of_float_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_of_float_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "power_of_2_of_float_round_ref(&{}, {}) = {:?}",
            x,
            rm,
            Float::power_of_2_of_float_round_ref(&x, rm)
        );
    }
}

fn demo_float_power_of_2_of_float_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let (p, o) = Float::power_of_2_of_float_round_ref(&x, rm);
        println!(
            "power_of_2_of_float_round_ref(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(p),
            o
        );
    }
}

fn demo_float_power_of_2_of_float_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.power_of_2_of_float_round_assign(rm);
        println!("x := {x_old}; x.power_of_2_of_float_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_power_of_2_of_float_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.power_of_2_of_float_round_assign(rm);
        println!(
            "x := {:#x}; x.power_of_2_of_float_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

// -------- PowerOf2 trait (no prec or rm) --------

fn demo_float_power_of_2_of_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("power_of_2({}) = {}", x.clone(), Float::power_of_2(x));
    }
}

fn demo_float_power_of_2_of_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "power_of_2({:#x}) = {:#x}",
            ComparableFloat(x.clone()),
            ComparableFloat(Float::power_of_2(x))
        );
    }
}

fn demo_float_power_of_2_of_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("power_of_2(&{}) = {}", x, Float::power_of_2(&x));
    }
}

fn demo_float_power_of_2_of_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "power_of_2(&{:#x}) = {:#x}",
            ComparableFloat(x.clone()),
            ComparableFloat(Float::power_of_2(&x))
        );
    }
}

fn demo_float_power_of_2_of_float_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.power_of_2_assign();
        println!("x := {x_old}; x.power_of_2_assign(); x = {x}");
    }
}

fn demo_float_power_of_2_of_float_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.power_of_2_assign();
        println!(
            "x := {:#x}; x.power_of_2_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

// -------- benchmarks --------

fn benchmark_float_power_of_2_of_float_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_of_float_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float::power_of_2_of_float_prec_round(Float, u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(Float::power_of_2_of_float_prec_round(x, prec, rm)),
            ),
            (
                "Float::power_of_2_of_float_prec_round_ref(&Float, u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(Float::power_of_2_of_float_prec_round_ref(&x, prec, rm))
                },
            ),
        ],
    );
}

fn benchmark_float_power_of_2_of_float_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_of_float_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Malachite", &mut |(mut x, prec, rm)| {
            no_out!(x.power_of_2_of_float_prec_round_assign(prec, rm))
        })],
    );
}

fn benchmark_float_power_of_2_of_float_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_of_float_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float::power_of_2_of_float_prec(Float, u64)",
                &mut |(x, prec)| no_out!(Float::power_of_2_of_float_prec(x, prec)),
            ),
            (
                "Float::power_of_2_of_float_prec_ref(&Float, u64)",
                &mut |(x, prec)| no_out!(Float::power_of_2_of_float_prec_ref(&x, prec)),
            ),
        ],
    );
}

fn benchmark_float_power_of_2_of_float_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_of_float_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Malachite", &mut |(mut x, prec)| {
            no_out!(x.power_of_2_of_float_prec_assign(prec))
        })],
    );
}

fn benchmark_float_power_of_2_of_float_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_of_float_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float::power_of_2_of_float_round(Float, RoundingMode)",
                &mut |(x, rm)| no_out!(Float::power_of_2_of_float_round(x, rm)),
            ),
            (
                "Float::power_of_2_of_float_round_ref(&Float, RoundingMode)",
                &mut |(x, rm)| no_out!(Float::power_of_2_of_float_round_ref(&x, rm)),
            ),
        ],
    );
}

fn benchmark_float_power_of_2_of_float_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_of_float_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, rm)| {
            no_out!(x.power_of_2_of_float_round_assign(rm))
        })],
    );
}

fn benchmark_float_power_of_2_of_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2(Float)",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float::power_of_2(Float)", &mut |x| {
                no_out!(Float::power_of_2(x))
            }),
            ("Float::power_of_2(&Float)", &mut |x| {
                no_out!(Float::power_of_2(&x))
            }),
        ],
    );
}

fn benchmark_float_power_of_2_of_float_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| no_out!(x.power_of_2_assign()))],
    );
}
