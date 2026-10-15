// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, unsigned_matrix_gen};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_from_str);
    register_demo!(runner, demo_unsigned_matrix_from_str_targeted);
    register_bench!(runner, benchmark_unsigned_matrix_from_str);
}

fn demo_unsigned_matrix_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedMatrix::<u64>::from_str({:?}) = {:?}",
            s,
            UnsignedMatrix::<u64>::from_str(&s)
        );
    }
}

fn demo_unsigned_matrix_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in unsigned_matrix_gen().get(gm, config).take(limit) {
        let s = m.to_string();
        println!(
            "UnsignedMatrix::<u64>::from_str({:?}) = {:?}",
            s,
            UnsignedMatrix::<u64>::from_str(&s)
        );
    }
}

fn benchmark_unsigned_matrix_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix::<u64>::from_str(&str)",
        BenchmarkType::Single,
        unsigned_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_matrix_size_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            no_out!(UnsignedMatrix::<u64>::from_str(&m.to_string()).unwrap());
        })],
    );
}
