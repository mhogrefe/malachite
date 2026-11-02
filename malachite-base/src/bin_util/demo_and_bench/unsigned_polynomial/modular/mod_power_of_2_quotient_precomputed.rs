// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2Quotient, ModPowerOf2QuotientPrecomputed, ModPowerOf2QuotientPrecomputedAssign,
};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_quotient_precomputed
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_quotient_precomputed_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_quotient_precomputed_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_quotient_precomputed_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_quotient_precomputed_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_quotient_precomputed<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = UnsignedPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
        println!(
            "({p_old}).mod_power_of_2_quotient_precomputed({q_old}, {pow}, &data) = {}",
            p.mod_power_of_2_quotient_precomputed(q, pow, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_quotient_precomputed_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = UnsignedPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
        println!(
            "(&({p})).mod_power_of_2_quotient_precomputed(&({q}), {pow}, &data) = {}",
            (&p).mod_power_of_2_quotient_precomputed(&q, pow, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_quotient_precomputed_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = UnsignedPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
        p.mod_power_of_2_quotient_precomputed_assign(q, pow, &data);
        println!(
            "p := {p_old}; p.mod_power_of_2_quotient_precomputed_assign({q_old}, {pow}, \
            &data); p = {p}"
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_quotient_precomputed_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_quotient_precomputed(\
            UnsignedPolynomial<{}>, u64, &data)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
            .get(gm, config)
            .map(|(p, q, pow)| {
                let data = UnsignedPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
                (p, q, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
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

fn benchmark_unsigned_polynomial_mod_power_of_2_quotient_precomputed_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_quotient_precomputed(\
            UnsignedPolynomial<{}>, u64, &data)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
            .get(gm, config)
            .map(|(p, q, pow)| {
                let data = UnsignedPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
                (p, q, (pow, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("precomputed", &mut |(p, q, (pow, data))| {
                no_out!((&p).mod_power_of_2_quotient_precomputed(&q, pow, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, q, (pow, _))| {
                    let data = UnsignedPolynomial::precompute_mod_power_of_2_quotient_data(&q, pow);
                    no_out!((&p).mod_power_of_2_quotient_precomputed(&q, pow, &data));
                },
            ),
            ("not precomputed", &mut |(p, q, (pow, _))| {
                no_out!((&p).mod_power_of_2_quotient(&q, pow));
            }),
        ],
    );
}
