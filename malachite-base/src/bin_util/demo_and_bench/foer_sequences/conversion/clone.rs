// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    foer_sequence_len_bucketer, pair_foer_sequence_max_len_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_foer_sequence_gen, unsigned_foer_sequence_pair_gen,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_foer_sequence_clone);
    register_demo!(runner, demo_foer_sequence_clone_from);
    register_bench!(runner, benchmark_foer_sequence_clone);
    register_bench!(runner, benchmark_foer_sequence_clone_from);
}

fn demo_foer_sequence_clone(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_foer_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("clone({}) = {}", xs, xs.clone());
    }
}

fn demo_foer_sequence_clone_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_foer_sequence_pair_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        xs.clone_from(&ys);
        println!("xs := {xs_old}; xs.clone_from({ys}); xs = {xs}");
    }
}

#[allow(clippy::redundant_clone, unused_must_use)]
fn benchmark_foer_sequence_clone(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "FoerSequence.clone()",
        BenchmarkType::Single,
        unsigned_foer_sequence_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &foer_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.clone()))],
    );
}

fn benchmark_foer_sequence_clone_from(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "FoerSequence.clone_from(&FoerSequence)",
        BenchmarkType::Single,
        unsigned_foer_sequence_pair_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_foer_sequence_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, ys)| xs.clone_from(&ys))],
    );
}
