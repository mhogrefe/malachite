// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Pow, ModPowerOf2PowAssign};
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::arithmetic::mod_power_of_2_pow::*;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_unsigned_triple_gen_var_4;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_pow);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_pow_ref);
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_pow_assign);

    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_pow_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_pow_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_pow({e}, {pow}) = {}",
            p.mod_power_of_2_pow(e, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, e, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_pow({e}, {pow}) = {}",
            (&p).mod_power_of_2_pow(e, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_pow_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, e, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_pow_assign(e, pow);
        println!("p := {p_old}; p.mod_power_of_2_pow_assign({e}, {pow}); p = {p}");
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_pow(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            (
                "NaturalPolynomial.mod_power_of_2_pow(u64, u64)",
                &mut |(p, e, pow)| no_out!(p.mod_power_of_2_pow(e, pow)),
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_pow(u64, u64)",
                &mut |(p, e, pow)| no_out!((&p).mod_power_of_2_pow(e, pow)),
            ),
            (
                "NaturalPolynomial.mod_power_of_2_pow_assign(u64, u64)",
                &mut |(mut p, e, pow)| p.mod_power_of_2_pow_assign(e, pow),
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_pow(u64, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("exp"),
        &mut [
            ("default", &mut |(p, e, pow)| {
                no_out!((&p).mod_power_of_2_pow(e, pow));
            }),
            ("binary exponentiation", &mut |(p, e, pow)| {
                let xs = p.coefficients_asc();
                if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && pow != 0 {
                    no_out!(mod_power_of_2_pow_binexp(xs, e, pow));
                }
            }),
            ("integer power, then reduction", &mut |(p, e, pow)| {
                let xs = p.coefficients_asc();
                if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && pow != 0 {
                    no_out!(mod_power_of_2_pow_exact(xs, e, pow));
                }
            }),
            ("naive", &mut |(p, e, pow)| {
                no_out!(mod_power_of_2_pow_naive(&p, e, pow));
            }),
        ],
    );
}
