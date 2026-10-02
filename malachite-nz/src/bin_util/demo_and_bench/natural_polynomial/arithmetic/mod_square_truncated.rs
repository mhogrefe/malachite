// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{ModSquareTruncated, ModSquareTruncatedAssign, Polynomial};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_square_truncated::{
    mod_square_truncated_full, mod_square_truncated_word,
};
use malachite_nz::platform::Limb;
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
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_square_truncated_word_algorithms
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

// The word kernels against the full truncated square, for moduli that fit in a limb. Both kernels
// compute no more coefficients than the whole square has.
fn benchmark_natural_polynomial_mod_square_truncated_word_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    let out_len = |p: &NaturalPolynomial, len: u64| {
        usize::try_from(len)
            .unwrap_or(usize::MAX)
            .min((p.coefficients_asc().len() << 1) - 1)
    };
    run_benchmark(
        "NaturalPolynomial.mod_square_truncated(u64, &Natural) with a word modulus",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>()
            .get(gm, config)
            .filter(|(p, len, m)| *len != 0 && p.len() > 1 && m.significant_bits() <= Limb::WIDTH),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, len, m)| {
                no_out!(p.mod_square_truncated(len, &m));
            }),
            ("full", &mut |(p, len, m)| {
                no_out!(mod_square_truncated_full(
                    p.coefficients_asc(),
                    out_len(&p, len),
                    &m
                ));
            }),
            ("word", &mut |(p, len, m)| {
                no_out!(mod_square_truncated_word(
                    p.coefficients_asc(),
                    out_len(&p, len),
                    &m
                ));
            }),
        ],
    );
}
