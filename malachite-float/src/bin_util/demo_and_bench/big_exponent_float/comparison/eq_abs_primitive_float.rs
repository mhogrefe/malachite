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
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::pair_1_big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_primitive_float_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_big_exponent_float_eq_abs_primitive_float);
    register_primitive_float_demos!(runner, demo_primitive_float_eq_abs_big_exponent_float);

    register_primitive_float_benches!(runner, benchmark_big_exponent_float_eq_abs_primitive_float);
}

fn demo_big_exponent_float_eq_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BigExponentFloat: EqAbs<T>,
{
    for (x, y) in big_exponent_float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let ny = NiceFloat(y);
        if x.eq_abs(&y) {
            println!("|{x:#x}| = |{ny}|");
        } else {
            println!("|{x:#x}| ≠ |{ny}|");
        }
    }
}

fn demo_primitive_float_eq_abs_big_exponent_float<T: PrimitiveFloat + EqAbs<BigExponentFloat>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in big_exponent_float_primitive_float_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let ny = NiceFloat(y);
        if y.eq_abs(&x) {
            println!("|{ny}| = |{x:#x}|");
        } else {
            println!("|{ny}| ≠ |{x:#x}|");
        }
    }
}

fn benchmark_big_exponent_float_eq_abs_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BigExponentFloat: EqAbs<T>,
{
    run_benchmark(
        "BigExponentFloat.eq_abs(&T)",
        BenchmarkType::Single,
        big_exponent_float_primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.eq_abs(&y));
        })],
    );
}
