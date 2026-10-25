// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModSubMul, ModSubMulAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_matrix_mod_sub_mul);
    register_unsigned_demos!(runner, demo_unsigned_matrix_mod_sub_mul_ref);
    register_unsigned_demos!(runner, demo_unsigned_matrix_mod_sub_mul_assign);

    register_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_mod_sub_mul_evaluation_strategy
    );
}

fn demo_unsigned_matrix_mod_sub_mul<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, m) in unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        println!(
            "{u_old}.mod_sub_mul({v_old}, {c}, {m}) = {}",
            u.mod_sub_mul(v, c, m)
        );
    }
}

fn demo_unsigned_matrix_mod_sub_mul_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, m) in unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{u}).mod_sub_mul(&{v}, {c}, {m}) = {}",
            (&u).mod_sub_mul(&v, c, m)
        );
    }
}

fn demo_unsigned_matrix_mod_sub_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut u, v, c, m) in
        unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_3::<T>()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        u.mod_sub_mul_assign(&v, c, m);
        println!("u := {u_old}; u.mod_sub_mul_assign(&{v}, {c}, {m}); u = {u}");
    }
}

fn benchmark_unsigned_matrix_mod_sub_mul_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedMatrix<{}>.mod_sub_mul(UnsignedMatrix<{}>, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_3::<T>()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_matrix_size_bucketer("u"),
        &mut [
            (
                "UnsignedMatrix.mod_sub_mul(UnsignedMatrix, T, T)",
                &mut |(u, v, c, m)| {
                    no_out!(u.mod_sub_mul(v, c, m));
                },
            ),
            (
                "UnsignedMatrix.mod_sub_mul(&UnsignedMatrix, T, T)",
                &mut |(u, v, c, m)| {
                    no_out!(u.mod_sub_mul(&v, c, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_sub_mul(&UnsignedMatrix, T, T)",
                &mut |(u, v, c, m)| {
                    no_out!((&u).mod_sub_mul(&v, c, m));
                },
            ),
            (
                "UnsignedMatrix.mod_sub_mul_assign(UnsignedMatrix, T, T)",
                &mut |(mut u, v, c, m)| {
                    u.mod_sub_mul_assign(v, c, m);
                },
            ),
            (
                "UnsignedMatrix.mod_sub_mul_assign(&UnsignedMatrix, T, T)",
                &mut |(mut u, v, c, m)| {
                    u.mod_sub_mul_assign(&v, c, m);
                },
            ),
        ],
    );
}
