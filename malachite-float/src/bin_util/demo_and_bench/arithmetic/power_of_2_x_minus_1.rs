// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{PowerOf2XMinus1, PowerOf2XMinus1Assign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
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
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

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
