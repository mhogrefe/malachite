// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::{
    ModPowerOf2Quotient, ModPowerOf2QuotientPrecomputed, ModPowerOf2QuotientPrecomputedAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_2_natural_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_quotient_precomputed
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_quotient_precomputed_ref_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_quotient_precomputed_assign
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_quotient_precomputed_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_quotient_precomputed_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_quotient_precomputed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
        println!(
            "({p_old}).mod_power_of_2_quotient_precomputed({q_old}, {pow}, &data) = {}",
            p.mod_power_of_2_quotient_precomputed(q, pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_quotient_precomputed_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
        println!(
            "(&({p})).mod_power_of_2_quotient_precomputed(&({q}), {pow}, &data) = {}",
            (&p).mod_power_of_2_quotient_precomputed(&q, pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_quotient_precomputed_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
        p.mod_power_of_2_quotient_precomputed_assign(q, pow, &data);
        println!(
            "p := {p_old}; p.mod_power_of_2_quotient_precomputed_assign({q_old}, {pow}, &data); \
            p = {p}"
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_quotient_precomputed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_quotient_precomputed(NaturalPolynomial, u64, &data)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
            .get(gm, config)
            .map(|(p, q, pow)| {
                let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
                (p, q, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            (
                "p.mod_power_of_2_quotient_precomputed(q, pow, &data)",
                &mut |(p, q, (pow, data))| {
                    no_out!(p.mod_power_of_2_quotient_precomputed(q, pow, &data));
                },
            ),
            (
                "p.mod_power_of_2_quotient_precomputed(&q, pow, &data)",
                &mut |(p, q, (pow, data))| {
                    no_out!(p.mod_power_of_2_quotient_precomputed(&q, pow, &data));
                },
            ),
            (
                "(&p).mod_power_of_2_quotient_precomputed(q, pow, &data)",
                &mut |(p, q, (pow, data))| {
                    no_out!((&p).mod_power_of_2_quotient_precomputed(q, pow, &data));
                },
            ),
            (
                "(&p).mod_power_of_2_quotient_precomputed(&q, pow, &data)",
                &mut |(p, q, (pow, data))| {
                    no_out!((&p).mod_power_of_2_quotient_precomputed(&q, pow, &data));
                },
            ),
            (
                "p.mod_power_of_2_quotient_precomputed_assign(q, pow, &data)",
                &mut |(mut p, q, (pow, data))| {
                    no_out!(p.mod_power_of_2_quotient_precomputed_assign(q, pow, &data));
                },
            ),
            (
                "p.mod_power_of_2_quotient_precomputed_assign(&q, pow, &data)",
                &mut |(mut p, q, (pow, data))| {
                    no_out!(p.mod_power_of_2_quotient_precomputed_assign(&q, pow, &data));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_quotient_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_quotient_precomputed(NaturalPolynomial, u64, &data)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5()
            .get(gm, config)
            .map(|(p, q, pow)| {
                let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
                (p, q, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("precomputed", &mut |(p, q, (pow, data))| {
                no_out!((&p).mod_power_of_2_quotient_precomputed(&q, pow, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, q, (pow, _))| {
                    let data = NaturalPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
                    no_out!((&p).mod_power_of_2_quotient_precomputed(&q, pow, &data));
                },
            ),
            ("not precomputed", &mut |(p, q, (pow, _))| {
                no_out!((&p).mod_power_of_2_quotient(&q, pow));
            }),
        ],
    );
}
