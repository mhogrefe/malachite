// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{PowerOf2, PowerOf2Assign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::arithmetic::power_of_2_of_float::primitive_float_power_of_2;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
    pair_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;

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
    register_demo!(runner, demo_float_power_of_2_rational_prec);
    register_demo!(runner, demo_float_power_of_2_rational_prec_debug);
    register_demo!(runner, demo_float_power_of_2_rational_prec_ref);
    register_demo!(runner, demo_float_power_of_2_rational_prec_ref_debug);
    register_demo!(runner, demo_float_power_of_2_rational_prec_round);
    register_demo!(runner, demo_float_power_of_2_rational_prec_round_debug);
    register_demo!(runner, demo_float_power_of_2_rational_prec_round_ref);
    register_demo!(runner, demo_float_power_of_2_rational_prec_round_ref_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_power_of_2);

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
    register_bench!(
        runner,
        benchmark_float_power_of_2_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_power_of_2);
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

// -------- power_of_2_rational --------

fn demo_float_power_of_2_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::power_of_2_rational_prec(n, p)
        );
    }
}

fn demo_float_power_of_2_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_rational_prec(n.clone(), p);
        println!(
            "Float::power_of_2_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_power_of_2_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::power_of_2_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_power_of_2_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_rational_prec_ref(&n, p);
        println!(
            "Float::power_of_2_rational_prec_ref(&{}, {}) = {:x?}",
            n,
            p,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_power_of_2_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_rational_prec_round({}, {}, {:?}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::power_of_2_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_power_of_2_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::power_of_2_rational_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_power_of_2_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_rational_prec_round_ref(&{}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::power_of_2_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_power_of_2_rational_prec_round_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::power_of_2_rational_prec_round_ref(&{}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn benchmark_float_power_of_2_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::power_of_2_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::power_of_2_rational_prec(n, prec)),
            ),
            (
                "Float::power_of_2_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::power_of_2_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_power_of_2_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::power_of_2_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::power_of_2_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::power_of_2_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| {
                    no_out!(Float::power_of_2_rational_prec_round_ref(&n, prec, rm))
                },
            ),
        ],
    );
}

// -------- primitive_float_power_of_2 --------

fn demo_primitive_float_power_of_2<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_power_of_2({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_power_of_2(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_power_of_2<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_power_of_2({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_power_of_2(x));
        })],
    );
}
