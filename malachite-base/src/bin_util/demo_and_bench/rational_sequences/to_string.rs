// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::rational_sequence_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_rational_sequence_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_to_string);
    register_demo!(runner, demo_rational_sequence_to_debug_string);

    register_bench!(runner, benchmark_rational_sequence_to_string);
    register_bench!(runner, benchmark_rational_sequence_to_debug_string);
}

fn demo_rational_sequence_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("{xs}");
    }
}

fn demo_rational_sequence_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("{xs:?}");
    }
}

fn benchmark_rational_sequence_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.to_string()",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.to_string()))],
    );
}

fn benchmark_rational_sequence_to_debug_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.to_debug_string()",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.to_debug_string()))],
    );
}
