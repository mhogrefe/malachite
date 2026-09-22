// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Height, ModIsReduced, ModPowerOf2IsReduced};
use malachite_base::test_util::bench::bucketers::unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_is_reduced);
    register_demo!(runner, demo_unsigned_polynomial_mod_power_of_2_is_reduced);

    register_bench!(runner, benchmark_unsigned_polynomial_mod_is_reduced);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_is_reduced
    );
}

// A polynomial is reduced modulo anything above its height, so the moduli here are drawn from
// around it: one above is always reduced, the height itself never is.
fn demo_unsigned_polynomial_mod_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        let m = p.to_height().saturating_add(1);
        println!("({p}).mod_is_reduced(&{m}) = {}", p.mod_is_reduced(&m));
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        for pow in [0, 1, 8, 64] {
            println!(
                "({p}).mod_power_of_2_is_reduced({pow}) = {}",
                p.mod_power_of_2_is_reduced(pow)
            );
        }
    }
}

fn benchmark_unsigned_polynomial_mod_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_is_reduced(&u64)",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            let m = p.to_height().saturating_add(1);
            no_out!(p.mod_is_reduced(&m));
        })],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_power_of_2_is_reduced(u64)",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(p.mod_power_of_2_is_reduced(64));
        })],
    );
}
