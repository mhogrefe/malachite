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
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::pair_1_gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::{
    gaussian_rational_signed_pair_gen_var_1, gaussian_rational_unsigned_pair_gen_var_1,
};
use std::ops::{Shr, ShrAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_gaussian_rational_shr_assign_unsigned);
    register_signed_demos!(runner, demo_gaussian_rational_shr_assign_signed);
    register_unsigned_demos!(runner, demo_gaussian_rational_shr_unsigned);
    register_signed_demos!(runner, demo_gaussian_rational_shr_signed);
    register_unsigned_demos!(runner, demo_gaussian_rational_shr_unsigned_ref);
    register_signed_demos!(runner, demo_gaussian_rational_shr_signed_ref);

    register_unsigned_benches!(runner, benchmark_gaussian_rational_shr_assign_unsigned);
    register_signed_benches!(runner, benchmark_gaussian_rational_shr_assign_signed);
    register_unsigned_benches!(
        runner,
        benchmark_gaussian_rational_shr_unsigned_evaluation_strategy
    );
    register_signed_benches!(
        runner,
        benchmark_gaussian_rational_shr_signed_evaluation_strategy
    );
}

fn demo_gaussian_rational_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: ShrAssign<T>,
{
    for (mut n, u) in gaussian_rational_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {n_old}; x >>= {u}; x = {n}");
    }
}

fn demo_gaussian_rational_shr_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: Shr<T, Output = GaussianRational>,
{
    for (n, u) in gaussian_rational_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("({}) >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_gaussian_rational_shr_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>,
{
    for (n, u) in gaussian_rational_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&({}) >> {} = {}", n, u, &n >> u);
    }
}

fn benchmark_gaussian_rational_shr_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: ShrAssign<T>,
{
    run_benchmark(
        &format!("GaussianRational >>= {}", T::NAME),
        BenchmarkType::Single,
        gaussian_rational_unsigned_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u)| n >>= u)],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: Shr<T, Output = GaussianRational>,
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>,
{
    run_benchmark(
        &format!("GaussianRational >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_unsigned_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("n"),
        &mut [
            (
                &format!("GaussianRational >> {}", T::NAME),
                &mut |(x, y)| {
                    no_out!(x >> y);
                },
            ),
            (
                &format!("&GaussianRational >> {}", T::NAME),
                &mut |(x, y)| {
                    no_out!(&x >> y);
                },
            ),
        ],
    );
}

fn demo_gaussian_rational_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: ShrAssign<T>,
{
    for (mut n, u) in gaussian_rational_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {n_old}; x >>= {u}; x = {n}");
    }
}

fn demo_gaussian_rational_shr_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: Shr<T, Output = GaussianRational>,
{
    for (n, u) in gaussian_rational_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("({}) >> {} = {}", n_old, u, n >> u);
    }
}

fn demo_gaussian_rational_shr_signed_ref<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>,
{
    for (n, u) in gaussian_rational_signed_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&({}) >> {} = {}", n, u, &n >> u);
    }
}

fn benchmark_gaussian_rational_shr_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: ShrAssign<T>,
{
    run_benchmark(
        &format!("GaussianRational >>= {}", T::NAME),
        BenchmarkType::Single,
        gaussian_rational_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u)| n >>= u)],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_rational_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: Shr<T, Output = GaussianRational>,
    for<'a> &'a GaussianRational: Shr<T, Output = GaussianRational>,
{
    run_benchmark(
        &format!("GaussianRational >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_signed_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("n"),
        &mut [
            (
                &format!("GaussianRational >> {}", T::NAME),
                &mut |(x, y)| {
                    no_out!(x >> y);
                },
            ),
            (
                &format!("&GaussianRational >> {}", T::NAME),
                &mut |(x, y)| {
                    no_out!(&x >> y);
                },
            ),
        ],
    );
}
