// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::arithmetic::max_limbs::vec_max_limbs;
use malachite_nz::test_util::generators::integer_vec_gen;
use malachite_nz::test_util::integer_vector::arithmetic::max_limbs::vec_max_limbs_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_vec_max_limbs);
    register_bench!(runner, benchmark_vec_max_limbs_algorithms);
}

fn demo_vec_max_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen().get(gm, config).take(limit) {
        println!("vec_max_limbs({:?}) = {:?}", xs, vec_max_limbs(&xs));
    }
}

fn benchmark_vec_max_limbs_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "vec_max_limbs(&[Integer])",
        BenchmarkType::Algorithms,
        integer_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |xs| {
                no_out!(vec_max_limbs(&xs));
            }),
            ("naive", &mut |xs| {
                no_out!(vec_max_limbs_naive(&xs));
            }),
        ],
    );
}
