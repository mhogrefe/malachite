// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    CheckedModPowerOf2Quotient, ModPowerOf2DivRem, ModPowerOf2Quotient, ModPowerOf2QuotientAssign,
};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_quotient);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_quotient_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_quotient_assign
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_quotient
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_quotient_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_quotient_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_quotient_algorithms
    );
}

fn show_checked<T: PrimitiveUnsigned>(x: Option<UnsignedPolynomial<T>>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_unsigned_polynomial_mod_power_of_2_quotient<T: PrimitiveUnsigned>(
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
        println!(
            "({p_old}).mod_power_of_2_quotient({q_old}, {pow}) = {}",
            p.mod_power_of_2_quotient(q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_quotient_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_quotient(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_quotient(&q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_quotient_assign<T: PrimitiveUnsigned>(
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
        p.mod_power_of_2_quotient_assign(q, pow);
        println!("p := {p_old}; p.mod_power_of_2_quotient_assign({q_old}, {pow}); p = {p}");
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_quotient<T: PrimitiveUnsigned>(
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
            "({p_old}).checked_mod_power_of_2_quotient({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_quotient(q, pow))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_quotient_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_quotient(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_quotient(&q, pow))
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_quotient_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_quotient(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_power_of_2_quotient(q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_quotient(q, pow));
            }),
            ("p.mod_power_of_2_quotient(&q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_quotient(&q, pow));
            }),
            ("(&p).mod_power_of_2_quotient(q, pow)", &mut |(
                p,
                q,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_quotient(q, pow));
            }),
            (
                "(&p).mod_power_of_2_quotient(&q, pow)",
                &mut |(p, q, pow)| {
                    no_out!((&p).mod_power_of_2_quotient(&q, pow));
                },
            ),
            (
                "p.mod_power_of_2_quotient_assign(q, pow)",
                &mut |(mut p, q, pow)| {
                    no_out!(p.mod_power_of_2_quotient_assign(q, pow));
                },
            ),
            (
                "p.mod_power_of_2_quotient_assign(&q, pow)",
                &mut |(mut p, q, pow)| {
                    no_out!(p.mod_power_of_2_quotient_assign(&q, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_quotient_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_quotient(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_quotient(&q, pow));
            }),
            ("using div_rem", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_div_rem(&q, pow).0);
            }),
        ],
    );
}
