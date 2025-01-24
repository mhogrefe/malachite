// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::log_base_2::{
    limbs_ceiling_log_base_2, limbs_checked_log_base_2, limbs_floor_log_base_2,
};
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_floor_log_base_2);
    register_demo!(runner, demo_limbs_ceiling_log_base_2);
    register_demo!(runner, demo_limbs_checked_log_base_2);
    register_demo!(runner, demo_natural_floor_log_base_2);
    register_demo!(runner, demo_natural_ceiling_log_base_2);
    register_demo!(runner, demo_natural_checked_log_base_2);
    register_bench!(runner, benchmark_limbs_floor_log_base_2);
    register_bench!(runner, benchmark_limbs_ceiling_log_base_2);
    register_bench!(runner, benchmark_limbs_checked_log_base_2);
    register_bench!(runner, benchmark_natural_floor_log_base_2);
    register_bench!(runner, benchmark_natural_ceiling_log_base_2);
    register_bench!(runner, benchmark_natural_checked_log_base_2);
}

fn demo_limbs_floor_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_floor_log_base_2({:?}) = {}",
            xs,
            limbs_floor_log_base_2(&xs)
        );
    }
}

fn demo_limbs_ceiling_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_ceiling_log_base_2({:?}) = {}",
            xs,
            limbs_ceiling_log_base_2(&xs)
        );
    }
}

fn demo_limbs_checked_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "limbs_checked_log_base_2({:?}) = {:?}",
            xs,
            limbs_checked_log_base_2(&xs)
        );
    }
}

fn demo_natural_floor_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("floor_log_base_2({}) = {}", n, n.floor_log_base_2());
    }
}

fn demo_natural_ceiling_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("ceiling_log_base_2({}) = {}", n, n.ceiling_log_base_2());
    }
}

fn demo_natural_checked_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("checked_log_base_2({}) = {:?}", n, n.checked_log_base_2());
    }
}

fn benchmark_limbs_floor_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_floor_log_base_2(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |ref xs| {
            no_out!(limbs_floor_log_base_2(xs))
        })],
    );
}

fn benchmark_limbs_ceiling_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_ceiling_log_base_2(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |ref xs| {
            no_out!(limbs_ceiling_log_base_2(xs))
        })],
    );
}

fn benchmark_limbs_checked_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_checked_log_base_2(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |ref xs| {
            no_out!(limbs_checked_log_base_2(xs))
        })],
    );
}

fn benchmark_natural_floor_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_log_base_2()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2()))],
    );
}

fn benchmark_natural_ceiling_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_log_base_2()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2()))],
    );
}

fn benchmark_natural_checked_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_log_base_2()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.checked_log_base_2()))],
    );
}
