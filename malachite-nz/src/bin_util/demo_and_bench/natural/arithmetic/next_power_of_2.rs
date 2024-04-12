// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign};
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::next_power_of_2::{
    limbs_next_power_of_2, limbs_slice_next_power_of_2_in_place, limbs_vec_next_power_of_2_in_place,
};
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_next_power_of_2);
    register_demo!(runner, demo_limbs_slice_next_power_of_2_in_place);
    register_demo!(runner, demo_limbs_vec_next_power_of_2_in_place);
    register_demo!(runner, demo_natural_next_power_of_2_assign);
    register_demo!(runner, demo_natural_next_power_of_2);
    register_demo!(runner, demo_natural_next_power_of_2_ref);

    register_bench!(runner, benchmark_limbs_next_power_of_2);
    register_bench!(runner, benchmark_limbs_slice_next_power_of_2_in_place);
    register_bench!(runner, benchmark_limbs_vec_next_power_of_2_in_place);
    register_bench!(runner, benchmark_natural_next_power_of_2_assign);
    register_bench!(runner, benchmark_natural_next_power_of_2_library_comparison);
    register_bench!(
        runner,
        benchmark_natural_next_power_of_2_evaluation_strategy
    );
}

fn demo_limbs_next_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_next_power_of_2({:?}) = {:?}",
            xs,
            limbs_next_power_of_2(&xs)
        );
    }
}

fn demo_limbs_slice_next_power_of_2_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let carry = limbs_slice_next_power_of_2_in_place(&mut xs);
        println!(
            "xs := {xs_old:?}; \
            limbs_slice_next_power_of_2_in_place(&mut xs) = {carry}; xs = {xs:?}",
        );
    }
}

fn demo_limbs_vec_next_power_of_2_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_next_power_of_2_in_place(&mut xs);
        println!("xs := {xs_old:?}; limbs_vec_next_power_of_2_in_place(&mut xs); xs = {xs:?}");
    }
}

fn demo_natural_next_power_of_2_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in natural_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.next_power_of_2_assign();
        println!("x := {n_old}; x.next_power_of_2_assign(); x = {n}");
    }
}

fn demo_natural_next_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("{}.next_power_of_2() = {}", n_old, n.next_power_of_2());
    }
}

fn demo_natural_next_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("(&{}).next_power_of_2() = {}", n, (&n).next_power_of_2());
    }
}

fn benchmark_limbs_next_power_of_2(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_next_power_of_2(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_next_power_of_2(&xs)))],
    );
}

fn benchmark_limbs_slice_next_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_next_power_of_2_in_place(&mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut xs| {
            no_out!(limbs_slice_next_power_of_2_in_place(&mut xs))
        })],
    );
}

fn benchmark_limbs_vec_next_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_next_power_of_2_in_place(&mut Vec<Limb>)",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut xs| {
            limbs_vec_next_power_of_2_in_place(&mut xs)
        })],
    );
}

fn benchmark_natural_next_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.next_power_of_2_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |mut n| n.next_power_of_2_assign())],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_next_power_of_2_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.next_power_of_2()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.next_power_of_2())),
            ("rug", &mut |(n, _)| no_out!(n.next_power_of_two())),
        ],
    );
}

fn benchmark_natural_next_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.next_power_of_2()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Natural.next_power_of_2()", &mut |n| {
                no_out!(n.next_power_of_2())
            }),
            ("(&Natural).next_power_of_2()", &mut |n| {
                no_out!((&n).next_power_of_2())
            }),
        ],
    );
}
