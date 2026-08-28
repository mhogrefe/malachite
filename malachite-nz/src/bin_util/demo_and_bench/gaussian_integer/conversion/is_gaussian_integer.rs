// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsGaussianInteger;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_is_gaussian_integer);
    register_bench!(runner, benchmark_gaussian_integer_is_gaussian_integer);
}

fn demo_gaussian_integer_is_gaussian_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in gaussian_integer_gen().get(gm, config).take(limit) {
        if n.is_gaussian_integer() {
            println!("{n} is a Gaussian integer");
        } else {
            println!("{n} is not a Gaussian integer");
        }
    }
}

fn benchmark_gaussian_integer_is_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.is_gaussian_integer()",
        BenchmarkType::Single,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_gaussian_integer()))],
    );
}
