// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::slices::slice_move_left;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_slice_move_left);
    register_bench!(runner, benchmark_slice_move_left);
}

fn demo_slice_move_left(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, amount) in unsigned_vec_unsigned_pair_gen_var_1::<u8>()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        slice_move_left(&mut xs, amount);
        println!("xs := {old_xs:?}; slice_move_left(&mut xs, {amount}); xs = {xs:?}");
    }
}

fn benchmark_slice_move_left(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "slice_move_left(&mut [T], usize)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_1::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, amount)| {
            slice_move_left(&mut xs, amount)
        })],
    );
}
