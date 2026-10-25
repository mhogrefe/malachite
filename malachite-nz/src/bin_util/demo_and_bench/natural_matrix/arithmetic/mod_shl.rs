// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_natural_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_matrix_mod_shl);
    register_unsigned_demos!(runner, demo_natural_matrix_mod_shl_ref);
    register_unsigned_demos!(runner, demo_natural_matrix_mod_shl_assign);

    register_unsigned_benches!(runner, benchmark_natural_matrix_mod_shl_algorithms);
}

fn demo_natural_matrix_mod_shl<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    NaturalMatrix: for<'a> ModShl<T, &'a Natural, Output = NaturalMatrix>,
{
    for (a, bits, m) in natural_matrix_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.mod_shl({bits}, {m}) = {}", a.mod_shl(bits, &m));
    }
}

fn demo_natural_matrix_mod_shl_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a, 'b> &'a NaturalMatrix: ModShl<T, &'b Natural, Output = NaturalMatrix>,
{
    for (a, bits, m) in natural_matrix_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).mod_shl({bits}, &{m}) = {}", (&a).mod_shl(bits, &m));
    }
}

fn demo_natural_matrix_mod_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: for<'a> ModShlAssign<T, &'a Natural>,
{
    for (mut a, bits, m) in natural_matrix_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_shl_assign(bits, &m);
        println!("a := {a_old}; a.mod_shl_assign({bits}, {m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix:
        for<'a> ModShl<T, &'a Natural, Output = NaturalMatrix> + Shl<T, Output = NaturalMatrix>,
    Natural: for<'a> ModShl<T, &'a Natural, Output = Natural>,
{
    run_benchmark(
        &format!("NaturalMatrix.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_natural_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits, m)| {
                no_out!(a.mod_shl(bits, &m));
            }),
            ("element by element", &mut |(a, bits, m)| {
                let (height, width) = (a.height(), a.width());
                no_out!(NaturalMatrix::from_owned_elements(
                    height,
                    width,
                    a.into_elements()
                        .into_iter()
                        .map(|x| x.mod_shl(bits, &m))
                        .collect::<Vec<_>>()
                ));
            }),
            ("shift, then reduce", &mut |(a, bits, m)| {
                no_out!((a << bits) % m);
            }),
        ],
    );
}
