// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Mul, ModPowerOf2Square, ModPowerOf2SquareAssign, ModSquare, PowerOf2,
    Square,
};
use malachite_base::test_util::bench::bucketers::{pair_2_bucketer, pair_2_vec_len_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_20,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_power_of_2_square::{
    limbs_mod_power_of_2_square, limbs_mod_power_of_2_square_ref, limbs_square_low,
    limbs_square_low_basecase, limbs_square_low_divide_and_conquer, limbs_square_low_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_11, unsigned_vec_pair_gen_var_21,
    unsigned_vec_unsigned_pair_gen_var_30,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_square::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_square_low_basecase);
    register_demo!(runner, demo_limbs_square_low_divide_and_conquer);
    register_demo!(runner, demo_limbs_square_low);
    register_demo!(runner, demo_limbs_mod_power_of_2_square);
    register_demo!(runner, demo_limbs_mod_power_of_2_square_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_square_assign);
    register_demo!(runner, demo_natural_mod_power_of_2_square);
    register_demo!(runner, demo_natural_mod_power_of_2_square_ref);

    register_bench!(runner, benchmark_limbs_square_low_basecase);
    register_bench!(
        runner,
        benchmark_limbs_square_low_divide_and_conquer_algorithms
    );
    register_bench!(runner, benchmark_limbs_square_low);
    register_bench!(
        runner,
        benchmark_limbs_mod_power_of_2_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_square_assign);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_square_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_square_algorithms);
}

fn demo_limbs_square_low_basecase(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_21().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_square_low_basecase(&mut out, &xs);
        println!("out := {out_old:?}; limbs_square_low_basecase(&mut out, {xs:?}); out = {out:?}");
    }
}

fn demo_limbs_square_low_divide_and_conquer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_20().get(gm, config).take(limit) {
        let out_old = out.clone();
        let mut scratch = vec![0; limbs_square_low_scratch_len(xs.len())];
        limbs_square_low_divide_and_conquer(&mut out, &xs, &mut scratch);
        println!(
            "out := {out_old:?}; \
            limbs_square_low_divide_and_conquer(&mut out, {xs:?}, &mut scratch); \
            out = {out:?}",
        );
    }
}

fn demo_limbs_square_low(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_square_low(&mut out, &xs);
        println!("out := {out_old:?}; limbs_square_low(&mut out, {xs:?}); out = {out:?}");
    }
}

fn demo_limbs_mod_power_of_2_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_30()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        println!(
            "limbs_mod_power_of_2_square({:?}, {}) = {:?}",
            xs_old,
            pow,
            limbs_mod_power_of_2_square(&mut xs, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_30()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2_square_ref({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_mod_power_of_2_square_ref(&xs, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.mod_power_of_2_square_assign(pow);
        println!("x := {n_old}; x.mod_power_of_2_square_assign({pow}); x = {n}");
    }
}

fn demo_natural_mod_power_of_2_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.square() ≡ {} mod 2^{}",
            n_old,
            n.mod_power_of_2_square(pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).square() ≡ {} mod 2^{}",
            n,
            (&n).mod_power_of_2_square(pow),
            pow
        );
    }
}

fn benchmark_limbs_square_low_basecase(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_low_basecase(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs)| {
            limbs_square_low_basecase(&mut out, &xs)
        })],
    );
}

fn benchmark_limbs_square_low_divide_and_conquer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_low_divide_and_conquer(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs)| {
                limbs_square_low_basecase_unrestricted(&mut out, &xs)
            }),
            ("divide and conquer", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_low_scratch_len(xs.len())];
                limbs_square_low_divide_and_conquer(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_square_low(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_square_low(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs)| {
            limbs_square_low(&mut out, &xs)
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_square(&[Limb], u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_pair_gen_var_30().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("limbs_mod_power_of_2_square", &mut |(ref mut xs, pow)| {
                no_out!(limbs_mod_power_of_2_square(xs, pow))
            }),
            ("limbs_mod_power_of_2_square_ref", &mut |(
                ref mut xs,
                pow,
            )| {
                no_out!(limbs_mod_power_of_2_square_ref(xs, pow))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_square_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_square_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [(
            "Natural.mod_power_of_2_square_assign(u64)",
            &mut |(mut n, pow)| n.mod_power_of_2_square_assign(pow),
        )],
    );
}

fn benchmark_natural_mod_power_of_2_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_square(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2_square(u64)", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_square(pow))
            }),
            ("(&Natural).mod_power_of_2_square(u64)", &mut |(n, pow)| {
                no_out!((&n).mod_power_of_2_square(pow))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_square(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("pow"),
        &mut [
            ("Natural.mod_power_of_2_square(u64)", &mut |(n, pow)| {
                no_out!(n.mod_power_of_2_square(pow))
            }),
            (
                "Natural.mod_power_of_2_mul(Natural, u64)",
                &mut |(n, pow)| no_out!(n.clone().mod_power_of_2_mul(n, pow)),
            ),
            ("Natural.square().mod_power_of_2(u64)", &mut |(n, pow)| {
                no_out!(n.square().mod_power_of_2(pow))
            }),
            (
                "Natural.mod_square(Natural::power_of_2(u64))",
                &mut |(n, pow)| no_out!(n.mod_square(Natural::power_of_2(pow))),
            ),
        ],
    );
}
