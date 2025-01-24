// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Reciprocal, ReciprocalAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::reciprocal::{
    reciprocal_prec_round_naive_1, reciprocal_prec_round_naive_2, rug_reciprocal,
    rug_reciprocal_prec, rug_reciprocal_prec_round, rug_reciprocal_round,
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
    float_gen, float_gen_rm, float_rounding_mode_pair_gen_var_13,
    float_rounding_mode_pair_gen_var_13_rm, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_1_rm, float_unsigned_rounding_mode_triple_gen_var_3,
    float_unsigned_rounding_mode_triple_gen_var_3_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_reciprocal);
    register_demo!(runner, demo_float_reciprocal_debug);
    register_demo!(runner, demo_float_reciprocal_ref);
    register_demo!(runner, demo_float_reciprocal_ref_debug);
    register_demo!(runner, demo_float_reciprocal_assign);
    register_demo!(runner, demo_float_reciprocal_assign_debug);
    register_demo!(runner, demo_float_reciprocal_prec);
    register_demo!(runner, demo_float_reciprocal_prec_debug);
    register_demo!(runner, demo_float_reciprocal_prec_ref);
    register_demo!(runner, demo_float_reciprocal_prec_ref_debug);
    register_demo!(runner, demo_float_reciprocal_prec_assign);
    register_demo!(runner, demo_float_reciprocal_prec_assign_debug);
    register_demo!(runner, demo_float_reciprocal_round);
    register_demo!(runner, demo_float_reciprocal_round_debug);
    register_demo!(runner, demo_float_reciprocal_round_ref);
    register_demo!(runner, demo_float_reciprocal_round_ref_debug);
    register_demo!(runner, demo_float_reciprocal_round_assign);
    register_demo!(runner, demo_float_reciprocal_round_assign_debug);
    register_demo!(runner, demo_float_reciprocal_prec_round);
    register_demo!(runner, demo_float_reciprocal_prec_round_debug);
    register_demo!(runner, demo_float_reciprocal_prec_round_ref);
    register_demo!(runner, demo_float_reciprocal_prec_round_ref_debug);
    register_demo!(runner, demo_float_reciprocal_prec_round_assign);
    register_demo!(runner, demo_float_reciprocal_prec_round_assign_debug);

    register_bench!(runner, benchmark_float_reciprocal_evaluation_strategy);
    register_bench!(runner, benchmark_float_reciprocal_library_comparison);
    register_bench!(runner, benchmark_float_reciprocal_algorithms);
    register_bench!(runner, benchmark_float_reciprocal_assign);
    register_bench!(runner, benchmark_float_reciprocal_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_reciprocal_prec_library_comparison);
    register_bench!(runner, benchmark_float_reciprocal_prec_algorithms);
    register_bench!(runner, benchmark_float_reciprocal_prec_assign);
    register_bench!(runner, benchmark_float_reciprocal_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_reciprocal_round_library_comparison);
    register_bench!(runner, benchmark_float_reciprocal_round_algorithms);
    register_bench!(runner, benchmark_float_reciprocal_round_assign);
    register_bench!(
        runner,
        benchmark_float_reciprocal_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_reciprocal_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_reciprocal_prec_round_algorithms);
    register_bench!(runner, benchmark_float_reciprocal_prec_round_assign);
}

fn demo_float_reciprocal(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).reciprocal() = {}", x_old, x.reciprocal());
    }
}

fn demo_float_reciprocal_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).reciprocal() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.reciprocal())
        );
    }
}

fn demo_float_reciprocal_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).reciprocal() = {}", x, (&x).reciprocal());
    }
}

fn demo_float_reciprocal_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).reciprocal() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).reciprocal())
        );
    }
}

fn demo_float_reciprocal_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.reciprocal_assign();
        println!("x := {x_old}; x.reciprocal_assign(); x = {x}");
    }
}

fn demo_float_reciprocal_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.reciprocal_assign();
        println!(
            "x := {:#x}; x.reciprocal_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_reciprocal_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).reciprocal_prec({}) = {:?}",
            x_old,
            prec,
            x.reciprocal_prec(prec)
        );
    }
}

fn demo_float_reciprocal_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.reciprocal_prec(prec);
        println!(
            "({:#x}).reciprocal_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_reciprocal_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).reciprocal_prec_ref({}) = {:?}",
            x,
            prec,
            x.reciprocal_prec_ref(prec)
        );
    }
}

fn demo_float_reciprocal_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (sum, o) = x.reciprocal_prec_ref(prec);
        println!(
            "(&{:#x}).reciprocal_prec_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_reciprocal_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.reciprocal_prec_assign(prec);
        println!("x := {x_old}; x.reciprocal_prec_assign({prec}); x = {x}");
    }
}

fn demo_float_reciprocal_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.reciprocal_prec_assign(prec);
        println!(
            "x := {:#x}; x.reciprocal_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_reciprocal_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).reciprocal_round({}) = {:?}",
            x_old,
            rm,
            x.reciprocal_round(rm)
        );
    }
}

fn demo_float_reciprocal_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.reciprocal_round(rm);
        println!(
            "({:#x}).reciprocal_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_reciprocal_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).reciprocal_round_ref({}) = {:?}",
            x,
            rm,
            x.reciprocal_round_ref(rm)
        );
    }
}

