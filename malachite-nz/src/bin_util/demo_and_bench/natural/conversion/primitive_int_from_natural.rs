// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_rm, natural_gen_var_6, natural_gen_var_7,
};
use std::fmt::Debug;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_primitive_int_try_from_natural);
    register_unsigned_demos!(runner, demo_unsigned_exact_from_natural);
    register_signed_demos!(runner, demo_signed_exact_from_natural);
    register_primitive_int_demos!(runner, demo_primitive_int_wrapping_from_natural);
    register_primitive_int_demos!(runner, demo_primitive_int_saturating_from_natural);
    register_primitive_int_demos!(runner, demo_primitive_int_overflowing_from_natural);
    register_primitive_int_demos!(runner, demo_primitive_int_convertible_from_natural);

    register_primitive_int_benches!(runner, benchmark_primitive_int_try_from_natural_algorithms);
    register_unsigned_benches!(runner, benchmark_unsigned_exact_from_natural);
    register_signed_benches!(runner, benchmark_signed_exact_from_natural);
    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_wrapping_from_natural_algorithms
    );
    register_primitive_int_benches!(runner, benchmark_primitive_int_saturating_from_natural);
    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_overflowing_from_natural_algorithms
    );
    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_convertible_from_natural_algorithms
    );

    register_bench!(runner, benchmark_u32_try_from_natural_library_comparison);
    register_bench!(
        runner,
        benchmark_u32_wrapping_from_natural_library_comparison
    );
    register_bench!(runner, benchmark_u64_try_from_natural_library_comparison);
    register_bench!(
        runner,
        benchmark_u64_wrapping_from_natural_library_comparison
    );
    register_bench!(runner, benchmark_i32_try_from_natural_library_comparison);
    register_bench!(
        runner,
        benchmark_i32_wrapping_from_natural_library_comparison
    );
    register_bench!(runner, benchmark_i64_try_from_natural_library_comparison);
    register_bench!(
        runner,
        benchmark_i64_wrapping_from_natural_library_comparison
    );
}

fn demo_primitive_int_try_from_natural<T: for<'a> TryFrom<&'a Natural> + PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> <T as TryFrom<&'a Natural>>::Error: Debug,
{
    for n in natural_gen().get(gm, config).take(limit) {
        println!("{}::try_from(&{}) = {:?}", T::NAME, n, T::try_from(&n));
    }
}

fn demo_unsigned_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for n in natural_gen_var_6::<T>().get(gm, config).take(limit) {
        println!("{}::exact_from(&{}) = {}", T::NAME, n, T::exact_from(&n));
    }
}

fn demo_signed_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ExactFrom<T>,
{
    for n in natural_gen_var_7::<T>().get(gm, config).take(limit) {
        println!("{}::exact_from(&{}) = {}", T::NAME, n, T::exact_from(&n));
    }
}

fn demo_primitive_int_wrapping_from_natural<T: for<'a> WrappingFrom<&'a Natural> + PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "{}::wrapping_from(&{}) = {}",
            T::NAME,
            n,
            T::wrapping_from(&n)
        );
    }
}

fn demo_primitive_int_saturating_from_natural<
    T: for<'a> SaturatingFrom<&'a Natural> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "{}::saturating_from(&{}) = {}",
            T::NAME,
            n,
            T::saturating_from(&n)
        );
    }
}

fn demo_primitive_int_overflowing_from_natural<
    T: for<'a> OverflowingFrom<&'a Natural> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "{}::overflowing_from(&{}) = {:?}",
            T::NAME,
            n,
            T::overflowing_from(&n)
        );
    }
}

fn demo_primitive_int_convertible_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            n,
            if T::convertible_from(&n) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn benchmark_primitive_int_try_from_natural_algorithms<
    T: for<'a> TryFrom<&'a Natural> + for<'a> OverflowingFrom<&'a Natural> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::try_from(&Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |n| no_out!(T::try_from(&n).ok())),
            ("using overflowing_from", &mut |n| {
                let (value, overflow) = T::overflowing_from(&n);
                if overflow {
                    None
                } else {
                    Some(value)
                };
            }),
        ],
    );
}

fn benchmark_unsigned_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!("{}::exact_from(&Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(&n)))],
    );
}

fn benchmark_signed_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ExactFrom<T>,
{
    run_benchmark(
        &format!("{}::exact_from(&Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen_var_7::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(&n)))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_primitive_int_wrapping_from_natural_algorithms<
    T: for<'a> OverflowingFrom<&'a Natural> + PrimitiveInt + for<'a> WrappingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::wrapping_from(&Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |n| no_out!(T::wrapping_from(&n))),
            ("using overflowing_from", &mut |n| {
                T::overflowing_from(&n).0;
            }),
        ],
    );
}

fn benchmark_primitive_int_saturating_from_natural<
    T: PrimitiveInt + for<'a> SaturatingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::saturating_from(&Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::saturating_from(&n)))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_primitive_int_overflowing_from_natural_algorithms<
    T: for<'a> ConvertibleFrom<&'a Natural>
        + for<'a> OverflowingFrom<&'a Natural>
        + PrimitiveInt
        + for<'a> WrappingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::overflowing_from(&Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |n| no_out!(T::overflowing_from(&n))),
            ("using wrapping_from and convertible_from", &mut |n| {
                no_out!((T::wrapping_from(&n), !T::convertible_from(&n)))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_primitive_int_convertible_from_natural_algorithms<
    T: for<'a> TryFrom<&'a Natural> + for<'a> ConvertibleFrom<&'a Natural> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(&Natural)", T::NAME),
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("standard", &mut |n| no_out!(T::convertible_from(&n))),
            ("using try_from", &mut |n| no_out!(T::try_from(&n).is_ok())),
        ],
    );
}

fn benchmark_u32_try_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32::try_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u32::try_from(&n).ok())),
            ("rug", &mut |(n, _)| no_out!(n.to_u32())),
        ],
    );
}

fn benchmark_u32_wrapping_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32::wrapping_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u32::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_u32_wrapping())),
        ],
    );
}

fn benchmark_u64_try_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64::try_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u64::try_from(&n).ok())),
            ("rug", &mut |(n, _)| no_out!(n.to_u64())),
        ],
    );
}

fn benchmark_u64_wrapping_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64::wrapping_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u64::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_u64_wrapping())),
        ],
    );
}

fn benchmark_i32_try_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i32::try_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i32::try_from(&n).ok())),
            ("rug", &mut |(n, _)| no_out!(n.to_i32())),
        ],
    );
}

fn benchmark_i32_wrapping_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i32::wrapping_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i32::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_i32_wrapping())),
        ],
    );
}

fn benchmark_i64_try_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i64::try_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i64::try_from(&n).ok())),
            ("rug", &mut |(n, _)| no_out!(n.to_i64())),
        ],
    );
}

fn benchmark_i64_wrapping_from_natural_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i64::wrapping_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i64::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_i64_wrapping())),
        ],
    );
}
