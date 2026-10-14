// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModPowerOf2Compose, ModPowerOf2ComposeAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compose::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_compose);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_compose_ref);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_compose<T: PrimitiveUnsigned>(
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
            "({p_old}).mod_power_of_2_compose({q_old}, {pow}) = {}",
            p.mod_power_of_2_compose(q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_compose(&({q}), {pow}) = {}",
            (&p).mod_power_of_2_compose(&q, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, pow) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_compose_assign(&q, pow);
        println!("p := {p_old}; p.mod_power_of_2_compose_assign(&({q}), {pow}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose(UnsignedPolynomial<{}>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_power_of_2_compose(q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_compose(q, pow));
            }),
            ("p.mod_power_of_2_compose(&q, pow)", &mut |(p, q, pow)| {
                no_out!(p.mod_power_of_2_compose(&q, pow));
            }),
            ("(&p).mod_power_of_2_compose(q, pow)", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_compose(q, pow));
            }),
            ("(&p).mod_power_of_2_compose(&q, pow)", &mut |(
                p,
                q,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_compose(&q, pow));
            }),
            (
                "p.mod_power_of_2_compose_assign(q, pow)",
                &mut |(mut p, q, pow)| {
                    p.mod_power_of_2_compose_assign(q, pow);
                },
            ),
            (
                "p.mod_power_of_2_compose_assign(&q, pow)",
                &mut |(mut p, q, pow)| {
                    p.mod_power_of_2_compose_assign(&q, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose(UnsignedPolynomial<{}>, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, q, pow)| {
                no_out!((&p).mod_power_of_2_compose(&q, pow));
            }),
            ("naive", &mut |(p, q, pow)| {
                no_out!(mod_power_of_2_compose_naive(&p, &q, pow));
            }),
        ],
    );
}
