// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::hash::hash;
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_gen, integer_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_hash);
    register_bench!(runner, benchmark_integer_hash_library_comparison);
}

fn demo_integer_hash(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

fn benchmark_integer_hash_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer hash",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(hash(&n))),
            ("num", &mut |(_, n, _)| no_out!(hash(&n))),
            ("rug", &mut |(n, _, _)| no_out!(hash(&n))),
        ],
    );
}
