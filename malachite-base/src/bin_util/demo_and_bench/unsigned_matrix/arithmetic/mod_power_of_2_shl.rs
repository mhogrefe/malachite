// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_mod_power_of_2_shl);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_mod_power_of_2_shl_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_mod_power_of_2_shl_assign);

    register_unsigned_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_shl_evaluation_strategy
    );
}

fn demo_unsigned_matrix_mod_power_of_2_shl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedMatrix<T>: ModPowerOf2Shl<U, Output = UnsignedMatrix<T>>,
{
    for (a, bits, pow) in unsigned_matrix_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_power_of_2_shl({bits}, {pow}) = {}",
            a.mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedMatrix<T>: ModPowerOf2Shl<U, Output = UnsignedMatrix<T>>,
{
    for (a, bits, pow) in unsigned_matrix_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_shl({bits}, {pow}) = {}",
            (&a).mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedMatrix<T>: ModPowerOf2ShlAssign<U>,
{
    for (mut a, bits, pow) in unsigned_matrix_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_shl_assign(bits, pow);
        println!("a := {a_old}; a.mod_power_of_2_shl_assign({bits}, {pow}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_shl_evaluation_strategy<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedMatrix<T>: ModPowerOf2Shl<U, Output = UnsignedMatrix<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedMatrix<T>: ModPowerOf2Shl<U, Output = UnsignedMatrix<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedMatrix<{}>.mod_power_of_2_shl({}, u64)",
            T::NAME,
            U::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_shl(U, u64)",
                &mut |(a, bits, pow)| {
                    no_out!(a.mod_power_of_2_shl(bits, pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_shl(U, u64)",
                &mut |(a, bits, pow)| {
                    no_out!((&a).mod_power_of_2_shl(bits, pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_shl_assign(U, u64)",
                &mut |(mut a, bits, pow)| {
                    a.mod_power_of_2_shl_assign(bits, pow);
                },
            ),
        ],
    );
}
