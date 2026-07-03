// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{PowerOf2XMinus1, PowerOf2XMinus1Assign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::Float;
use malachite_float::arithmetic::power_of_2_x_minus_1::{
    primitive_float_power_of_2_x_minus_1, primitive_float_power_of_2_x_minus_1_rational,
};
use malachite_float::test_util::arithmetic::power_of_2_x_minus_1::{
    rug_power_of_2_x_minus_1, rug_power_of_2_x_minus_1_prec, rug_power_of_2_x_minus_1_prec_round,
    rug_power_of_2_x_minus_1_round,
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
    float_gen, float_gen_rm, float_rounding_mode_pair_gen_var_44_rm,
    float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_1_rm, float_unsigned_rounding_mode_triple_gen_var_31_rm,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, rational_bit_bucketer,
    triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_power_of_2_x_minus_1);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_ref);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_ref_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_assign);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_assign_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_ref);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_assign);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_round);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_round_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_round_ref);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_round_assign);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_round);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_round_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_round_ref);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_prec_round_assign);
    register_primitive_float_demos!(runner, demo_primitive_float_power_of_2_x_minus_1);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_rational_prec);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_rational_prec_debug);
    register_demo!(runner, demo_float_power_of_2_x_minus_1_rational_prec_ref);
    register_demo!(
        runner,
        demo_float_power_of_2_x_minus_1_rational_prec_ref_debug
    );
    register_demo!(runner, demo_float_power_of_2_x_minus_1_rational_prec_round);
    register_demo!(
        runner,
        demo_float_power_of_2_x_minus_1_rational_prec_round_debug
    );
    register_demo!(
        runner,
        demo_float_power_of_2_x_minus_1_rational_prec_round_ref
    );
    register_demo!(
        runner,
        demo_float_power_of_2_x_minus_1_rational_prec_round_ref_debug
    );
    register_primitive_float_demos!(runner, demo_primitive_float_power_of_2_x_minus_1_rational);

    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_library_comparison
    );
    register_bench!(runner, benchmark_float_power_of_2_x_minus_1_assign);
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_prec_library_comparison
    );
    register_bench!(runner, benchmark_float_power_of_2_x_minus_1_prec_assign);
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_round_library_comparison
    );
    register_bench!(runner, benchmark_float_power_of_2_x_minus_1_round_assign);
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_prec_round_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_prec_round_assign
    );
    register_primitive_float_benches!(runner, benchmark_primitive_float_power_of_2_x_minus_1);
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_power_of_2_x_minus_1_rational_prec_round_evaluation_strategy
    );
    register_primitive_float_benches!(
        runner,
        benchmark_primitive_float_power_of_2_x_minus_1_rational
    );
}

fn demo_float_power_of_2_x_minus_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).power_of_2_x_minus_1() = {}",
            x_old,
            x.power_of_2_x_minus_1()
        );
    }
}

fn demo_float_power_of_2_x_minus_1_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).power_of_2_x_minus_1() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.power_of_2_x_minus_1())
        );
    }
}

fn demo_float_power_of_2_x_minus_1_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).power_of_2_x_minus_1() = {}",
            x,
            (&x).power_of_2_x_minus_1()
        );
    }
}

fn demo_float_power_of_2_x_minus_1_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).power_of_2_x_minus_1() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).power_of_2_x_minus_1())
        );
    }
}

fn demo_float_power_of_2_x_minus_1_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.power_of_2_x_minus_1_assign();
        println!("x := {x_old}; x.power_of_2_x_minus_1_assign(); x = {x}");
    }
}

fn demo_float_power_of_2_x_minus_1_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.power_of_2_x_minus_1_assign();
        println!(
            "x := {:#x}; x.power_of_2_x_minus_1_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).power_of_2_x_minus_1_prec({}) = {:?}",
            x_old,
            prec,
            x.power_of_2_x_minus_1_prec(prec)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (e, o) = x.power_of_2_x_minus_1_prec(prec);
        println!(
            "({:#x}).power_of_2_x_minus_1_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(e),
            o
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).power_of_2_x_minus_1_prec_ref({}) = {:?}",
            x,
            prec,
            x.power_of_2_x_minus_1_prec_ref(prec)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.power_of_2_x_minus_1_prec_assign(prec);
        println!("x := {x_old}; x.power_of_2_x_minus_1_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_power_of_2_x_minus_1_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).power_of_2_x_minus_1_round({}) = {:?}",
            x_old,
            rm,
            x.power_of_2_x_minus_1_round(rm)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (e, o) = x.power_of_2_x_minus_1_round(rm);
        println!(
            "({:#x}).power_of_2_x_minus_1_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(e),
            o
        );
    }
}

fn demo_float_power_of_2_x_minus_1_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).power_of_2_x_minus_1_round_ref({}) = {:?}",
            x,
            rm,
            x.power_of_2_x_minus_1_round_ref(rm)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.power_of_2_x_minus_1_round_assign(rm);
        println!("x := {x_old}; x.power_of_2_x_minus_1_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_power_of_2_x_minus_1_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).power_of_2_x_minus_1_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.power_of_2_x_minus_1_prec_round(prec, rm)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (e, o) = x.power_of_2_x_minus_1_prec_round(prec, rm);
        println!(
            "({:#x}).power_of_2_x_minus_1_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(e),
            o
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).power_of_2_x_minus_1_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.power_of_2_x_minus_1_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.power_of_2_x_minus_1_prec_round_assign(prec, rm);
        println!(
            "x := {x_old}; x.power_of_2_x_minus_1_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_power_of_2_x_minus_1_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.power_of_2_x_minus_1()", &mut |x| {
                no_out!(x.power_of_2_x_minus_1())
            }),
            ("(&Float).power_of_2_x_minus_1()", &mut |x| {
                no_out!((&x).power_of_2_x_minus_1())
            }),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| {
                no_out!((&x).power_of_2_x_minus_1())
            }),
            ("rug", &mut |(x, _)| no_out!(rug_power_of_2_x_minus_1(&x))),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.power_of_2_x_minus_1_assign()", &mut |mut x| {
            x.power_of_2_x_minus_1_assign()
        })],
    );
}

