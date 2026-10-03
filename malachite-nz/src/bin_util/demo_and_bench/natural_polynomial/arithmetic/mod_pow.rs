// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::arithmetic::mod_pow::*;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_3;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_pow);
    register_demo!(runner, demo_natural_polynomial_mod_pow_ref);
    register_demo!(runner, demo_natural_polynomial_mod_pow_assign);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_pow_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_pow_algorithms);
}

fn demo_natural_polynomial_mod_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, m) in natural_polynomial_unsigned_natural_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_pow({e}, {m}) = {}", p.mod_pow(e, &m));
    }
}

fn demo_natural_polynomial_mod_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, m) in natural_polynomial_unsigned_natural_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_pow({e}, {m}) = {}", (&p).mod_pow(e, &m));
    }
}

fn demo_natural_polynomial_mod_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, e, m) in natural_polynomial_unsigned_natural_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_pow_assign(e, &m);
        println!("p := {p_old}; p.mod_pow_assign({e}, {m}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_pow(u64, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            (
                "NaturalPolynomial.mod_pow(u64, &Natural)",
                &mut |(p, e, m)| no_out!(p.mod_pow(e, &m)),
            ),
            (
                "(&NaturalPolynomial).mod_pow(u64, &Natural)",
                &mut |(p, e, m)| no_out!((&p).mod_pow(e, &m)),
            ),
            (
                "NaturalPolynomial.mod_pow_assign(u64, &Natural)",
                &mut |(mut p, e, m)| p.mod_pow_assign(e, &m),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_pow(u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e, m)| {
                no_out!((&p).mod_pow(e, &m));
            }),
            ("binary exponentiation", &mut |(p, e, m)| {
                let xs = p.coefficients_asc();
                if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && m > 1u32 {
                    no_out!(mod_pow_binexp(xs, e, &m));
                }
            }),
            ("integer power, then reduction", &mut |(p, e, m)| {
                let xs = p.coefficients_asc();
                if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && m > 1u32 {
                    no_out!(mod_pow_exact(xs, e, &m));
                }
            }),
            ("naive", &mut |(p, e, m)| {
                no_out!(mod_pow_naive(&p, e, &m));
            }),
        ],
    );
}
