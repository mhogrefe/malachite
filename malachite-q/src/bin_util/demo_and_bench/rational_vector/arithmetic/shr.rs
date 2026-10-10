// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::bench::bucketers::pair_1_rational_vector_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_vector_signed_pair_gen_var_1, rational_vector_unsigned_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_vector_shr_unsigned);
    register_unsigned_demos!(runner, demo_rational_vector_shr_unsigned_ref);
    register_unsigned_demos!(runner, demo_rational_vector_shr_assign_unsigned);
    register_signed_demos!(runner, demo_rational_vector_shr_signed);
    register_signed_demos!(runner, demo_rational_vector_shr_signed_ref);
    register_signed_demos!(runner, demo_rational_vector_shr_assign_signed);

    register_unsigned_benches!(
        runner,
        benchmark_rational_vector_shr_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_rational_vector_shr_unsigned_algorithms);
    register_signed_benches!(
        runner,
        benchmark_rational_vector_shr_signed_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_rational_vector_shr_signed_algorithms);
}

fn demo_rational_vector_shr_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalVector: Shr<T, Output = RationalVector>,
{
    for (v, bits) in rational_vector_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} >> {bits} = {}", v >> bits);
    }
}

fn demo_rational_vector_shr_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a RationalVector: Shr<T, Output = RationalVector>,
{
    for (v, bits) in rational_vector_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} >> {bits} = {}", &v >> bits);
    }
}

fn demo_rational_vector_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalVector: ShrAssign<T>,
{
    for (mut v, bits) in rational_vector_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v >>= bits;
        println!("v := {v_old}; v >>= {bits}; v = {v}");
    }
}

fn benchmark_rational_vector_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalVector: Shr<T, Output = RationalVector> + ShrAssign<T>,
    for<'a> &'a RationalVector: Shr<T, Output = RationalVector>,
{
    run_benchmark(
        &format!("RationalVector >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_vector_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector >> T", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("&RationalVector >> T", &mut |(v, bits)| {
                no_out!(&v >> bits);
            }),
            ("RationalVector >>= T", &mut |(mut v, bits)| {
                v >>= bits;
            }),
        ],
    );
}

fn benchmark_rational_vector_shr_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalVector: Shr<T, Output = RationalVector>,
    i64: ExactFrom<T>,
{
    run_benchmark(
        &format!("RationalVector >> {}", T::NAME),
        BenchmarkType::Algorithms,
        rational_vector_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("dividing by a power of 2", &mut |(v, bits)| {
                no_out!(v * Rational::power_of_2(-i64::exact_from(bits)));
            }),
        ],
    );
}

fn demo_rational_vector_shr_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalVector: Shr<T, Output = RationalVector>,
{
    for (v, bits) in rational_vector_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} >> {bits} = {}", v >> bits);
    }
}

fn demo_rational_vector_shr_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a RationalVector: Shr<T, Output = RationalVector>,
{
    for (v, bits) in rational_vector_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} >> {bits} = {}", &v >> bits);
    }
}

fn demo_rational_vector_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalVector: ShrAssign<T>,
{
    for (mut v, bits) in rational_vector_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v >>= bits;
        println!("v := {v_old}; v >>= {bits}; v = {v}");
    }
}

fn benchmark_rational_vector_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalVector: Shr<T, Output = RationalVector> + ShrAssign<T>,
    for<'a> &'a RationalVector: Shr<T, Output = RationalVector>,
{
    run_benchmark(
        &format!("RationalVector >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_vector_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector >> T", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("&RationalVector >> T", &mut |(v, bits)| {
                no_out!(&v >> bits);
            }),
            ("RationalVector >>= T", &mut |(mut v, bits)| {
                v >>= bits;
            }),
        ],
    );
}

fn benchmark_rational_vector_shr_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalVector: Shr<T, Output = RationalVector>,
    i64: ExactFrom<T>,
{
    run_benchmark(
        &format!("RationalVector >> {}", T::NAME),
        BenchmarkType::Algorithms,
        rational_vector_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, bits)| {
                no_out!(v >> bits);
            }),
            ("dividing by a power of 2", &mut |(v, bits)| {
                no_out!(v * Rational::power_of_2(-i64::exact_from(bits)));
            }),
        ],
    );
}
