// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, ModPowerOf2, ModPowerOf2Assign, RemPowerOf2,
    RemPowerOf2Assign,
};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_mod_power_of_2_assign);
    register_demo!(runner, demo_integer_mod_power_of_2);
    register_demo!(runner, demo_integer_mod_power_of_2_ref);
    register_demo!(runner, demo_integer_rem_power_of_2_assign);
    register_demo!(runner, demo_integer_rem_power_of_2);
    register_demo!(runner, demo_integer_rem_power_of_2_ref);
    register_demo!(runner, demo_integer_ceiling_mod_power_of_2_assign);
    register_demo!(runner, demo_integer_ceiling_mod_power_of_2);
    register_demo!(runner, demo_integer_ceiling_mod_power_of_2_ref);

    register_bench!(runner, benchmark_integer_mod_power_of_2_assign);
    register_bench!(runner, benchmark_integer_mod_power_of_2_evaluation_strategy);
    register_bench!(runner, benchmark_integer_rem_power_of_2_assign);
    register_bench!(runner, benchmark_integer_rem_power_of_2_evaluation_strategy);
    register_bench!(runner, benchmark_integer_ceiling_mod_power_of_2_assign);
    register_bench!(
        runner,
        benchmark_integer_ceiling_mod_power_of_2_evaluation_strategy
    );
}

fn demo_integer_mod_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {n_old}; x.mod_power_of_2_assign({u}); x = {n}");
    }
}

fn demo_integer_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

fn demo_integer_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mod_power_of_2({}) = {}",
            n,
            u,
            (&n).mod_power_of_2(u)
        );
    }
}

fn demo_integer_rem_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {n_old}; x.rem_power_of_2_assign({u}); x = {n}");
    }
}

fn demo_integer_rem_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

fn demo_integer_rem_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).rem_power_of_2({}) = {}",
            n,
            u,
            (&n).rem_power_of_2(u)
        );
    }
}

fn demo_integer_ceiling_mod_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.ceiling_mod_power_of_2_assign(u);
        println!("x := {n_old}; x.ceiling_mod_power_of_2_assign({u}); x = {n}");
    }
}

fn demo_integer_ceiling_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_2(u)
        );
    }
}

fn demo_integer_ceiling_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).ceiling_mod_power_of_2({}) = {}",
            n,
            u,
            (&n).ceiling_mod_power_of_2(u)
        );
    }
}

fn benchmark_integer_mod_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, u)| n.mod_power_of_2_assign(u))],
    );
}

fn benchmark_integer_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [
            ("Integer.mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!(n.mod_power_of_2(u))
            }),
            ("(&Integer).mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!((&n).mod_power_of_2(u))
            }),
        ],
    );
}

fn benchmark_integer_rem_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.rem_power_of_2_assign(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, u)| n.rem_power_of_2_assign(u))],
    );
}

fn benchmark_integer_rem_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.rem_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [
            ("Integer.rem_power_of_2(u64)", &mut |(n, u)| {
                no_out!(n.rem_power_of_2(u))
            }),
            ("(&Integer).rem_power_of_2(u64)", &mut |(n, u)| {
                no_out!((&n).rem_power_of_2(u))
            }),
        ],
    );
}

fn benchmark_integer_ceiling_mod_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, u)| {
            n.ceiling_mod_power_of_2_assign(u)
        })],
    );
}

fn benchmark_integer_ceiling_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [
            ("Integer.ceiling_mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!(n.ceiling_mod_power_of_2(u))
            }),
            ("(&Integer).ceiling_mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!((&n).ceiling_mod_power_of_2(u))
            }),
        ],
    );
}
