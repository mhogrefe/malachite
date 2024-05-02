// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Sub, ModPowerOf2SubAssign, ModSub, PowerOf2,
};
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mod_power_of_2_sub::{
    limbs_mod_power_of_2_limb_sub_limbs, limbs_mod_power_of_2_limb_sub_limbs_in_place,
    limbs_mod_power_of_2_sub, limbs_mod_power_of_2_sub_in_place_either,
    limbs_mod_power_of_2_sub_in_place_left, limbs_mod_power_of_2_sub_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_4, unsigned_vec_unsigned_unsigned_triple_gen_var_16,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mod_power_of_2_limb_sub_limbs);
    register_demo!(runner, demo_limbs_mod_power_of_2_limb_sub_limbs_in_place);
    register_demo!(runner, demo_limbs_mod_power_of_2_sub);
    register_demo!(runner, demo_limbs_mod_power_of_2_sub_in_place_left);
    register_demo!(runner, demo_limbs_mod_power_of_2_sub_in_place_right);
    register_demo!(runner, demo_limbs_mod_power_of_2_sub_in_place_either);
    register_demo!(runner, demo_natural_mod_power_of_2_sub_assign);
    register_demo!(runner, demo_natural_mod_power_of_2_sub_assign_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_sub);
    register_demo!(runner, demo_natural_mod_power_of_2_sub_val_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_sub_ref_val);
    register_demo!(runner, demo_natural_mod_power_of_2_sub_ref_ref);

    register_bench!(runner, benchmark_limbs_mod_power_of_2_limb_sub_limbs);
    register_bench!(
        runner,
        benchmark_limbs_mod_power_of_2_limb_sub_limbs_in_place
    );
    register_bench!(runner, benchmark_limbs_mod_power_of_2_sub);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_sub_in_place_left);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_sub_in_place_right);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_sub_in_place_either);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_sub_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_sub_algorithms);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_sub_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2_limb_sub_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ys, x, pow) in unsigned_vec_unsigned_unsigned_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2_limb_sub_limbs({}, {:?}, {}) = {:?}",
            x,
            ys,
            pow,
            limbs_mod_power_of_2_limb_sub_limbs(x, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_limb_sub_limbs_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut ys, x, pow) in unsigned_vec_unsigned_unsigned_triple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let ys_old = ys.clone();
        limbs_mod_power_of_2_limb_sub_limbs_in_place(x, &mut ys, pow);
        println!(
            "ys := {ys_old:?}; limbs_mod_power_of_2_limb_sub_limbs_in_place({x}, &mut ys, {pow}); \
            ys = {ys:?}",
        );
    }
}

fn demo_limbs_mod_power_of_2_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2_sub({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_2_sub(&xs, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_sub_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_mod_power_of_2_sub_in_place_left(&mut xs, &ys, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_mod_power_of_2_sub_in_place_left(&mut xs, {ys:?}, {pow}); xs = {xs:?}",
        );
    }
}

fn demo_limbs_mod_power_of_2_sub_in_place_right(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let ys_old = ys.clone();
        limbs_mod_power_of_2_sub_in_place_right(&xs, &mut ys, pow);
        println!(
            "ys := {ys_old:?}; \
            limbs_mod_power_of_2_sub_in_place_right({xs:?}, &mut ys, {pow}); ys = {ys:?}",
        );
    }
}

fn demo_limbs_mod_power_of_2_sub_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_mod_power_of_2_sub_in_place_either(&mut xs, &mut ys, pow);
        println!(
            "xs := {xs_old:?}; ys := {ys_old:?}; \
            limbs_mod_power_of_2_sub_in_place_either(&mut xs, &mut ys, {pow}) = {right}; \
            xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_natural_mod_power_of_2_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_power_of_2_sub_assign(y, pow);
        println!("x := {x_old}; x.mod_power_of_2_sub_assign({y_old}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_sub_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_sub_assign(&y, pow);
        println!("x := {x_old}; x.mod_power_of_2_sub_assign(&{y}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} - {} ≡ {} mod 2^{}",
            x_old,
            y_old,
            x.mod_power_of_2_sub(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{} - {} ≡ {} mod 2^{}",
            x_old,
            y,
            x.mod_power_of_2_sub(&y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "{} - {} ≡ {} mod 2^{}",
            x,
            y_old,
            (&x).mod_power_of_2_sub(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} - {} ≡ {} mod 2^{}",
            x,
            y,
            (&x).mod_power_of_2_sub(&y, pow),
            pow
        );
    }
}

fn benchmark_limbs_mod_power_of_2_limb_sub_limbs(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_limb_sub_limbs(Limb, &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(ys, x, pow)| {
            no_out!(limbs_mod_power_of_2_limb_sub_limbs(x, &ys, pow))
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_limb_sub_limbs_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_limb_sub_limbs_in_place(Limb, &mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut ys, x, pow)| {
            limbs_mod_power_of_2_limb_sub_limbs_in_place(x, &mut ys, pow)
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_sub(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_sub(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(ref xs, ref ys, pow)| {
            no_out!(limbs_mod_power_of_2_sub(xs, ys, pow))
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_sub_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_sub_in_place_left(&mut Vec<Limb>, &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys, pow)| {
            limbs_mod_power_of_2_sub_in_place_left(xs, ys, pow)
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_sub_in_place_right(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_sub_in_place_right(&[Limb], &mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(ref xs, ref mut ys, pow)| {
            limbs_mod_power_of_2_sub_in_place_right(xs, ys, pow)
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_sub_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_sub_in_place_left(&mut Vec<Limb>, &mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(ref mut xs, ref mut ys, pow)| {
            no_out!(limbs_mod_power_of_2_sub_in_place_either(xs, ys, pow))
        })],
    );
}

fn benchmark_natural_mod_power_of_2_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_sub_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_sub_assign(Natural, u64)",
                &mut |(mut x, y, pow)| no_out!(x.mod_power_of_2_sub_assign(y, pow)),
            ),
            (
                "Natural.mod_power_of_2_sub_assign(&Natural, u64)",
                &mut |(mut x, y, pow)| no_out!(x.mod_power_of_2_sub_assign(&y, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_sub_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_sub(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, pow)| {
                no_out!(x.mod_power_of_2_sub(y, pow))
            }),
            ("alt", &mut |(x, y, pow)| {
                if x >= y {
                    x - y
                } else {
                    let mut x = x;
                    x.set_bit(pow);
                    x - y
                };
            }),
            ("naive", &mut |(x, y, pow)| {
                no_out!((Integer::from(x) - Integer::from(y)).mod_power_of_2(pow))
            }),
            ("using mod_sub", &mut |(x, y, pow)| {
                no_out!(x.mod_sub(y, Natural::power_of_2(pow)))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_sub(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_sub(Natural, u64)",
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_sub(y, pow)),
            ),
            (
                "Natural.mod_power_of_2_sub(&Natural, u64)",
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_sub(&y, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_sub(Natural, u64)",
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_sub(y, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_sub(&Natural, u64)",
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_sub(&y, pow)),
            ),
        ],
    );
}
