// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_1_rational_sequence_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_rational_sequence_unsigned_pair_gen_var_1,
    unsigned_rational_sequence_unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_get);
    register_demo!(runner, demo_rational_sequence_index);

    register_bench!(runner, benchmark_rational_sequence_get);
    register_bench!(runner, benchmark_rational_sequence_index);
}

fn demo_rational_sequence_get(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, index) in unsigned_rational_sequence_unsigned_pair_gen_var_1::<u8, usize>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.get({}) = {:?}", xs, index, xs.get(index));
    }
}

fn demo_rational_sequence_index(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, index) in unsigned_rational_sequence_unsigned_pair_gen_var_2::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}[{}] = {}", xs, index, xs[index]);
    }
}

fn benchmark_rational_sequence_get(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "RationalSequence.get(usize)",
        BenchmarkType::Single,
        unsigned_rational_sequence_unsigned_pair_gen_var_1::<u8, usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, index)| no_out!(xs.get(index)))],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_rational_sequence_index(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence[usize]",
        BenchmarkType::Single,
        unsigned_rational_sequence_unsigned_pair_gen_var_2::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, index)| no_out!(xs[index]))],
    );
}
