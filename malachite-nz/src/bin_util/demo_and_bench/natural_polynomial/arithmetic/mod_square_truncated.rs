// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModSquareTruncated, ModSquareTruncatedAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_square_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_square_truncated);
    register_demo!(runner, demo_natural_polynomial_mod_square_truncated_ref);
    register_demo!(runner, demo_natural_polynomial_mod_square_truncated_assign);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_square_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_square_truncated_algorithms
    );
}

fn demo_natural_polynomial_mod_square_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_square_truncated({len}, {m}) = {}",
            p.mod_square_truncated(len, &m)
        );
    }
}

fn demo_natural_polynomial_mod_square_truncated_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_square_truncated({len}, {m}) = {}",
            (&p).mod_square_truncated(len, &m)
        );
    }
}

fn demo_natural_polynomial_mod_square_truncated_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, len, m) in natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_square_truncated_assign(len, &m);
        println!("p := {p_old}; p.mod_square_truncated_assign({len}, {m}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_square_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_square_truncated(u64, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_square_truncated(u64, &Natural)",
                &mut |(p, len, m)| {
                    no_out!(p.mod_square_truncated(len, &m));
                },
            ),
            (
                "(&NaturalPolynomial).mod_square_truncated(u64, &Natural)",
                &mut |(p, len, m)| {
                    no_out!((&p).mod_square_truncated(len, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_square_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_square_truncated(u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len, m)| {
                no_out!(p.mod_square_truncated(len, &m));
            }),
            ("naive", &mut |(p, len, m)| {
                no_out!(mod_square_truncated_naive(&p, len, &m));
            }),
        ],
    );
}
