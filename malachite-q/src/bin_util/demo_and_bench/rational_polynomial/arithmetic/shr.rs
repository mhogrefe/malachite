// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::bench::bucketers::pair_1_rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_polynomial_signed_pair_gen_var_1, rational_polynomial_unsigned_pair_gen_var_3,
};
use malachite_q::test_util::rational_polynomial::arithmetic::shr::shr_naive;
use std::ops::{Shr, ShrAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_rational_polynomial_shr_unsigned);
    register_unsigned_demos!(runner, demo_rational_polynomial_shr_unsigned_ref);
    register_unsigned_demos!(runner, demo_rational_polynomial_shr_assign_unsigned);
    register_signed_demos!(runner, demo_rational_polynomial_shr_signed);
    register_signed_demos!(runner, demo_rational_polynomial_shr_signed_ref);
    register_signed_demos!(runner, demo_rational_polynomial_shr_assign_signed);

    register_unsigned_benches!(
        runner,
        benchmark_rational_polynomial_shr_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_rational_polynomial_shr_unsigned_algorithms
    );
    register_signed_benches!(
        runner,
        benchmark_rational_polynomial_shr_signed_evaluation_strategy
    );
    register_signed_benches!(runner, benchmark_rational_polynomial_shr_signed_algorithms);
}

fn demo_rational_polynomial_shr_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalPolynomial: Shr<T, Output = RationalPolynomial>,
{
    for (p, bits) in rational_polynomial_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}) >> {bits} = {}", p >> bits);
    }
}

fn demo_rational_polynomial_shr_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a RationalPolynomial: Shr<T, Output = RationalPolynomial>,
{
    for (p, bits) in rational_polynomial_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&({p}) >> {bits} = {}", &p >> bits);
    }
}

fn demo_rational_polynomial_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalPolynomial: ShrAssign<T>,
{
    for (mut p, bits) in rational_polynomial_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p >>= bits;
        println!("p := {p_old}; p >>= {bits}; p = {p}");
    }
}

fn benchmark_rational_polynomial_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalPolynomial: Shr<T, Output = RationalPolynomial> + ShrAssign<T>,
    for<'a> &'a RationalPolynomial: Shr<T, Output = RationalPolynomial>,
{
    run_benchmark(
        &format!("RationalPolynomial >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            (
                &format!("RationalPolynomial >> {}", T::NAME),
                &mut |(p, bits)| {
                    no_out!(p >> bits);
                },
            ),
            (
                &format!("&RationalPolynomial >> {}", T::NAME),
                &mut |(p, bits)| {
                    no_out!(&p >> bits);
                },
            ),
            (
                &format!("RationalPolynomial >>= {}", T::NAME),
                &mut |(mut p, bits)| {
                    p >>= bits;
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_shr_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a RationalPolynomial: Shr<T, Output = RationalPolynomial>,
    Rational: Shr<T, Output = Rational>,
{
    run_benchmark(
        &format!("RationalPolynomial >> {}", T::NAME),
        BenchmarkType::Algorithms,
        rational_polynomial_unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits)| no_out!(&p >> bits)),
            ("naive", &mut |(p, bits)| no_out!(shr_naive(&p, bits))),
        ],
    );
}

fn demo_rational_polynomial_shr_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalPolynomial: Shr<T, Output = RationalPolynomial>,
{
    for (p, bits) in rational_polynomial_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}) >> {bits} = {}", p >> bits);
    }
}

fn demo_rational_polynomial_shr_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a RationalPolynomial: Shr<T, Output = RationalPolynomial>,
{
    for (p, bits) in rational_polynomial_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&({p}) >> {bits} = {}", &p >> bits);
    }
}

fn demo_rational_polynomial_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    RationalPolynomial: ShrAssign<T>,
{
    for (mut p, bits) in rational_polynomial_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p >>= bits;
        println!("p := {p_old}; p >>= {bits}; p = {p}");
    }
}

fn benchmark_rational_polynomial_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    RationalPolynomial: Shr<T, Output = RationalPolynomial> + ShrAssign<T>,
    for<'a> &'a RationalPolynomial: Shr<T, Output = RationalPolynomial>,
{
    run_benchmark(
        &format!("RationalPolynomial >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            (
                &format!("RationalPolynomial >> {}", T::NAME),
                &mut |(p, bits)| {
                    no_out!(p >> bits);
                },
            ),
            (
                &format!("&RationalPolynomial >> {}", T::NAME),
                &mut |(p, bits)| {
                    no_out!(&p >> bits);
                },
            ),
            (
                &format!("RationalPolynomial >>= {}", T::NAME),
                &mut |(mut p, bits)| {
                    p >>= bits;
                },
            ),
        ],
    );
}

fn benchmark_rational_polynomial_shr_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a RationalPolynomial: Shr<T, Output = RationalPolynomial>,
    Rational: Shr<T, Output = Rational>,
{
    run_benchmark(
        &format!("RationalPolynomial >> {}", T::NAME),
        BenchmarkType::Algorithms,
        rational_polynomial_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits)| no_out!(&p >> bits)),
            ("naive", &mut |(p, bits)| no_out!(shr_naive(&p, bits))),
        ],
    );
}
