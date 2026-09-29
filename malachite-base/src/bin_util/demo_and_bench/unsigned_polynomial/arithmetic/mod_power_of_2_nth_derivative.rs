// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModPowerOf2NthDerivative, ModPowerOf2NthDerivativeAssign};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_6;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_nth_derivative::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_nth_derivative
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_nth_derivative_ref
    );
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_nth_derivative_assign
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_nth_derivative_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_nth_derivative_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_nth_derivative<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, n, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_nth_derivative({n}, {pow}) = {}",
            p.mod_power_of_2_nth_derivative(n, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_nth_derivative_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, n, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_nth_derivative({n}, {pow}) = {}",
            (&p).mod_power_of_2_nth_derivative(n, pow)
        );
    }
}

fn demo_unsigned_polynomial_mod_power_of_2_nth_derivative_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, n, pow) in unsigned_polynomial_unsigned_unsigned_triple_gen_var_6::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_power_of_2_nth_derivative_assign(n, pow);
        println!("p := {p_old}; p.mod_power_of_2_nth_derivative_assign({n}, {pow}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_nth_derivative_evaluation_strategy<
    T: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial.mod_power_of_2_nth_derivative(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            (
                "p.mod_power_of_2_nth_derivative(n, pow)",
                &mut |(p, n, pow)| {
                    no_out!(p.mod_power_of_2_nth_derivative(n, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_nth_derivative(n, pow)",
                &mut |(p, n, pow)| {
                    no_out!((&p).mod_power_of_2_nth_derivative(n, pow));
                },
            ),
            (
                "p.mod_power_of_2_nth_derivative_assign(n, pow)",
                &mut |(mut p, n, pow)| {
                    p.mod_power_of_2_nth_derivative_assign(n, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_power_of_2_nth_derivative_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial.mod_power_of_2_nth_derivative(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_unsigned_triple_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, n, pow)| {
                no_out!((&p).mod_power_of_2_nth_derivative(n, pow));
            }),
            ("naive", &mut |(p, n, pow)| {
                no_out!(mod_power_of_2_nth_derivative_naive(&p, n, pow));
            }),
        ],
    );
}
