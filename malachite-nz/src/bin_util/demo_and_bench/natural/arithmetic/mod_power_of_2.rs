// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, NegModPowerOf2, NegModPowerOf2Assign, RemPowerOf2,
    RemPowerOf2Assign,
};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_power_of_2::{
    limbs_mod_power_of_2, limbs_neg_mod_power_of_2, limbs_neg_mod_power_of_2_in_place,
    limbs_slice_mod_power_of_2_in_place, limbs_vec_mod_power_of_2_in_place,
};
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mod_power_of_2);
    register_demo!(runner, demo_limbs_slice_mod_power_of_2_in_place);
    register_demo!(runner, demo_limbs_vec_mod_power_of_2_in_place);
    register_demo!(runner, demo_limbs_neg_mod_power_of_2);
    register_demo!(runner, demo_limbs_neg_mod_power_of_2_in_place);
    register_demo!(runner, demo_natural_mod_power_of_2_assign);
    register_demo!(runner, demo_natural_mod_power_of_2);
    register_demo!(runner, demo_natural_mod_power_of_2_ref);
    register_demo!(runner, demo_natural_rem_power_of_2_assign);
    register_demo!(runner, demo_natural_rem_power_of_2);
    register_demo!(runner, demo_natural_rem_power_of_2_ref);
    register_demo!(runner, demo_natural_neg_mod_power_of_2_assign);
    register_demo!(runner, demo_natural_neg_mod_power_of_2);
    register_demo!(runner, demo_natural_neg_mod_power_of_2_ref);

    register_bench!(runner, benchmark_limbs_mod_power_of_2);
    register_bench!(runner, benchmark_limbs_slice_mod_power_of_2_in_place);
    register_bench!(runner, benchmark_limbs_vec_mod_power_of_2_in_place);
    register_bench!(runner, benchmark_limbs_neg_mod_power_of_2);
    register_bench!(runner, benchmark_limbs_neg_mod_power_of_2_in_place);
    register_bench!(runner, benchmark_natural_mod_power_of_2_assign);
    register_bench!(runner, benchmark_natural_mod_power_of_2_evaluation_strategy);
    register_bench!(runner, benchmark_natural_rem_power_of_2_assign);
    register_bench!(runner, benchmark_natural_rem_power_of_2_evaluation_strategy);
    register_bench!(runner, benchmark_natural_neg_mod_power_of_2_assign);
    register_bench!(
        runner,
        benchmark_natural_neg_mod_power_of_2_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_mod_power_of_2(&xs, pow)
        );
    }
}

fn demo_limbs_slice_mod_power_of_2_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_slice_mod_power_of_2_in_place(&mut xs, pow);
        println!(
            "xs := {xs_old:?}; limbs_slice_mod_power_of_2_in_place(&mut xs, {pow}); xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_mod_power_of_2_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_mod_power_of_2_in_place(&mut xs, pow);
        println!(
            "xs := {xs_old:?}; limbs_vec_mod_power_of_2_in_place(&mut xs, {pow}); xs = {xs:?}",
        );
    }
}

fn demo_limbs_neg_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_neg_mod_power_of_2({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_neg_mod_power_of_2(&xs, pow)
        );
    }
}

fn demo_limbs_neg_mod_power_of_2_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_neg_mod_power_of_2_in_place(&mut xs, pow);
        println!(
            "xs := {xs_old:?}; limbs_neg_mod_power_of_2_in_place(&mut xs, {pow}); xs = {xs:?}",
        );
    }
}

fn demo_natural_mod_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {n_old}; x.mod_power_of_2_assign({u}); x = {n}");
    }
}

fn demo_natural_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

fn demo_natural_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
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

fn demo_natural_rem_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {n_old}; x.rem_power_of_2_assign({u}); x = {n}");
    }
}

fn demo_natural_rem_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

fn demo_natural_rem_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
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

fn demo_natural_neg_mod_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.neg_mod_power_of_2_assign(u);
        println!("x := {n_old}; x.neg_mod_power_of_2_assign({u}); x = {n}");
    }
}

fn demo_natural_neg_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.neg_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.neg_mod_power_of_2(u)
        );
    }
}

fn demo_natural_neg_mod_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, u) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).neg_mod_power_of_2({}) = {}",
            n,
            u,
            (&n).neg_mod_power_of_2(u)
        );
    }
}

fn benchmark_limbs_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mod_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(xs, pow)| {
            no_out!(limbs_mod_power_of_2(&xs, pow))
        })],
    );
}

fn benchmark_limbs_slice_mod_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mod_power_of_2_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut xs, pow)| {
            limbs_slice_mod_power_of_2_in_place(&mut xs, pow)
        })],
    );
}

fn benchmark_limbs_vec_mod_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_mod_power_of_2_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut xs, pow)| {
            limbs_vec_mod_power_of_2_in_place(&mut xs, pow)
        })],
    );
}

fn benchmark_limbs_neg_mod_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_mod_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(xs, pow)| {
            no_out!(limbs_neg_mod_power_of_2(&xs, pow))
        })],
    );
}

fn benchmark_limbs_neg_mod_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_mod_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut xs, pow)| {
            limbs_neg_mod_power_of_2_in_place(&mut xs, pow)
        })],
    );
}

fn benchmark_natural_mod_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut n, u)| n.mod_power_of_2_assign(u))],
    );
}

fn benchmark_natural_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!(n.mod_power_of_2(u))
            }),
            ("(&Natural).mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!((&n).mod_power_of_2(u))
            }),
        ],
    );
}

fn benchmark_natural_rem_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.rem_power_of_2_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut n, u)| n.rem_power_of_2_assign(u))],
    );
}

fn benchmark_natural_rem_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.rem_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.rem_power_of_2(u64)", &mut |(n, u)| {
                no_out!(n.rem_power_of_2(u))
            }),
            ("(&Natural).rem_power_of_2(u64)", &mut |(n, u)| {
                no_out!((&n).rem_power_of_2(u))
            }),
        ],
    );
}

fn benchmark_natural_neg_mod_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.neg_mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [("Malachite", &mut |(mut n, u)| {
            n.neg_mod_power_of_2_assign(u)
        })],
    );
}

fn benchmark_natural_neg_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.neg_mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.neg_mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!(n.neg_mod_power_of_2(u))
            }),
            ("(&Natural).neg_mod_power_of_2(u64)", &mut |(n, u)| {
                no_out!((&n).neg_mod_power_of_2(u))
            }),
        ],
    );
}
