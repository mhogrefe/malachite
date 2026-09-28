// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_3;
use malachite_nz::test_util::integer_polynomial::arithmetic::shl::shl_naive;
use std::ops::{Shl, ShlAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_polynomial_shl_unsigned);
    register_unsigned_demos!(runner, demo_integer_polynomial_shl_unsigned_ref);
    register_unsigned_demos!(runner, demo_integer_polynomial_shl_assign_unsigned);

    register_unsigned_benches!(
        runner,
        benchmark_integer_polynomial_shl_unsigned_evaluation_strategy
    );
    register_unsigned_benches!(runner, benchmark_integer_polynomial_shl_unsigned_algorithms);
}

fn demo_integer_polynomial_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerPolynomial: Shl<T, Output = IntegerPolynomial>,
{
    for (p, bits) in integer_polynomial_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}) << {bits} = {}", p << bits);
    }
}

fn demo_integer_polynomial_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a IntegerPolynomial: Shl<T, Output = IntegerPolynomial>,
{
    for (p, bits) in integer_polynomial_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&({p}) << {bits} = {}", &p << bits);
    }
}

fn demo_integer_polynomial_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    IntegerPolynomial: ShlAssign<T>,
{
    for (mut p, bits) in integer_polynomial_unsigned_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p <<= bits;
        println!("p := {p_old}; p <<= {bits}; p = {p}");
    }
}

fn benchmark_integer_polynomial_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    IntegerPolynomial: Shl<T, Output = IntegerPolynomial> + ShlAssign<T>,
    for<'a> &'a IntegerPolynomial: Shl<T, Output = IntegerPolynomial>,
{
    run_benchmark(
        &format!("IntegerPolynomial << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            (
                &format!("IntegerPolynomial << {}", T::NAME),
                &mut |(p, bits)| {
                    no_out!(p << bits);
                },
            ),
            (
                &format!("&IntegerPolynomial << {}", T::NAME),
                &mut |(p, bits)| {
                    no_out!(&p << bits);
                },
            ),
            (
                &format!("IntegerPolynomial <<= {}", T::NAME),
                &mut |(mut p, bits)| {
                    p <<= bits;
                },
            ),
        ],
    );
}

fn benchmark_integer_polynomial_shl_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a IntegerPolynomial: Shl<T, Output = IntegerPolynomial>,
    u64: ExactFrom<T>,
{
    run_benchmark(
        &format!("IntegerPolynomial << {}", T::NAME),
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, bits)| no_out!(&p << bits)),
            ("naive", &mut |(p, bits)| no_out!(shl_naive(&p, bits))),
        ],
    );
}
