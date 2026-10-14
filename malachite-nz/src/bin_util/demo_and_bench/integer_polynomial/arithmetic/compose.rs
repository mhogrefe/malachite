// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{Compose, ComposeAssign, Polynomial};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::compose::{
    compose_divide_and_conquer, compose_horner,
};
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_1,
    integer_polynomial_pair_gen_var_2,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::compose::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_compose);
    register_demo!(runner, demo_integer_polynomial_compose_val_ref);
    register_demo!(runner, demo_integer_polynomial_compose_ref_val);
    register_demo!(runner, demo_integer_polynomial_compose_ref_ref);
    register_demo!(runner, demo_integer_polynomial_compose_assign);
    register_demo!(runner, demo_integer_polynomial_compose_assign_ref);
    register_demo!(runner, demo_integer_polynomial_compose_long);
    register_demo!(runner, demo_integer_polynomial_compose_linear);
    register_demo!(runner, demo_integer_polynomial_compose_horner);
    register_demo!(runner, demo_integer_polynomial_compose_divide_and_conquer);
    register_bench!(
        runner,
        benchmark_integer_polynomial_compose_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_polynomial_compose_algorithms);
    register_bench!(runner, benchmark_integer_polynomial_compose_algorithms_long);
}

fn demo_integer_polynomial_compose(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).compose({q_old}) = {}", p.compose(q));
    }
}

fn demo_integer_polynomial_compose_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        println!("({p_old}).compose(&({q})) = {}", p.compose(&q));
    }
}

fn demo_integer_polynomial_compose_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let q_old = q.clone();
        println!("(&({p})).compose({q_old}) = {}", (&p).compose(q));
    }
}

fn demo_integer_polynomial_compose_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        println!("(&({p})).compose(&({q})) = {}", (&p).compose(&q));
    }
}

fn demo_integer_polynomial_compose_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        p.compose_assign(q);
        println!("p := {p_old}; p.compose_assign({q_old}); p = {p}");
    }
}

fn demo_integer_polynomial_compose_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.compose_assign(&q);
        println!("p := {p_old}; p.compose_assign(&({q})); p = {p}");
    }
}

// The first polynomial has degree at least 4 and the second at least 2, so that the
// divide-and-conquer path is taken.
fn demo_integer_polynomial_compose_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).compose(&({q})) = {}", (&p).compose(&q));
    }
}

// The first polynomial has degree at least 4 and the second is linear, so that the composition is a
// Taylor shift and a scaling.
fn demo_integer_polynomial_compose_linear(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).compose(&({q})) = {}", (&p).compose(&q));
    }
}

fn demo_integer_polynomial_compose_horner(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let r = IntegerPolynomial::from_coefficients_asc(compose_horner(
            p.coefficients_asc(),
            q.coefficients_asc(),
        ));
        println!("compose_horner({p}, {q}) = {r}");
    }
}

fn demo_integer_polynomial_compose_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q) in integer_polynomial_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let r = IntegerPolynomial::from_coefficients_asc(compose_divide_and_conquer(
            p.coefficients_asc(),
            q.coefficients_asc(),
        ));
        println!("compose_divide_and_conquer({p}, {q}) = {r}");
    }
}

fn benchmark_integer_polynomial_compose_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.compose(IntegerPolynomial)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "IntegerPolynomial.compose(IntegerPolynomial)",
                &mut |(p, q)| {
                    no_out!(p.compose(q));
                },
            ),
            (
                "IntegerPolynomial.compose(&IntegerPolynomial)",
                &mut |(p, q)| {
                    no_out!(p.compose(&q));
                },
            ),
            (
                "(&IntegerPolynomial).compose(IntegerPolynomial)",
                &mut |(p, q)| {
                    no_out!((&p).compose(q));
                },
            ),
            (
                "(&IntegerPolynomial).compose(&IntegerPolynomial)",
                &mut |(p, q)| {
                    no_out!((&p).compose(&q));
                },
            ),
            (
                "IntegerPolynomial.compose_assign(IntegerPolynomial)",
                &mut |(mut p, q)| {
                    p.compose_assign(q);
                },
            ),
            (
                "IntegerPolynomial.compose_assign(&IntegerPolynomial)",
                &mut |(mut p, q)| {
                    p.compose_assign(&q);
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_compose_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.compose(IntegerPolynomial)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).compose(&q));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(compose_naive(&p, &q));
            }),
        ],
    );
}

fn benchmark_integer_polynomial_compose_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.compose(IntegerPolynomial)",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!((&p).compose(&q));
            }),
            ("Horner", &mut |(p, q)| {
                no_out!(compose_horner(p.coefficients_asc(), q.coefficients_asc()));
            }),
            ("divide and conquer", &mut |(p, q)| {
                no_out!(compose_divide_and_conquer(
                    p.coefficients_asc(),
                    q.coefficients_asc()
                ));
            }),
            ("naive", &mut |(p, q)| {
                no_out!(compose_naive(&p, &q));
            }),
        ],
    );
}
