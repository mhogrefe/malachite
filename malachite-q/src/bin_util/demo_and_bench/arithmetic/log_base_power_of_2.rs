// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBasePowerOf2, FloorLogBasePowerOf2,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_signed_pair_gen_var_5;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_floor_log_base_power_of_2);
    register_demo!(runner, demo_rational_ceiling_log_base_power_of_2);
    register_demo!(runner, demo_rational_checked_log_base_power_of_2);
    register_bench!(runner, benchmark_rational_floor_log_base_power_of_2);
    register_bench!(runner, benchmark_rational_ceiling_log_base_power_of_2);
    register_bench!(runner, benchmark_rational_checked_log_base_power_of_2);
}

fn demo_rational_floor_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in rational_signed_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "floor_log_base_power_of_2({}, {}) = {}",
            n,
            pow,
            n.floor_log_base_power_of_2(pow)
        );
    }
}

fn demo_rational_ceiling_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in rational_signed_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "ceiling_log_base_power_of_2({}, {}) = {}",
            n,
            pow,
            n.ceiling_log_base_power_of_2(pow)
        );
    }
}

fn demo_rational_checked_log_base_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in rational_signed_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "checked_log_base_power_of_2({}, {}) = {:?}",
            n,
            pow,
            n.checked_log_base_power_of_2(pow)
        );
    }
}

fn benchmark_rational_floor_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_power_of_2(u64)",
        BenchmarkType::Single,
        rational_signed_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.floor_log_base_power_of_2(pow))
        })],
    );
}

fn benchmark_rational_ceiling_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_log_base_power_of_2(u64)",
        BenchmarkType::Single,
        rational_signed_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.ceiling_log_base_power_of_2(pow))
        })],
    );
}

fn benchmark_rational_checked_log_base_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_power_of_2(u64)",
        BenchmarkType::Single,
        rational_signed_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.checked_log_base_power_of_2(pow))
        })],
    );
}
