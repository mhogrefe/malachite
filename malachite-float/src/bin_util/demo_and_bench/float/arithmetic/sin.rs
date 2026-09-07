// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Sin, SinAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::Exact;
use malachite_base::test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer, quadruple_3_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::sin::{
    primitive_float_sin, primitive_float_sin_pi, primitive_float_sin_pi_rational,
    primitive_float_sin_rational, primitive_float_sin_with_period,
    primitive_float_sin_with_period_rational,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    quadruple_1_float_complexity_bucketer, triple_1_2_float_primitive_int_max_complexity_bucketer,
    triple_1_float_complexity_bucketer,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::float::arithmetic::sin::{
    rug_sin, rug_sin_prec_round, rug_sin_with_period_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
    float_unsigned_rounding_mode_triple_gen_var_39,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_10,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sin_pi_prec_round);
    register_demo!(runner, demo_float_sin_pi_prec_round_debug);
    register_demo!(runner, demo_float_sin_pi_prec);
    register_demo!(runner, demo_float_sin_pi_round);
    register_demo!(runner, demo_float_sin_pi_prec_round_assign);
    register_demo!(runner, demo_float_sin_pi_rational_prec_round);
    register_demo!(runner, demo_float_sin_pi_rational_prec);
    register_primitive_float_demos!(runner, demo_primitive_float_sin_pi);
    register_primitive_float_demos!(runner, demo_primitive_float_sin_pi_rational);
    register_bench!(
        runner,
        benchmark_float_sin_pi_prec_round_evaluation_strategy
    );
    register_demo!(runner, demo_float_sin_with_period_rational_prec_round);
    register_demo!(runner, demo_float_sin_with_period_rational_prec_round_debug);
    register_demo!(runner, demo_float_sin_with_period_rational_prec_round_ref);
    register_demo!(runner, demo_float_sin_with_period_rational_prec);
    register_demo!(runner, demo_float_sin_with_period_rational_prec_debug);
    register_demo!(runner, demo_float_sin_with_period_rational_prec_ref);
    register_bench!(
        runner,
        benchmark_float_sin_with_period_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sin_with_period_rational_prec_evaluation_strategy
    );
    register_demo!(runner, demo_float_sin_with_period_prec_round);
    register_demo!(runner, demo_float_sin_with_period_prec_round_debug);
    register_demo!(runner, demo_float_sin_with_period_prec_round_extreme);
    register_demo!(runner, demo_float_sin_with_period_prec_round_ref);
    register_demo!(runner, demo_float_sin_with_period_prec_round_assign);
    register_demo!(runner, demo_float_sin_with_period_prec);
    register_demo!(runner, demo_float_sin_with_period_prec_debug);
    register_demo!(runner, demo_float_sin_with_period_prec_ref);
    register_demo!(runner, demo_float_sin_with_period_prec_assign);
    register_demo!(runner, demo_float_sin_with_period_round);
    register_demo!(runner, demo_float_sin_with_period_round_debug);
    register_demo!(runner, demo_float_sin_with_period_round_ref);
    register_demo!(runner, demo_float_sin_with_period_round_assign);
    register_bench!(
        runner,
        benchmark_float_sin_with_period_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sin_with_period_prec_round_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_sin_with_period_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sin_with_period_round_evaluation_strategy
    );
    register_primitive_float_demos!(runner, demo_primitive_float_sin);
    register_primitive_float_demos!(runner, demo_primitive_float_sin_rational);
    register_primitive_float_demos!(runner, demo_primitive_float_sin_with_period);
    register_primitive_float_demos!(runner, demo_primitive_float_sin_with_period_rational);
    register_primitive_float_benches!(runner, benchmark_primitive_float_sin);
    register_primitive_float_benches!(runner, benchmark_primitive_float_sin_rational);
    register_primitive_float_benches!(runner, benchmark_primitive_float_sin_with_period);
    register_primitive_float_benches!(runner, benchmark_primitive_float_sin_with_period_rational);
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
    register_demo!(runner, demo_float_sin_rational_prec);
    register_demo!(runner, demo_float_sin_rational_prec_debug);
    register_demo!(runner, demo_float_sin_rational_prec_ref);
    register_demo!(runner, demo_float_sin_rational_prec_ref_debug);
    register_demo!(runner, demo_float_sin_rational_prec_round);
    register_demo!(runner, demo_float_sin_rational_prec_round_debug);
    register_demo!(runner, demo_float_sin_rational_prec_round_ref);
    register_demo!(runner, demo_float_sin_rational_prec_round_ref_debug);
    register_bench!(
        runner,
        benchmark_float_sin_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sin_rational_prec_round_evaluation_strategy
    );
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

fn demo_float_sin_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::sin_rational_prec(n, p)
        );
    }
}

