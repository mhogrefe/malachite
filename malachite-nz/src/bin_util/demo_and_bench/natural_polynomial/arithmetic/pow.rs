// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::pow::binexp::pow_to_out_binexp;
use malachite_nz::integer_polynomial::arithmetic::pow::binomial::pow_to_out_binomial;
use malachite_nz::integer_polynomial::arithmetic::pow::multinomial::*;
use malachite_nz::integer_polynomial::arithmetic::pow::{
    pow_ref_with_kernel, pow_to_out, pow_to_out_addchains_e,
};
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_pair_gen_var_5, natural_polynomial_unsigned_pair_gen_var_6,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::pow::pow_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_pow);
    register_demo!(runner, demo_natural_polynomial_pow_ref);
    register_demo!(runner, demo_natural_polynomial_pow_assign);

    register_bench!(runner, benchmark_natural_polynomial_pow_evaluation_strategy);
    register_bench!(runner, benchmark_natural_polynomial_pow_algorithms);
    register_bench!(runner, benchmark_natural_polynomial_pow_binomial_algorithms);
    register_bench!(
        runner,
        benchmark_natural_polynomial_pow_multinomial_algorithms
    );
}

fn demo_natural_polynomial_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e) in natural_polynomial_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).pow({e}) = {}", p.pow(e));
    }
}

fn demo_natural_polynomial_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e) in natural_polynomial_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).pow({e}) = {}", (&p).pow(e));
    }
}

fn demo_natural_polynomial_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, e) in natural_polynomial_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.pow_assign(e);
        println!("p := {p_old}; p.pow_assign({e}); p = {p}");
    }
}

fn benchmark_natural_polynomial_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("exp"),
        &mut [
            ("NaturalPolynomial.pow(u64)", &mut |(p, e)| {
                no_out!(p.pow(e));
            }),
            ("(&NaturalPolynomial).pow(u64)", &mut |(p, e)| {
                no_out!((&p).pow(e));
            }),
            ("NaturalPolynomial.pow_assign(u64)", &mut |(mut p, e)| {
                p.pow_assign(e);
            }),
        ],
    );
}

fn benchmark_natural_polynomial_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).pow(u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(p.coefficients_asc(), e, pow_to_out));
            }),
            ("addition chains", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_addchains_e
                ));
            }),
            ("binary exponentiation", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_binexp
                ));
            }),
            ("multinomial", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_multinomial
                ));
            }),
            ("naive", &mut |(p, e)| {
                no_out!(pow_naive(&p, e));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_pow_binomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).pow(u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(p.coefficients_asc(), e, pow_to_out));
            }),
            ("addition chains", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_addchains_e
                ));
            }),
            ("binary exponentiation", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_binexp
                ));
            }),
            ("binomial", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_binomial
                ));
            }),
            ("multinomial", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_multinomial
                ));
            }),
            ("naive", &mut |(p, e)| {
                no_out!(pow_naive(&p, e));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_pow_multinomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).pow(u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("exp"),
        &mut [
            ("multinomial", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_multinomial
                ));
            }),
            ("multinomial, precomputed multiples", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_multinomial_multiples
                ));
            }),
            ("multinomial, split by sign", &mut |(p, e)| {
                no_out!(pow_ref_with_kernel(
                    p.coefficients_asc(),
                    e,
                    pow_to_out_multinomial_split
                ));
            }),
        ],
    );
}
