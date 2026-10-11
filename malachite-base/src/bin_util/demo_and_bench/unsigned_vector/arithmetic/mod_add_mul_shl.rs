// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAddMulShl, ModAddMulShlAssign, ModShl};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quintuple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_add_mul_shl);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_add_mul_shl_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_add_mul_shl_assign);

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_mod_add_mul_shl_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_add_mul_shl<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, bits, m) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_2::<T>()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        println!(
            "{u_old}.mod_add_mul_shl({v_old}, {c}, {bits}, {m}) = {}",
            u.mod_add_mul_shl(v, c, bits, m)
        );
    }
}

fn demo_unsigned_vector_mod_add_mul_shl_ref<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, bits, m) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_2::<T>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&{u}).mod_add_mul_shl(&{v}, {c}, {bits}, {m}) = {}",
            (&u).mod_add_mul_shl(&v, c, bits, m)
        );
    }
}

fn demo_unsigned_vector_mod_add_mul_shl_assign<
    T: PrimitiveUnsigned + ModShl<u64, T, Output = T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut u, v, c, bits, m) in
        unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_2::<T>()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        u.mod_add_mul_shl_assign(&v, c, bits, m);
        println!("u := {u_old}; u.mod_add_mul_shl_assign(&{v}, {c}, {bits}, {m}); u = {u}");
    }
}

fn benchmark_unsigned_vector_mod_add_mul_shl_evaluation_strategy<
    T: PrimitiveUnsigned + ModShl<u64, T, Output = T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_add_mul_shl(UnsignedVector<{}>, {}, u64, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_2::<T>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_unsigned_vector_dimension_bucketer("u"),
        &mut [
            (
                "UnsignedVector.mod_add_mul_shl(UnsignedVector, T, u64, T)",
                &mut |(u, v, c, bits, m)| {
                    no_out!(u.mod_add_mul_shl(v, c, bits, m));
                },
            ),
            (
                "UnsignedVector.mod_add_mul_shl(&UnsignedVector, T, u64, T)",
                &mut |(u, v, c, bits, m)| {
                    no_out!(u.mod_add_mul_shl(&v, c, bits, m));
                },
            ),
            (
                "(&UnsignedVector).mod_add_mul_shl(&UnsignedVector, T, u64, T)",
                &mut |(u, v, c, bits, m)| {
                    no_out!((&u).mod_add_mul_shl(&v, c, bits, m));
                },
            ),
            (
                "UnsignedVector.mod_add_mul_shl_assign(UnsignedVector, T, u64, T)",
                &mut |(mut u, v, c, bits, m)| {
                    u.mod_add_mul_shl_assign(v, c, bits, m);
                },
            ),
            (
                "UnsignedVector.mod_add_mul_shl_assign(&UnsignedVector, T, u64, T)",
                &mut |(mut u, v, c, bits, m)| {
                    u.mod_add_mul_shl_assign(&v, c, bits, m);
                },
            ),
        ],
    );
}
