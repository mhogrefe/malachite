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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_5;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_matrix_shl_unsigned);
    register_unsigned_demos!(runner, demo_natural_matrix_shl_unsigned_ref);
    register_unsigned_demos!(runner, demo_natural_matrix_shl_assign_unsigned);

    register_unsigned_benches!(
        runner,
        benchmark_natural_matrix_shl_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_natural_matrix_shl_unsigned_algorithms);
}

fn demo_natural_matrix_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: Shl<T, Output = NaturalMatrix>,
{
    for (a, bits) in natural_matrix_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} << {bits} = {}", a << bits);
    }
}

fn demo_natural_matrix_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a NaturalMatrix: Shl<T, Output = NaturalMatrix>,
{
    for (a, bits) in natural_matrix_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} << {bits} = {}", &a << bits);
    }
}

fn demo_natural_matrix_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    NaturalMatrix: ShlAssign<T>,
{
    for (mut a, bits) in natural_matrix_unsigned_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a <<= bits;
        println!("a := {a_old}; a <<= {bits}; a = {a}");
    }
}

fn benchmark_natural_matrix_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: Shl<T, Output = NaturalMatrix> + ShlAssign<T>,
    for<'a> &'a NaturalMatrix: Shl<T, Output = NaturalMatrix>,
{
    run_benchmark(
        &format!("NaturalMatrix << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix << T", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("&NaturalMatrix << T", &mut |(a, bits)| {
                no_out!(&a << bits);
            }),
            ("NaturalMatrix <<= T", &mut |(mut a, bits)| {
                a <<= bits;
            }),
        ],
    );
}

fn benchmark_natural_matrix_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    NaturalMatrix: Shl<T, Output = NaturalMatrix>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("NaturalMatrix << {}", T::NAME),
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("multiplying by a power of 2", &mut |(a, bits)| {
                no_out!(a * Natural::power_of_2(u64::exact_from(bits)));
            }),
        ],
    );
}
