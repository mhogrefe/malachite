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
use malachite_q::test_util::bench::bucketers::triple_3_pair_rational_max_bit_bucketer;
use malachite_q::test_util::generators::{rational_pair_gen, rational_pair_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_eq);
    register_bench!(runner, benchmark_rational_eq_library_comparison);
}

fn demo_rational_eq(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        if x == y {
            println!("{x} = {y}");
        } else {
            println!("{x} ≠ {y}");
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_eq_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational == Rational",
        BenchmarkType::LibraryComparison,
        rational_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x == y)),
            ("num", &mut |((x, y), _, _)| no_out!(x == y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x == y)),
        ],
    );
}
