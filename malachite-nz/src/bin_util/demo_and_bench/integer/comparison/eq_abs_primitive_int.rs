// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_signed_pair_gen, integer_unsigned_pair_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_partial_eq_abs_unsigned);
    register_signed_demos!(runner, demo_integer_partial_eq_abs_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_eq_abs_integer);
    register_signed_demos!(runner, demo_signed_partial_eq_abs_integer);

    register_unsigned_benches!(runner, benchmark_integer_eq_abs_unsigned);
    register_signed_benches!(runner, benchmark_integer_eq_abs_signed);
    register_unsigned_benches!(runner, benchmark_unsigned_eq_abs_integer);
    register_signed_benches!(runner, benchmark_signed_eq_abs_integer);
}

fn demo_integer_partial_eq_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: EqAbs<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if n.eq_abs(&u) {
            println!("|{n}| = |{u}|");
        } else {
            println!("|{n}| ≠ |{u}|");
        }
    }
}

fn demo_integer_partial_eq_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: EqAbs<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if n.eq_abs(&i) {
            println!("|{n}| = |{i}|");
        } else {
            println!("|{n}| ≠ |{i}|");
        }
    }
}

fn demo_unsigned_partial_eq_abs_integer<T: EqAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if u.eq_abs(&n) {
            println!("|{u}| = |{n}|");
        } else {
            println!("|{u}| ≠ |{n}|");
        }
    }
}

fn demo_signed_partial_eq_abs_integer<T: EqAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if i.eq_abs(&n) {
            println!("|{i}| = |{n}|");
        } else {
            println!("|{i}| ≠ |{n}|");
        }
    }
}

fn benchmark_integer_eq_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: EqAbs<T>,
{
    run_benchmark(
        &format!("Integer.eq_abs({})", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.eq_abs(&y)))],
    );
}

fn benchmark_integer_eq_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: EqAbs<T>,
{
    run_benchmark(
        &format!("Integer.eq_abs({})", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.eq_abs(&y)))],
    );
}

fn benchmark_unsigned_eq_abs_integer<T: EqAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.eq_abs(&x)))],
    );
}

fn benchmark_signed_eq_abs_integer<T: EqAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.eq_abs(&x)))],
    );
}
