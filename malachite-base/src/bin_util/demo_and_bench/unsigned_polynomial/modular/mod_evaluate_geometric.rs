// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::modular::traits::ModPow;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModEvaluateGeometric};
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_geometric::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_geometric);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_geometric_algorithms
    );
}

fn demo_unsigned_polynomial_mod_evaluate_geometric(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, k, m) in unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate_geometric({q}, {k}, {m}) = {:?}",
            (&p).mod_evaluate_geometric(q, k, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_evaluate_geometric_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).mod_evaluate_geometric(u64, u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, q, k, m)| {
                no_out!((&p).mod_evaluate_geometric(q, k, m));
            }),
            ("one at a time", &mut |(p, q, k, m)| {
                no_out!(
                    (0..k)
                        .map(|j| (&p).mod_evaluate(q.mod_pow(j, m), m))
                        .collect::<Vec<_>>()
                );
            }),
            ("naive", &mut |(p, q, k, m)| {
                no_out!(mod_evaluate_geometric_naive(&p, q, k, m));
            }),
        ],
    );
}
