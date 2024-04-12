// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    abs_usize_convertible_pair_max_bucketer, usize_convertible_pair_max_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_pair_gen_var_11, signed_pair_gen_var_12, unsigned_pair_gen_var_28,
    unsigned_pair_gen_var_44,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_binomial_coefficient_unsigned);
    register_signed_demos!(runner, demo_binomial_coefficient_signed);
    register_unsigned_demos!(runner, demo_checked_binomial_coefficient_unsigned);
    register_signed_demos!(runner, demo_checked_binomial_coefficient_signed);

    register_unsigned_benches!(runner, benchmark_binomial_coefficient_unsigned);
    register_signed_benches!(runner, benchmark_binomial_coefficient_signed);
    register_unsigned_benches!(runner, benchmark_checked_binomial_coefficient_unsigned);
    register_signed_benches!(runner, benchmark_checked_binomial_coefficient_signed);
}

fn demo_binomial_coefficient_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_44().get(gm, config).take(limit) {
        println!("C({}, {}) = {}", n, k, T::binomial_coefficient(n, k));
    }
}

fn demo_binomial_coefficient_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in signed_pair_gen_var_12().get(gm, config).take(limit) {
        println!("C({}, {}) = {}", n, k, T::binomial_coefficient(n, k));
    }
}

fn demo_checked_binomial_coefficient_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_28().get(gm, config).take(limit) {
        println!(
            "C({}, {}) = {:?}",
            n,
            k,
            T::checked_binomial_coefficient(n, k)
        );
    }
}

fn demo_checked_binomial_coefficient_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in signed_pair_gen_var_11().get(gm, config).take(limit) {
        println!(
            "C({}, {}) = {:?}",
            n,
            k,
            T::checked_binomial_coefficient(n, k)
        );
    }
}

fn benchmark_binomial_coefficient_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!(
            "{}::binomial_coefficient({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_44().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::binomial_coefficient(n, k))
        })],
    );
}

fn benchmark_binomial_coefficient_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: Ord,
{
    run_benchmark(
        &format!(
            "{}::binomial_coefficient({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        signed_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &abs_usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::binomial_coefficient(n, k))
        })],
    );
}

fn benchmark_checked_binomial_coefficient_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!(
            "{}::checked_binomial_coefficient({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::checked_binomial_coefficient(n, k))
        })],
    );
}

fn benchmark_checked_binomial_coefficient_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: Ord,
{
    run_benchmark(
        &format!(
            "{}::checked_binomial_coefficient({}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        signed_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &abs_usize_convertible_pair_max_bucketer("n", "k"),
        &mut [("Malachite", &mut |(n, k)| {
            no_out!(T::checked_binomial_coefficient(n, k))
        })],
    );
}
