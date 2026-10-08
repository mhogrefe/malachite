// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::string_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::bench::bucketers::integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_gen;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_from_str);
    register_demo!(runner, demo_integer_vector_from_str_targeted);
    register_bench!(runner, benchmark_integer_vector_from_str);
}

fn demo_integer_vector_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "IntegerVector::from_str({:?}) = {:?}",
            s,
            IntegerVector::from_str(&s)
        );
    }
}

fn demo_integer_vector_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        let s = v.to_string();
        println!(
            "IntegerVector::from_str({:?}) = {:?}",
            s,
            IntegerVector::from_str(&s)
        );
    }
}

fn benchmark_integer_vector_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::from_str(&str)",
        BenchmarkType::Single,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |v| {
            no_out!(IntegerVector::from_str(&v.to_string()).unwrap());
        })],
    );
}
