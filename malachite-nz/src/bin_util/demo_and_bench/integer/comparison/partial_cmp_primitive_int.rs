// Copyright © 2025 Mikhail Hogrefe
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
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_1_integer_bit_bucketer, triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_signed_pair_gen, integer_signed_pair_gen_rm, integer_unsigned_pair_gen,
    integer_unsigned_pair_gen_nrm, integer_unsigned_pair_gen_rm,
};
use malachite_nz::test_util::integer::comparison::partial_cmp_primitive_int::*;
use num::BigInt;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_partial_cmp_unsigned);
    register_signed_demos!(runner, demo_integer_partial_cmp_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_integer);
    register_signed_demos!(runner, demo_signed_partial_cmp_integer);

    register_unsigned_benches!(
        runner,
        benchmark_integer_partial_cmp_unsigned_library_comparison
    );
    register_signed_benches!(
        runner,
        benchmark_integer_partial_cmp_signed_library_comparison
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_partial_cmp_integer_library_comparison
    );
    register_signed_benches!(
        runner,
        benchmark_signed_partial_cmp_integer_library_comparison
    );
}

fn demo_integer_partial_cmp_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialOrd<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Less => println!("{n} < {u}"),
            Equal => println!("{n} = {u}"),
            Greater => println!("{n} > {u}"),
        }
    }
}

fn demo_integer_partial_cmp_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialOrd<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        match n.partial_cmp(&i).unwrap() {
            Less => println!("{n} < {i}"),
            Equal => println!("{n} = {i}"),
            Greater => println!("{n} > {i}"),
        }
    }
}

fn demo_unsigned_partial_cmp_integer<T: PartialOrd<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Less => println!("{u} < {n}"),
            Equal => println!("{u} = {n}"),
            Greater => println!("{u} > {n}"),
        }
    }
}

fn demo_signed_partial_cmp_integer<T: PartialOrd<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        match i.partial_cmp(&n).unwrap() {
            Less => println!("{i} < {n}"),
            Equal => println!("{i} = {n}"),
            Greater => println!("{i} > {n}"),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_integer_partial_cmp_unsigned_library_comparison<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BigInt: From<T>,
    Integer: PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    run_benchmark(
        &format!("Integer.partial_cmp(&{})", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_nrm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| {
                no_out!(x.partial_cmp(&y));
            }),
            ("num", &mut |((x, y), _, _)| {
                no_out!(num_partial_cmp_primitive(&x, y));
            }),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_partial_cmp_signed_library_comparison<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrd<T>,
    rug::Integer: PartialOrd<T>,
{
    run_benchmark(
        &format!("Integer.partial_cmp(&{})", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_signed_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.partial_cmp(&y))),
            ("rug", &mut |((x, y), _)| no_out!(x.partial_cmp(&y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_unsigned_partial_cmp_integer_library_comparison<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp(&Integer)", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y.partial_cmp(&x))),
            ("rug", &mut |((x, y), _)| no_out!(y.partial_cmp(&x))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_signed_partial_cmp_integer_library_comparison<
    T: PartialOrd<Integer> + PartialOrd<rug::Integer> + PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp(&Integer)", T::NAME),
        BenchmarkType::LibraryComparison,
        integer_signed_pair_gen_rm::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(y.partial_cmp(&x))),
            ("rug", &mut |((x, y), _)| no_out!(y.partial_cmp(&x))),
        ],
    );
}
