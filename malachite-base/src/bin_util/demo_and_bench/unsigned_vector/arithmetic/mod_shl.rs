// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_4;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_mod_shl);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_mod_shl_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_mod_shl_assign);

    register_unsigned_unsigned_benches!(runner, benchmark_unsigned_vector_mod_shl_algorithms);
}

fn demo_unsigned_vector_mod_shl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedVector<T>: ModShl<U, T, Output = UnsignedVector<T>>,
{
    for (v, bits, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.mod_shl({bits}, {m}) = {}", v.mod_shl(bits, m));
    }
}

fn demo_unsigned_vector_mod_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedVector<T>: ModShl<U, T, Output = UnsignedVector<T>>,
{
    for (v, bits, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_shl({bits}, {m}) = {}", (&v).mod_shl(bits, m));
    }
}

fn demo_unsigned_vector_mod_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedVector<T>: ModShlAssign<U, T>,
{
    for (mut v, bits, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_4::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_shl_assign(bits, m);
        println!("v := {v_old}; v.mod_shl_assign({bits}, {m}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_shl_algorithms<
    T: ModShl<U, T, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedVector<T>: ModShl<U, T, Output = UnsignedVector<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_shl({}, {})",
            T::NAME,
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_unsigned_triple_gen_var_4::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits, m)| {
                no_out!(v.mod_shl(bits, m));
            }),
            ("element by element", &mut |(v, bits, m)| {
                no_out!(UnsignedVector {
                    elements: v
                        .elements
                        .into_iter()
                        .map(|x| x.mod_shl(bits, m))
                        .collect::<Vec<T>>()
                });
            }),
        ],
    );
}
