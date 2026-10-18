// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_unsigned_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_matrix_mod_power_of_2_shl);
    register_unsigned_demos!(runner, demo_natural_matrix_mod_power_of_2_shl_ref);
    register_unsigned_demos!(runner, demo_natural_matrix_mod_power_of_2_shl_assign);

    register_unsigned_benches!(
        runner,
        benchmark_natural_matrix_mod_power_of_2_shl_algorithms
    );
}

fn demo_natural_matrix_mod_power_of_2_shl<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: ModPowerOf2Shl<T, Output = NaturalMatrix>,
{
    for (a, bits, pow) in natural_matrix_unsigned_unsigned_triple_gen_var_2::<T>()
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

fn demo_natural_matrix_mod_power_of_2_shl_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalMatrix: ModPowerOf2Shl<T, Output = NaturalMatrix>,
{
    for (a, bits, pow) in natural_matrix_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_power_of_2_shl({bits}, {pow}) = {}",
            (&a).mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_natural_matrix_mod_power_of_2_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: ModPowerOf2ShlAssign<T>,
{
    for (mut a, bits, pow) in natural_matrix_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_shl_assign(bits, pow);
        println!("a := {a_old}; a.mod_power_of_2_shl_assign({bits}, {pow}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_power_of_2_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: ModPowerOf2Shl<T, Output = NaturalMatrix> + Shl<T, Output = NaturalMatrix>,
{
    run_benchmark(
        &format!("NaturalMatrix.mod_power_of_2_shl({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits, pow)| {
                no_out!(a.mod_power_of_2_shl(bits, pow));
            }),
            ("shift, then reduce", &mut |(a, bits, pow)| {
                no_out!((a << bits).mod_power_of_2(pow));
            }),
        ],
    );
}
