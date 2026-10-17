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
use malachite_nz::integer_matrix::ShortlexIntegerMatrixRef;
use malachite_nz::test_util::bench::bucketers::pair_integer_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_pair_gen;
use malachite_nz::test_util::integer_matrix::comparison::shortlex_cmp::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_shortlex_integer_matrix_cmp);

    register_bench!(runner, benchmark_shortlex_integer_matrix_cmp_algorithms);
}

fn demo_shortlex_integer_matrix_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, n) in integer_matrix_pair_gen().get(gm, config).take(limit) {
        println!(
            "{}.cmp(&{}) = {:?}",
            m,
            n,
            ShortlexIntegerMatrixRef(&m).cmp(&ShortlexIntegerMatrixRef(&n))
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_shortlex_integer_matrix_cmp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ShortlexIntegerMatrixRef.cmp(&ShortlexIntegerMatrixRef)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("m", "n"),
        &mut [
            ("default", &mut |(m, n)| {
                no_out!(ShortlexIntegerMatrixRef(&m).cmp(&ShortlexIntegerMatrixRef(&n)));
            }),
            ("walking the elements", &mut |(m, n)| {
                no_out!(integer_matrix_shortlex_cmp_naive(&m, &n));
            }),
        ],
    );
}
