// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_vector_shl_unsigned);
    register_unsigned_demos!(runner, demo_natural_vector_shl_unsigned_ref);
    register_unsigned_demos!(runner, demo_natural_vector_shl_assign_unsigned);

    register_unsigned_benches!(
        runner,
        benchmark_natural_vector_shl_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_natural_vector_shl_unsigned_algorithms);
}

fn demo_natural_vector_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: Shl<T, Output = NaturalVector>,
{
    for (v, bits) in natural_vector_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} << {bits} = {}", v << bits);
    }
}

fn demo_natural_vector_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalVector: Shl<T, Output = NaturalVector>,
{
    for (v, bits) in natural_vector_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} << {bits} = {}", &v << bits);
    }
}

fn demo_natural_vector_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalVector: ShlAssign<T>,
{
    for (mut v, bits) in natural_vector_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v <<= bits;
        println!("v := {v_old}; v <<= {bits}; v = {v}");
    }
}

fn benchmark_natural_vector_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalVector: Shl<T, Output = NaturalVector> + ShlAssign<T>,
    for<'a> &'a NaturalVector: Shl<T, Output = NaturalVector>,
{
    run_benchmark(
        &format!("NaturalVector << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_vector_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector << T", &mut |(v, bits)| {
                no_out!(v << bits);
            }),
            ("&NaturalVector << T", &mut |(v, bits)| {
                no_out!(&v << bits);
            }),
            ("NaturalVector <<= T", &mut |(mut v, bits)| {
                v <<= bits;
            }),
        ],
    );
}

fn benchmark_natural_vector_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalVector: Shl<T, Output = NaturalVector>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("NaturalVector << {}", T::NAME),
        BenchmarkType::Algorithms,
        natural_vector_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v << bits);
            }),
            ("multiplying by a power of 2", &mut |(v, bits)| {
                no_out!(v * Natural::power_of_2(u64::exact_from(bits)));
            }),
        ],
    );
}
