// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_significant_bits);
    register_bench!(runner, benchmark_gaussian_integer_significant_bits);
    register_demo!(runner, demo_gaussian_integer_max_significant_bits);
    register_bench!(runner, benchmark_gaussian_integer_max_significant_bits);
}

fn demo_gaussian_integer_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("significant_bits({}) = {}", x, x.significant_bits());
    }
}

fn benchmark_gaussian_integer_significant_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.significant_bits()",
        BenchmarkType::Single,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.significant_bits()))],
    );
}

fn demo_gaussian_integer_max_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in gaussian_integer_gen().get(gm, config).take(limit) {
        println!("max_significant_bits({}) = {}", x, x.max_significant_bits());
    }
}

fn benchmark_gaussian_integer_max_significant_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.max_significant_bits()",
        BenchmarkType::Single,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.max_significant_bits()))],
    );
}
