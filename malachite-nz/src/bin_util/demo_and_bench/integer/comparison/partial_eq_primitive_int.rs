// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_1_integer_bit_bucketer, triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_signed_pair_gen, integer_signed_pair_gen_rm, integer_unsigned_pair_gen,
    integer_unsigned_pair_gen_nrm, integer_unsigned_pair_gen_rm,
};
use malachite_nz::test_util::integer::comparison::partial_eq_primitive_int::*;
use num::BigInt;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_partial_eq_unsigned);
    register_signed_demos!(runner, demo_integer_partial_eq_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_eq_integer);
    register_signed_demos!(runner, demo_signed_partial_eq_integer);

    register_unsigned_benches!(
        runner,
        benchmark_integer_partial_eq_unsigned_library_comparison
    );
    register_signed_benches!(
        runner,
        benchmark_integer_partial_eq_signed_library_comparison
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_partial_eq_integer_library_comparison
    );
    register_signed_benches!(
        runner,
        benchmark_signed_partial_eq_integer_library_comparison
    );
}

fn demo_integer_partial_eq_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialEq<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if n == u {
            println!("{n} = {u}");
        } else {
            println!("{n} ≠ {u}");
        }
    }
}

fn demo_integer_partial_eq_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialEq<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if n == i {
            println!("{n} = {i}");
        } else {
            println!("{n} ≠ {i}");
        }
    }
}

fn demo_unsigned_partial_eq_integer<T: PartialEq<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if u == n {
            println!("{u} = {n}");
        } else {
            println!("{u} ≠ {n}");
        }
    }
}

fn demo_signed_partial_eq_integer<T: PartialEq<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if i == n {
            println!("{i} = {n}");
        } else {
            println!("{i} ≠ {n}");
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_partial_eq_unsigned_library_comparison<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BigInt: From<T>,
    Integer: PartialEq<T>,
    rug::Integer: PartialEq<T>,
{
    run_benchmark(
        &format!("Integer == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_nrm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x == y)),
            ("num", &mut |((x, y), _, _)| {
                no_out!(num_partial_eq_primitive(&x, y))
            }),
            ("rug", &mut |(_, (x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_partial_eq_signed_library_comparison<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialEq<T>,
    rug::Integer: PartialEq<T>,
{
    run_benchmark(
        &format!("Integer == {}", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_signed_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x == y)),
            ("rug", &mut |((x, y), _)| no_out!(x == y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_unsigned_partial_eq_integer_library_comparison<
    T: PartialEq<Integer> + PartialEq<rug::Integer> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{} == Integer", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y == x)),
            ("rug", &mut |((x, y), _)| no_out!(y == x)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_signed_partial_eq_integer_library_comparison<
    T: PartialEq<Integer> + PartialEq<rug::Integer> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{} == Integer", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_signed_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y == x)),
            ("rug", &mut |((x, y), _)| no_out!(y == x)),
        ],
    );
}
