// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::rational_primitive_float_pair_gen;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_eq_abs_primitive_float);
    register_primitive_float_demos!(runner, demo_primitive_float_eq_abs_rational);

    register_primitive_float_benches!(runner, benchmark_rational_eq_abs_primitive_float);
    register_primitive_float_benches!(runner, benchmark_primitive_float_eq_abs_rational);
}

fn demo_rational_eq_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: EqAbs<T>,
{
    for (n, x) in rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if n.eq_abs(&x) {
            println!("|{}| = |{}|", n, NiceFloat(x));
        } else {
            println!("|{}| ≠ |{}|", n, NiceFloat(x));
        }
    }
}

fn demo_primitive_float_eq_abs_rational<T: EqAbs<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, x) in rational_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_abs(&n) {
            println!("|{}| = |{}|", NiceFloat(x), n);
        } else {
            println!("|{}| ≠ |{}|", NiceFloat(x), n);
        }
    }
}

fn benchmark_rational_eq_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: EqAbs<T>,
{
    run_benchmark(
        &format!("Rational.eq_abs(&{})", T::NAME),
        BenchmarkType::Single,
        rational_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.eq_abs(&y)))],
    );
}

fn benchmark_primitive_float_eq_abs_rational<T: EqAbs<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_abs(&Rational)", T::NAME),
        BenchmarkType::Single,
        rational_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| no_out!(y.eq_abs(&x)))],
    );
}
