// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModPowTruncated, ModPowTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_pow_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_pow_truncated);
    register_demo!(runner, demo_natural_polynomial_mod_pow_truncated_ref);
    register_demo!(runner, demo_natural_polynomial_mod_pow_truncated_assign);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_pow_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_pow_truncated_algorithms
    );
}

fn demo_natural_polynomial_mod_pow_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, len, m) in natural_polynomial_unsigned_unsigned_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_pow_truncated({e}, {len}, {m}) = {}",
            p.mod_pow_truncated(e, len, &m)
        );
    }
}

fn demo_natural_polynomial_mod_pow_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, len, m) in natural_polynomial_unsigned_unsigned_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_pow_truncated({e}, {len}, {m}) = {}",
            (&p).mod_pow_truncated(e, len, &m)
        );
    }
}

fn demo_natural_polynomial_mod_pow_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, e, len, m) in natural_polynomial_unsigned_unsigned_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_pow_truncated_assign(e, len, &m);
        println!("p := {p_old}; p.mod_pow_truncated_assign({e}, {len}, {m}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_pow_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_pow_truncated(u64, u64, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_unsigned_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            (
                "NaturalPolynomial.mod_pow_truncated(u64, u64, &Natural)",
                &mut |(p, e, len, m)| no_out!(p.mod_pow_truncated(e, len, &m)),
            ),
            (
                "(&NaturalPolynomial).mod_pow_truncated(u64, u64, &Natural)",
                &mut |(p, e, len, m)| no_out!((&p).mod_pow_truncated(e, len, &m)),
            ),
            (
                "NaturalPolynomial.mod_pow_truncated_assign(u64, u64, &Natural)",
                &mut |(mut p, e, len, m)| p.mod_pow_truncated_assign(e, len, &m),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_pow_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_pow_truncated(u64, u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_unsigned_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, e, len, m)| {
                no_out!((&p).mod_pow_truncated(e, len, &m));
            }),
            ("naive", &mut |(p, e, len, m)| {
                no_out!(mod_pow_truncated_naive(&p, e, len, &m));
            }),
        ],
    );
}
