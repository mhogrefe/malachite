// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_3;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_shl);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_shl_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_mod_power_of_2_shl_assign);

    register_unsigned_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_power_of_2_shl_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_power_of_2_shl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedVector<T>: ModPowerOf2Shl<U, Output = UnsignedVector<T>>,
{
    for (v, bits, pow) in unsigned_vector_unsigned_unsigned_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.mod_power_of_2_shl({bits}, {pow}) = {}",
            v.mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedVector<T>: ModPowerOf2Shl<U, Output = UnsignedVector<T>>,
{
    for (v, bits, pow) in unsigned_vector_unsigned_unsigned_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_shl({bits}, {pow}) = {}",
            (&v).mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_unsigned_vector_mod_power_of_2_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedVector<T>: ModPowerOf2ShlAssign<U>,
{
    for (mut v, bits, pow) in unsigned_vector_unsigned_unsigned_triple_gen_var_3::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_shl_assign(bits, pow);
        println!("v := {v_old}; v.mod_power_of_2_shl_assign({bits}, {pow}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_power_of_2_shl_evaluation_strategy<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedVector<T>: ModPowerOf2Shl<U, Output = UnsignedVector<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedVector<T>: ModPowerOf2Shl<U, Output = UnsignedVector<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_power_of_2_shl({}, u64)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_unsigned_triple_gen_var_3::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            (
                "UnsignedVector.mod_power_of_2_shl(U, u64)",
                &mut |(v, bits, pow)| {
                    no_out!(v.mod_power_of_2_shl(bits, pow));
                },
            ),
            (
                "(&UnsignedVector).mod_power_of_2_shl(U, u64)",
                &mut |(v, bits, pow)| {
                    no_out!((&v).mod_power_of_2_shl(bits, pow));
                },
            ),
            (
                "UnsignedVector.mod_power_of_2_shl_assign(U, u64)",
                &mut |(mut v, bits, pow)| {
                    v.mod_power_of_2_shl_assign(bits, pow);
                },
            ),
        ],
    );
}
