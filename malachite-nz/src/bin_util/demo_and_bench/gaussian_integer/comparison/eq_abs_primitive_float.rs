// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::bench::bucketers::pair_1_gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_primitive_float_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_gaussian_integer_eq_abs_float);
    register_primitive_float_demos!(runner, demo_float_eq_abs_gaussian_integer);

    register_primitive_float_benches!(runner, benchmark_gaussian_integer_eq_abs_float);
}

fn demo_gaussian_integer_eq_abs_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    GaussianInteger: EqAbs<T>,
{
    for (n, f) in gaussian_integer_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n.eq_abs(&f) {
            println!("|{n}| = |{}|", NiceFloat(f));
        } else {
            println!("|{n}| ≠ |{}|", NiceFloat(f));
        }
    }
}

fn demo_float_eq_abs_gaussian_integer<T: EqAbs<GaussianInteger> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, f) in gaussian_integer_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if f.eq_abs(&n) {
            println!("|{}| = |{n}|", NiceFloat(f));
        } else {
            println!("|{}| ≠ |{n}|", NiceFloat(f));
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_gaussian_integer_eq_abs_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    GaussianInteger: EqAbs<T>,
{
    run_benchmark(
        &format!("GaussianInteger.eq_abs(&{})", T::NAME),
        BenchmarkType::Single,
        gaussian_integer_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_gaussian_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.eq_abs(&y)))],
    );
}
