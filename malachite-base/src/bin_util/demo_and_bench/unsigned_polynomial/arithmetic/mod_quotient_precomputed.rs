// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModQuotient, ModQuotientPrecomputed, ModQuotientPrecomputedAssign,
};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_quotient_precomputed);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_quotient_precomputed_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_quotient_precomputed_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_quotient_precomputed_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_quotient_precomputed_algorithms
    );
}

fn demo_unsigned_polynomial_mod_quotient_precomputed<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &m);
        println!(
            "({p_old}).mod_quotient_precomputed({q_old}, {m}, &data) = {}",
            p.mod_quotient_precomputed(q, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_quotient_precomputed_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &m);
        println!(
            "(&({p})).mod_quotient_precomputed(&({q}), {m}, &data) = {}",
            (&p).mod_quotient_precomputed(&q, m, &data)
        );
    }
}

fn demo_unsigned_polynomial_mod_quotient_precomputed_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &m);
        p.mod_quotient_precomputed_assign(q, m, &data);
        println!("p := {p_old}; p.mod_quotient_precomputed_assign({q_old}, {m}, &data); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_quotient_precomputed_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_quotient_precomputed(UnsignedPolynomial<{}>, {}, &data)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
            .get(gm, config)
            .map(|(p, q, m)| {
                let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &m);
                (p, q, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "p.mod_quotient_precomputed(q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!(p.mod_quotient_precomputed(q, m, &data));
                },
            ),
            (
                "p.mod_quotient_precomputed(&q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!(p.mod_quotient_precomputed(&q, m, &data));
                },
            ),
            (
                "(&p).mod_quotient_precomputed(q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!((&p).mod_quotient_precomputed(q, m, &data));
                },
            ),
            (
                "(&p).mod_quotient_precomputed(&q, m, &data)",
                &mut |(p, q, (m, data))| {
                    no_out!((&p).mod_quotient_precomputed(&q, m, &data));
                },
            ),
            (
                "p.mod_quotient_precomputed_assign(q, m, &data)",
                &mut |(mut p, q, (m, data))| {
                    no_out!(p.mod_quotient_precomputed_assign(q, m, &data));
                },
            ),
            (
                "p.mod_quotient_precomputed_assign(&q, m, &data)",
                &mut |(mut p, q, (m, data))| {
                    no_out!(p.mod_quotient_precomputed_assign(&q, m, &data));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_quotient_precomputed_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_quotient_precomputed(UnsignedPolynomial<{}>, {}, &data)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
            .get(gm, config)
            .map(|(p, q, m)| {
                let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &m);
                (p, q, (m, data))
            }),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("precomputed", &mut |(p, q, (m, data))| {
                no_out!((&p).mod_quotient_precomputed(&q, m, &data));
            }),
            (
                "precomputed, including precomputation",
                &mut |(p, q, (m, _))| {
                    let data = UnsignedPolynomial::precompute_mod_quotient_data(&q, &m);
                    no_out!((&p).mod_quotient_precomputed(&q, m, &data));
                },
            ),
            ("not precomputed", &mut |(p, q, (m, _))| {
                no_out!((&p).mod_quotient(&q, m));
            }),
        ],
    );
}
