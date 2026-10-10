// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_natural_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_vector_mod_shl);
    register_unsigned_demos!(runner, demo_natural_vector_mod_shl_ref);
    register_unsigned_demos!(runner, demo_natural_vector_mod_shl_assign);

    register_unsigned_benches!(runner, benchmark_natural_vector_mod_shl_algorithms);
}

fn demo_natural_vector_mod_shl<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    NaturalVector: for<'a> ModShl<T, &'a Natural, Output = NaturalVector>,
{
    for (v, bits, m) in natural_vector_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.mod_shl({bits}, {m}) = {}", v.mod_shl(bits, &m));
    }
}

fn demo_natural_vector_mod_shl_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a, 'b> &'a NaturalVector: ModShl<T, &'b Natural, Output = NaturalVector>,
{
    for (v, bits, m) in natural_vector_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_shl({bits}, &{m}) = {}", (&v).mod_shl(bits, &m));
    }
}

fn demo_natural_vector_mod_shl_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: for<'a> ModShlAssign<T, &'a Natural>,
{
    for (mut v, bits, m) in natural_vector_unsigned_natural_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_shl_assign(bits, &m);
        println!("v := {v_old}; v.mod_shl_assign({bits}, {m}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_shl_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalVector:
        for<'a> ModShl<T, &'a Natural, Output = NaturalVector> + Shl<T, Output = NaturalVector>,
    Natural: for<'a> ModShl<T, &'a Natural, Output = Natural>,
{
    run_benchmark(
        &format!("NaturalVector.mod_shl({}, Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_vector_unsigned_natural_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits, m)| {
                no_out!(v.mod_shl(bits, &m));
            }),
            ("element by element", &mut |(v, bits, m)| {
                no_out!(NaturalVector {
                    elements: v
                        .elements
                        .into_iter()
                        .map(|x| x.mod_shl(bits, &m))
                        .collect()
                });
            }),
            ("shift, then reduce", &mut |(v, bits, m)| {
                no_out!((v << bits) % m);
            }),
        ],
    );
}
