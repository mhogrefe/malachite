// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::crement::char_to_contiguous_range;
use malachite_base::test_util::bench::bucketers::char_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::char_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_char_to_contiguous_range);
    register_bench!(runner, benchmark_char_to_contiguous_range);
}

fn demo_char_to_contiguous_range(gm: GenMode, config: &GenConfig, limit: usize) {
    for c in char_gen().get(gm, config).take(limit) {
        println!(
            "char_to_contiguous_range({:?}) = {}",
            c,
            char_to_contiguous_range(c)
        );
    }
}

fn benchmark_char_to_contiguous_range(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "char_to_contiguous_range(char)",
        BenchmarkType::Single,
        char_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &char_bucketer(),
        &mut [("Malachite", &mut |c| no_out!(char_to_contiguous_range(c)))],
    );
}
