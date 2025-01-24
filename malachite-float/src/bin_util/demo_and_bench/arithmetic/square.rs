// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::square::{
    rug_square, rug_square_prec, rug_square_prec_round, rug_square_round, square_prec_round_naive,
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
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_22,
    float_rounding_mode_pair_gen_var_7, float_rounding_mode_pair_gen_var_7_rm,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_1_rm, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_11, float_unsigned_rounding_mode_triple_gen_var_2,
    float_unsigned_rounding_mode_triple_gen_var_2_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_square);
    register_demo!(runner, demo_float_square_debug);
    register_demo!(runner, demo_float_square_extreme);
    register_demo!(runner, demo_float_square_extreme_debug);
    register_demo!(runner, demo_float_square_ref);
    register_demo!(runner, demo_float_square_ref_debug);
    register_demo!(runner, demo_float_square_assign);
    register_demo!(runner, demo_float_square_assign_debug);
    register_demo!(runner, demo_float_square_prec);
    register_demo!(runner, demo_float_square_prec_debug);
    register_demo!(runner, demo_float_square_prec_extreme);
    register_demo!(runner, demo_float_square_prec_extreme_debug);
    register_demo!(runner, demo_float_square_prec_ref);
    register_demo!(runner, demo_float_square_prec_ref_debug);
    register_demo!(runner, demo_float_square_prec_assign);
    register_demo!(runner, demo_float_square_prec_assign_debug);
    register_demo!(runner, demo_float_square_round);
    register_demo!(runner, demo_float_square_round_debug);
    register_demo!(runner, demo_float_square_round_extreme);
    register_demo!(runner, demo_float_square_round_extreme_debug);
    register_demo!(runner, demo_float_square_round_ref);
    register_demo!(runner, demo_float_square_round_ref_debug);
    register_demo!(runner, demo_float_square_round_assign);
    register_demo!(runner, demo_float_square_round_assign_debug);
    register_demo!(runner, demo_float_square_prec_round);
    register_demo!(runner, demo_float_square_prec_round_debug);
    register_demo!(runner, demo_float_square_prec_round_extreme);
    register_demo!(runner, demo_float_square_prec_round_extreme_debug);
    register_demo!(runner, demo_float_square_prec_round_ref);
    register_demo!(runner, demo_float_square_prec_round_ref_debug);
    register_demo!(runner, demo_float_square_prec_round_assign);
    register_demo!(runner, demo_float_square_prec_round_assign_debug);

    register_bench!(runner, benchmark_float_square_evaluation_strategy);
    register_bench!(runner, benchmark_float_square_library_comparison);
    register_bench!(runner, benchmark_float_square_algorithms);
    register_bench!(runner, benchmark_float_square_assign);
    register_bench!(runner, benchmark_float_square_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_square_prec_library_comparison);
    register_bench!(runner, benchmark_float_square_prec_algorithms);
    register_bench!(runner, benchmark_float_square_prec_assign);
    register_bench!(runner, benchmark_float_square_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_square_round_library_comparison);
    register_bench!(runner, benchmark_float_square_round_algorithms);
    register_bench!(runner, benchmark_float_square_round_assign);
    register_bench!(
        runner,
        benchmark_float_square_prec_round_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_square_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_square_prec_round_algorithms);
    register_bench!(runner, benchmark_float_square_prec_round_assign);
}

fn demo_float_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}) ^ 2 = {}", x_old, x.square());
    }
}

fn demo_float_square_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}) ^ 2 = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.square())
        );
    }
}

fn demo_float_square_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}) ^ 2 = {}", x_old, x.square());
    }
}

fn demo_float_square_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}) ^ 2 = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.square())
        );
    }
}

fn demo_float_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}) ^ 2 = {}", x, (&x).square());
    }
}

fn demo_float_square_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}) ^ 2 = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).square())
        );
    }
}

fn demo_float_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.square_assign();
        println!("x := {x_old}; x ^= 2; x = {x}");
    }
}

fn demo_float_square_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.square_assign();
        println!(
            "x := {:#x}; x ^= 2; x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_square_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).square_prec({}) = {:?}",
            x_old,
            prec,
            x.square_prec(prec)
        );
    }
}

fn demo_float_square_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.square_prec(prec);
        println!(
            "({:#x}).square_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).square_prec({}) = {:?}",
            x_old,
            prec,
            x.square_prec(prec)
        );
    }
}

fn demo_float_square_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.square_prec(prec);
        println!(
            "({:#x}).square_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).square_prec_ref({}) = {:?}",
            x,
            prec,
            x.square_prec_ref(prec)
        );
    }
}

fn demo_float_square_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (sum, o) = x.square_prec_ref(prec);
        println!(
            "(&{:#x}).square_prec_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.square_prec_assign(prec);
        println!("x := {x_old}; x.square_prec_assign({prec}); x = {x}");
    }
}