fn demo_float_sin_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::sin_rational_prec(n.clone(), p);
        println!(
            "Float::sin_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_sin_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::sin_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_sin_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::sin_rational_prec_ref(&n, p);
        println!(
            "Float::sin_rational_prec_ref(&{}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_sin_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_rational_prec_round({}, {}, {}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::sin_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_sin_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::sin_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::sin_rational_prec_round({}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_sin_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_rational_prec_round_ref(&{}, {}, {}) = {:?}",
            n,
            p,
            rm,
            Float::sin_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_sin_rational_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::sin_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::sin_rational_prec_round_ref(&{}, {}, {}) = ({:#x}, {:?})",
            n,
            p,
            rm,
            ComparableFloat(f),
            o
        );
    }
}

fn benchmark_float_sin_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sin_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::sin_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::sin_rational_prec(n, prec)),
            ),
            (
                "Float::sin_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::sin_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_sin_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sin_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::sin_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::sin_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::sin_rational_prec_round_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::sin_rational_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sin<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_sin({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_sin(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sin<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_sin({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_sin(x));
        })],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sin_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_sin_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_sin_rational::<T>(&x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sin_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_sin_rational::<{}>(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_sin_rational::<T>(&x));
        })],
    );
}

fn demo_float_sin_with_period_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sin_with_period_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            x.sin_with_period_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_sin_with_period_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().sin_with_period_prec_round(u, prec, rm);
        println!(
            "({:#x}).sin_with_period_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_with_period_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sin_with_period_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            x.sin_with_period_prec_round(u, prec, rm)
        );
    }
}

fn demo_float_sin_with_period_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sin_with_period_prec_round_ref({}, {}, {}) = {:?}",
            x,
            u,
            prec,
            rm,
            x.sin_with_period_prec_round_ref(u, prec, rm)
        );
    }
}

fn demo_float_sin_with_period_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sin_with_period_prec_round_assign(u, prec, rm);
        println!(
            "x := {x_old}; x.sin_with_period_prec_round_assign({u}, {prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_float_sin_with_period_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sin_with_period_prec({}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            x.sin_with_period_prec(u, prec)
        );
    }
}

fn demo_float_sin_with_period_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().sin_with_period_prec(u, prec);
        println!(
            "({:#x}).sin_with_period_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_with_period_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sin_with_period_prec_ref({}, {}) = {:?}",
            x,
            u,
            prec,
            x.sin_with_period_prec_ref(u, prec)
        );
    }
}

fn demo_float_sin_with_period_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sin_with_period_prec_assign(u, prec);
        println!("x := {x_old}; x.sin_with_period_prec_assign({u}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_sin_with_period_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sin_with_period_round({}, {}) = {:?}",
            x.clone(),
            u,
            rm,
            x.sin_with_period_round(u, rm)
        );
    }
}

fn demo_float_sin_with_period_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().sin_with_period_round(u, rm);
        println!(
            "({:#x}).sin_with_period_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            u,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_with_period_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sin_with_period_round_ref({}, {}) = {:?}",
            x,
            u,
            rm,
            x.sin_with_period_round_ref(u, rm)
        );
    }
}

fn demo_float_sin_with_period_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, u, rm) in float_unsigned_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sin_with_period_round_assign(u, rm);
        println!("x := {x_old}; x.sin_with_period_round_assign({u}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_float_sin_with_period_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_with_period_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.sin_with_period_prec_round(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| no_out!(x.sin_with_period_prec_round(u, prec, rm)),
            ),
            (
                "(&Float).sin_with_period_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| no_out!(x.sin_with_period_prec_round_ref(u, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_sin_with_period_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_with_period_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .filter(|(_, u, _, rm)| u32::try_from(*u).is_ok() && *rm != Exact),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(x, u, prec, rm)| {
                no_out!(x.sin_with_period_prec_round(u, prec, rm));
            }),
            ("rug", &mut |(x, u, prec, rm)| {
                no_out!(rug_sin_with_period_prec_round(
                    &rug::Float::exact_from(&x),
                    u,
                    prec,
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ));
            }),
        ],
    );
}

fn benchmark_float_sin_with_period_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_with_period_prec(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.sin_with_period_prec(u64, u64)", &mut |(
                x,
                u,
                prec,
            )| {
                no_out!(x.sin_with_period_prec(u, prec));
            }),
            (
                "(&Float).sin_with_period_prec_ref(u64, u64)",
                &mut |(x, u, prec)| {
                    no_out!(x.sin_with_period_prec_ref(u, prec));
                },
            ),
        ],
    );
}

fn benchmark_float_sin_with_period_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_with_period_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_39().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.sin_with_period_round(u64, RoundingMode)",
                &mut |(x, u, rm)| {
                    no_out!(x.sin_with_period_round(u, rm));
                },
            ),
            (
                "(&Float).sin_with_period_round_ref(u64, RoundingMode)",
                &mut |(x, u, rm)| no_out!(x.sin_with_period_round_ref(u, rm)),
            ),
        ],
    );
}

