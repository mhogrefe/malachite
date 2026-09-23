// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BalancedMod;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_balanced_mod);
    register_demo!(runner, demo_natural_polynomial_balanced_mod_ref);

    register_bench!(
        runner,
        benchmark_natural_polynomial_balanced_mod_evaluation_strategy
    );
}

fn demo_natural_polynomial_balanced_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).balanced_mod({m}) = {}", p.balanced_mod(&m));
    }
}

fn demo_natural_polynomial_balanced_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in natural_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).balanced_mod({m}) = {}", (&p).balanced_mod(&m));
    }
}

fn benchmark_natural_polynomial_balanced_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.balanced_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("NaturalPolynomial.balanced_mod(Natural)", &mut |(p, m)| {
                no_out!(p.balanced_mod(m));
            }),
            ("NaturalPolynomial.balanced_mod(&Natural)", &mut |(p, m)| {
                no_out!(p.balanced_mod(&m));
            }),
            (
                "(&NaturalPolynomial).balanced_mod(Natural)",
                &mut |(p, m)| {
                    no_out!((&p).balanced_mod(m));
                },
            ),
            (
                "(&NaturalPolynomial).balanced_mod(&Natural)",
                &mut |(p, m)| {
                    no_out!((&p).balanced_mod(&m));
                },
            ),
        ],
    );
}
