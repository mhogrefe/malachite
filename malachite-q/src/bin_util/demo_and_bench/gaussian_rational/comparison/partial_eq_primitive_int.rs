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
    gaussian_rational_signed_pair_gen, gaussian_rational_unsigned_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_gaussian_rational_partial_eq_unsigned);
    register_signed_demos!(runner, demo_gaussian_rational_partial_eq_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_eq_gaussian_rational);
    register_signed_demos!(runner, demo_signed_partial_eq_gaussian_rational);

    register_unsigned_benches!(runner, benchmark_gaussian_rational_partial_eq_unsigned);
    register_signed_benches!(runner, benchmark_gaussian_rational_partial_eq_signed);
}

fn demo_gaussian_rational_partial_eq_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: PartialEq<T>,
{
    for (n, u) in gaussian_rational_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n == u {
            println!("{n} = {u}");
        } else {
            println!("{n} ≠ {u}");
        }
    }
}

fn demo_gaussian_rational_partial_eq_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: PartialEq<T>,
{
    for (n, i) in gaussian_rational_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n == i {
            println!("{n} = {i}");
        } else {
            println!("{n} ≠ {i}");
        }
    }
}

fn demo_unsigned_partial_eq_gaussian_rational<
    T: PartialEq<GaussianRational> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in gaussian_rational_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if u == n {
            println!("{u} = {n}");
        } else {
            println!("{u} ≠ {n}");
        }
    }
}

fn demo_signed_partial_eq_gaussian_rational<T: PartialEq<GaussianRational> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in gaussian_rational_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if i == n {
            println!("{i} = {n}");
        } else {
            println!("{i} ≠ {n}");
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_gaussian_rational_partial_eq_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: PartialEq<T>,
{
    run_benchmark(
        &format!("GaussianRational == {}", T::NAME),
        BenchmarkType::Single,
        gaussian_rational_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x == y))],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_gaussian_rational_partial_eq_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianRational: PartialEq<T>,
{
    run_benchmark(
        &format!("GaussianRational == {}", T::NAME),
        BenchmarkType::Single,
        gaussian_rational_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x == y))],
    );
}
