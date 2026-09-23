// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_try_from_integer_polynomial);
    register_demo!(
        runner,
        demo_natural_polynomial_try_from_integer_polynomial_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_try_from_integer_polynomial_evaluation_strategy
    );
}

fn demo_natural_polynomial_try_from_integer_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!(
            "NaturalPolynomial::try_from({}) = {:?}",
            p_old,
            NaturalPolynomial::try_from(p).map(|q| q.to_string())
        );
    }
}

fn demo_natural_polynomial_try_from_integer_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "NaturalPolynomial::try_from(&{}) = {:?}",
            p,
            NaturalPolynomial::try_from(&p).map(|q| q.to_string())
        );
    }
}

fn benchmark_natural_polynomial_try_from_integer_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::try_from(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("NaturalPolynomial::try_from(IntegerPolynomial)", &mut |p| {
                let _ = NaturalPolynomial::try_from(p);
            }),
            (
                "NaturalPolynomial::try_from(&IntegerPolynomial)",
                &mut |p| {
                    let _ = NaturalPolynomial::try_from(&p);
                },
            ),
        ],
    );
}
