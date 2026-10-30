// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModRemovePower, ModRemovePowerAssign};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_remove_power::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_remove_power);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_remove_power_ref_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_remove_power_assign);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_remove_power_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_remove_power_algorithms
    );
}

fn demo_unsigned_polynomial_mod_remove_power<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let (s, e) = p.mod_remove_power(q, m);
        println!("({p_old}).mod_remove_power({q_old}, {m}) = ({s}, {e})");
    }
}

fn demo_unsigned_polynomial_mod_remove_power_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (s, e) = (&p).mod_remove_power(&q, m);
        println!("(&({p})).mod_remove_power(&({q}), {m}) = ({s}, {e})");
    }
}

fn demo_unsigned_polynomial_mod_remove_power_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let e = p.mod_remove_power_assign(q, m);
        println!("p := {p_old}; p.mod_remove_power_assign({q_old}, {m}) = {e}; p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_remove_power_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_remove_power(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_8::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_remove_power(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_remove_power(q, m));
            }),
            ("p.mod_remove_power(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_remove_power(&q, m));
            }),
            ("(&p).mod_remove_power(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_remove_power(q, m));
            }),
            ("(&p).mod_remove_power(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_remove_power(&q, m));
            }),
            ("p.mod_remove_power_assign(q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_remove_power_assign(q, m));
            }),
            ("p.mod_remove_power_assign(&q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_remove_power_assign(&q, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_remove_power_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_remove_power(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_8::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_remove_power(&q, m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_remove_power_naive(&p, &q, m));
            }),
        ],
    );
}
