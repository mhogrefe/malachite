// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{CheckedModDivTruncated, ModDivTruncated, ModDivTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_9,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_div_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_div_truncated);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_div_truncated_ref_ref);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_div_truncated_assign);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_checked_mod_div_truncated);
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_div_truncated_ref_ref
    );
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_div_truncated_long);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_div_truncated_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_div_truncated_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_div_truncated_algorithms_long
    );
}

fn show<T: PrimitiveUnsigned>(p: Option<UnsignedPolynomial<T>>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_unsigned_polynomial_mod_div_truncated<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_div_truncated({q_old}, {len}, {m}) = {}",
            p.mod_div_truncated(q, len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_div_truncated_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_div_truncated(&({q}), {len}, {m}) = {}",
            (&p).mod_div_truncated(&q, len, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_div_truncated_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_div_truncated_assign(q, len, m);
        println!("p := {p_old}; p.mod_div_truncated_assign({q_old}, {len}, {m}); p = {p}");
    }
}

fn demo_unsigned_polynomial_checked_mod_div_truncated<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_div_truncated({q_old}, {len}, {m}) = {}",
            show(p.checked_mod_div_truncated(q, len, m))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_div_truncated_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_div_truncated(&({q}), {len}, {m}) = {}",
            show((&p).checked_mod_div_truncated(&q, len, m))
        );
    }
}

fn demo_unsigned_polynomial_mod_div_truncated_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_9::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_div_truncated(&({q}), {len}, {m}) = {}",
            (&p).mod_div_truncated(&q, len, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_div_truncated_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_div_truncated(UnsignedPolynomial<{}>, u64, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("p.mod_div_truncated(q, len, m)", &mut |(p, q, len, m)| {
                no_out!(p.mod_div_truncated(q, len, m));
            }),
            ("p.mod_div_truncated(&q, len, m)", &mut |(p, q, len, m)| {
                no_out!(p.mod_div_truncated(&q, len, m));
            }),
            ("(&p).mod_div_truncated(q, len, m)", &mut |(
                p,
                q,
                len,
                m,
            )| {
                no_out!((&p).mod_div_truncated(q, len, m));
            }),
            ("(&p).mod_div_truncated(&q, len, m)", &mut |(
                p,
                q,
                len,
                m,
            )| {
                no_out!((&p).mod_div_truncated(&q, len, m));
            }),
            (
                "p.mod_div_truncated_assign(q, len, m)",
                &mut |(mut p, q, len, m)| {
                    p.mod_div_truncated_assign(q, len, m);
                },
            ),
            (
                "p.mod_div_truncated_assign(&q, len, m)",
                &mut |(mut p, q, len, m)| {
                    p.mod_div_truncated_assign(&q, len, m);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_div_truncated_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_div_truncated(UnsignedPolynomial<{}>, u64, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_7::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, m)| {
                no_out!((&p).mod_div_truncated(&q, len, m));
            }),
            ("naive", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_naive(&p, &q, len, m));
            }),
            ("basecase", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_basecase_wrapper(&p, &q, len, m));
            }),
            ("Karp-Markstein", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_newton_wrapper(&p, &q, len, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_div_truncated_algorithms_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_div_truncated(UnsignedPolynomial<{}>, u64, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_9::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, m)| {
                no_out!((&p).mod_div_truncated(&q, len, m));
            }),
            ("basecase", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_basecase_wrapper(&p, &q, len, m));
            }),
            ("Karp-Markstein", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_newton_wrapper(&p, &q, len, m));
            }),
        ],
    );
}
