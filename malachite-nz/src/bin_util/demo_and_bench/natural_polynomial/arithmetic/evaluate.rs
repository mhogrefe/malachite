// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{Evaluate, EvaluateMod, EvaluateModPowerOf2};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::evaluate::{
    evaluate_divide_and_conquer, evaluate_horner,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_1_natural_polynomial_bit_bucketer, triple_1_natural_polynomial_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_triple_gen_var_1, natural_polynomial_natural_pair_gen,
    natural_polynomial_natural_pair_gen_var_2,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::evaluate::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_evaluate);
    register_demo!(runner, demo_natural_polynomial_evaluate_ref);
    register_demo!(runner, demo_natural_polynomial_evaluate_long);
    register_demo!(runner, demo_natural_polynomial_evaluate_horner);
    register_demo!(runner, demo_natural_polynomial_evaluate_divide_and_conquer);
    register_demo!(
        runner,
        demo_natural_polynomial_evaluate_divide_and_conquer_long
    );

    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_evaluate_algorithms);
    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_algorithms_long
    );
    register_demo!(runner, demo_natural_polynomial_evaluate_mod_power_of_2);
    register_demo!(runner, demo_natural_polynomial_evaluate_mod_power_of_2_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_mod_power_of_2_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_mod_power_of_2_algorithms
    );
    register_demo!(runner, demo_natural_polynomial_evaluate_mod);
    register_demo!(runner, demo_natural_polynomial_evaluate_mod_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_mod_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_evaluate_mod_algorithms);
}

fn demo_natural_polynomial_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in natural_polynomial_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("(&({p})).evaluate({x_old}) = {}", (&p).evaluate(x));
    }
}

fn demo_natural_polynomial_evaluate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in natural_polynomial_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

// Polynomials of degree at least 50, which take the divide-and-conquer path when the value has more
// than one limb.
fn demo_natural_polynomial_evaluate_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in natural_polynomial_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).evaluate({x}) = {}", (&p).evaluate(&x));
    }
}

fn demo_natural_polynomial_evaluate_horner(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x) in natural_polynomial_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_horner({x}) = {}",
            evaluate_horner(p.coefficients_asc(), &x)
        );
    }
}

fn demo_natural_polynomial_evaluate_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x) in natural_polynomial_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_divide_and_conquer({x}) = {}",
            evaluate_divide_and_conquer(p.coefficients_asc(), &x)
        );
    }
}

fn demo_natural_polynomial_evaluate_divide_and_conquer_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x) in natural_polynomial_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_divide_and_conquer({x}) = {}",
            evaluate_divide_and_conquer(p.coefficients_asc(), &x)
        );
    }
}

fn benchmark_natural_polynomial_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("(&NaturalPolynomial).evaluate(Natural)", &mut |(p, x)| {
                no_out!((&p).evaluate(x));
            }),
            ("(&NaturalPolynomial).evaluate(&Natural)", &mut |(p, x)| {
                no_out!((&p).evaluate(&x));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate(&Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("Horner", &mut |(p, x)| {
                no_out!(evaluate_horner(p.coefficients_asc(), &x));
            }),
            ("divide and conquer", &mut |(p, x)| {
                no_out!(evaluate_divide_and_conquer(p.coefficients_asc(), &x));
            }),
            ("naive", &mut |(p, x)| no_out!(evaluate_naive(&p, &x))),
        ],
    );
}

fn benchmark_natural_polynomial_evaluate_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate(&Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x)| no_out!((&p).evaluate(&x))),
            ("Horner", &mut |(p, x)| {
                no_out!(evaluate_horner(p.coefficients_asc(), &x));
            }),
            ("divide and conquer", &mut |(p, x)| {
                no_out!(evaluate_divide_and_conquer(p.coefficients_asc(), &x));
            }),
            ("naive", &mut |(p, x)| no_out!(evaluate_naive(&p, &x))),
        ],
    );
}

fn demo_natural_polynomial_evaluate_mod_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "(&({p})).evaluate_mod_power_of_2({x_old}, {pow}) = {}",
            (&p).evaluate_mod_power_of_2(x, pow)
        );
    }
}

fn demo_natural_polynomial_evaluate_mod_power_of_2_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_mod_power_of_2({x}, {pow}) = {}",
            (&p).evaluate_mod_power_of_2(&x, pow)
        );
    }
}

fn benchmark_natural_polynomial_evaluate_mod_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate_mod_power_of_2(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "(&NaturalPolynomial).evaluate_mod_power_of_2(Natural, u64)",
                &mut |(p, x, pow)| no_out!((&p).evaluate_mod_power_of_2(x, pow)),
            ),
            (
                "(&NaturalPolynomial).evaluate_mod_power_of_2(&Natural, u64)",
                &mut |(p, x, pow)| no_out!((&p).evaluate_mod_power_of_2(&x, pow)),
            ),
            (
                "NaturalPolynomial.evaluate_mod_power_of_2(Natural, u64)",
                &mut |(p, x, pow)| no_out!(p.evaluate_mod_power_of_2(x, pow)),
            ),
            (
                "NaturalPolynomial.evaluate_mod_power_of_2(&Natural, u64)",
                &mut |(p, x, pow)| no_out!(p.evaluate_mod_power_of_2(&x, pow)),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_evaluate_mod_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate_mod_power_of_2(&Natural, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, pow)| {
                no_out!((&p).evaluate_mod_power_of_2(&x, pow));
            }),
            ("evaluate, then reduce", &mut |(p, x, pow)| {
                no_out!(evaluate_mod_power_of_2_naive(&p, &x, pow));
            }),
        ],
    );
}

fn demo_natural_polynomial_evaluate_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p_old, x_old, m_old) = (p.clone(), x.clone(), m.clone());
        println!(
            "({p_old}).evaluate_mod({x_old}, {m_old}) = {}",
            p.evaluate_mod(x, m)
        );
    }
}

fn demo_natural_polynomial_evaluate_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_mod({x}, {m}) = {}",
            (&p).evaluate_mod(&x, &m)
        );
    }
}

fn benchmark_natural_polynomial_evaluate_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.evaluate_mod(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.evaluate_mod(Natural, Natural)",
                &mut |(p, x, m)| no_out!(p.evaluate_mod(x, m)),
            ),
            (
                "NaturalPolynomial.evaluate_mod(&Natural, &Natural)",
                &mut |(p, x, m)| no_out!(p.evaluate_mod(&x, &m)),
            ),
            (
                "(&NaturalPolynomial).evaluate_mod(Natural, Natural)",
                &mut |(p, x, m)| no_out!((&p).evaluate_mod(x, m)),
            ),
            (
                "(&NaturalPolynomial).evaluate_mod(&Natural, &Natural)",
                &mut |(p, x, m)| no_out!((&p).evaluate_mod(&x, &m)),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_evaluate_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate_mod(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, m)| {
                no_out!((&p).evaluate_mod(&x, &m));
            }),
            ("evaluate, then reduce", &mut |(p, x, m)| {
                no_out!(evaluate_mod_naive(&p, &x, &m));
            }),
        ],
    );
}
