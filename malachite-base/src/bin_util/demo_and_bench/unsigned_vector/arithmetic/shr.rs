// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_6;
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_shr);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_shr_ref);
    register_unsigned_unsigned_demos!(runner, demo_unsigned_vector_shr_assign);

    register_unsigned_unsigned_benches!(runner, benchmark_unsigned_vector_shr_evaluation_strategy);
    register_unsigned_unsigned_benches!(runner, benchmark_unsigned_vector_shr_algorithms);
}

fn demo_unsigned_vector_shr<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedVector<T>: Shr<U, Output = UnsignedVector<T>>,
{
    for (v, bits) in unsigned_vector_unsigned_pair_gen_var_6::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} >> {bits} = {}", v >> bits);
    }
}

fn demo_unsigned_vector_shr_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a UnsignedVector<T>: Shr<U, Output = UnsignedVector<T>>,
{
    for (v, bits) in unsigned_vector_unsigned_pair_gen_var_6::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} >> {bits} = {}", &v >> bits);
    }
}

fn demo_unsigned_vector_shr_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    UnsignedVector<T>: ShrAssign<U>,
{
    for (mut v, bits) in unsigned_vector_unsigned_pair_gen_var_6::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v >>= bits;
        println!("v := {v_old}; v >>= {bits}; v = {v}");
    }
}

fn benchmark_unsigned_vector_shr_evaluation_strategy<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedVector<T>: Shr<U, Output = UnsignedVector<T>> + ShrAssign<U>,
    for<'a> &'a UnsignedVector<T>: Shr<U, Output = UnsignedVector<T>>,
{
    run_benchmark(
        &format!("UnsignedVector<{}> >> {}", T::NAME, U::NAME),
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_pair_gen_var_6::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector >> U", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("&UnsignedVector >> U", &mut |(v, bits)| {
                no_out!(&v >> bits);
            }),
            ("UnsignedVector >>= U", &mut |(mut v, bits)| {
                v >>= bits;
            }),
        ],
    );
}

fn benchmark_unsigned_vector_shr_algorithms<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    UnsignedVector<T>:
        Shr<U, Output = UnsignedVector<T>> + EntrywiseShrRound<U, Output = UnsignedVector<T>>,
{
    run_benchmark(
        &format!("UnsignedVector<{}> >> {}", T::NAME, U::NAME),
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_pair_gen_var_6::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("using entrywise_shr_round with Floor", &mut |(v, bits)| {
                no_out!(v.entrywise_shr_round(bits, Floor));
            }),
        ],
    );
}
