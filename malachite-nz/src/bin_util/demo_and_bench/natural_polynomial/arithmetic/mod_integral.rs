// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModIntegral, ModIntegralAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_integral::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_integral);
    register_demo!(runner, demo_natural_polynomial_mod_integral_ref);
    register_demo!(runner, demo_natural_polynomial_mod_integral_assign);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_integral_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_integral_algorithms);
}

fn demo_natural_polynomial_mod_integral(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .filter(|(p, _, m)| mod_integral_is_defined(p, m))
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_integral({m}) = {}", p.mod_integral(&m));
    }
}

fn demo_natural_polynomial_mod_integral_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, _, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .filter(|(p, _, m)| mod_integral_is_defined(p, m))
        .take(limit)
    {
        println!("(&({p})).mod_integral({m}) = {}", (&p).mod_integral(&m));
    }
}

fn demo_natural_polynomial_mod_integral_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, _, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .filter(|(p, _, m)| mod_integral_is_defined(p, m))
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_integral_assign(&m);
        println!("p := {p_old}; p.mod_integral_assign({m}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_integral_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_integral(&Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_natural_triple_gen_var_1()
            .get(gm, config)
            .filter(|(p, _, m)| mod_integral_is_defined(p, m)),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("p.mod_integral(&m)", &mut |(p, _, m)| {
                no_out!(p.mod_integral(&m));
            }),
            ("(&p).mod_integral(&m)", &mut |(p, _, m)| {
                no_out!((&p).mod_integral(&m));
            }),
            ("p.mod_integral_assign(&m)", &mut |(mut p, _, m)| {
                p.mod_integral_assign(&m);
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_integral_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_integral(&Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_natural_triple_gen_var_1()
            .get(gm, config)
            .filter(|(p, _, m)| mod_integral_is_defined(p, m)),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, _, m)| no_out!((&p).mod_integral(&m))),
            ("naive", &mut |(p, _, m)| {
                no_out!(mod_integral_naive(&p, &m));
            }),
        ],
    );
}
