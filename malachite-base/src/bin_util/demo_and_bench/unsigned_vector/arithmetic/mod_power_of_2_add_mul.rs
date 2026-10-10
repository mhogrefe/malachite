// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMul, ModPowerOf2AddMulAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_add_mul);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_add_mul_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_add_mul_assign);

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_add_mul_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_power_of_2_add_mul<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, pow) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        println!(
            "{u_old}.mod_power_of_2_add_mul({v_old}, {c}, {pow}) = {}",
            u.mod_power_of_2_add_mul(v, c, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_add_mul_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, pow) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&{u}).mod_power_of_2_add_mul(&{v}, {c}, {pow}) = {}",
            (&u).mod_power_of_2_add_mul(&v, c, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_add_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut u, v, c, pow) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        u.mod_power_of_2_add_mul_assign(&v, c, pow);
        println!("u := {u_old}; u.mod_power_of_2_add_mul_assign(&{v}, {c}, {pow}); u = {u}");
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_add_mul_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_power_of_2_add_mul(UnsignedVector<{}>, {}, u64)",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_vector_dimension_bucketer("u"),
        &mut [
            (
                "UnsignedVector.mod_power_of_2_add_mul(UnsignedVector, T, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!(u.mod_power_of_2_add_mul(v, c, pow));
                },
            ),
            (
                "UnsignedVector.mod_power_of_2_add_mul(&UnsignedVector, T, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!(u.mod_power_of_2_add_mul(&v, c, pow));
                },
            ),
            (
                "(&UnsignedVector).mod_power_of_2_add_mul(&UnsignedVector, T, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!((&u).mod_power_of_2_add_mul(&v, c, pow));
                },
            ),
            (
                "UnsignedVector.mod_power_of_2_add_mul_assign(UnsignedVector, T, u64)",
                &mut |(mut u, v, c, pow)| {
                    u.mod_power_of_2_add_mul_assign(v, c, pow);
                },
            ),
            (
                "UnsignedVector.mod_power_of_2_add_mul_assign(&UnsignedVector, T, u64)",
                &mut |(mut u, v, c, pow)| {
                    u.mod_power_of_2_add_mul_assign(&v, c, pow);
                },
            ),
        ],
    );
}
