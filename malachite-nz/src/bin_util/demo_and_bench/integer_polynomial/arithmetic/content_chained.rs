// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::content_chained::integers_content_chained;
use malachite_nz::test_util::generators::integer_vec_natural_pair_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::content_chained::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integers_content_chained);

    register_bench!(runner, benchmark_integers_content_chained_algorithms);
}

fn demo_integers_content_chained(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, x) in integer_vec_natural_pair_gen().get(gm, config).take(limit) {
        println!(
            "integers_content_chained({xs:?}, {x}) = {}",
            integers_content_chained(&xs, &x)
        );
    }
}

fn benchmark_integers_content_chained_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "integers_content_chained(&[Integer], &Natural)",
        BenchmarkType::Algorithms,
        integer_vec_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, x)| {
                no_out!(integers_content_chained(&xs, &x));
            }),
            ("naive", &mut |(xs, x)| {
                no_out!(integers_content_chained_naive(&xs, &x));
            }),
        ],
    );
}
