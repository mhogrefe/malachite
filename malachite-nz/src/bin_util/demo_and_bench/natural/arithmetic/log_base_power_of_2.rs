// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBasePowerOf2, FloorLogBasePowerOf2,
};
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_13;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::log_base_power_of_2::{
    limbs_ceiling_log_base_power_of_2, limbs_checked_log_base_power_of_2,
    limbs_floor_log_base_power_of_2,
};
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_8;
use malachite_nz::test_util::natural::arithmetic::log_base_power_of_2::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_floor_log_base_power_of_2);
    register_demo!(runner, demo_limbs_ceiling_log_base_power_of_2);
    register_demo!(runner, demo_limbs_checked_log_base_power_of_2);
    register_demo!(runner, demo_natural_floor_log_base_power_of_2);
    register_demo!(runner, demo_natural_ceiling_log_base_power_of_2);
    register_demo!(runner, demo_natural_checked_log_base_power_of_2);
    register_bench!(runner, benchmark_limbs_floor_log_base_power_of_2);
    register_bench!(runner, benchmark_limbs_ceiling_log_base_power_of_2);
    register_bench!(runner, benchmark_limbs_checked_log_base_power_of_2);
    register_bench!(runner, benchmark_natural_floor_log_base_power_of_2);
    register_bench!(
        runner,
        benchmark_natural_ceiling_log_base_power_of_2_algorithms
    );
    register_bench!(runner, benchmark_natural_checked_log_base_power_of_2);
}

fn demo_limbs_floor_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_floor_log_base_power_of_2({:?}, {}) = {}",
            xs,
            pow,
            limbs_floor_log_base_power_of_2(&xs, pow)
        );
    }
}

fn demo_limbs_ceiling_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_ceiling_log_base_power_of_2({:?}, {}) = {}",
            xs,
            pow,
            limbs_ceiling_log_base_power_of_2(&xs, pow)
        );
    }
}

fn demo_limbs_checked_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_checked_log_base_power_of_2({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_checked_log_base_power_of_2(&xs, pow)
        );
    }
}

fn demo_natural_floor_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "floor_log_base_power_of_2({}, {}) = {}",
            n,
            pow,
            n.floor_log_base_power_of_2(pow)
        );
    }
}

fn demo_natural_ceiling_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "ceiling_log_base_power_of_2({}, {}) = {}",
            n,
            pow,
            n.ceiling_log_base_power_of_2(pow)
        );
    }
}

fn demo_natural_checked_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_8()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "checked_log_base_power_of_2({}, {}) = {:?}",
            n,
            pow,
            n.checked_log_base_power_of_2(pow)
        );
    }
}

fn benchmark_limbs_floor_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_floor_log_base_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, pow)| {
            no_out!(limbs_floor_log_base_power_of_2(xs, pow))
        })],
    );
}

fn benchmark_limbs_ceiling_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_ceiling_log_base_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, pow)| {
            no_out!(limbs_ceiling_log_base_power_of_2(xs, pow))
        })],
    );
}

fn benchmark_limbs_checked_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_checked_log_base_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, pow)| {
            no_out!(limbs_checked_log_base_power_of_2(xs, pow))
        })],
    );
}

fn benchmark_natural_floor_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_log_base_power_of_2(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.floor_log_base_power_of_2(pow))
        })],
    );
}

fn benchmark_natural_ceiling_log_base_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.ceiling_log_base_power_of_2(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, pow)| {
                no_out!(n.ceiling_log_base_power_of_2(pow))
            }),
            ("naive", &mut |(ref n, pow)| {
                no_out!(ceiling_log_base_power_of_2_naive_nz(n, pow))
            }),
        ],
    );
}

fn benchmark_natural_checked_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.floor_log_base_power_of_2(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.checked_log_base_power_of_2(pow))
        })],
    );
}
