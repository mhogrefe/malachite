// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModCompose, ModComposeAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_compose::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_compose);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_compose_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_compose_assign);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_compose_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_unsigned_polynomial_mod_compose_algorithms);
}

fn demo_unsigned_polynomial_mod_compose<T: PrimitiveUnsigned>(
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
            "({p_old}).mod_compose({q_old}, {m}) = {}",
            p.mod_compose(q, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_compose_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_compose(&({q}), {m}) = {}",
            (&p).mod_compose(&q, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_compose_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_compose_assign(&q, m);
        println!("p := {p_old}; p.mod_compose_assign(&({q}), {m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_compose_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_compose(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("p.mod_compose(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_compose(q, m));
            }),
            ("p.mod_compose(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_compose(&q, m));
            }),
            ("(&p).mod_compose(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_compose(q, m));
            }),
            ("(&p).mod_compose(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_compose(&q, m));
            }),
            ("p.mod_compose_assign(q, m)", &mut |(mut p, q, m)| {
                p.mod_compose_assign(q, m);
            }),
            ("p.mod_compose_assign(&q, m)", &mut |(mut p, q, m)| {
                p.mod_compose_assign(&q, m);
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_compose_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_compose(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_compose(&q, m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_compose_naive(&p, &q, m));
            }),
        ],
    );
}
