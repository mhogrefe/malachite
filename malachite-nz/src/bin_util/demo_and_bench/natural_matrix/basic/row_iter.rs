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
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_row_iter);
    register_demo!(runner, demo_natural_matrix_row_iter_mut);
    register_bench!(runner, benchmark_natural_matrix_row_iter);
    register_bench!(runner, benchmark_natural_matrix_row_iter_mut);
}

fn demo_natural_matrix_row_iter(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, i) in natural_matrix_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{m}.row_iter({i}) = {}",
            prefix_to_string(m.row_iter(i), 20)
        );
    }
}

fn demo_natural_matrix_row_iter_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, i) in natural_matrix_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let old = m.clone();
        for x in m.row_iter_mut(i) {
            *x += Natural::ONE;
        }
        println!("m := {old}; for x in m.row_iter_mut({i}) {{ *x += 1; }}; m = {m}");
    }
}

fn benchmark_natural_matrix_row_iter(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.row_iter(usize)",
        BenchmarkType::Single,
        natural_matrix_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(m, i)| {
            for x in m.row_iter(i) {
                no_out!(x);
            }
        })],
    );
}

fn benchmark_natural_matrix_row_iter_mut(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.row_iter_mut(usize)",
        BenchmarkType::Single,
        natural_matrix_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, i)| {
            for x in m.row_iter_mut(i) {
                *x += Natural::ONE;
            }
        })],
    );
}
