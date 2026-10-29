// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::ModPowerOf2InverseTruncated;
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_7,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_15,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_inverse_truncated
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_inverse_truncated_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_inverse_truncated_long
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_inverse_truncated_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_inverse_truncated_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_inverse_truncated_algorithms_long
    );
}

fn show<T: PrimitiveUnsigned>(p: Option<UnsignedPolynomial<T>>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_unsigned_polynomial_mod_power_of_2_inverse_truncated<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_inverse_truncated({len}, {pow}) = {}",
            show(p.mod_power_of_2_inverse_truncated(len, pow))
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_inverse_truncated_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_inverse_truncated({len}, {pow}) = {}",
            show((&p).mod_power_of_2_inverse_truncated(len, pow))
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_inverse_truncated_long<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_15::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_inverse_truncated({len}, {pow}) = {}",
            show((&p).mod_power_of_2_inverse_truncated(len, pow))
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_inverse_truncated_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_inverse_truncated(u64, {})",
            T::NAME,
            "u64"
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            (
                "p.mod_power_of_2_inverse_truncated(len, pow)",
                &mut |(p, len, pow)| {
                    no_out!(p.mod_power_of_2_inverse_truncated(len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_inverse_truncated(len, pow)",
                &mut |(p, len, pow)| {
                    no_out!((&p).mod_power_of_2_inverse_truncated(len, pow));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_inverse_truncated_algorithms<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_inverse_truncated(u64, {})",
            T::NAME,
            "u64"
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_7::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            ("default", &mut |(p, len, pow)| {
                no_out!((&p).mod_power_of_2_inverse_truncated(len, pow));
            }),
            ("naive", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_naive(&p, len, pow));
            }),
            ("basecase", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_basecase_wrapper(
                    &p, len, pow
                ));
            }),
            ("Newton", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_newton_wrapper(
                    &p, len, pow
                ));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_inverse_truncated_algorithms_long<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_power_of_2_inverse_truncated(u64, {})",
            T::NAME,
            "u64"
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_15::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            ("default", &mut |(p, len, pow)| {
                no_out!((&p).mod_power_of_2_inverse_truncated(len, pow));
            }),
            ("basecase", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_basecase_wrapper(
                    &p, len, pow
                ));
            }),
            ("Newton", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_newton_wrapper(
                    &p, len, pow
                ));
            }),
        ],
    );
}
