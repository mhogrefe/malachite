// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_mul);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_mul_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_mod_mul_assign);

    register_unsigned_benches!(runner, benchmark_unsigned_vector_mod_mul_algorithms);
}

fn demo_unsigned_vector_mod_mul<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.mod_mul({c}, {m}) = {}", v.mod_mul(c, m));
    }
}

fn demo_unsigned_vector_mod_mul_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).mod_mul({c}, {m}) = {}", (&v).mod_mul(c, m));
    }
}

fn demo_unsigned_vector_mod_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut v, c, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_mul_assign(c, m);
        println!("v := {v_old}; v.mod_mul_assign({c}, {m}); v = {v}");
    }
}

fn benchmark_unsigned_vector_mod_mul_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector<{}>.mod_mul({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("default", &mut |(v, c, m)| {
                no_out!(v.mod_mul(c, m));
            }),
            ("element by element, no precomputation", &mut |(v, c, m)| {
                no_out!(UnsignedVector {
                    elements: v
                        .elements
                        .into_iter()
                        .map(|x| x.mod_mul(c, m))
                        .collect::<Vec<T>>()
                });
            }),
        ],
    );
}