fn demo_float_sin_with_period_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_with_period_rational_prec_round({}, {}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            rm,
            Float::sin_with_period_rational_prec_round(x, u, prec, rm)
        );
    }
}

fn demo_float_sin_with_period_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = Float::sin_with_period_rational_prec_round(x.clone(), u, prec, rm);
        println!(
            "Float::sin_with_period_rational_prec_round({}, {}, {}, {}) = ({:#x}, {:?})",
            x,
            u,
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_with_period_rational_prec_round_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_with_period_rational_prec_round_ref(&{}, {}, {}, {}) = {:?}",
            x,
            u,
            prec,
            rm,
            Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm)
        );
    }
}

fn demo_float_sin_with_period_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_with_period_rational_prec({}, {}, {}) = {:?}",
            x.clone(),
            u,
            prec,
            Float::sin_with_period_rational_prec(x, u, prec)
        );
    }
}

fn demo_float_sin_with_period_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = Float::sin_with_period_rational_prec(x.clone(), u, prec);
        println!(
            "Float::sin_with_period_rational_prec({}, {}, {}) = ({:#x}, {:?})",
            x,
            u,
            prec,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_with_period_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, u, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_with_period_rational_prec_ref(&{}, {}, {}) = {:?}",
            x,
            u,
            prec,
            Float::sin_with_period_rational_prec_ref(&x, u, prec)
        );
    }
}

fn benchmark_float_sin_with_period_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sin_with_period_rational_prec_round(Rational, u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "Float::sin_with_period_rational_prec_round(Rational, u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(Float::sin_with_period_rational_prec_round(x, u, prec, rm));
                },
            ),
            (
                "Float::sin_with_period_rational_prec_round_ref(&Rational, u64, u64, RoundingMode)",
                &mut |(x, u, prec, rm)| {
                    no_out!(Float::sin_with_period_rational_prec_round_ref(
                        &x, u, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_float_sin_with_period_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sin_with_period_rational_prec(Rational, u64, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [
            (
                "Float::sin_with_period_rational_prec(Rational, u64, u64)",
                &mut |(x, u, prec, _)| no_out!(Float::sin_with_period_rational_prec(x, u, prec)),
            ),
            (
                "Float::sin_with_period_rational_prec_ref(&Rational, u64, u64)",
                &mut |(x, u, prec, _)| {
                    no_out!(Float::sin_with_period_rational_prec_ref(&x, u, prec));
                },
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sin_with_period<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, u) in primitive_float_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_sin_with_period({}, {}) = {}",
            NiceFloat(x),
            u,
            NiceFloat(primitive_float_sin_with_period(x, u))
        );
    }
}
#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sin_with_period<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_sin_with_period({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("malachite", &mut |(x, u)| {
            no_out!(primitive_float_sin_with_period(x, u));
        })],
    );
}
#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sin_with_period_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, u) in rational_unsigned_pair_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_sin_with_period_rational({}, {}) = {:?}",
            x,
            u,
            NiceFloat(primitive_float_sin_with_period_rational::<T>(&x, u))
        );
    }
}
#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sin_with_period_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!(
            "primitive_float_sin_with_period_rational::<{}>(&Rational, u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("malachite", &mut |(x, u)| {
            no_out!(primitive_float_sin_with_period_rational::<T>(&x, u));
        })],
    );
}

fn demo_float_sin_pi_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sin_pi_prec_round({}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            x.sin_pi_prec_round(prec, rm)
        );
    }
}

fn demo_float_sin_pi_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let (c, o) = x.clone().sin_pi_prec_round(prec, rm);
        println!(
            "({:#x}).sin_pi_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(c),
            o
        );
    }
}

fn demo_float_sin_pi_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "({}).sin_pi_prec({}) = {:?}",
            x.clone(),
            prec,
            x.sin_pi_prec(prec)
        );
    }
}

fn demo_float_sin_pi_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sin_pi_round({}) = {:?}",
            x.clone(),
            rm,
            x.sin_pi_round(rm)
        );
    }
}

fn demo_float_sin_pi_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.sin_pi_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.sin_pi_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_sin_pi_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_pi_rational_prec_round({}, {}, {}) = {:?}",
            x.clone(),
            prec,
            rm,
            Float::sin_pi_rational_prec_round(x, prec, rm)
        );
    }
}

fn demo_float_sin_pi_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::sin_pi_rational_prec({}, {}) = {:?}",
            x.clone(),
            prec,
            Float::sin_pi_rational_prec(x, prec)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sin_pi<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_sin_pi({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_sin_pi(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sin_pi_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_sin_pi_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_sin_pi_rational::<T>(&x))
        );
    }
}

fn benchmark_float_sin_pi_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sin_pi_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.sin_pi_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sin_pi_prec_round(prec, rm)),
            ),
            (
                "(&Float).sin_pi_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.sin_pi_prec_round_ref(prec, rm)),
            ),
        ],
    );
}
