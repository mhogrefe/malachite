// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::foer_sequences::FoerSequence;
use malachite_base::test_util::bench::bucketers::pair_sum_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_foer_sequence_from_vecs);
    register_demo!(runner, demo_foer_sequence_from_slices);

    register_bench!(
        runner,
        benchmark_foer_sequence_from_vecs_evaluation_strategy
    );
}

fn demo_foer_sequence_from_vecs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen::<u8>().get(gm, config).take(limit) {
        println!(
            "from_vecs({:?}, {:?}) = {}",
            xs.clone(),
            ys.clone(),
            FoerSequence::from_vecs(xs, ys)
        );
    }
}

fn demo_foer_sequence_from_slices(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen::<u8>().get(gm, config).take(limit) {
        println!(
            "from_slices(&{:?}, &{:?}) = {}",
            xs,
            ys,
            FoerSequence::from_slices(&xs, &ys)
        );
    }
}

fn benchmark_foer_sequence_from_vecs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "FoerSequence::from_vecs(Vec<T>, Vec<T>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_pair_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("from_vecs", &mut |(xs, ys)| {
                no_out!(FoerSequence::from_vecs(xs, ys));
            }),
            ("from_slices", &mut |(xs, ys)| {
                no_out!(FoerSequence::from_slices(&xs, &ys));
            }),
        ],
    );
}
