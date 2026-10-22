// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModLuDecomposition;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_lu_decomposition::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_lu_decomposition);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_lu_decomposition_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_lu_decomposition_algorithms
    );
}

fn demo_natural_matrix_mod_lu_decomposition(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let (p, l, u) = (&a).mod_lu_decomposition(&m);
        println!("{a}.mod_lu_decomposition({m}) = ({p:?}, {l}, {u})");
    }
}

fn benchmark_natural_matrix_mod_lu_decomposition_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_lu_decomposition(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_lu_decomposition(Natural)",
                &mut |(a, m)| {
                    no_out!(a.mod_lu_decomposition(m));
                },
            ),
            (
                "NaturalMatrix.mod_lu_decomposition(&Natural)",
                &mut |(a, m)| {
                    no_out!(a.mod_lu_decomposition(&m));
                },
            ),
            (
                "(&NaturalMatrix).mod_lu_decomposition(Natural)",
                &mut |(a, m)| {
                    no_out!((&a).mod_lu_decomposition(m));
                },
            ),
            (
                "(&NaturalMatrix).mod_lu_decomposition(&Natural)",
                &mut |(a, m)| {
                    no_out!((&a).mod_lu_decomposition(&m));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_lu_decomposition_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_lu_decomposition(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_lu_decomposition(&m));
            }),
            ("classical", &mut |(a, m)| {
                no_out!(natural_matrix_mod_lu_packed_classical(&a, &m, false));
            }),
            ("recursive", &mut |(a, m)| {
                no_out!(natural_matrix_mod_lu_packed_recursive(&a, &m, false));
            }),
        ],
    );
}
