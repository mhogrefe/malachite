// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::pair_1_gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_primitive_float_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_gaussian_rational_partial_eq_float);
    register_primitive_float_demos!(runner, demo_float_partial_eq_gaussian_rational);

    register_primitive_float_benches!(runner, benchmark_gaussian_rational_partial_eq_float);
}

fn demo_gaussian_rational_partial_eq_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianRational: PartialEq<T>,
{
    for (n, f) in gaussian_rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n == f {
            println!("{n} = {}", NiceFloat(f));
        } else {
            println!("{n} ≠ {}", NiceFloat(f));
        }
    }
}

fn demo_float_partial_eq_gaussian_rational<T: PartialEq<GaussianRational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, f) in gaussian_rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if f == n {
            println!("{} = {n}", NiceFloat(f));
        } else {
            println!("{} ≠ {n}", NiceFloat(f));
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_gaussian_rational_partial_eq_float<T: PrimitiveFloat>(
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
        gaussian_rational_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x == y))],
    );
}
