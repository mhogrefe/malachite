// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Pow, ModPowerOf2PowAssign};
use malachite_base::test_util::bench::bucketers::{
    pair_product_vec_len_bucketer, triple_1_2_product_vec_len_bucketer,
    triple_2_bits_times_triple_3_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_pair_gen_var_3;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_power_of_2_pow::{
    limbs_mod_power_of_2_pow, limbs_pow_low,
};
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_5, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_21,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_pow_low);
    register_demo!(runner, demo_limbs_mod_power_of_2_pow);
    register_demo!(runner, demo_natural_mod_power_of_2_pow_assign);
    register_demo!(runner, demo_natural_mod_power_of_2_pow_assign_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_pow);
    register_demo!(runner, demo_natural_mod_power_of_2_pow_val_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_pow_ref_val);
    register_demo!(runner, demo_natural_mod_power_of_2_pow_ref_ref);

    register_bench!(runner, benchmark_limbs_pow_low);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_pow);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_pow_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_pow_algorithms);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_pow_evaluation_strategy
    );
}

fn demo_limbs_pow_low(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, es) in unsigned_vec_pair_gen_var_3().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let mut scratch = vec![0; xs.len()];
        limbs_pow_low(&mut xs, &es, &mut scratch);
        println!("xs := {xs_old:?}; limbs_pow_low(&mut xs, {es:?}, &mut scratch); xs = {xs:?}");
    }
}

fn demo_limbs_mod_power_of_2_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, es, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_mod_power_of_2_pow(&mut xs, &es, pow);
        println!("xs := {xs_old:?}; limbs_mod_power_of_2_pow(&mut xs, {es:?}, {pow}); xs = {xs:?}");
    }
}

fn demo_natural_mod_power_of_2_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, pow) in natural_natural_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let exp_old = exp.clone();
        x.mod_power_of_2_pow_assign(exp, pow);
        println!("x := {x_old}; x.mod_power_of_2_pow_assign({exp_old}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_pow_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, pow) in natural_natural_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_pow_assign(&exp, pow);
        println!("x := {x_old}; x.mod_power_of_2_pow_assign({exp}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, pow) in natural_natural_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) ≡ {} mod 2^{}",
            x_old,
            exp_old,
            x.mod_power_of_2_pow(exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_pow_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, pow) in natural_natural_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{}.pow({}) ≡ {} mod 2^{}",
            x_old,
            exp,
            x.mod_power_of_2_pow(&exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_pow_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, pow) in natural_natural_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) ≡ {} mod 2^{}",
            x,
            exp_old,
            (&x).mod_power_of_2_pow(exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_pow_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, pow) in natural_natural_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.pow({}) ≡ {} mod 2^{}",
            x,
            exp,
            (&x).mod_power_of_2_pow(&exp, pow),
            pow
        );
    }
}

fn benchmark_limbs_pow_low(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_pow_low(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_product_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, es)| {
            let mut scratch = vec![0; xs.len()];
            limbs_pow_low(&mut xs, &es, &mut scratch)
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_pow(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_pow(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_product_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, es, pow)| {
            limbs_mod_power_of_2_pow(&mut xs, &es, pow)
        })],
    );
}

fn benchmark_natural_mod_power_of_2_pow_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_pow_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bits_times_triple_3_bucketer("exp", "pow"),
        &mut [
            (
                "Natural.mod_power_of_2_pow_assign(Natural, u64)",
                &mut |(mut x, exp, pow)| no_out!(x.mod_power_of_2_pow_assign(exp, pow)),
            ),
            (
                "Natural.mod_power_of_2_pow_assign(&Natural, u64)",
                &mut |(mut x, exp, pow)| no_out!(x.mod_power_of_2_pow_assign(&exp, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_pow(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_natural_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bits_times_triple_3_bucketer("exp", "pow"),
        &mut [
            ("default", &mut |(x, exp, pow)| {
                no_out!(x.mod_power_of_2_pow(exp, pow))
            }),
            ("simple binary", &mut |(x, exp, pow)| {
                no_out!(simple_binary_mod_power_of_2_pow(&x, &exp, pow))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_pow(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bits_times_triple_3_bucketer("exp", "pow"),
        &mut [
            (
                "Natural.mod_power_of_2_pow(Natural, u64)",
                &mut |(x, exp, pow)| no_out!(x.mod_power_of_2_pow(exp, pow)),
            ),
            (
                "Natural.mod_power_of_2_pow(&Natural, u64)",
                &mut |(x, exp, pow)| no_out!(x.mod_power_of_2_pow(&exp, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_pow(Natural, u64)",
                &mut |(x, exp, pow)| no_out!((&x).mod_power_of_2_pow(exp, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_pow(&Natural, u64)",
                &mut |(x, exp, pow)| no_out!((&x).mod_power_of_2_pow(&exp, pow)),
            ),
        ],
    );
}
