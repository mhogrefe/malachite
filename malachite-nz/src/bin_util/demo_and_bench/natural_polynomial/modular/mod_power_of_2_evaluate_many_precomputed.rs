// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::{
    ModPowerOf2EvaluateMany, ModPowerOf2EvaluateManyPrecomputed,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_polynomial_len_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_power_of_2_evaluate_many::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_evaluate_many_precomputed
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_evaluate_many_precomputed_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_evaluate_many_precomputed_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_evaluate_many_precomputed_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_evaluate_many_precomputed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, pow) in natural_polynomial_natural_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
        println!(
            "({p_old}).mod_power_of_2_evaluate_many_precomputed({pow}, &data({xs:?})) = {:?}",
            p.mod_power_of_2_evaluate_many_precomputed(pow, &data)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_evaluate_many_precomputed_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, pow) in natural_polynomial_natural_vec_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
        println!(
            "(&({p})).mod_power_of_2_evaluate_many_precomputed({pow}, &data({xs:?})) = {:?}",
            (&p).mod_power_of_2_evaluate_many_precomputed(pow, &data)
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_evaluate_many_precomputed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_evaluate_many_precomputed(u64, &data)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_vec_unsigned_triple_gen_var_1()
            .get(gm, config)
            .map(|(p, xs, pow)| {
                let data =
                    NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
                (p, pow, data)
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_len_bucketer("p"),
        &mut [
            (
                "NaturalPolynomial.mod_power_of_2_evaluate_many_precomputed(u64, &data)",
                &mut |(p, pow, data)| {
                    no_out!(p.mod_power_of_2_evaluate_many_precomputed(pow, &data));
                },
            ),
            (
                "(&NaturalPolynomial).mod_power_of_2_evaluate_many_precomputed(u64, &data)",
                &mut |(p, pow, data)| {
                    no_out!((&p).mod_power_of_2_evaluate_many_precomputed(pow, &data));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_evaluate_many_precomputed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_evaluate_many_precomputed(u64, &data)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_vec_unsigned_triple_gen_var_1()
            .get(gm, config)
            .map(|(p, xs, pow)| {
                let data =
                    NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
                (p, xs, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_polynomial_len_bucketer("p"),
        &mut [
            ("precomputed", &mut |(p, _, (pow, data))| {
                no_out!((&p).mod_power_of_2_evaluate_many_precomputed(pow, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, xs, (pow, _))| {
                    let data =
                        NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
                    no_out!((&p).mod_power_of_2_evaluate_many_precomputed(pow, &data));
                },
            ),
            ("mod_power_of_2_evaluate_many", &mut |(p, xs, (pow, _))| {
                no_out!((&p).mod_power_of_2_evaluate_many(&xs, pow));
            }),
            ("naive", &mut |(p, xs, (pow, _))| {
                no_out!(mod_power_of_2_evaluate_many_naive(&p, &xs, pow));
            }),
        ],
    );
}
