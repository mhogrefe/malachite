// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::Factor;
use malachite_base::polynomial::modular::traits::ModCoprimeWith;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_coprime_with::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_coprime_with);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_coprime_with_ref_ref);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_coprime_with_evaluation_strategy
    );
    register_generic_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_coprime_with_algorithms,
        u8,
        u16,
        u32,
        u64,
        usize
    );
}

fn demo_unsigned_polynomial_mod_coprime_with<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_coprime_with({q_old}, {m}) = {}",
            p.mod_coprime_with(q, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_coprime_with_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_coprime_with(&({q}), {m}) = {}",
            (&p).mod_coprime_with(&q, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_coprime_with_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_coprime_with(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_coprime_with(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_coprime_with(q, m));
            }),
            ("p.mod_coprime_with(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_coprime_with(&q, m));
            }),
            ("(&p).mod_coprime_with(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_coprime_with(q, m));
            }),
            ("(&p).mod_coprime_with(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_coprime_with(&q, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_coprime_with_algorithms<T: PrimitiveUnsigned + Factor>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::FACTORS: IntoIterator<Item = (T, u8)>,
{
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_coprime_with(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_coprime_with(&q, m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_coprime_with_naive(&p, &q, m));
            }),
        ],
    );
}
