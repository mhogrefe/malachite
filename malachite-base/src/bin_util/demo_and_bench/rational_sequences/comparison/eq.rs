// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_rational_sequence_max_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_rational_sequence_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_eq);
    register_bench!(runner, benchmark_rational_sequence_eq);
}

fn demo_rational_sequence_eq(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_rational_sequence_pair_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        if xs == ys {
            println!("{xs} = {ys}");
        } else {
            println!("{xs} ≠ {ys}");
        }
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_sequence_eq(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "RationalSequence == RationalSequence",
        BenchmarkType::Single,
        unsigned_rational_sequence_pair_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_sequence_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(xs == ys))],
    );
}
