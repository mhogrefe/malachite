// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::slices::slice_trailing_zeros;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_slice_trailing_zeros);
    register_bench!(runner, benchmark_slice_trailing_zeros);
}

fn demo_slice_trailing_zeros(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        println!(
            "slice_trailing_zeros({:?}) = {}",
            xs,
            slice_trailing_zeros(&xs)
        );
    }
}

fn benchmark_slice_trailing_zeros(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "slice_trailing_zeros(&[T])",
        BenchmarkType::Single,
        unsigned_vec_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(slice_trailing_zeros(&xs)))],
    );
}
