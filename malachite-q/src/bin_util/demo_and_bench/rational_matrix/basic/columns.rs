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
use malachite_q::Rational;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_columns);
    register_demo!(runner, demo_rational_matrix_columns_mut);
    register_bench!(runner, benchmark_rational_matrix_columns);
    register_bench!(runner, benchmark_rational_matrix_columns_mut);
}

fn demo_rational_matrix_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in rational_matrix_gen().get(gm, config).take(limit) {
        println!(
            "{m}.columns() = {}",
            prefix_to_string(m.columns().map(|xs| prefix_to_string(xs, 20)), 20)
        );
    }
}

fn demo_rational_matrix_columns_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut m in rational_matrix_gen().get(gm, config).take(limit) {
        let old = m.clone();
        for xs in m.columns_mut() {
            for x in xs {
                *x += Rational::ONE;
            }
        }
        println!("m := {old}; for xs in m.columns_mut() {{ for x in xs {{ *x += 1; }} }}; m = {m}");
    }
}

fn benchmark_rational_matrix_columns(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.columns()",
        BenchmarkType::Single,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |m| {
            for xs in m.columns() {
                for x in xs {
                    no_out!(x);
                }
            }
        })],
    );
}

fn benchmark_rational_matrix_columns_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.columns_mut()",
        BenchmarkType::Single,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |mut m| {
            for xs in m.columns_mut() {
                for x in xs {
                    *x += Rational::ONE;
                }
            }
        })],
    );
}
