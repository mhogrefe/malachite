// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_8;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_div);
    register_unsigned_demos!(runner, demo_unsigned_vector_div_ref);
    register_unsigned_demos!(runner, demo_unsigned_vector_div_assign);

    register_unsigned_benches!(runner, benchmark_unsigned_vector_div_evaluation_strategy);
}

fn demo_unsigned_vector_div<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in unsigned_vector_unsigned_pair_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} / {c} = {}", v / c);
    }
}

fn demo_unsigned_vector_div_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c) in unsigned_vector_unsigned_pair_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} / {c} = {}", &v / c);
    }
}

fn demo_unsigned_vector_div_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut v, c) in unsigned_vector_unsigned_pair_gen_var_8::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v /= c;
        println!("v := {v_old}; v /= {c}; v = {v}");
    }
}

fn benchmark_unsigned_vector_div_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("UnsignedVector<{}> / {}", T::NAME, T::NAME),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_pair_gen_var_8::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector / T", &mut |(v, c)| {
                no_out!(v / c);
            }),
            ("&UnsignedVector / T", &mut |(v, c)| {
                no_out!(&v / c);
            }),
            ("UnsignedVector /= T", &mut |(mut v, c)| {
                v /= c;
            }),
        ],
    );
}
