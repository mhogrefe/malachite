// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::arithmetic::traits::EvaluateMany;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_pair_gen;
use malachite_nz::test_util::natural_polynomial::arithmetic::evaluate_many::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_evaluate_many);
    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_many_algorithms
    );
}

fn demo_natural_polynomial_evaluate_many(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs) in natural_polynomial_natural_vec_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_many(&{xs:?}) = {:?}",
            (&p).evaluate_many(&xs)
        );
    }
}

fn benchmark_natural_polynomial_evaluate_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate_many(&[Natural])",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs)| no_out!((&p).evaluate_many(&xs))),
            ("naive", &mut |(p, xs)| {
                no_out!(evaluate_many_naive(&p, &xs));
            }),
        ],
    );
}
