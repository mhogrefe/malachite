// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{MakeMonicMod, MakeMonicModAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_make_monic_mod);
    register_demo!(runner, demo_natural_polynomial_make_monic_mod_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_make_monic_mod_evaluation_strategy
    );
}

fn demo_natural_polynomial_make_monic_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).make_monic_mod({m}) = {:?}",
            (&p).make_monic_mod(&m).map(|q| q.to_string())
        );
    }
}

fn demo_natural_polynomial_make_monic_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, _, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let result = p.make_monic_mod_assign(&m);
        println!("p := {p_old}; p.make_monic_mod_assign({m}) = {result:?}; p = {p}");
    }
}

fn benchmark_natural_polynomial_make_monic_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.make_monic_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.make_monic_mod(&Natural)",
                &mut |(p, _, m)| {
                    let _ = p.make_monic_mod(&m);
                },
            ),
            (
                "(&NaturalPolynomial).make_monic_mod(&Natural)",
                &mut |(p, _, m)| {
                    let _ = (&p).make_monic_mod(&m);
                },
            ),
            (
                "NaturalPolynomial.make_monic_mod_assign(&Natural)",
                &mut |(mut p, _, m)| {
                    let _ = p.make_monic_mod_assign(&m);
                },
            ),
        ],
    );
}
