// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::slices::min_repeating_len;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen_var_4;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_min_repeating_len);
    register_bench!(runner, benchmark_min_repeating_len);
}

fn demo_min_repeating_len(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_4::<u8>().get(gm, config).take(limit) {
        println!("min_repeating_len({:?}) = {}", xs, min_repeating_len(&xs));
    }
}

fn benchmark_min_repeating_len(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "min_repeating_len(&[T])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_4::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(min_repeating_len(&xs)))],
    );
}
