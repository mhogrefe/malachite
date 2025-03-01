// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsInteger;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_is_integer);
    register_bench!(runner, benchmark_natural_is_integer);
}

fn demo_natural_is_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        if n.is_integer() {
            println!("{n} is an integer");
        } else {
            println!("{n} is not an integer");
        }
    }
}

fn benchmark_natural_is_integer(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.is_integer()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_integer()))],
    );
}
