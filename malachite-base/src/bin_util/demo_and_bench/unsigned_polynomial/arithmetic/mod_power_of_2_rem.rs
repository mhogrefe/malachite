// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{CheckedModPowerOf2Rem, ModPowerOf2Rem, ModPowerOf2RemAssign};
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
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_rem);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_rem_ref_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_rem_assign);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_checked_mod_power_of_2_rem);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_rem_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_rem_evaluation_strategy
    );
}

fn show_checked<T: PrimitiveUnsigned>(x: Option<UnsignedPolynomial<T>>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_unsigned_polynomial_mod_power_of_2_rem<T: PrimitiveUnsigned>(
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
            "({p_old}).mod_power_of_2_rem({q_old}, {pow}) = {}",
            p.mod_power_of_2_rem(q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_rem_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_rem(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_rem(&q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_rem_assign<T: PrimitiveUnsigned>(
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
        p.mod_power_of_2_rem_assign(q, pow);
        println!("p := {p_old}; p.mod_power_of_2_rem_assign({q_old}, {pow}); p = {p}");
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_rem<T: PrimitiveUnsigned>(
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
            "({p_old}).checked_mod_power_of_2_rem({q_old}, {pow}) = {}",
            show_checked(p.checked_mod_power_of_2_rem(q, pow))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_rem_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_rem(&({q}), {pow}) = {}",
            show_checked((&p).checked_mod_power_of_2_rem(&q, pow))
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_rem_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_rem(UnsignedPolynomial<{}>, {})",
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
            ("p.mod_power_of_2_rem(q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_rem(q, pow));
            }),
            ("p.mod_power_of_2_rem(&q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_rem(&q, pow));
            }),
            ("(&p).mod_power_of_2_rem(q, pow)", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_rem(q, pow));
            }),
            ("(&p).mod_power_of_2_rem(&q, pow)", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_rem(&q, pow));
            }),
            ("p.mod_power_of_2_rem_assign(q, pow)", &mut |(
                mut p,
                q,
                pow,
            )| {
                no_out!(p.mod_power_of_2_rem_assign(q, pow));
            }),
            ("p.mod_power_of_2_rem_assign(&q, pow)", &mut |(
                mut p,
                q,
                pow,
            )| {
                no_out!(p.mod_power_of_2_rem_assign(&q, pow));
            }),
        ],
    );
}
