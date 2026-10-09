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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_unsigned_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_vector_mod_power_of_2_shl);
    register_unsigned_demos!(runner, demo_natural_vector_mod_power_of_2_shl_ref);
    register_unsigned_demos!(runner, demo_natural_vector_mod_power_of_2_shl_assign);

    register_unsigned_benches!(
        runner,
        benchmark_natural_vector_mod_power_of_2_shl_algorithms
    );
}

fn demo_natural_vector_mod_power_of_2_shl<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: ModPowerOf2Shl<T, Output = NaturalVector>,
{
    for (v, bits, pow) in natural_vector_unsigned_unsigned_triple_gen_var_1::<T>()
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

fn demo_natural_vector_mod_power_of_2_shl_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalVector: ModPowerOf2Shl<T, Output = NaturalVector>,
{
    for (v, bits, pow) in natural_vector_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_shl({bits}, {pow}) = {}",
            (&v).mod_power_of_2_shl(bits, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: ModPowerOf2ShlAssign<T>,
{
    for (mut v, bits, pow) in natural_vector_unsigned_unsigned_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_shl_assign(bits, pow);
        println!("v := {v_old}; v.mod_power_of_2_shl_assign({bits}, {pow}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_power_of_2_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalVector: ModPowerOf2Shl<T, Output = NaturalVector> + Shl<T, Output = NaturalVector>,
{
    run_benchmark(
        &format!("NaturalVector.mod_power_of_2_shl({}, u64)", T::NAME),
        BenchmarkType::Algorithms,
        natural_vector_unsigned_unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits, pow)| {
                no_out!(v.mod_power_of_2_shl(bits, pow));
            }),
            ("shift, then reduce", &mut |(v, bits, pow)| {
                no_out!((v << bits).mod_power_of_2(pow));
            }),
        ],
    );
}
