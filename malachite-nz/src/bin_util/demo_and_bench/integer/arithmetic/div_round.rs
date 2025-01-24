// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingDivMod, DivRound, DivRoundAssign};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_2_pair_1_integer_bit_bucketer, triple_1_integer_bit_bucketer,
    triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_integer_rounding_mode_triple_gen_var_1, integer_pair_gen_var_1,
    integer_pair_gen_var_1_nrm, integer_pair_gen_var_1_rm,
};
use num::Integer as NumInteger;
use rug::ops::DivRounding;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_div_round);
    register_demo!(runner, demo_integer_div_round_val_ref);
    register_demo!(runner, demo_integer_div_round_ref_val);
    register_demo!(runner, demo_integer_div_round_ref_ref);
    register_demo!(runner, demo_integer_div_round_assign);
    register_demo!(runner, demo_integer_div_round_assign_ref);

    register_bench!(runner, benchmark_integer_div_round_down_library_comparison);
    register_bench!(runner, benchmark_integer_div_round_floor_library_comparison);
    register_bench!(
        runner,
        benchmark_integer_div_round_ceiling_library_comparison
    );
    register_bench!(runner, benchmark_integer_div_round_ceiling_algorithms);
    register_bench!(runner, benchmark_integer_div_round_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_integer_div_round_assign_evaluation_strategy
    );
}

fn demo_integer_div_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.div_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.div_round(y, rm)
        );
    }
}

fn demo_integer_div_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{}.div_round(&{}, {}) = {:?}",
            x_old,
            y,
            rm,
            x.div_round(&y, rm)
        );
    }
}

fn demo_integer_div_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{}).div_round({}, {}) = {:?}",
            x,
            y_old,
            rm,
            (&x).div_round(y, rm)
        );
    }
}

fn demo_integer_div_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in integer_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).div_round(&{}, {}) = {:?}",
            x,
            y,
            rm,
            (&x).div_round(&y, rm)
        );
    }
}

fn demo_integer_div_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in integer_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.div_round_assign(y, rm);
        println!("x := {x_old}; x.div_round_assign({y_old}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_integer_div_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in integer_integer_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.div_round_assign(&y, rm);
        println!("x := {x_old}; x.div_round_assign(&{y}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_integer_div_round_down_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_round(Integer, Down)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Malachite",
                &mut |(_, (x, y))| no_out!(x.div_round(y, Down)),
            ),
            ("rug", &mut |((x, y), _)| no_out!(x.div_trunc(y))),
        ],
    );
}

fn benchmark_integer_div_round_floor_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_round(Integer, Floor)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| {
                no_out!(x.div_round(y, Floor))
            }),
            ("num", &mut |((x, y), _, _)| no_out!(x.div_floor(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_floor(y))),
        ],
    );
}

fn benchmark_integer_div_round_ceiling_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_round(Integer, Ceiling)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.div_round(y, Ceiling))
            }),
            ("rug", &mut |((x, y), _)| no_out!(x.div_ceil(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_div_round_ceiling_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_round(Integer, Ceiling)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.div_round(y, Ceiling))),
            ("using ceiling_div_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_mod(y).0)
            }),
        ],
    );
}

fn benchmark_integer_div_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_round(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_integer_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Integer.div_round(Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.div_round(y, rm)),
            ),
            (
                "Integer.div_round(&Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.div_round(&y, rm)),
            ),
            (
                "(&Integer).div_round(Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).div_round(y, rm)),
            ),
            (
                "(&Integer).div_round(&Integer, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).div_round(&y, rm)),
            ),
        ],
    );
}

fn benchmark_integer_div_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_round_assign(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_integer_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
        &mut [
            (
                "Integer.div_round_assign(Integer, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.div_round_assign(y, rm)),
            ),
            (
                "Integer.div_round_assign(&Integer, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.div_round_assign(&y, rm)),
            ),
        ],
    );
}
