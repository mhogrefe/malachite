// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_primitive_float_pair_gen;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_integer_partial_cmp_abs_float);
    register_primitive_float_demos!(runner, demo_float_partial_cmp_abs_integer);

    register_primitive_float_benches!(runner, benchmark_integer_partial_cmp_abs_float);
    register_primitive_float_benches!(runner, benchmark_float_partial_cmp_abs_integer);
}

fn demo_integer_partial_cmp_abs_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Integer: PartialOrdAbs<T>,
{
    for (n, f) in integer_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match n.partial_cmp_abs(&f) {
            None => println!("{} is not comparable with {}", n, NiceFloat(f)),
            Some(Less) => println!("|{}| < |{}|", n, NiceFloat(f)),
            Some(Equal) => println!("|{}| = |{}|", n, NiceFloat(f)),
            Some(Greater) => println!("|{}| > |{}|", n, NiceFloat(f)),
        }
    }
}

fn demo_float_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, f) in integer_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        match f.partial_cmp_abs(&n) {
            None => println!("{} is not comparable with {}", NiceFloat(f), n),
            Some(Less) => println!("|{}| < |{}|", NiceFloat(f), n),
            Some(Equal) => println!("|{}| = |{}|", NiceFloat(f), n),
            Some(Greater) => println!("|{}| > |{}|", NiceFloat(f), n),
        }
    }
}

fn benchmark_integer_partial_cmp_abs_float<T: PrimitiveFloat>(
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
        integer_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.partial_cmp_abs(&y)))],
    );
}

fn benchmark_float_partial_cmp_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.partial_cmp_abs(&x)))],
    );
}
