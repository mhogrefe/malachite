// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_pair_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::{integer_pair_gen, integer_pair_gen_nrm};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_cmp);
    register_bench!(runner, benchmark_integer_cmp_library_comparison);
}

fn demo_integer_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        match x.cmp(&y) {
            Less => println!("{x} < {y}"),
            Equal => println!("{x} = {y}"),
            Greater => println!("{x} > {y}"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_integer_cmp_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.cmp(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.cmp(&y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.cmp(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.cmp(&y))),
        ],
    );
}
