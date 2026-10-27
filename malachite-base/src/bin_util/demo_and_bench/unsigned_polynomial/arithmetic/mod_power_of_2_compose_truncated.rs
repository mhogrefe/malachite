// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModPowerOf2ComposeTruncated, ModPowerOf2ComposeTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_4,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compose_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated_val_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated_ref_val
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated_assign
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated_assign_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_compose_truncated_long
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_truncated_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_truncated_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_compose_truncated_algorithms_long
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_compose_truncated({q_old}, {len}, {pow}) = {}",
            p.mod_power_of_2_compose_truncated(q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated_val_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_compose_truncated(&({q}), {len}, {pow}) = {}",
            p.mod_power_of_2_compose_truncated(&q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated_ref_val<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!(
            "(&({p})).mod_power_of_2_compose_truncated({q_old}, {len}, {pow}) = {}",
            (&p).mod_power_of_2_compose_truncated(q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_compose_truncated(&({q}), {len}, {pow}) = {}",
            (&p).mod_power_of_2_compose_truncated(&q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, pow) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_compose_truncated_assign(q, len, pow);
        println!(
            "p := {p_old}; p.mod_power_of_2_compose_truncated_assign({q_old}, {len}, {pow}); p = \
            {p}"
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated_assign_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, pow) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();

        p.mod_power_of_2_compose_truncated_assign(&q, len, pow);
        println!(
            "p := {p_old}; p.mod_power_of_2_compose_truncated_assign(&({q}), {len}, {pow}); p = {p}"
        );
    }
}

// Lengths up to 1000, so that all the algorithms are reached.
fn demo_unsigned_polynomial_mod_power_of_2_compose_truncated_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_compose_truncated(&({q}), {len}, {pow}) = {}",
            (&p).mod_power_of_2_compose_truncated(&q, len, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_truncated_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose_truncated(UnsignedPolynomial<{}>, u64, \
            u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            (
                "p.mod_power_of_2_compose_truncated(q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!(p.mod_power_of_2_compose_truncated(q, len, pow));
                },
            ),
            (
                "p.mod_power_of_2_compose_truncated(&q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!(p.mod_power_of_2_compose_truncated(&q, len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_compose_truncated(q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!((&p).mod_power_of_2_compose_truncated(q, len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_compose_truncated(&q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!((&p).mod_power_of_2_compose_truncated(&q, len, pow));
                },
            ),
            (
                "p.mod_power_of_2_compose_truncated_assign(q, len, pow)",
                &mut |(mut p, q, len, pow)| {
                    p.mod_power_of_2_compose_truncated_assign(q, len, pow);
                },
            ),
            (
                "p.mod_power_of_2_compose_truncated_assign(&q, len, pow)",
                &mut |(mut p, q, len, pow)| {
                    p.mod_power_of_2_compose_truncated_assign(&q, len, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_truncated_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose_truncated(UnsignedPolynomial<{}>, u64, \
            u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, pow)| {
                no_out!((&p).mod_power_of_2_compose_truncated(&q, len, pow));
            }),
            ("naive", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_naive(&p, &q, len, pow));
            }),
            ("Horner", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_horner_wrapper(
                    &p, &q, len, pow
                ));
            }),
            ("divide and conquer", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_divide_and_conquer_wrapper(
                    &p, &q, len, pow
                ));
            }),
            ("Brent-Kung", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_brent_kung_wrapper(
                    &p, &q, len, pow
                ));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_compose_truncated_algorithms_long<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_compose_truncated(UnsignedPolynomial<{}>, u64, \
            u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, pow)| {
                no_out!((&p).mod_power_of_2_compose_truncated(&q, len, pow));
            }),
            ("naive", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_naive(&p, &q, len, pow));
            }),
            ("Horner", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_horner_wrapper(
                    &p, &q, len, pow
                ));
            }),
            ("divide and conquer", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_divide_and_conquer_wrapper(
                    &p, &q, len, pow
                ));
            }),
            ("Brent-Kung", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_compose_truncated_brent_kung_wrapper(
                    &p, &q, len, pow
                ));
            }),
        ],
    );
}
