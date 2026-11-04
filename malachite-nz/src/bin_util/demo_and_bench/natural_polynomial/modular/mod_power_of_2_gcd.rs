// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::modular::mod_gcd::{
    mod_power_of_2_gcd_euclidean, mod_power_of_2_gcd_hgcd,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_checked_mod_power_of_2_gcd);
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_power_of_2_gcd_ref_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_checked_mod_power_of_2_gcd_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_checked_mod_power_of_2_gcd_algorithms
    );
}

fn show_checked(x: Option<NaturalPolynomial>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_checked_mod_power_of_2_gcd(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_power_of_2_gcd({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_gcd(q, pow))
        );
    }
}

fn demo_natural_polynomial_checked_mod_power_of_2_gcd_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_gcd(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_gcd(&q, pow))
        );
    }
}

fn benchmark_natural_polynomial_checked_mod_power_of_2_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.checked_mod_power_of_2_gcd(NaturalPolynomial, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p.checked_mod_power_of_2_gcd(q, pow)", &mut |(
                p,
                q,
                pow,
            )| {
                no_out!(p.checked_mod_power_of_2_gcd(q, pow));
            }),
            (
                "p.checked_mod_power_of_2_gcd(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!(p.checked_mod_power_of_2_gcd(&q, pow));
                },
            ),
            (
                "(&p).checked_mod_power_of_2_gcd(q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).checked_mod_power_of_2_gcd(q, pow));
                },
            ),
            (
                "(&p).checked_mod_power_of_2_gcd(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).checked_mod_power_of_2_gcd(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_checked_mod_power_of_2_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.checked_mod_power_of_2_gcd(NaturalPolynomial, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!((&p).checked_mod_power_of_2_gcd(&q, pow));
            }),
            ("Euclid", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_gcd_euclidean(&p, &q, pow));
            }),
            ("half-GCD", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_gcd_hgcd(&p, &q, pow));
            }),
        ],
    );
}
