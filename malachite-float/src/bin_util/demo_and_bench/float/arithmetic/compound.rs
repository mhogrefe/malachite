// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Compound, CompoundAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::{
    pair_2_quadruple_1_3_float_primitive_int_max_complexity_bucketer,
    pair_float_signed_max_complexity_bucketer, quadruple_1_float_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::compound::rug_compound_prec_round;
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_unsigned_rounding_mode_quadruple_gen_var_17,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_17_rm,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_18,
    float_signed_unsigned_triple_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_compound_prec_round_extreme);
    register_demo!(runner, demo_float_compound_prec_extreme);
    register_demo!(runner, demo_float_compound_prec_round);
    register_demo!(runner, demo_float_compound_prec_round_debug);
    register_demo!(runner, demo_float_compound_prec);
    register_demo!(runner, demo_float_compound_prec_debug);
    register_demo!(runner, demo_float_compound_round);
    register_demo!(runner, demo_float_compound_round_debug);
    register_demo!(runner, demo_float_compound);
    register_demo!(runner, demo_float_compound_debug);
    register_demo!(runner, demo_float_compound_assign);
    register_bench!(runner, benchmark_float_compound_prec_round);
    register_bench!(runner, benchmark_float_compound_prec);
    register_bench!(runner, benchmark_float_compound_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_float_compound_prec_round_library_comparison
    );
}

fn demo_float_compound_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).compound_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.compound_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_compound_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).compound_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.compound_prec(n, prec)
        );
    }
}

fn demo_float_compound_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).compound_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.compound_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_compound_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (compound, o) = x.compound_prec_round(n, prec, rm);
        println!(
            "({:#x}).compound_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            rm,
            ComparableFloat(compound),
            o
        );
    }
}

fn demo_float_compound_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_signed_unsigned_triple_gen_var_1::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).compound_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.compound_prec(n, prec)
        );
    }
}

fn demo_float_compound_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_signed_unsigned_triple_gen_var_1::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (compound, o) = x.compound_prec(n, prec);
        println!(
            "({:#x}).compound_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            ComparableFloat(compound),
            o
        );
    }
}

fn demo_float_compound_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).compound_round({}, {}) = {:?}",
            x_old,
            n,
            rm,
            x.compound_round(n, rm)
        );
    }
}

fn demo_float_compound_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let (compound, o) = x.compound_round(n, rm);
        println!(
            "({:#x}).compound_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            rm,
            ComparableFloat(compound),
            o
        );
    }
}

fn demo_float_compound(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).compound({}) = {}", x_old, n, x.compound(n));
    }
}

fn demo_float_compound_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).compound({}) = {:#x}",
            ComparableFloat(x_old),
            n,
            ComparableFloat(x.compound(n))
        );
    }
}

fn demo_float_compound_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.compound_assign(n);
        println!("x := {x_old}; x.compound_assign({n}); x = {x}");
    }
}

fn benchmark_float_compound_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.compound_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_17().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, n, prec, rm)| {
            no_out!(x.compound_prec_round(n, prec, rm));
        })],
    );
}

fn benchmark_float_compound_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.compound_prec(i64, u64)",
        BenchmarkType::Single,
        float_signed_unsigned_triple_gen_var_1::<i64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "n"),
        &mut [("Malachite", &mut |(x, n, prec)| {
            no_out!(x.compound_prec(n, prec));
        })],
    );
}

fn benchmark_float_compound_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.compound(i64)",
        BenchmarkType::EvaluationStrategy,
        float_signed_pair_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_signed_max_complexity_bucketer("x", "n"),
        &mut [
            ("Float.compound(i64)", &mut |(x, n)| no_out!(x.compound(n))),
            ("(&Float).compound(i64)", &mut |(x, n)| {
                no_out!((&x).compound(n));
            }),
        ],
    );
}

fn benchmark_float_compound_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.compound_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_17_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_3_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, n, prec, rm))| {
                no_out!(x.compound_prec_round(n, prec, rm));
            }),
            ("rug", &mut |((x, n, prec, rm), _)| {
                no_out!(rug_compound_prec_round(&x, i32::exact_from(n), prec, rm));
            }),
        ],
    );
}
