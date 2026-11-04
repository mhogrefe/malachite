// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::polynomial::modular::traits::{CheckedModExtendedGcd, ModExtendedGcd};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_extended_gcd::{
    mod_extended_gcd_euclidean, mod_extended_gcd_hgcd, mod_extended_gcd_naive,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_generic_demos!(
        runner,
        demo_unsigned_polynomial_mod_extended_gcd,
        u8,
        u16,
        u32,
        u64,
        usize
    );
    register_generic_demos!(
        runner,
        demo_unsigned_polynomial_mod_extended_gcd_ref_ref,
        u8,
        u16,
        u32,
        u64,
        usize
    );
    register_unsigned_demos!(runner, demo_unsigned_polynomial_checked_mod_extended_gcd);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_extended_gcd_ref_ref
    );
    register_generic_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_extended_gcd_evaluation_strategy,
        u8,
        u16,
        u32,
        u64,
        usize
    );
    register_generic_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_extended_gcd_algorithms,
        u8,
        u16,
        u32,
        u64,
        usize
    );
}

fn show<T: PrimitiveUnsigned>(
    (g, s, t): (
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    ),
) -> String {
    format!("({g}, {s}, {t})")
}

fn show_checked<T: PrimitiveUnsigned>(
    x: Option<(
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
    )>,
) -> String {
    x.map_or_else(|| "None".to_string(), show)
}

fn demo_unsigned_polynomial_mod_extended_gcd<T: PrimitiveUnsigned + IsPrime>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_extended_gcd({q_old}, {m}) = {}",
            show(p.mod_extended_gcd(q, m))
        );
    }
}

fn demo_unsigned_polynomial_mod_extended_gcd_ref_ref<T: PrimitiveUnsigned + IsPrime>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_extended_gcd(&({q}), {m}) = {}",
            show((&p).mod_extended_gcd(&q, m))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_extended_gcd<T: PrimitiveUnsigned>(
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
            "({p_old}).checked_mod_extended_gcd({q_old}, {m}) = {}",
            show_checked(p.checked_mod_extended_gcd(q, m))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_extended_gcd_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_extended_gcd(&({q}), {m}) = {}",
            show_checked((&p).checked_mod_extended_gcd(&q, m))
        );
    }
}

fn benchmark_unsigned_polynomial_mod_extended_gcd_evaluation_strategy<
    T: PrimitiveUnsigned + IsPrime,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_extended_gcd(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_extended_gcd(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_extended_gcd(q, m));
            }),
            ("p.mod_extended_gcd(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_extended_gcd(&q, m));
            }),
            ("(&p).mod_extended_gcd(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_extended_gcd(q, m));
            }),
            ("(&p).mod_extended_gcd(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_extended_gcd(&q, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_extended_gcd_algorithms<T: PrimitiveUnsigned + IsPrime>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_extended_gcd(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_extended_gcd(&q, m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_extended_gcd_naive(&p, &q, m));
            }),
            ("Euclid", &mut |(p, q, m)| {
                no_out!(mod_extended_gcd_euclidean(&p, &q, m));
            }),
            ("half-GCD", &mut |(p, q, m)| {
                no_out!(mod_extended_gcd_hgcd(&p, &q, m));
            }),
        ],
    );
}
