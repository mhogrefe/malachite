// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use malachite_base::num::arithmetic::traits::{EntrywiseShlRound, PowerOf2};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_signed_pair_gen_var_1, integer_matrix_unsigned_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_matrix_shl_unsigned);
    register_unsigned_demos!(runner, demo_integer_matrix_shl_unsigned_ref);
    register_unsigned_demos!(runner, demo_integer_matrix_shl_assign_unsigned);

    register_unsigned_benches!(
        runner,
        benchmark_integer_matrix_shl_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_integer_matrix_shl_unsigned_algorithms);
    register_signed_demos!(runner, demo_integer_matrix_shl_signed);
    register_signed_demos!(runner, demo_integer_matrix_shl_signed_ref);
    register_signed_demos!(runner, demo_integer_matrix_shl_assign_signed);
    register_signed_benches!(
        runner,
        benchmark_integer_matrix_shl_signed_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_integer_matrix_shl_signed_algorithms);
}

fn demo_integer_matrix_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    for (a, bits) in integer_matrix_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} << {bits} = {}", a << bits);
    }
}

fn demo_integer_matrix_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    for (a, bits) in integer_matrix_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} << {bits} = {}", &a << bits);
    }
}

fn demo_integer_matrix_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerMatrix: ShlAssign<T>,
{
    for (mut a, bits) in integer_matrix_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a <<= bits;
        println!("a := {a_old}; a <<= {bits}; a = {a}");
    }
}

fn benchmark_integer_matrix_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    IntegerMatrix: Shl<T, Output = IntegerMatrix> + ShlAssign<T>,
    for<'a> &'a IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    run_benchmark(
        &format!("IntegerMatrix << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        integer_matrix_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix << T", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("&IntegerMatrix << T", &mut |(a, bits)| {
                no_out!(&a << bits);
            }),
            ("IntegerMatrix <<= T", &mut |(mut a, bits)| {
                a <<= bits;
            }),
        ],
    );
}

fn benchmark_integer_matrix_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    IntegerMatrix: Shl<T, Output = IntegerMatrix>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("IntegerMatrix << {}", T::NAME),
        BenchmarkType::Algorithms,
        integer_matrix_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("multiplying by a power of 2", &mut |(a, bits)| {
                no_out!(a * Integer::power_of_2(u64::exact_from(bits)));
            }),
        ],
    );
}

fn demo_integer_matrix_shl_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    for (v, bits) in integer_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} << {bits} = {}", v << bits);
    }
}

fn demo_integer_matrix_shl_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    for (v, bits) in integer_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} << {bits} = {}", &v << bits);
    }
}

fn demo_integer_matrix_shl_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerMatrix: ShlAssign<T>,
{
    for (mut v, bits) in integer_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v <<= bits;
        println!("v := {v_old}; v <<= {bits}; v = {v}");
    }
}

fn benchmark_integer_matrix_shl_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    IntegerMatrix: Shl<T, Output = IntegerMatrix> + ShlAssign<T>,
    for<'a> &'a IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    run_benchmark(
        &format!("IntegerMatrix << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        integer_matrix_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("v"),
        &mut [
            ("IntegerMatrix << T", &mut |(v, bits)| {
                no_out!(v << bits);
            }),
            ("&IntegerMatrix << T", &mut |(v, bits)| {
                no_out!(&v << bits);
            }),
            ("IntegerMatrix <<= T", &mut |(mut v, bits)| {
                v <<= bits;
            }),
        ],
    );
}

fn benchmark_integer_matrix_shl_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    IntegerMatrix: Shl<T, Output = IntegerMatrix> + EntrywiseShlRound<T, Output = IntegerMatrix>,
{
    run_benchmark(
        &format!("IntegerMatrix << {}", T::NAME),
        BenchmarkType::Algorithms,
        integer_matrix_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v << bits);
            }),
            ("using entrywise_shl_round with Floor", &mut |(v, bits)| {
                no_out!(v.entrywise_shl_round(bits, Floor));
            }),
        ],
    );
}