fn demo_float_square_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.square_prec_assign(prec);
        println!(
            "x := {:#x}; x.square_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_square_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).square_round({}) = {:?}",
            x_old,
            rm,
            x.square_round(rm)
        );
    }
}

fn demo_float_square_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.square_round(rm);
        println!(
            "({:#x}).square_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).square_round({}) = {:?}",
            x_old,
            rm,
            x.square_round(rm)
        );
    }
}

fn demo_float_square_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_22()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.square_round(rm);
        println!(
            "({:#x}).square_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).square_round_ref({}) = {:?}",
            x,
            rm,
            x.square_round_ref(rm)
        );
    }
}

fn demo_float_square_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.square_round_ref(rm);
        println!(
            "(&{:#x}).square_round_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.square_round_assign(rm);
        println!("x := {x_old}; x.square_round_assign({rm}); x = {x}");
    }
}

fn demo_float_square_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.square_round_assign(rm);
        println!(
            "x := {:#x}; x.square_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_square_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).square_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.square_prec_round(prec, rm)
        );
    }
}

fn demo_float_square_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.square_prec_round(prec, rm);
        println!(
            "({:#x}).square_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).square_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.square_prec_round(prec, rm)
        );
    }
}

fn demo_float_square_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.square_prec_round(prec, rm);
        println!(
            "({:#x}).square_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).square_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.square_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_square_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.square_prec_round_ref(prec, rm);
        println!(
            "({:#x}).square_prec_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_square_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.square_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.square_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_square_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.square_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.square_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.square()", &mut |x| no_out!(x.square())),
            ("(&Float).square()", &mut |x| no_out!((&x).square())),
        ],
    );
}

fn benchmark_float_square_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!((&x).square())),
            ("rug", &mut |(x, _)| no_out!(rug_square(&x))),
        ],
    );
}

fn benchmark_float_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square()",
        BenchmarkType::Algorithms,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.square())),
            ("naive", &mut |x| {
                let xsb = x.significant_bits();
                no_out!(square_prec_round_naive(x, xsb, Nearest).0)
            }),
        ],
    );
}

fn benchmark_float_square_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.square_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.square_assign()", &mut |mut x| x.square_assign())],
    );
}

fn benchmark_float_square_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.square_prec(u64)", &mut |(x, prec)| {
                no_out!(x.square_prec(prec))
            }),
            ("(&Float).square_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.square_prec_ref(prec))
            }),
        ],
    );
}

fn benchmark_float_square_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.square_prec_ref(prec))
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_square_prec(&x, prec))
            }),
        ],
    );
}

fn benchmark_float_square_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec(u64)",
        BenchmarkType::Algorithms,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("default", &mut |(x, prec)| no_out!(x.square_prec(prec))),
            ("naive", &mut |(x, prec)| {
                no_out!(square_prec_round_naive(x, prec, Nearest))
            }),
        ],
    );
}

fn benchmark_float_square_prec_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Float.square_prec_assign(u64)", &mut |(mut x, prec)| {
            no_out!(x.square_prec_assign(prec))
        })],
    );
}

fn benchmark_float_square_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.square_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.square_round(rm))
            }),
            ("(&Float).square_round_ref(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.square_round_ref(rm))
            }),
        ],
    );
}

fn benchmark_float_square_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_round(RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_7_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.square_round_ref(rm))
            }),
            ("rug", &mut |((x, rm), _)| no_out!(rug_square_round(&x, rm))),
        ],
    );
}

fn benchmark_float_square_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_round(RoundingMode)",
        BenchmarkType::Algorithms,
        float_rounding_mode_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("default", &mut |(x, rm)| no_out!(x.square_round(rm))),
            ("naive", &mut |(x, rm)| {
                let xsb = x.significant_bits();
                square_prec_round_naive(x, xsb, rm);
            }),
        ],
    );
}

fn benchmark_float_square_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Float.square_round_assign(RoundingMode)", &mut |(
            mut x,
            rm,
        )| {
            no_out!(x.square_round_assign(rm))
        })],
    );
}

fn benchmark_float_square_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.square_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.square_prec_round(prec, rm)),
            ),
            (
                "(&Float).square_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.square_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_square_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_2_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.square_prec_round_ref(prec, rm))
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_square_prec_round(&x, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_square_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("default", &mut |(x, prec, rm)| {
                no_out!(x.square_prec_round(prec, rm))
            }),
            ("naive", &mut |(x, prec, rm)| {
                no_out!(square_prec_round_naive(x, prec, rm))
            }),
        ],
    );
}

fn benchmark_float_square_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.square_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.square_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.square_prec_round_assign(prec, rm)),
        )],
    );
}
