// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{
    Evaluate, EvaluateMany, ModEvaluate, ModEvaluateMany, ModPowerOf2Evaluate,
};
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
    natural_polynomial_natural_vec_natural_triple_gen_var_1,
    natural_polynomial_natural_vec_pair_gen,
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
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_evaluate);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_evaluate_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_evaluate_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_evaluate_algorithms
    );
    register_demo!(runner, demo_natural_polynomial_mod_evaluate);
    register_demo!(runner, demo_natural_polynomial_mod_evaluate_ref);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_evaluate_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_polynomial_mod_evaluate_algorithms);
    register_demo!(runner, demo_natural_polynomial_evaluate_many);
    register_demo!(runner, demo_natural_polynomial_mod_evaluate_many);
    register_bench!(
        runner,
        benchmark_natural_polynomial_evaluate_many_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_evaluate_many_algorithms
    );
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

fn demo_natural_polynomial_mod_power_of_2_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "(&({p})).mod_power_of_2_evaluate({x_old}, {pow}) = {}",
            (&p).mod_power_of_2_evaluate(x, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_evaluate_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, x, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_evaluate({x}, {pow}) = {}",
            (&p).mod_power_of_2_evaluate(&x, pow)
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_evaluate(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "(&NaturalPolynomial).mod_power_of_2_evaluate(Natural, u64)",
                &mut |(p, x, pow)| no_out!((&p).mod_power_of_2_evaluate(x, pow)),
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_evaluate(&Natural, u64)",
                &mut |(p, x, pow)| no_out!((&p).mod_power_of_2_evaluate(&x, pow)),
            ),
            (
                "NaturalPolynomial.mod_power_of_2_evaluate(Natural, u64)",
                &mut |(p, x, pow)| no_out!(p.mod_power_of_2_evaluate(x, pow)),
            ),
            (
                "NaturalPolynomial.mod_power_of_2_evaluate(&Natural, u64)",
                &mut |(p, x, pow)| no_out!(p.mod_power_of_2_evaluate(&x, pow)),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_evaluate(&Natural, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, pow)| {
                no_out!((&p).mod_power_of_2_evaluate(&x, pow));
            }),
            ("evaluate, then reduce", &mut |(p, x, pow)| {
                no_out!(mod_power_of_2_evaluate_naive(&p, &x, pow));
            }),
        ],
    );
}

fn demo_natural_polynomial_mod_evaluate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p_old, x_old, m_old) = (p.clone(), x.clone(), m.clone());
        println!(
            "({p_old}).mod_evaluate({x_old}, {m_old}) = {}",
            p.mod_evaluate(x, m)
        );
    }
}

fn demo_natural_polynomial_mod_evaluate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in natural_polynomial_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate({x}, {m}) = {}",
            (&p).mod_evaluate(&x, &m)
        );
    }
}

fn benchmark_natural_polynomial_mod_evaluate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_evaluate(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_evaluate(Natural, Natural)",
                &mut |(p, x, m)| no_out!(p.mod_evaluate(x, m)),
            ),
            (
                "NaturalPolynomial.mod_evaluate(&Natural, &Natural)",
                &mut |(p, x, m)| no_out!(p.mod_evaluate(&x, &m)),
            ),
            (
                "(&NaturalPolynomial).mod_evaluate(Natural, Natural)",
                &mut |(p, x, m)| no_out!((&p).mod_evaluate(x, m)),
            ),
            (
                "(&NaturalPolynomial).mod_evaluate(&Natural, &Natural)",
                &mut |(p, x, m)| no_out!((&p).mod_evaluate(&x, &m)),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_evaluate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_evaluate(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, m)| {
                no_out!((&p).mod_evaluate(&x, &m));
            }),
            ("evaluate, then reduce", &mut |(p, x, m)| {
                no_out!(mod_evaluate_naive(&p, &x, &m));
            }),
        ],
    );
}

fn demo_natural_polynomial_evaluate_many(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs) in natural_polynomial_natural_vec_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).evaluate_many(&{xs:?}) = {:?}",
            (&p).evaluate_many(&xs)
        );
    }
}

fn demo_natural_polynomial_mod_evaluate_many(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs, m) in natural_polynomial_natural_vec_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate_many(&{xs:?}, &{m}) = {:?}",
            (&p).mod_evaluate_many(&xs, &m)
        );
    }
}

fn benchmark_natural_polynomial_evaluate_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).evaluate_many(&[Natural])",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_vec_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs)| no_out!((&p).evaluate_many(&xs))),
            ("naive", &mut |(p, xs)| {
                no_out!(evaluate_many_naive(&p, &xs));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_evaluate_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_evaluate_many(&[Natural], &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_vec_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs, m)| {
                no_out!((&p).mod_evaluate_many(&xs, &m));
            }),
            ("one at a time", &mut |(p, xs, m)| {
                no_out!(
                    xs.iter()
                        .map(|x| (&p).mod_evaluate(x, &m))
                        .collect::<Vec<_>>()
                );
            }),
            ("naive", &mut |(p, xs, m)| {
                no_out!(mod_evaluate_many_naive(&p, &xs, &m));
            }),
        ],
    );
}
