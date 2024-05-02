// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::slices::slice_set_zero;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_slice_set_zero);
    register_bench!(runner, benchmark_slice_set_zero);
}

fn demo_slice_set_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        let old_xs = xs.clone();
        slice_set_zero(&mut xs);
        println!("xs := {old_xs:?}; slice_set_zero(&mut xs); xs = {xs:?}");
    }
}

fn benchmark_slice_set_zero(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "slice_set_zero(&mut [T])",
        BenchmarkType::Single,
        unsigned_vec_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut xs| slice_set_zero(&mut xs))],
    );
}
