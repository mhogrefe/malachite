// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, FloorLogBase2};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_bit_bucketer;
use malachite_q::test_util::generators::{rational_gen_var_1, rational_gen_var_2};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_floor_log_base_2_abs);
    register_demo!(runner, demo_rational_ceiling_log_base_2_abs);
    register_demo!(runner, demo_rational_floor_log_base_2);
    register_demo!(runner, demo_rational_ceiling_log_base_2);
    register_demo!(runner, demo_rational_checked_log_base_2);

    register_bench!(runner, benchmark_rational_floor_log_base_2_abs);
    register_bench!(runner, benchmark_rational_ceiling_log_base_2_abs);
    register_bench!(runner, benchmark_rational_floor_log_base_2);
    register_bench!(runner, benchmark_rational_ceiling_log_base_2);
    register_bench!(runner, benchmark_rational_checked_log_base_2);
}

fn demo_rational_floor_log_base_2_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        println!("floor_log_base_2_abs({}) = {}", n, n.floor_log_base_2_abs());
    }
}

fn demo_rational_ceiling_log_base_2_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        println!(
            "ceiling_log_base_2_abs({}) = {}",
            n,
            n.ceiling_log_base_2_abs()
        );
    }
}

fn demo_rational_floor_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, config).take(limit) {
        println!("floor_log_base_2({}) = {}", n, n.floor_log_base_2());
    }
}

fn demo_rational_ceiling_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, config).take(limit) {
        println!("ceiling_log_base_2({}) = {}", n, n.ceiling_log_base_2());
    }
}

fn demo_rational_checked_log_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, config).take(limit) {
        println!("checked_log_base_2({}) = {:?}", n, n.checked_log_base_2());
    }
}

fn benchmark_rational_floor_log_base_2_abs(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_2_abs()",
        BenchmarkType::Single,
        rational_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2_abs()))],
    );
}

fn benchmark_rational_ceiling_log_base_2_abs(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_log_base_2_abs()",
        BenchmarkType::Single,
        rational_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2_abs()))],
    );
}

fn benchmark_rational_floor_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_2()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.floor_log_base_2()))],
    );
}

fn benchmark_rational_ceiling_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.ceiling_log_base_2()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_log_base_2()))],
    );
}

fn benchmark_rational_checked_log_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.floor_log_base_2()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(n.checked_log_base_2()))],
    );
}
