// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{ModEvaluateMany, ModEvaluateManyPrecomputed};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    Modulus, evaluate_many_fast_precomputed,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_evaluate_many_precomputed
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_evaluate_many_precomputed_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_many_precomputed_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_many_precomputed_algorithms
    );
}

fn demo_unsigned_polynomial_mod_evaluate_many_precomputed<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, m) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
        println!(
            "({p_old}).mod_evaluate_many_precomputed({m}, &data({xs:?})) = {:?}",
            p.mod_evaluate_many_precomputed(m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_evaluate_many_precomputed_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, m) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
        println!(
            "(&({p})).mod_evaluate_many_precomputed({m}, &data({xs:?})) = {:?}",
            (&p).mod_evaluate_many_precomputed(m, &data)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_evaluate_many_precomputed_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_evaluate_many_precomputed({}, &data)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
            .get(gm, config)
            .map(|(p, xs, m)| {
                let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
                (p, xs, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial.mod_evaluate_many_precomputed(T, &data)",
                &mut |(p, _, (m, data))| {
                    no_out!(p.mod_evaluate_many_precomputed(m, &data));
                },
            ),
            (
                "(&UnsignedPolynomial).mod_evaluate_many_precomputed(T, &data)",
                &mut |(p, _, (m, data))| {
                    no_out!((&p).mod_evaluate_many_precomputed(m, &data));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_evaluate_many_precomputed_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "(&UnsignedPolynomial<{}>).mod_evaluate_many_precomputed({}, &data)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
            .get(gm, config)
            .map(|(p, xs, m)| {
                let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
                (p, xs, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("precomputed", &mut |(p, _, (m, data))| {
                no_out!((&p).mod_evaluate_many_precomputed(m, &data));
            }),
            ("precomputed, without inverses", &mut |(p, _, (m, data))| {
                no_out!(evaluate_many_fast_precomputed(
                    p.coefficients_asc(),
                    &data.tree,
                    &[],
                    data.len(),
                    &Modulus(m)
                ));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, xs, (m, _))| {
                    let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
                    no_out!((&p).mod_evaluate_many_precomputed(m, &data));
                },
            ),
            ("mod_evaluate_many", &mut |(p, xs, (m, _))| {
                no_out!((&p).mod_evaluate_many(&xs, m));
            }),
            ("naive", &mut |(p, xs, (m, _))| {
                no_out!(mod_evaluate_many_naive(&p, &xs, m));
            }),
        ],
    );
}
