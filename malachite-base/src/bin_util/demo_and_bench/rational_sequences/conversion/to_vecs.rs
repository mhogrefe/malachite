// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::rational_sequence_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_rational_sequence_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_to_vecs);
    register_demo!(runner, demo_rational_sequence_into_vecs);
    register_demo!(runner, demo_rational_sequence_slices_ref);

    register_bench!(
        runner,
        benchmark_rational_sequence_to_vecs_evaluation_strategy
    );
}

fn demo_rational_sequence_to_vecs(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("to_vecs(&{}) = {:?}", xs, xs.to_vecs());
    }
}

fn demo_rational_sequence_into_vecs(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("into_vecs({}) = {:?}", xs.clone(), xs.into_vecs());
    }
}

fn demo_rational_sequence_slices_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("slices_ref(&{}) = {:?}", xs, xs.slices_ref());
    }
}

fn benchmark_rational_sequence_to_vecs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.to_vecs()",
        BenchmarkType::EvaluationStrategy,
        unsigned_rational_sequence_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [
            ("to_vecs", &mut |xs| no_out!(xs.to_vecs())),
            ("into_vecs", &mut |xs| no_out!(xs.into_vecs())),
            ("slices_ref", &mut |xs| no_out!(xs.slices_ref())),
        ],
    );
}
