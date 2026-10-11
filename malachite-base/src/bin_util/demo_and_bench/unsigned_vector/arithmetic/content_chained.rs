// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ContentChained;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_content_chained);
    register_demo!(runner, demo_unsigned_vector_content_chained_ref);
    register_bench!(
        runner,
        benchmark_unsigned_vector_content_chained_evaluation_strategy
    );
}

fn demo_unsigned_vector_content_chained(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, x) in unsigned_vec_unsigned_pair_gen::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let v = UnsignedVector::from_owned_elements(xs);
        let v_old = v.clone();
        println!("{v_old}.content_chained({x}) = {}", v.content_chained(x));
    }
}

fn demo_unsigned_vector_content_chained_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, x) in unsigned_vec_unsigned_pair_gen::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let v = UnsignedVector::from_owned_elements(xs);
        println!("(&{v}).content_chained({x}) = {}", (&v).content_chained(x));
    }
}

fn benchmark_unsigned_vector_content_chained_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.content_chained(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_pair_gen::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            (
                "UnsignedVector<u64>.content_chained(u64)",
                &mut |(xs, x)| {
                    no_out!(UnsignedVector::from_owned_elements(xs).content_chained(x));
                },
            ),
            (
                "(&UnsignedVector<u64>).content_chained(u64)",
                &mut |(xs, x)| {
                    no_out!((&UnsignedVector::from_owned_elements(xs)).content_chained(x));
                },
            ),
        ],
    );
}
