// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::prefix_to_string;
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_columns);
    register_demo!(runner, demo_integer_matrix_columns_mut);
    register_bench!(runner, benchmark_integer_matrix_columns);
    register_bench!(runner, benchmark_integer_matrix_columns_mut);
}

fn demo_integer_matrix_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        println!(
            "{m}.columns() = {}",
            prefix_to_string(m.columns().map(|xs| prefix_to_string(xs, 20)), 20)
        );
    }
}

fn demo_integer_matrix_columns_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut m in integer_matrix_gen().get(gm, config).take(limit) {
        let old = m.clone();
        for xs in m.columns_mut() {
            for x in xs {
                *x += Integer::ONE;
            }
        }
        println!("m := {old}; for xs in m.columns_mut() {{ for x in xs {{ *x += 1; }} }}; m = {m}");
    }
}

fn benchmark_integer_matrix_columns(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.columns()",
        BenchmarkType::Single,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            for xs in m.columns() {
                for x in xs {
                    no_out!(x);
                }
            }
        })],
    );
}

fn benchmark_integer_matrix_columns_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.columns_mut()",
        BenchmarkType::Single,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |mut m| {
            for xs in m.columns_mut() {
                for x in xs {
                    *x += Integer::ONE;
                }
            }
        })],
    );
}
