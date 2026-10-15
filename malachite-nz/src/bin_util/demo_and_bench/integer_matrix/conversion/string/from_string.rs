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
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_from_str);
    register_demo!(runner, demo_integer_matrix_from_str_targeted);
    register_bench!(runner, benchmark_integer_matrix_from_str);
}

fn demo_integer_matrix_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "IntegerMatrix::from_str({:?}) = {:?}",
            s,
            IntegerMatrix::from_str(&s)
        );
    }
}

fn demo_integer_matrix_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        let s = m.to_string();
        println!(
            "IntegerMatrix::from_str({:?}) = {:?}",
            s,
            IntegerMatrix::from_str(&s)
        );
    }
}

fn benchmark_integer_matrix_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::from_str(&str)",
        BenchmarkType::Single,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            no_out!(IntegerMatrix::from_str(&m.to_string()).unwrap());
        })],
    );
}
