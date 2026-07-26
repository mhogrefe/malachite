// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::foer_sequences::FoerSequence;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_foer_sequence_from_vec);
    register_demo!(runner, demo_foer_sequence_from_slice);

    register_bench!(runner, benchmark_foer_sequence_from_vec_evaluation_strategy);
}

fn demo_foer_sequence_from_vec(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        println!(
            "from_vec({:?}) = {}",
            xs.clone(),
            FoerSequence::from_vec(xs)
        );
    }
}

fn demo_foer_sequence_from_slice(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        println!("from_slice(&{:?}) = {}", xs, FoerSequence::from_slice(&xs));
    }
}

fn benchmark_foer_sequence_from_vec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "FoerSequence::from_vec(Vec<T>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("from_vec", &mut |xs| {
                no_out!(FoerSequence::from_vec(xs));
            }),
            ("from_slice", &mut |xs| {
                no_out!(FoerSequence::from_slice(&xs));
            }),
        ],
    );
}
