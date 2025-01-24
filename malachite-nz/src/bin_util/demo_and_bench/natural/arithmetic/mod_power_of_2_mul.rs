// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModMul, ModPowerOf2, ModPowerOf2Mul, ModPowerOf2MulAssign, PowerOf2,
};
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_power_of_2_mul::{
    limbs_mod_power_of_2_mul, limbs_mod_power_of_2_mul_ref_ref, limbs_mod_power_of_2_mul_val_ref,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_4, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mod_power_of_2_mul);
    register_demo!(runner, demo_limbs_mod_power_of_2_mul_val_ref);
    register_demo!(runner, demo_limbs_mod_power_of_2_mul_ref_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_mul_assign);
    register_demo!(runner, demo_natural_mod_power_of_2_mul_assign_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_mul);
    register_demo!(runner, demo_natural_mod_power_of_2_mul_val_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_mul_ref_val);
    register_demo!(runner, demo_natural_mod_power_of_2_mul_ref_ref);

    register_bench!(
        runner,
        benchmark_limbs_mod_power_of_2_mul_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_mul_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_mul_algorithms);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_mul_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        println!(
            "limbs_mod_power_of_2_mul({:?}, {:?}, {}) = {:?}",
            xs_old,
            ys_old,
            pow,
            limbs_mod_power_of_2_mul(&mut xs, &mut ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        println!(
            "limbs_mod_power_of_2_mul({:?}, {:?}, {}) = {:?}",
            xs_old,
            ys,
            pow,
            limbs_mod_power_of_2_mul_val_ref(&mut xs, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2_mul_ref_ref({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_2_mul_ref_ref(&xs, &ys, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_power_of_2_mul_assign(y, pow);
        println!("x := {x_old}; x.mod_power_of_2_mul_assign({y_old}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_mul_assign(&y, pow);
        println!("x := {x_old}; x.mod_power_of_2_mul_assign(&{y}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} * {} ≡ {} mod 2^{}",
            x_old,
            y_old,
            x.mod_power_of_2_mul(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{} * {} ≡ {} mod 2^{}",
            x_old,
            y,
            x.mod_power_of_2_mul(&y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "{} * {} ≡ {} mod 2^{}",
            x,
            y_old,
            (&x).mod_power_of_2_mul(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} * {} ≡ {} mod 2^{}",
            x,
            y,
            (&x).mod_power_of_2_mul(&y, pow),
            pow
        );
    }
}

fn benchmark_limbs_mod_power_of_2_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_mul(&[Limb], &[Limb], u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("limbs_mod_power_of_2_mul", &mut |(
                ref mut xs,
                ref mut ys,
                pow,
            )| {
                no_out!(limbs_mod_power_of_2_mul(xs, ys, pow))
            }),
            ("limbs_mod_power_of_2_mul_val_ref", &mut |(
                ref mut xs,
                ref ys,
                pow,
            )| {
                no_out!(limbs_mod_power_of_2_mul_val_ref(xs, ys, pow))
            }),
            ("limbs_mod_power_of_2_mul_ref_ref", &mut |(
                ref xs,
                ref ys,
                pow,
            )| {
                no_out!(limbs_mod_power_of_2_mul_ref_ref(xs, ys, pow))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_mul_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_mul_assign(Natural, u64)",
                &mut |(mut x, y, pow)| no_out!(x.mod_power_of_2_mul_assign(y, pow)),
            ),
            (
                "Natural.mod_power_of_2_mul_assign(&Natural, u64)",
                &mut |(mut x, y, pow)| no_out!(x.mod_power_of_2_mul_assign(&y, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_mul(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, pow)| {
                no_out!(x.mod_power_of_2_mul(y, pow))
            }),
            ("naive", &mut |(x, y, pow)| {
                no_out!((x * y).mod_power_of_2(pow))
            }),
            ("using mod_mul", &mut |(x, y, pow)| {
                no_out!(x.mod_mul(y, Natural::power_of_2(pow)))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_mul(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_mul(Natural, u64)",
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_mul(y, pow)),
            ),
            (
                "Natural.mod_power_of_2_mul(&Natural, u64)",
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_mul(&y, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_mul(Natural, u64)",
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_mul(y, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_mul(&Natural, u64)",
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_mul(&y, pow)),
            ),
        ],
    );
}
