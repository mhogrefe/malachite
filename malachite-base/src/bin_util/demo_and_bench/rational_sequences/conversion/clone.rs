// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_rational_sequence_max_len_bucketer, rational_sequence_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_rational_sequence_gen, unsigned_rational_sequence_pair_gen,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_clone);
    register_demo!(runner, demo_rational_sequence_clone_from);
    register_bench!(runner, benchmark_rational_sequence_clone);
    register_bench!(runner, benchmark_rational_sequence_clone_from);
}

fn demo_rational_sequence_clone(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("clone({}) = {}", xs, xs.clone());
    }
}

fn demo_rational_sequence_clone_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_rational_sequence_pair_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        xs.clone_from(&ys);
        println!("xs := {xs_old}; xs.clone_from({ys}); xs = {xs}");
    }
}

#[allow(clippy::redundant_clone, unused_must_use)]
fn benchmark_rational_sequence_clone(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.clone()",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.clone()))],
    );
}

fn benchmark_rational_sequence_clone_from(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.clone_from(&RationalSequence)",
        BenchmarkType::Single,
        unsigned_rational_sequence_pair_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_sequence_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, ys)| xs.clone_from(&ys))],
    );
}
