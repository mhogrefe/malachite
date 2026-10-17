// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_unsigned_matrix_max_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_pair_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::comparison::shortlex_cmp::*;
use malachite_base::unsigned_matrix::ShortlexUnsignedMatrixRef;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_shortlex_unsigned_matrix_cmp);

    register_bench!(runner, benchmark_shortlex_unsigned_matrix_cmp_algorithms);
}

fn demo_shortlex_unsigned_matrix_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, n) in unsigned_matrix_pair_gen().get(gm, config).take(limit) {
        println!(
            "{}.cmp(&{}) = {:?}",
            m,
            n,
            ShortlexUnsignedMatrixRef(&m).cmp(&ShortlexUnsignedMatrixRef(&n))
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_shortlex_unsigned_matrix_cmp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ShortlexUnsignedMatrixRef.cmp(&ShortlexUnsignedMatrixRef)",
        BenchmarkType::Algorithms,
        unsigned_matrix_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_unsigned_matrix_max_size_bucketer("m", "n"),
        &mut [
            ("default", &mut |(m, n)| {
                no_out!(ShortlexUnsignedMatrixRef(&m).cmp(&ShortlexUnsignedMatrixRef(&n)));
            }),
            ("walking the elements", &mut |(m, n)| {
                no_out!(unsigned_matrix_shortlex_cmp_naive(&m, &n));
            }),
        ],
    );
}
