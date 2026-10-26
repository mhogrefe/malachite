// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModPermanent;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::*;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_3, natural_matrix_natural_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_permanent);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_permanent_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_matrix_mod_permanent_algorithms);
}

fn demo_natural_matrix_mod_permanent(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_matrix_natural_pair_gen_var_3()
        .get(gm, config)
        .filter(|(a, _)| a.height() <= 8)
        .take(limit)
    {
        println!("{a}.mod_permanent({m}) = {}", (&a).mod_permanent(&m));
    }
}

fn benchmark_natural_matrix_mod_permanent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_permanent(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_pair_gen_var_3()
            .get(gm, config)
            .filter(|(a, _)| a.height() <= 8),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.mod_permanent(Natural)", &mut |(a, m)| {
                no_out!(a.mod_permanent(m));
            }),
            ("NaturalMatrix.mod_permanent(&Natural)", &mut |(a, m)| {
                no_out!(a.mod_permanent(&m));
            }),
            ("(&NaturalMatrix).mod_permanent(Natural)", &mut |(a, m)| {
                no_out!((&a).mod_permanent(m));
            }),
            ("(&NaturalMatrix).mod_permanent(&Natural)", &mut |(a, m)| {
                no_out!((&a).mod_permanent(&m));
            }),
        ],
    );
}

fn benchmark_natural_matrix_mod_permanent_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_permanent(&Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_pair_gen_var_4()
            .get(gm, config)
            .filter(|(a, _)| a.is_square() && a.height() <= 8),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_permanent(&m));
            }),
            ("without splitting off the power of 2", &mut |(a, m)| {
                let n = usize::exact_from(a.height());
                no_out!(permanent_structured(
                    &ModRing::new(m),
                    a.elements_ref().to_vec(),
                    n
                ));
            }),
            ("without looking for structure", &mut |(a, m)| {
                let n = usize::exact_from(a.height());
                no_out!(permanent_dense(&ModRing::new(m), a.elements_ref(), n));
            }),
        ],
    );
}
