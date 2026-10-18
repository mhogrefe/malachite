// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_matrix_signed_pair_gen_var_1, rational_matrix_unsigned_pair_gen_var_3,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_matrix_shl_unsigned);
    register_unsigned_demos!(runner, demo_rational_matrix_shl_unsigned_ref);
    register_unsigned_demos!(runner, demo_rational_matrix_shl_assign_unsigned);
    register_signed_demos!(runner, demo_rational_matrix_shl_signed);
    register_signed_demos!(runner, demo_rational_matrix_shl_signed_ref);
    register_signed_demos!(runner, demo_rational_matrix_shl_assign_signed);

    register_unsigned_benches!(
        runner,
        benchmark_rational_matrix_shl_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_rational_matrix_shl_unsigned_algorithms);
    register_signed_benches!(
        runner,
        benchmark_rational_matrix_shl_signed_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_rational_matrix_shl_signed_algorithms);
}

fn demo_rational_matrix_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalMatrix: Shl<T, Output = RationalMatrix>,
{
    for (a, bits) in rational_matrix_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} << {bits} = {}", a << bits);
    }
}

fn demo_rational_matrix_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a RationalMatrix: Shl<T, Output = RationalMatrix>,
{
    for (a, bits) in rational_matrix_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} << {bits} = {}", &a << bits);
    }
}

fn demo_rational_matrix_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalMatrix: ShlAssign<T>,
{
    for (mut a, bits) in rational_matrix_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a <<= bits;
        println!("a := {a_old}; a <<= {bits}; a = {a}");
    }
}

fn benchmark_rational_matrix_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalMatrix: Shl<T, Output = RationalMatrix> + ShlAssign<T>,
    for<'a> &'a RationalMatrix: Shl<T, Output = RationalMatrix>,
{
    run_benchmark(
        &format!("RationalMatrix << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix << T", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("&RationalMatrix << T", &mut |(a, bits)| {
                no_out!(&a << bits);
            }),
            ("RationalMatrix <<= T", &mut |(mut a, bits)| {
                a <<= bits;
            }),
        ],
    );
}

fn benchmark_rational_matrix_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalMatrix: Shl<T, Output = RationalMatrix>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("RationalMatrix << {}", T::NAME),
        BenchmarkType::Algorithms,
        rational_matrix_unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("multiplying by a power of 2", &mut |(a, bits)| {
                no_out!(a * Rational::power_of_2(u64::exact_from(bits)));
            }),
        ],
    );
}

fn demo_rational_matrix_shl_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalMatrix: Shl<T, Output = RationalMatrix>,
{
    for (a, bits) in rational_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old} << {bits} = {}", a << bits);
    }
}

fn demo_rational_matrix_shl_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a RationalMatrix: Shl<T, Output = RationalMatrix>,
{
    for (a, bits) in rational_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{a} << {bits} = {}", &a << bits);
    }
}

fn demo_rational_matrix_shl_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalMatrix: ShlAssign<T>,
{
    for (mut a, bits) in rational_matrix_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a <<= bits;
        println!("a := {a_old}; a <<= {bits}; a = {a}");
    }
}

fn benchmark_rational_matrix_shl_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalMatrix: Shl<T, Output = RationalMatrix> + ShlAssign<T>,
    for<'a> &'a RationalMatrix: Shl<T, Output = RationalMatrix>,
{
    run_benchmark(
        &format!("RationalMatrix << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_matrix_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix << T", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("&RationalMatrix << T", &mut |(a, bits)| {
                no_out!(&a << bits);
            }),
            ("RationalMatrix <<= T", &mut |(mut a, bits)| {
                a <<= bits;
            }),
        ],
    );
}

fn benchmark_rational_matrix_shl_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalMatrix: Shl<T, Output = RationalMatrix>,
    i64: ExactFrom<T>,
{
    run_benchmark(
        &format!("RationalMatrix << {}", T::NAME),
        BenchmarkType::Algorithms,
        rational_matrix_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, bits)| {
                no_out!(a << bits);
            }),
            ("multiplying by a power of 2", &mut |(a, bits)| {
                no_out!(a * Rational::power_of_2(i64::exact_from(bits)));
            }),
        ],
    );
}
