// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    CheckedModPowerOf2DivTruncated, ModPowerOf2DivTruncated, ModPowerOf2DivTruncatedAssign,
};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8,
    unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_10,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_div_truncated
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_div_truncated_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_div_truncated_assign
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_div_truncated
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_checked_mod_power_of_2_div_truncated_ref_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_div_truncated_long
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_div_truncated_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_div_truncated_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_div_truncated_algorithms_long
    );
}

fn show<T: PrimitiveUnsigned>(p: Option<UnsignedPolynomial<T>>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_unsigned_polynomial_mod_power_of_2_div_truncated<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_power_of_2_div_truncated({q_old}, {len}, {pow}) = {}",
            p.mod_power_of_2_div_truncated(q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_div_truncated_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_div_truncated(&({q}), {len}, {pow}) = {}",
            (&p).mod_power_of_2_div_truncated(&q, len, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_div_truncated_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, q, len, pow) in
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8::<T>()
            .get(gm, config)
            .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_power_of_2_div_truncated_assign(q, len, pow);
        println!(
            "p := {p_old}; p.mod_power_of_2_div_truncated_assign({q_old}, {len}, {pow}); p = {p}"
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_div_truncated<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_mod_power_of_2_div_truncated({q_old}, {len}, {pow}) = {}",
            show(p.checked_mod_power_of_2_div_truncated(q, len, pow))
        );
    }
}

fn demo_unsigned_polynomial_checked_mod_power_of_2_div_truncated_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_power_of_2_div_truncated(&({q}), {len}, {pow}) = {}",
            show((&p).checked_mod_power_of_2_div_truncated(&q, len, pow))
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_div_truncated_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, pow) in unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_div_truncated(&({q}), {len}, {pow}) = {}",
            (&p).mod_power_of_2_div_truncated(&q, len, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_div_truncated_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_div_truncated(UnsignedPolynomial<{}>, u64, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            (
                "p.mod_power_of_2_div_truncated(q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!(p.mod_power_of_2_div_truncated(q, len, pow));
                },
            ),
            (
                "p.mod_power_of_2_div_truncated(&q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!(p.mod_power_of_2_div_truncated(&q, len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_div_truncated(q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!((&p).mod_power_of_2_div_truncated(q, len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_div_truncated(&q, len, pow)",
                &mut |(p, q, len, pow)| {
                    no_out!((&p).mod_power_of_2_div_truncated(&q, len, pow));
                },
            ),
            (
                "p.mod_power_of_2_div_truncated_assign(q, len, pow)",
                &mut |(mut p, q, len, pow)| {
                    p.mod_power_of_2_div_truncated_assign(q, len, pow);
                },
            ),
            (
                "p.mod_power_of_2_div_truncated_assign(&q, len, pow)",
                &mut |(mut p, q, len, pow)| {
                    p.mod_power_of_2_div_truncated_assign(&q, len, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_div_truncated_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_div_truncated(UnsignedPolynomial<{}>, u64, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_8::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, pow)| {
                no_out!((&p).mod_power_of_2_div_truncated(&q, len, pow));
            }),
            ("naive", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_div_truncated_naive(&p, &q, len, pow));
            }),
            ("basecase", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_div_truncated_basecase_wrapper(
                    &p, &q, len, pow
                ));
            }),
            ("Karp-Markstein", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_div_truncated_newton_wrapper(
                    &p, &q, len, pow
                ));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_div_truncated_algorithms_long<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_div_truncated(UnsignedPolynomial<{}>, u64, {})",
            T::NAME,
            T::NAME,
            "u64"
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_pair_unsigned_unsigned_quadruple_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, pow)| {
                no_out!((&p).mod_power_of_2_div_truncated(&q, len, pow));
            }),
            ("basecase", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_div_truncated_basecase_wrapper(
                    &p, &q, len, pow
                ));
            }),
            ("Karp-Markstein", &mut |(p, q, len, pow)| {
                no_out!(mod_power_of_2_div_truncated_newton_wrapper(
                    &p, &q, len, pow
                ));
            }),
        ],
    );
}
