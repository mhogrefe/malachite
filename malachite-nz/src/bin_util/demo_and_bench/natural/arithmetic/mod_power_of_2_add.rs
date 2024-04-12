// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModAdd, ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, PowerOf2,
};
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::bucketers::{triple_1_vec_len_bucketer, triple_3_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_power_of_2_add::{
    limbs_mod_power_of_2_add, limbs_mod_power_of_2_add_greater,
    limbs_mod_power_of_2_add_in_place_either, limbs_mod_power_of_2_add_limb,
    limbs_slice_mod_power_of_2_add_greater_in_place_left,
    limbs_slice_mod_power_of_2_add_limb_in_place, limbs_vec_mod_power_of_2_add_in_place_left,
    limbs_vec_mod_power_of_2_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_4, unsigned_vec_unsigned_unsigned_triple_gen_var_14,
    unsigned_vec_unsigned_unsigned_triple_gen_var_15,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mod_power_of_2_add_limb);
    register_demo!(runner, demo_limbs_slice_mod_power_of_2_add_limb_in_place);
    register_demo!(runner, demo_limbs_vec_mod_power_of_2_add_limb_in_place);
    register_demo!(runner, demo_limbs_mod_power_of_2_add_greater);
    register_demo!(runner, demo_limbs_mod_power_of_2_add);
    register_demo!(
        runner,
        demo_limbs_slice_mod_power_of_2_add_greater_in_place_left
    );
    register_demo!(runner, demo_limbs_vec_mod_power_of_2_add_in_place_left);
    register_demo!(runner, demo_limbs_mod_power_of_2_add_in_place_either);
    register_demo!(runner, demo_natural_mod_power_of_2_add_assign);
    register_demo!(runner, demo_natural_mod_power_of_2_add_assign_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_add);
    register_demo!(runner, demo_natural_mod_power_of_2_add_val_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_add_ref_val);
    register_demo!(runner, demo_natural_mod_power_of_2_add_ref_ref);

    register_bench!(runner, benchmark_limbs_mod_power_of_2_add_limb);
    register_bench!(
        runner,
        benchmark_limbs_slice_mod_power_of_2_add_limb_in_place
    );
    register_bench!(runner, benchmark_limbs_vec_mod_power_of_2_add_limb_in_place);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_add_greater);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_add);
    register_bench!(
        runner,
        benchmark_limbs_slice_mod_power_of_2_add_greater_in_place_left
    );
    register_bench!(runner, benchmark_limbs_vec_mod_power_of_2_add_in_place_left);
    register_bench!(runner, benchmark_limbs_mod_power_of_2_add_in_place_either);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_add_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_add_algorithms);
    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_add_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2_add_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y, pow) in unsigned_vec_unsigned_unsigned_triple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2_add_limb({:?}, {}, {}) = {:?}",
            xs,
            y,
            pow,
            limbs_mod_power_of_2_add_limb(&xs, y, pow)
        );
    }
}

fn demo_limbs_slice_mod_power_of_2_add_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, y, pow) in unsigned_vec_unsigned_unsigned_triple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_mod_power_of_2_add_limb_in_place(&mut xs, y, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_mod_power_of_2_add_limb_in_place(&mut xs, {y}, {pow}) = {carry}; \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_mod_power_of_2_add_limb_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, y, pow) in unsigned_vec_unsigned_unsigned_triple_gen_var_15()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_mod_power_of_2_add_limb_in_place(&mut xs, y, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_vec_mod_power_of_2_add_limb_in_place(&mut xs, {y}, {pow}); xs = {xs:?}",
        );
    }
}

fn demo_limbs_mod_power_of_2_add_greater(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_add_greater({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_2_add_greater(&xs, &ys, pow)
        );
    }
}

fn demo_limbs_mod_power_of_2_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_power_of_2_add({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_mod_power_of_2_add(&xs, &ys, pow)
        );
    }
}

fn demo_limbs_slice_mod_power_of_2_add_greater_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, &ys, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut xs, {ys:?}, {pow}) \
            = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_mod_power_of_2_add_in_place_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_mod_power_of_2_add_in_place_left(&mut xs, &ys, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_vec_mod_power_of_2_add_in_place_left(&mut xs, {ys:?}, {pow}); \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_mod_power_of_2_add_in_place_either(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys, pow) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_mod_power_of_2_add_in_place_either(&mut xs, &mut ys, pow);
        println!(
            "xs := {xs_old:?}; \
            ys := {ys_old:?}; limbs_mod_power_of_2_add_in_place_either(&mut xs, &mut ys, \
            {pow}) = {right}; xs = {xs:?}; ys = {ys:?}",
        );
    }
}

