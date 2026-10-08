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
use malachite_q::rational_vector::ShortlexRationalVectorRef;
use malachite_q::test_util::bench::bucketers::pair_rational_vector_max_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_pair_gen;
use malachite_q::test_util::rational_vector::comparison::shortlex_cmp::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_shortlex_rational_vector_cmp);

    register_bench!(runner, benchmark_shortlex_rational_vector_cmp_algorithms);
}

fn demo_shortlex_rational_vector_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in rational_vector_pair_gen().get(gm, config).take(limit) {
        println!(
            "{}.cmp(&{}) = {:?}",
            v,
            w,
            ShortlexRationalVectorRef(&v).cmp(&ShortlexRationalVectorRef(&w))
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_shortlex_rational_vector_cmp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ShortlexRationalVectorRef.cmp(&ShortlexRationalVectorRef)",
        BenchmarkType::Algorithms,
        rational_vector_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("default", &mut |(v, w)| {
                no_out!(ShortlexRationalVectorRef(&v).cmp(&ShortlexRationalVectorRef(&w)));
            }),
            ("walking the elements", &mut |(v, w)| {
                no_out!(rational_vector_shortlex_cmp_naive(&v, &w));
            }),
        ],
    );
}
