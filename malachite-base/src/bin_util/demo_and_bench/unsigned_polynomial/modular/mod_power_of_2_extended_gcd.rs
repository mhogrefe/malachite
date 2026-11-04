// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::CheckedModPowerOf2ExtendedGcd;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_extended_gcd::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_extended_gcd
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_extended_gcd_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_checked_mod_power_of_2_extended_gcd_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_checked_mod_power_of_2_extended_gcd_algorithms
    );
}

fn show_checked<T: PrimitiveUnsigned>(
    x: Option<(
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    )>,
) -> String {
    x.map_or_else(
        || "None".to_string(),
        |(g, s, t)| format!("({g}, {s}, {t})"),
    )
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_extended_gcd<T: PrimitiveUnsigned>(
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
            "({p_old}).checked_mod_power_of_2_extended_gcd({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_extended_gcd(q, pow))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_extended_gcd_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_extended_gcd(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_extended_gcd(&q, pow))
        );
    }
}

fn benchmark_unsigned_polynomial_checked_mod_power_of_2_extended_gcd_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            concat!(
                "UnsignedPolynomial<{}>.checked_mod_power_of_2_extended_gcd(",
                "UnsignedPolynomial<{}>, u64)"
            ),
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
            (
                "p.checked_mod_power_of_2_extended_gcd(q, pow)",
                &mut |(p, q, pow)| {
                    no_out!(p.checked_mod_power_of_2_extended_gcd(q, pow));
                },
            ),
            (
                "p.checked_mod_power_of_2_extended_gcd(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!(p.checked_mod_power_of_2_extended_gcd(&q, pow));
                },
            ),
            (
                "(&p).checked_mod_power_of_2_extended_gcd(q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).checked_mod_power_of_2_extended_gcd(q, pow));
                },
            ),
            (
                "(&p).checked_mod_power_of_2_extended_gcd(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).checked_mod_power_of_2_extended_gcd(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_checked_mod_power_of_2_extended_gcd_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            concat!(
                "UnsignedPolynomial<{}>.checked_mod_power_of_2_extended_gcd(",
                "UnsignedPolynomial<{}>, u64)"
            ),
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
                no_out!((&p).checked_mod_power_of_2_extended_gcd(&q, pow));
            }),
            ("naive", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_extended_gcd_naive(&p, &q, pow));
            }),
            ("Euclid", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_extended_gcd_euclidean(&p, &q, pow));
            }),
            ("half-GCD", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_extended_gcd_hgcd(&p, &q, pow));
            }),
        ],
    );
}
