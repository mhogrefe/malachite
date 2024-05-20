// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_rm};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_sign);
    register_bench!(runner, benchmark_natural_sign_library_comparison);
}

fn demo_natural_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        match n.sign() {
            Less => println!("{n} is negative"),
            Equal => println!("{n} is zero"),
            Greater => println!("{n} is positive"),
        }
    }
}

fn benchmark_natural_sign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sign()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.sign())),
            ("rug", &mut |(n, _)| no_out!(n.cmp0())),
        ],
    );
}
