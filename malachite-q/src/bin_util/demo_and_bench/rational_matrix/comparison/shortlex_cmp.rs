// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_matrix::ShortlexRationalMatrixRef;
use malachite_q::test_util::bench::bucketers::pair_rational_matrix_max_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_pair_gen;
use malachite_q::test_util::rational_matrix::comparison::shortlex_cmp::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_shortlex_rational_matrix_cmp);

    register_bench!(runner, benchmark_shortlex_rational_matrix_cmp_algorithms);
}

fn demo_shortlex_rational_matrix_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, n) in rational_matrix_pair_gen().get(gm, config).take(limit) {
        println!(
            "{}.cmp(&{}) = {:?}",
            m,
            n,
            ShortlexRationalMatrixRef(&m).cmp(&ShortlexRationalMatrixRef(&n))
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_shortlex_rational_matrix_cmp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ShortlexRationalMatrixRef.cmp(&ShortlexRationalMatrixRef)",
        BenchmarkType::Algorithms,
        rational_matrix_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_matrix_max_bit_bucketer("m", "n"),
        &mut [
            ("default", &mut |(m, n)| {
                no_out!(ShortlexRationalMatrixRef(&m).cmp(&ShortlexRationalMatrixRef(&n)));
            }),
            ("walking the elements", &mut |(m, n)| {
                no_out!(rational_matrix_shortlex_cmp_naive(&m, &n));
            }),
        ],
    );
}
