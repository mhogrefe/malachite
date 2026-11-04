// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::CheckedModPowerOf2Gcd;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_gcd::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_checked_mod_power_of_2_gcd);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_gcd_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_checked_mod_power_of_2_gcd_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_checked_mod_power_of_2_gcd_algorithms
    );
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_gcd<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_power_of_2_gcd({q_old}, {pow}) = {}",
            p.checked_mod_power_of_2_gcd(q, pow)
                .map_or_else(|| "None".to_string(), |g| g.to_string())
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_gcd_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_gcd(&({q}), {pow}) = {}",
            (&p).checked_mod_power_of_2_gcd(&q, pow)
                .map_or_else(|| "None".to_string(), |g| g.to_string())
        );
    }
}

fn benchmark_unsigned_polynomial_checked_mod_power_of_2_gcd_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.checked_mod_power_of_2_gcd(UnsignedPolynomial<{}>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
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

fn benchmark_unsigned_polynomial_checked_mod_power_of_2_gcd_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.checked_mod_power_of_2_gcd(UnsignedPolynomial<{}>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!((&p).checked_mod_power_of_2_gcd(&q, pow));
            }),
            ("naive", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_gcd_naive(&p, &q, pow));
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
