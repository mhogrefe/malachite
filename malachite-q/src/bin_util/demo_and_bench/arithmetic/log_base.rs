// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingLogBase, CheckedLogBase, FloorLogBase};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_rational_max_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen_var_2, rational_pair_gen_var_7};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_approx_log);
    register_demo!(runner, demo_rational_floor_log_base);
    register_demo!(runner, demo_rational_ceiling_log_base);
    register_demo!(runner, demo_rational_checked_log_base);
    register_bench!(runner, benchmark_approx_log);
    register_bench!(runner, benchmark_rational_floor_log_base);
    register_bench!(runner, benchmark_rational_ceiling_log_base);
    register_bench!(runner, benchmark_rational_checked_log_base);
}

fn demo_rational_approx_log(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_2().get(gm, config).take(limit) {
        println!("log({}) ≈ {}", n, NiceFloat(n.approx_log()));
    }
}

fn demo_rational_floor_log_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, base) in rational_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "floor_log_base({}, {}) = {}",
            n,
            base,
            n.floor_log_base(&base)
        );
    }
}

fn demo_rational_ceiling_log_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, base) in rational_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "ceiling_log_base({}, {}) = {}",
            n,
            base,
            n.ceiling_log_base(&base)
        );
    }
}

fn demo_rational_checked_log_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, base) in rational_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "checked_log_base({}, {}) = {:?}",
            n,
            base,
            n.checked_log_base(&base)
        );
    }
}

fn benchmark_approx_log(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "(&Rational).approx_log()",
        BenchmarkType::Single,
        rational_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("n"),
        &mut [("default", &mut |n| no_out!(n.approx_log()))],
    );
}

fn benchmark_rational_floor_log_base(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Rational).floor_log_base(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("n", "base"),
        &mut [("Malachite", &mut |(n, base)| {
            no_out!(n.floor_log_base(&base))
        })],
    );
}

fn benchmark_rational_ceiling_log_base(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Rational).ceiling_log_base(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("n", "base"),
        &mut [("Malachite", &mut |(n, base)| {
            no_out!(n.ceiling_log_base(&base))
        })],
    );
}

fn benchmark_rational_checked_log_base(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Rational).checked_log_base(&Rational)",
        BenchmarkType::Single,
        rational_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("n", "base"),
        &mut [("Malachite", &mut |(n, base)| {
            no_out!(n.checked_log_base(&base))
        })],
    );
}