fn demo_float_reciprocal_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.reciprocal_round_ref(rm);
        println!(
            "(&{:#x}).reciprocal_round_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_reciprocal_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.reciprocal_round_assign(rm);
        println!("x := {x_old}; x.reciprocal_round_assign({rm}); x = {x}");
    }
}

fn demo_float_reciprocal_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.reciprocal_round_assign(rm);
        println!(
            "x := {:#x}; x.reciprocal_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_reciprocal_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).reciprocal_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.reciprocal_prec_round(prec, rm)
        );
    }
}

fn demo_float_reciprocal_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.reciprocal_prec_round(prec, rm);
        println!(
            "({:#x}).reciprocal_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_reciprocal_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).reciprocal_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.reciprocal_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_reciprocal_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.reciprocal_prec_round_ref(prec, rm);
        println!(
            "({:#x}).reciprocal_prec_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_reciprocal_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.reciprocal_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.reciprocal_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_reciprocal_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.reciprocal_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.reciprocal_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn benchmark_float_reciprocal_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.reciprocal()", &mut |x| no_out!(x.reciprocal())),
            ("(&Float).reciprocal()", &mut |x| no_out!((&x).reciprocal())),
        ],
    );
}

fn benchmark_float_reciprocal_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!((&x).reciprocal())),
            ("rug", &mut |(x, _)| no_out!(rug_reciprocal(&x))),
        ],
    );
}

fn benchmark_float_reciprocal_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal()",
        BenchmarkType::Algorithms,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.reciprocal())),
            ("naive 1", &mut |x| {
                let xsb = x.significant_bits();
                no_out!(reciprocal_prec_round_naive_1(x, xsb, Nearest).0)
            }),
            ("naive 2", &mut |x| {
                let xsb = x.significant_bits();
                no_out!(reciprocal_prec_round_naive_2(x, xsb, Nearest).0)
            }),
        ],
    );
}

fn benchmark_float_reciprocal_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.reciprocal_assign()", &mut |mut x| {
            x.reciprocal_assign()
        })],
    );
}

fn benchmark_float_reciprocal_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.reciprocal_prec(u64)", &mut |(x, prec)| {
                no_out!(x.reciprocal_prec(prec))
            }),
            ("(&Float).reciprocal_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.reciprocal_prec_ref(prec))
            }),
        ],
    );
}

fn benchmark_float_reciprocal_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.reciprocal_prec_ref(prec))
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_reciprocal_prec(&x, prec))
            }),
        ],
    );
}

fn benchmark_float_reciprocal_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec(u64)",
        BenchmarkType::Algorithms,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("default", &mut |(x, prec)| no_out!(x.reciprocal_prec(prec))),
            ("naive 1", &mut |(x, prec)| {
                no_out!(reciprocal_prec_round_naive_1(x, prec, Nearest))
            }),
            ("naive 2", &mut |(x, prec)| {
                no_out!(reciprocal_prec_round_naive_2(x, prec, Nearest))
            }),
        ],
    );
}

fn benchmark_float_reciprocal_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Float.reciprocal_prec_assign(u64)", &mut |(mut x, prec)| {
            no_out!(x.reciprocal_prec_assign(prec))
        })],
    );
}

fn benchmark_float_reciprocal_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.reciprocal_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.reciprocal_round(rm))
            }),
            (
                "(&Float).reciprocal_round_ref(RoundingMode)",
                &mut |(x, rm)| no_out!(x.reciprocal_round_ref(rm)),
            ),
        ],
    );
}

fn benchmark_float_reciprocal_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_13_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.reciprocal_round_ref(rm))
            }),
            ("rug", &mut |((x, rm), _)| {
                no_out!(rug_reciprocal_round(&x, rm))
            }),
        ],
    );
}

fn benchmark_float_reciprocal_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_round(RoundingMode)",
        BenchmarkType::Algorithms,
        float_rounding_mode_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("default", &mut |(x, rm)| no_out!(x.reciprocal_round(rm))),
            ("naive 1", &mut |(x, rm)| {
                let xsb = x.significant_bits();
                reciprocal_prec_round_naive_1(x, xsb, rm);
            }),
            ("naive 2", &mut |(x, rm)| {
                let xsb = x.significant_bits();
                reciprocal_prec_round_naive_2(x, xsb, rm);
            }),
        ],
    );
}

fn benchmark_float_reciprocal_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.reciprocal_round_assign(RoundingMode)",
            &mut |(mut x, rm)| no_out!(x.reciprocal_round_assign(rm)),
        )],
    );
}

fn benchmark_float_reciprocal_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.reciprocal_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.reciprocal_prec_round(prec, rm)),
            ),
            (
                "(&Float).reciprocal_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.reciprocal_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_reciprocal_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_3_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.reciprocal_prec_round_ref(prec, rm))
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_reciprocal_prec_round(&x, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_reciprocal_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("default", &mut |(x, prec, rm)| {
                no_out!(x.reciprocal_prec_round(prec, rm))
            }),
            ("naive 1", &mut |(x, prec, rm)| {
                no_out!(reciprocal_prec_round_naive_1(x, prec, rm))
            }),
            ("naive 2", &mut |(x, prec, rm)| {
                no_out!(reciprocal_prec_round_naive_2(x, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_reciprocal_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.reciprocal_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.reciprocal_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.reciprocal_prec_round_assign(prec, rm)),
        )],
    );
}
