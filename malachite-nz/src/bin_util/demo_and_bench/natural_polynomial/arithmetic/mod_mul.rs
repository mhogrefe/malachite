// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::arithmetic::mod_mul::{mod_mul_full, mod_mul_word};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_mul);
    register_demo!(runner, demo_natural_polynomial_mod_mul_val_ref);
    register_demo!(runner, demo_natural_polynomial_mod_mul_ref_val);
    register_demo!(runner, demo_natural_polynomial_mod_mul_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_mul_assign);
    register_demo!(runner, demo_natural_polynomial_mod_mul_assign_ref);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_mul_algorithms);
    register_bench!(runner, benchmark_natural_polynomial_mod_mul_word_algorithms);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_mul_assign_evaluation_strategy
    );
}

fn demo_natural_polynomial_mod_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).mod_mul({q_old}, {m}) = {}", p.mod_mul(q, &m));
    }
}

fn demo_natural_polynomial_mod_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_mul(&({q}), {m}) = {}", p.mod_mul(&q, &m));
    }
}

fn demo_natural_polynomial_mod_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!("(&({p})).mod_mul({q_old}, {m}) = {}", (&p).mod_mul(q, &m));
    }
}

fn demo_natural_polynomial_mod_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_mul(&({q}), {m}) = {}", (&p).mod_mul(&q, &m));
    }
}

fn demo_natural_polynomial_mod_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_mul_assign(q, &m);
        println!("p := {p_old}; p.mod_mul_assign({q_old}, {m}); p = {p}");
    }
}

fn demo_natural_polynomial_mod_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_mul_assign(&q, &m);
        println!("p := {p_old}; p.mod_mul_assign(&({q}), {m}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_mul(NaturalPolynomial, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mod_mul(NaturalPolynomial, &Natural)",
                &mut |(p, q, m)| {
                    no_out!(p.mod_mul(q, &m));
                },
            ),
            (
                "NaturalPolynomial.mod_mul(&NaturalPolynomial, &Natural)",
                &mut |(p, q, m)| {
                    no_out!(p.mod_mul(&q, &m));
                },
            ),
            (
                "(&NaturalPolynomial).mod_mul(NaturalPolynomial, &Natural)",
                &mut |(p, q, m)| {
                    no_out!((&p).mod_mul(q, &m));
                },
            ),
            (
                "(&NaturalPolynomial).mod_mul(&NaturalPolynomial, &Natural)",
                &mut |(p, q, m)| {
                    no_out!((&p).mod_mul(&q, &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_mul(NaturalPolynomial, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!(p.mod_mul(q, &m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_mul_naive(&p, &q, &m));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_mul_assign(NaturalPolynomial, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "NaturalPolynomial.mod_mul_assign(NaturalPolynomial, &Natural)",
                &mut |(mut p, q, m)| p.mod_mul_assign(q, &m),
            ),
            (
                "NaturalPolynomial.mod_mul_assign(&NaturalPolynomial, &Natural)",
                &mut |(mut p, q, m)| p.mod_mul_assign(&q, &m),
            ),
        ],
    );
}

// The word kernels against the full product, for moduli that fit in a limb.
fn benchmark_natural_polynomial_mod_mul_word_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_mul(NaturalPolynomial, &Natural) with a word modulus",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_natural_triple_gen_var_1()
            .get(gm, config)
            .filter(|(p, q, m)| p.len() > 1 && q.len() > 1 && m.significant_bits() <= Limb::WIDTH),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!(p.mod_mul(q, &m));
            }),
            ("full", &mut |(p, q, m)| {
                no_out!(mod_mul_full(p.coefficients_asc(), q.coefficients_asc(), &m));
            }),
            ("word", &mut |(p, q, m)| {
                no_out!(mod_mul_word(p.coefficients_asc(), q.coefficients_asc(), &m));
            }),
        ],
    );
}
