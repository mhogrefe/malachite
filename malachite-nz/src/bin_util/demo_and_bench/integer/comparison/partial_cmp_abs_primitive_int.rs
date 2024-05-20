// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_signed_pair_gen, integer_unsigned_pair_gen};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_partial_cmp_abs_unsigned);
    register_signed_demos!(runner, demo_integer_partial_cmp_abs_signed);
    register_unsigned_demos!(runner, demo_unsigned_partial_cmp_abs_integer);
    register_signed_demos!(runner, demo_signed_partial_cmp_abs_integer);
    register_unsigned_demos!(runner, demo_integer_lt_abs_unsigned);
    register_signed_demos!(runner, demo_integer_lt_abs_signed);
    register_unsigned_demos!(runner, demo_integer_gt_abs_unsigned);
    register_signed_demos!(runner, demo_integer_gt_abs_signed);
    register_unsigned_demos!(runner, demo_integer_le_abs_unsigned);
    register_signed_demos!(runner, demo_integer_le_abs_signed);
    register_unsigned_demos!(runner, demo_integer_ge_abs_unsigned);
    register_signed_demos!(runner, demo_integer_ge_abs_signed);
    register_unsigned_demos!(runner, demo_unsigned_lt_abs_integer);
    register_signed_demos!(runner, demo_signed_lt_abs_integer);
    register_unsigned_demos!(runner, demo_unsigned_gt_abs_integer);
    register_signed_demos!(runner, demo_signed_gt_abs_integer);
    register_unsigned_demos!(runner, demo_unsigned_le_abs_integer);
    register_signed_demos!(runner, demo_signed_le_abs_integer);
    register_unsigned_demos!(runner, demo_unsigned_ge_abs_integer);
    register_signed_demos!(runner, demo_signed_ge_abs_integer);

    register_unsigned_benches!(runner, benchmark_integer_partial_cmp_abs_unsigned);
    register_signed_benches!(runner, benchmark_integer_partial_cmp_abs_signed);
    register_unsigned_benches!(runner, benchmark_unsigned_partial_cmp_abs_integer);
    register_signed_benches!(runner, benchmark_signed_partial_cmp_abs_integer);
    register_unsigned_benches!(runner, benchmark_integer_lt_abs_unsigned);
    register_signed_benches!(runner, benchmark_integer_lt_abs_signed);
    register_unsigned_benches!(runner, benchmark_integer_gt_abs_unsigned);
    register_signed_benches!(runner, benchmark_integer_gt_abs_signed);
    register_unsigned_benches!(runner, benchmark_integer_le_abs_unsigned);
    register_signed_benches!(runner, benchmark_integer_le_abs_signed);
    register_unsigned_benches!(runner, benchmark_integer_ge_abs_unsigned);
    register_signed_benches!(runner, benchmark_integer_ge_abs_signed);
    register_unsigned_benches!(runner, benchmark_unsigned_lt_abs_integer);
    register_signed_benches!(runner, benchmark_signed_lt_abs_integer);
    register_unsigned_benches!(runner, benchmark_unsigned_gt_abs_integer);
    register_signed_benches!(runner, benchmark_signed_gt_abs_integer);
    register_unsigned_benches!(runner, benchmark_unsigned_le_abs_integer);
    register_signed_benches!(runner, benchmark_signed_le_abs_integer);
    register_unsigned_benches!(runner, benchmark_unsigned_ge_abs_integer);
    register_signed_benches!(runner, benchmark_signed_ge_abs_integer);
}

fn demo_integer_partial_cmp_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialOrdAbs<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        match n.partial_cmp_abs(&u).unwrap() {
            Less => println!("|{n}| < |{u}|"),
            Equal => println!("|{n}| = |{u}|"),
            Greater => println!("|{n}| > |{u}|"),
        }
    }
}

fn demo_integer_partial_cmp_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialOrdAbs<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        match n.partial_cmp_abs(&i).unwrap() {
            Less => println!("|{n}| < |{i}|"),
            Equal => println!("|{n}| = |{i}|"),
            Greater => println!("|{n}| > |{i}|"),
        }
    }
}

fn demo_unsigned_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        match u.partial_cmp_abs(&n).unwrap() {
            Less => println!("|{u}| < |{n}|"),
            Equal => println!("|{u}| = |{n}|"),
            Greater => println!("|{u}| > |{n}|"),
        }
    }
}

fn demo_signed_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        match i.partial_cmp_abs(&n).unwrap() {
            Less => println!("|{i}| < |{n}|"),
            Equal => println!("|{i}| = |{n}|"),
            Greater => println!("|{i}| > |{n}|"),
        }
    }
}

