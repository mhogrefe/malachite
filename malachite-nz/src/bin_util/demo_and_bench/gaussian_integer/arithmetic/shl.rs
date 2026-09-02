// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::bench::bucketers::pair_1_gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_unsigned_pair_gen_var_1;
use std::ops::{Shl, ShlAssign};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_gaussian_integer_shl_assign_unsigned);
    register_unsigned_demos!(runner, demo_gaussian_integer_shl_unsigned);
    register_unsigned_demos!(runner, demo_gaussian_integer_shl_unsigned_ref);

    register_unsigned_benches!(runner, benchmark_gaussian_integer_shl_assign_unsigned);
    register_unsigned_benches!(
        runner,
        benchmark_gaussian_integer_shl_unsigned_evaluation_strategy
    );
}

fn demo_gaussian_integer_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianInteger: ShlAssign<T>,
{
    for (mut n, u) in gaussian_integer_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n <<= u;
        println!("x := {n_old}; x <<= {u}; x = {n}");
    }
}

fn demo_gaussian_integer_shl_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianInteger: Shl<T, Output = GaussianInteger>,
{
    for (n, u) in gaussian_integer_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("({}) << {} = {}", n_old, u, n << u);
    }
}

fn demo_gaussian_integer_shl_unsigned_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a GaussianInteger: Shl<T, Output = GaussianInteger>,
{
    for (n, u) in gaussian_integer_unsigned_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("&({}) << {} = {}", n, u, &n << u);
    }
}

fn benchmark_gaussian_integer_shl_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianInteger: ShlAssign<T>,
{
    run_benchmark(
        &format!("GaussianInteger <<= {}", T::NAME),
        BenchmarkType::Single,
        gaussian_integer_unsigned_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, u)| n <<= u)],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_shl_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianInteger: Shl<T, Output = GaussianInteger>,
    for<'a> &'a GaussianInteger: Shl<T, Output = GaussianInteger>,
{
    run_benchmark(
        &format!("GaussianInteger << {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_unsigned_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_integer_bit_bucketer("n"),
        &mut [
            (&format!("GaussianInteger << {}", T::NAME), &mut |(x, y)| {
                no_out!(x << y);
            }),
            (
                &format!("&GaussianInteger << {}", T::NAME),
                &mut |(x, y)| {
                    no_out!(&x << y);
                },
            ),
        ],
    );
}