fn demo_natural_mod_power_of_2_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_power_of_2_add_assign(y, pow);
        println!("x := {x_old}; x.mod_power_of_2_add_assign({y_old}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_add_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_add_assign(&y, pow);
        println!("x := {x_old}; x.mod_power_of_2_add_assign(&{y}, {pow}); x = {x}");
    }
}

fn demo_natural_mod_power_of_2_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{} + {} ≡ {} mod 2^{}",
            x_old,
            y_old,
            x.mod_power_of_2_add(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_add_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{} + {} ≡ {} mod 2^{}",
            x_old,
            y,
            x.mod_power_of_2_add(&y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_add_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "{} + {} ≡ {} mod 2^{}",
            x,
            y_old,
            (&x).mod_power_of_2_add(y, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in natural_natural_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{} + {} ≡ {} mod 2^{}",
            x,
            y,
            (&x).mod_power_of_2_add(&y, pow),
            pow
        );
    }
}

fn benchmark_limbs_mod_power_of_2_add_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_add_limb(&[Limb], Limb, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y, pow)| {
            no_out!(limbs_mod_power_of_2_add_limb(&xs, y, pow))
        })],
    );
}

fn benchmark_limbs_slice_mod_power_of_2_add_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mod_power_of_2_add_limb_in_place(&mut [Limb], Limb, u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y, pow)| {
            no_out!(limbs_slice_mod_power_of_2_add_limb_in_place(
                &mut xs, y, pow
            ))
        })],
    );
}

fn benchmark_limbs_vec_mod_power_of_2_add_limb_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_mod_power_of_2_add_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_14().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y, pow)| {
            limbs_vec_mod_power_of_2_add_limb_in_place(&mut xs, y, pow)
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_add_greater(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_add_greater(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, ref ys, pow)| {
            no_out!(limbs_mod_power_of_2_add_greater(xs, ys, pow))
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_add(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_add(&[Limb], &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(ref xs, ref ys, pow)| {
            no_out!(limbs_mod_power_of_2_add(xs, ys, pow))
        })],
    );
}

fn benchmark_limbs_slice_mod_power_of_2_add_greater_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut [Limb], &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, ys, pow)| {
            no_out!(limbs_slice_mod_power_of_2_add_greater_in_place_left(
                &mut xs, &ys, pow
            ))
        })],
    );
}

fn benchmark_limbs_vec_mod_power_of_2_add_in_place_left(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_mod_power_of_2_add_in_place_left(&Vec<Limb>, &[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut xs, ys, pow)| {
            no_out!(limbs_vec_mod_power_of_2_add_in_place_left(
                &mut xs, &ys, pow
            ))
        })],
    );
}

fn benchmark_limbs_mod_power_of_2_add_in_place_either(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_power_of_2_add_in_place_either(&mut [Limb], &mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_18().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut xs, mut ys, pow)| {
            no_out!(limbs_mod_power_of_2_add_in_place_either(
                &mut xs, &mut ys, pow
            ))
        })],
    );
}

fn benchmark_natural_mod_power_of_2_add_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_add_assign(Natural, u64)",
                &mut |(mut x, y, pow)| no_out!(x.mod_power_of_2_add_assign(y, pow)),
            ),
            (
                "Natural.mod_power_of_2_add_assign(&Natural, u64)",
                &mut |(mut x, y, pow)| no_out!(x.mod_power_of_2_add_assign(&y, pow)),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_add_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, pow)| {
                no_out!(x.mod_power_of_2_add(y, pow))
            }),
            ("alt", &mut |(x, y, pow)| {
                let mut sum = x + y;
                sum.clear_bit(pow);
            }),
            ("naive", &mut |(x, y, pow)| {
                no_out!((x + y).mod_power_of_2(pow))
            }),
            ("using mod_add", &mut |(x, y, pow)| {
                no_out!(x.mod_add(y, Natural::power_of_2(pow)))
            }),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_add(Natural, u64)",
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_add(y, pow)),
            ),
            (
                "Natural.mod_power_of_2_add(&Natural, u64)",
                &mut |(x, y, pow)| no_out!(x.mod_power_of_2_add(&y, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_add(Natural, u64)",
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_add(y, pow)),
            ),
            (
                "(&Natural).mod_power_of_2_add(&Natural, u64)",
                &mut |(x, y, pow)| no_out!((&x).mod_power_of_2_add(&y, pow)),
            ),
        ],
    );
}