fn demo_integer_lt_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if n.lt_abs(&u) {
            println!("|{n}| < |{u}|");
        } else {
            println!("|{n}| ≮ |{u}|");
        }
    }
}

fn demo_integer_lt_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if n.lt_abs(&i) {
            println!("|{n}| < |{i}|");
        } else {
            println!("|{n}| ≮ |{i}|");
        }
    }
}

fn demo_integer_gt_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if n.gt_abs(&u) {
            println!("|{n}| > |{u}|");
        } else {
            println!("|{n}| ≯ |{u}|");
        }
    }
}

fn demo_integer_gt_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if n.gt_abs(&i) {
            println!("|{n}| > |{i}|");
        } else {
            println!("|{n}| ≯ |{i}|");
        }
    }
}

fn demo_integer_le_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if n.le_abs(&u) {
            println!("|{n}| ≤ |{u}|");
        } else {
            println!("|{n}| ≰ |{u}|");
        }
    }
}

fn demo_integer_le_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if n.le_abs(&i) {
            println!("|{n}| ≤ |{i}|");
        } else {
            println!("|{n}| ≰ |{i}|");
        }
    }
}

fn demo_integer_ge_abs_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if n.ge_abs(&u) {
            println!("|{n}| ≥ |{u}|");
        } else {
            println!("|{n}| ≱ |{u}|");
        }
    }
}

fn demo_integer_ge_abs_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if n.ge_abs(&i) {
            println!("|{n}| ≥ |{i}|");
        } else {
            println!("|{n}| ≱ |{i}|");
        }
    }
}

fn demo_unsigned_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if u.lt_abs(&n) {
            println!("|{u}| < |{n}|");
        } else {
            println!("|{u}| ≮ |{n}|");
        }
    }
}

fn demo_signed_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if i.lt_abs(&n) {
            println!("|{i}| < |{n}|");
        } else {
            println!("|{i}| ≮ |{n}|");
        }
    }
}

fn demo_unsigned_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if u.gt_abs(&n) {
            println!("|{u}| > |{n}|");
        } else {
            println!("|{u}| ≯ |{n}|");
        }
    }
}

fn demo_signed_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if i.gt_abs(&n) {
            println!("|{i}| > |{n}|");
        } else {
            println!("|{i}| ≯ |{n}|");
        }
    }
}

fn demo_unsigned_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if u.le_abs(&n) {
            println!("|{u}| ≤ |{n}|");
        } else {
            println!("|{u}| ≰ |{n}|");
        }
    }
}

fn demo_signed_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if i.le_abs(&n) {
            println!("|{i}| ≤ |{n}|");
        } else {
            println!("|{i}| ≰ |{n}|");
        }
    }
}

fn demo_unsigned_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u) in integer_unsigned_pair_gen::<T>().get(gm, config).take(limit) {
        if u.ge_abs(&n) {
            println!("|{u}| ≥ |{n}|");
        } else {
            println!("|{u}| ≱ |{n}|");
        }
    }
}

fn demo_signed_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in integer_signed_pair_gen::<T>().get(gm, config).take(limit) {
        if i.ge_abs(&n) {
            println!("|{i}| ≥ |{n}|");
        } else {
            println!("|{i}| ≱ |{n}|");
        }
    }
}

fn benchmark_integer_partial_cmp_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_integer_partial_cmp_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.partial_cmp_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_unsigned_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}

fn benchmark_signed_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}

fn benchmark_integer_lt_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_integer_lt_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.lt_abs(&y)))],
    );
}

fn benchmark_integer_gt_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_integer_gt_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.gt_abs(&y)))],
    );
}

fn benchmark_integer_le_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_integer_le_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.le_abs(&y)))],
    );
}

fn benchmark_integer_ge_abs_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}

fn benchmark_integer_ge_abs_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.ge_abs(&y)))],
    );
}

fn benchmark_unsigned_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.lt_abs(&x)))],
    );
}

fn benchmark_signed_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.lt_abs(&x)))],
    );
}

fn benchmark_unsigned_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.gt_abs(&x)))],
    );
}

fn benchmark_signed_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.gt_abs(&x)))],
    );
}

fn benchmark_unsigned_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.le_abs(&x)))],
    );
}

fn benchmark_signed_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.le_abs(&x)))],
    );
}

fn benchmark_unsigned_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.ge_abs(&x)))],
    );
}

fn benchmark_signed_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.ge_abs(&x)))],
    );
}
