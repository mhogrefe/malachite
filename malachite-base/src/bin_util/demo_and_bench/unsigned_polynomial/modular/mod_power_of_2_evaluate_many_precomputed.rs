// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2EvaluateMany, ModPowerOf2EvaluateManyPrecomputed,
};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_evaluate_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, pow) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let data = UnsignedPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
        println!(
            "({p_old}).mod_power_of_2_evaluate_many_precomputed({pow}, &data({xs:?})) = {:?}",
            p.mod_power_of_2_evaluate_many_precomputed(pow, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, pow) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = UnsignedPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
        println!(
            "(&({p})).mod_power_of_2_evaluate_many_precomputed({pow}, &data({xs:?})) = {:?}",
            (&p).mod_power_of_2_evaluate_many_precomputed(pow, &data)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_evaluate_many_precomputed(u64, &data)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
            .get(gm, config)
            .map(|(p, xs, pow)| {
                let data =
                    UnsignedPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
                (p, xs, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial.mod_power_of_2_evaluate_many_precomputed(u64, &data)",
                &mut |(p, _, (pow, data))| {
                    no_out!(p.mod_power_of_2_evaluate_many_precomputed(pow, &data));
                },
            ),
            (
                "(&UnsignedPolynomial).mod_power_of_2_evaluate_many_precomputed(u64, &data)",
                &mut |(p, _, (pow, data))| {
                    no_out!((&p).mod_power_of_2_evaluate_many_precomputed(pow, &data));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_evaluate_many_precomputed_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "(&UnsignedPolynomial<{}>).mod_power_of_2_evaluate_many_precomputed(u64, &data)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
            .get(gm, config)
            .map(|(p, xs, pow)| {
                let data =
                    UnsignedPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
                (p, xs, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("precomputed", &mut |(p, _, (pow, data))| {
                no_out!((&p).mod_power_of_2_evaluate_many_precomputed(pow, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, xs, (pow, _))| {
                    let data =
                        UnsignedPolynomial::precompute_mod_power_of_2_evaluate_many_data(&xs, pow);
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
