// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingDivNegMod, DivRound, DivRoundAssign};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::bucketers::triple_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_rounding_mode_triple_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::div_round::limbs_limb_div_round_limbs;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_bit_bucketer, pair_2_pair_1_natural_bit_bucketer, triple_1_natural_bit_bucketer,
    triple_3_pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_natural_rounding_mode_triple_gen_var_1, natural_pair_gen_var_5,
    natural_pair_gen_var_5_nrm, natural_pair_gen_var_5_rm,
};
use num::Integer;
use rug::ops::DivRounding;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_limb_div_round_limbs);
    register_demo!(runner, demo_natural_div_round);
    register_demo!(runner, demo_natural_div_round_val_ref);
    register_demo!(runner, demo_natural_div_round_ref_val);
    register_demo!(runner, demo_natural_div_round_ref_ref);
    register_demo!(runner, demo_natural_div_round_assign);
    register_demo!(runner, demo_natural_div_round_assign_ref);

    register_bench!(runner, benchmark_limbs_limb_div_round_limbs);
    register_bench!(runner, benchmark_natural_div_round_down_library_comparison);
    register_bench!(runner, benchmark_natural_div_round_floor_library_comparison);
    register_bench!(
        runner,
        benchmark_natural_div_round_ceiling_library_comparison
    );
    register_bench!(runner, benchmark_natural_div_round_ceiling_algorithms);
    register_bench!(
        runner,
        benchmark_natural_div_round_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_div_round_evaluation_strategy);
}

fn demo_limbs_limb_div_round_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ys, x, rm) in unsigned_vec_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_limb_div_round_limbs({}, {:?}, {}) = {:?}",
            x,
            ys,
            rm,
            limbs_limb_div_round_limbs(x, &ys, rm)
        );
    }
}

fn demo_natural_div_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in natural_natural_rounding_mode_triple_gen_var_1()
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

fn demo_natural_div_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in natural_natural_rounding_mode_triple_gen_var_1()
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

fn demo_natural_div_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in natural_natural_rounding_mode_triple_gen_var_1()
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

fn demo_natural_div_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in natural_natural_rounding_mode_triple_gen_var_1()
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

fn demo_natural_div_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in natural_natural_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.div_round_assign(y, rm);
        println!("x := {x_old}; x.div_round_assign({y_old}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_natural_div_round_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, rm) in natural_natural_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.div_round_assign(&y, rm);
        println!("x := {x_old}; x.div_round_assign(&{y}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_limbs_limb_div_round_limbs(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_limb_div_round_limbs(Limb, &[Limb], RoundingMode)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("ys"),
        &mut [("Malachite", &mut |(ys, x, rm)| {
            no_out!(limbs_limb_div_round_limbs(x, &ys, rm))
        })],
    );
}

fn benchmark_natural_div_round_down_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_round(Natural, Down)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Malachite",
                &mut |(_, (x, y))| no_out!(x.div_round(y, Down)),
            ),
            ("rug", &mut |((x, y), _)| no_out!(x.div_trunc(y))),
        ],
    );
}

fn benchmark_natural_div_round_floor_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_round(Natural, Floor)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| {
                no_out!(x.div_round(y, Floor))
            }),
            ("num", &mut |((x, y), _, _)| no_out!(x.div_floor(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_floor(y))),
        ],
    );
}

fn benchmark_natural_div_round_ceiling_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_round(Natural, Ceiling)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_5_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.div_round(y, Ceiling))
            }),
            ("rug", &mut |((x, y), _)| no_out!(x.div_ceil(y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_natural_div_round_ceiling_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_round(Natural, Ceiling)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x.div_round(y, Ceiling))),
            ("using ceiling_div_neg_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_neg_mod(y).0)
            }),
        ],
    );
}

fn benchmark_natural_div_round_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_round_assign(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Natural.div_round_assign(Natural, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.div_round_assign(y, rm)),
            ),
            (
                "Natural.div_round_assign(&Natural, RoundingMode)",
                &mut |(mut x, y, rm)| no_out!(x.div_round_assign(&y, rm)),
            ),
        ],
    );
}

fn benchmark_natural_div_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.div_round(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Natural.div_round(Natural, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.div_round(y, rm)),
            ),
            (
                "Natural.div_round(&Natural, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.div_round(&y, rm)),
            ),
            (
                "(&Natural).div_round(Natural, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).div_round(y, rm)),
            ),
            (
                "(&Natural).div_round(&Natural, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).div_round(&y, rm)),
            ),
        ],
    );
}
