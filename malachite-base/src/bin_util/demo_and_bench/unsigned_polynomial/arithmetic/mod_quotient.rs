// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{CheckedModQuotient, ModDivRem, ModQuotient, ModQuotientAssign};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_quotient);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_quotient_ref_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_quotient_assign);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_checked_mod_quotient);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_quotient_ref_ref
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_quotient_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_quotient_algorithms
    );
}

fn show_checked<T: PrimitiveUnsigned>(x: Option<UnsignedPolynomial<T>>) -> String {
    x.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_unsigned_polynomial_mod_quotient<T: PrimitiveUnsigned>(
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
        println!(
            "({p_old}).mod_quotient({q_old}, {m}) = {}",
            p.mod_quotient(q, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_quotient_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_quotient(&({q}), {m}) = {}",
            (&p).mod_quotient(&q, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_quotient_assign<T: PrimitiveUnsigned>(
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
        p.mod_quotient_assign(q, m);
        println!("p := {p_old}; p.mod_quotient_assign({q_old}, {m}); p = {p}");
    }
}

fn demo_unsigned_polynomial_checked_mod_quotient<T: PrimitiveUnsigned>(
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
            "({p_old}).checked_mod_quotient({q_old}, {m}) = {}",
            show_checked(p.checked_mod_quotient(q, m))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_quotient_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_quotient(&({q}), {m}) = {}",
            show_checked((&p).checked_mod_quotient(&q, m))
        );
    }
}

fn benchmark_unsigned_polynomial_mod_quotient_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_quotient(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_quotient(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_quotient(q, m));
            }),
            ("p.mod_quotient(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_quotient(&q, m));
            }),
            ("(&p).mod_quotient(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(q, m));
            }),
            ("(&p).mod_quotient(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(&q, m));
            }),
            ("p.mod_quotient_assign(q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_quotient_assign(q, m));
            }),
            ("p.mod_quotient_assign(&q, m)", &mut |(mut p, q, m)| {
                no_out!(p.mod_quotient_assign(&q, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_quotient_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_quotient(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!((&p).mod_quotient(&q, m));
            }),
            ("using div_rem", &mut |(p, q, m)| {
                no_out!((&p).mod_div_rem(&q, m).0);
            }),
        ],
    );
}
