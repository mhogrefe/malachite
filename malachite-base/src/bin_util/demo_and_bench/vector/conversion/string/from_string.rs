// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, unsigned_vector_gen};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_from_str);
    register_demo!(runner, demo_unsigned_vector_from_str_targeted);
    register_bench!(runner, benchmark_unsigned_vector_from_str);
}

fn demo_unsigned_vector_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "Vector::<u64>::from_str({:?}) = {:?}",
            s,
            Vector::<u64>::from_str(&s)
        );
    }
}

fn demo_unsigned_vector_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let s = v.to_string();
        println!(
            "Vector::<u64>::from_str({:?}) = {:?}",
            s,
            Vector::<u64>::from_str(&s)
        );
    }
}

fn benchmark_unsigned_vector_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Vector::<u64>::from_str(&str)",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| {
            no_out!(Vector::<u64>::from_str(&v.to_string()).unwrap());
        })],
    );
}