fn benchmark_float_power_of_2_x_minus_1_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.power_of_2_x_minus_1_prec(u64)", &mut |(x, prec)| {
                no_out!(x.power_of_2_x_minus_1_prec(prec));
            }),
            (
                "(&Float).power_of_2_x_minus_1_prec_ref(u64)",
                &mut |(x, prec)| {
                    no_out!(x.power_of_2_x_minus_1_prec_ref(prec));
                },
            ),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.power_of_2_x_minus_1_prec_ref(prec));
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_power_of_2_x_minus_1_prec(&x, prec));
            }),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.power_of_2_x_minus_1_prec_assign(u64)",
            &mut |(mut x, prec)| {
                no_out!(x.power_of_2_x_minus_1_prec_assign(prec));
            },
        )],
    );
}

fn benchmark_float_power_of_2_x_minus_1_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.power_of_2_x_minus_1_round(RoundingMode)",
                &mut |(x, rm)| {
                    no_out!(x.power_of_2_x_minus_1_round(rm));
                },
            ),
            (
                "(&Float).power_of_2_x_minus_1_round_ref(RoundingMode)",
                &mut |(x, rm)| {
                    no_out!(x.power_of_2_x_minus_1_round_ref(rm));
                },
            ),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_round(RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_44_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.power_of_2_x_minus_1_round_ref(rm));
            }),
            ("rug", &mut |((x, rm), _)| {
                no_out!(rug_power_of_2_x_minus_1_round(&x, rm))
            }),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.power_of_2_x_minus_1_round_assign(RoundingMode)",
            &mut |(mut x, rm)| {
                no_out!(x.power_of_2_x_minus_1_round_assign(rm));
            },
        )],
    );
}

fn benchmark_float_power_of_2_x_minus_1_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.power_of_2_x_minus_1_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.power_of_2_x_minus_1_prec_round(prec, rm)),
            ),
            (
                "(&Float).power_of_2_x_minus_1_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.power_of_2_x_minus_1_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_31_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.power_of_2_x_minus_1_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_power_of_2_x_minus_1_prec_round(&x, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.power_of_2_x_minus_1_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.power_of_2_x_minus_1_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.power_of_2_x_minus_1_prec_round_assign(prec, rm)),
        )],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_power_of_2_x_minus_1<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_power_of_2_x_minus_1({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_power_of_2_x_minus_1(x))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_power_of_2_x_minus_1<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_power_of_2_x_minus_1({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_power_of_2_x_minus_1(x));
        })],
    );
}

fn demo_float_power_of_2_x_minus_1_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_x_minus_1_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::power_of_2_x_minus_1_rational_prec(n, p)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_x_minus_1_rational_prec(n.clone(), p);
        println!(
            "Float::power_of_2_x_minus_1_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_x_minus_1_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::power_of_2_x_minus_1_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_x_minus_1_rational_prec_ref(&n, p);
        println!(
            "Float::power_of_2_x_minus_1_rational_prec_ref(&{}, {}) = {:x?}",
            n,
            p,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_x_minus_1_rational_prec_round({}, {}, {:?}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::power_of_2_x_minus_1_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_x_minus_1_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::power_of_2_x_minus_1_rational_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_round_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::power_of_2_x_minus_1_rational_prec_round_ref(&{}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::power_of_2_x_minus_1_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_power_of_2_x_minus_1_rational_prec_round_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::power_of_2_x_minus_1_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::power_of_2_x_minus_1_rational_prec_round_ref(&{}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_power_of_2_x_minus_1_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "primitive_float_power_of_2_x_minus_1_rational({}) = {:?}",
            x,
            NiceFloat(primitive_float_power_of_2_x_minus_1_rational::<T>(&x))
        );
    }
}

fn benchmark_float_power_of_2_x_minus_1_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_x_minus_1_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::power_of_2_x_minus_1_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::power_of_2_x_minus_1_rational_prec(n, prec)),
            ),
            (
                "Float::power_of_2_x_minus_1_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::power_of_2_x_minus_1_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_power_of_2_x_minus_1_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::power_of_2_x_minus_1_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::power_of_2_x_minus_1_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| {
                    no_out!(Float::power_of_2_x_minus_1_rational_prec_round(n, prec, rm))
                },
            ),
            (
                "Float::power_of_2_x_minus_1_rational_prec_round_ref(&Rational, u64, \
                RoundingMode)",
                &mut |(n, prec, rm)| {
                    no_out!(Float::power_of_2_x_minus_1_rational_prec_round_ref(
                        &n, prec, rm
                    ))
                },
            ),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_power_of_2_x_minus_1_rational<T: PrimitiveFloat>(
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
            "primitive_float_power_of_2_x_minus_1_rational::<{}>(Rational)",
            T::NAME
        ),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(primitive_float_power_of_2_x_minus_1_rational::<T>(&x));
        })],
    );
}
