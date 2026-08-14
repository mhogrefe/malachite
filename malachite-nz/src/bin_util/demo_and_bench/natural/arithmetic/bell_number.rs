// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BellNumber;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::bell_number::bell_numbers_prefix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_bell_number);
    register_demo!(runner, demo_natural_bell_numbers_prefix);
    register_bench!(runner, benchmark_natural_bell_number);
    register_bench!(runner, benchmark_natural_bell_numbers_prefix);
}

fn demo_natural_bell_number(gm: GenMode, config: &GenConfig, limit: usize) {
    // The nth Bell number has on the order of n log n bits, so the demo argument is kept to u8
    // scale; that still reaches well past every internal tier boundary.
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "bell_number({}) = {}",
            n,
            Natural::bell_number(u64::from(n))
        );
    }
}

fn benchmark_natural_bell_number(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural::bell_number(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(Natural::bell_number(u64::from(n)));
        })],
    );
}

fn demo_natural_bell_numbers_prefix(gm: GenMode, config: &GenConfig, limit: usize) {
    // Prefix lengths stay at u8 scale: a length-255 prefix already ends with ~1700-bit entries.
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "bell_numbers_prefix({}) = {:?}",
            n,
            bell_numbers_prefix(u64::from(n))
        );
    }
}

fn benchmark_natural_bell_numbers_prefix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "bell_numbers_prefix(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(bell_numbers_prefix(u64::from(n)));
        })],
    );
}
