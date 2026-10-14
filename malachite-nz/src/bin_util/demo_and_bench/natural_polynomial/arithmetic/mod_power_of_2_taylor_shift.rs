// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{ModPowerOf2TaylorShift, ModPowerOf2TaylorShiftAssign};
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::arithmetic::mod_power_of_2_taylor_shift::{
    mod_power_of_2_taylor_shift_divide_and_conquer, mod_power_of_2_taylor_shift_horner,
};
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_unsigned_triple_gen_var_1,
    natural_polynomial_natural_unsigned_triple_gen_var_2,
    natural_polynomial_natural_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_taylor_shift::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_taylor_shift);
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_taylor_shift_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_taylor_shift_assign
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_taylor_shift_long
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_taylor_shift_small_shift
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_taylor_shift_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_taylor_shift_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_taylor_shift_algorithms_long
    );
}

fn demo_natural_polynomial_mod_power_of_2_taylor_shift(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            p.mod_power_of_2_taylor_shift(&c, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_taylor_shift_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            (&p).mod_power_of_2_taylor_shift(&c, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_taylor_shift_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_taylor_shift_assign(&c, pow);
        println!("p := {p_old}; p.mod_power_of_2_taylor_shift_assign({c}, {pow}); p = {p}");
    }
}

// Polynomials of degree at least 50, some long enough for the divide-and-conquer path.
fn demo_natural_polynomial_mod_power_of_2_taylor_shift_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            (&p).mod_power_of_2_taylor_shift(&c, pow)
        );
    }
}

// Shifts of at most 3, so that 0 and 1, which take their own paths, are common.
fn demo_natural_polynomial_mod_power_of_2_taylor_shift_small_shift(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, c, pow) in natural_polynomial_natural_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_taylor_shift({c}, {pow}) = {}",
            (&p).mod_power_of_2_taylor_shift(&c, pow)
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_taylor_shift_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_taylor_shift(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_power_of_2_taylor_shift(Natural, u64)",
                &mut |(p, c, pow)| {
                    no_out!(p.mod_power_of_2_taylor_shift(c, pow));
                },
            ),
            (
                "NaturalPolynomial.mod_power_of_2_taylor_shift(&Natural, u64)",
                &mut |(p, c, pow)| {
                    no_out!(p.mod_power_of_2_taylor_shift(&c, pow));
                },
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_taylor_shift(Natural, u64)",
                &mut |(p, c, pow)| {
                    no_out!((&p).mod_power_of_2_taylor_shift(c, pow));
                },
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_taylor_shift(&Natural, u64)",
                &mut |(p, c, pow)| {
                    no_out!((&p).mod_power_of_2_taylor_shift(&c, pow));
                },
            ),
            (
                "NaturalPolynomial.mod_power_of_2_taylor_shift_assign(Natural, u64)",
                &mut |(mut p, c, pow)| {
                    p.mod_power_of_2_taylor_shift_assign(c, pow);
                },
            ),
            (
                "NaturalPolynomial.mod_power_of_2_taylor_shift_assign(&Natural, u64)",
                &mut |(mut p, c, pow)| {
                    p.mod_power_of_2_taylor_shift_assign(&c, pow);
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_taylor_shift_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_taylor_shift(&Natural, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [
            ("default", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_taylor_shift(&c, pow));
            }),
            ("Horner", &mut |(p, c, pow)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_power_of_2_taylor_shift_horner(&mut xs, &c, pow);
            }),
            ("divide and conquer", &mut |(p, c, pow)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_power_of_2_taylor_shift_divide_and_conquer(&mut xs, &c, pow);
            }),
            ("exact shift, then reduce", &mut |(p, c, pow)| {
                no_out!(mod_power_of_2_taylor_shift_naive(&p, &c, pow));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_taylor_shift_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_taylor_shift(&Natural, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, c, pow)| {
                no_out!((&p).mod_power_of_2_taylor_shift(&c, pow));
            }),
            ("Horner", &mut |(p, c, pow)| {
                let mut xs = p.coefficients_asc().to_vec();
                mod_power_of_2_taylor_shift_horner(&mut xs, &c, pow);
            }),
            ("exact shift, then reduce", &mut |(p, c, pow)| {
                no_out!(mod_power_of_2_taylor_shift_naive(&p, &c, pow));
            }),
        ],
    );
}
