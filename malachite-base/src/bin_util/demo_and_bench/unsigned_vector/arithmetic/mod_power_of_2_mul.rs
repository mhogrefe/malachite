// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_mul);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_mul_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_mul_assign);

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_mul_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_power_of_2_mul<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, pow) in unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.mod_power_of_2_mul({c}, {pow}) = {}",
            v.mod_power_of_2_mul(c, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_mul_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, pow) in unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_mul({c}, {pow}) = {}",
            (&v).mod_power_of_2_mul(c, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut v, c, pow) in unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_mul_assign(c, pow);
        println!("v := {v_old}; v.mod_power_of_2_mul_assign({c}, {pow}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_mul_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_power_of_2_mul({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector.mod_power_of_2_mul(T, u64)",
                &mut |(v, c, pow)| {
                    no_out!(v.mod_power_of_2_mul(c, pow));
                },
            ),
            (
                "(&UnsignedVector).mod_power_of_2_mul(T, u64)",
                &mut |(v, c, pow)| {
                    no_out!((&v).mod_power_of_2_mul(c, pow));
                },
            ),
            (
                "UnsignedVector.mod_power_of_2_mul_assign(T, u64)",
                &mut |(mut v, c, pow)| {
                    v.mod_power_of_2_mul_assign(c, pow);
                },
            ),
        ],
    );
}
