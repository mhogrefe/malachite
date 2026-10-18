// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_matrix::UnsignedMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_mod_shl);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_mod_shl_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_matrix_mod_shl_assign);

    register_unsigned_unsigned_benches!(runner, benchmark_unsigned_matrix_mod_shl_algorithms);
}

fn demo_unsigned_matrix_mod_shl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedMatrix<T>: ModShl<U, T, Output = UnsignedMatrix<T>>,
{
    for (a, bits, m) in unsigned_matrix_unsigned_unsigned_triple_gen_var_5::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_shl({bits}, {m}) = {}", a.mod_shl(bits, m));
    }
}

fn demo_unsigned_matrix_mod_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedMatrix<T>: ModShl<U, T, Output = UnsignedMatrix<T>>,
{
    for (a, bits, m) in unsigned_matrix_unsigned_unsigned_triple_gen_var_5::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_shl({bits}, {m}) = {}", (&a).mod_shl(bits, m));
    }
}

fn demo_unsigned_matrix_mod_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedMatrix<T>: ModShlAssign<U, T>,
{
    for (mut a, bits, m) in unsigned_matrix_unsigned_unsigned_triple_gen_var_5::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_shl_assign(bits, m);
        println!("a := {a_old}; a.mod_shl_assign({bits}, {m}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_shl_algorithms<
    T: ModShl<U, T, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedMatrix<T>: ModShl<U, T, Output = UnsignedMatrix<T>>,
{
    run_benchmark(
        &format!(
            "UnsignedMatrix<{}>.mod_shl({}, {})",
            T::NAME,
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_unsigned_triple_gen_var_5::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits, m)| {
                no_out!(a.mod_shl(bits, m));
            }),
            ("element by element", &mut |(a, bits, m)| {
                let (height, width) = (a.height(), a.width());
                no_out!(UnsignedMatrix::from_owned_elements(
                    height,
                    width,
                    a.elements
                        .into_iter()
                        .map(|x| x.mod_shl(bits, m))
                        .collect::<Vec<T>>()
                ));
            }),
        ],
    );
}
