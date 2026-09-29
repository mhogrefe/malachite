// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModNthDerivative, ModNthDerivativeAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_2;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_nth_derivative::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_nth_derivative);
    register_demo!(runner, demo_natural_polynomial_mod_nth_derivative_ref);
    register_demo!(runner, demo_natural_polynomial_mod_nth_derivative_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_nth_derivative_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_nth_derivative_algorithms
    );
}

fn demo_natural_polynomial_mod_nth_derivative(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n, m) in natural_polynomial_unsigned_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_nth_derivative({n}, {m}) = {}",
            p.mod_nth_derivative(n, &m)
        );
    }
}

fn demo_natural_polynomial_mod_nth_derivative_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, n, m) in natural_polynomial_unsigned_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_nth_derivative({n}, {m}) = {}",
            (&p).mod_nth_derivative(n, &m)
        );
    }
}

fn demo_natural_polynomial_mod_nth_derivative_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, n, m) in natural_polynomial_unsigned_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_nth_derivative_assign(n, &m);
        println!("p := {p_old}; p.mod_nth_derivative_assign({n}, {m}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_nth_derivative_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_nth_derivative(u64, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("p.mod_nth_derivative(n, &m)", &mut |(p, n, m)| {
                no_out!(p.mod_nth_derivative(n, &m));
            }),
            ("(&p).mod_nth_derivative(n, &m)", &mut |(p, n, m)| {
                no_out!((&p).mod_nth_derivative(n, &m));
            }),
            ("p.mod_nth_derivative_assign(n, &m)", &mut |(
                mut p,
                n,
                m,
            )| {
                p.mod_nth_derivative_assign(n, &m);
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_nth_derivative_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_nth_derivative(u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, n, m)| {
                no_out!((&p).mod_nth_derivative(n, &m));
            }),
            ("naive", &mut |(p, n, m)| {
                no_out!(mod_nth_derivative_naive(&p, n, &m));
            }),
        ],
    );
}
