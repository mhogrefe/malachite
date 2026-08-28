// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::bench::bucketers::gaussian_rational_bit_bucketer;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::fmt::Debug;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_primitive_float_try_from_gaussian_rational);
    register_primitive_float_demos!(
        runner,
        demo_primitive_float_convertible_from_gaussian_rational
    );

    register_primitive_float_benches!(runner, benchmark_primitive_float_try_from_gaussian_rational);
    register_primitive_float_benches!(
        runner,
        benchmark_primitive_float_convertible_from_gaussian_rational
    );
}

fn demo_primitive_float_try_from_gaussian_rational<
    T: for<'a> TryFrom<&'a GaussianRational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> <T as TryFrom<&'a GaussianRational>>::Error: Debug,
{
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!("{}::try_from(&{}) = {:?}", T::NAME, x, T::try_from(&x));
    }
}

fn demo_primitive_float_convertible_from_gaussian_rational<
    T: for<'a> ConvertibleFrom<&'a GaussianRational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in gaussian_rational_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            x,
            if T::convertible_from(&x) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn benchmark_primitive_float_try_from_gaussian_rational<
    T: for<'a> TryFrom<&'a GaussianRational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::try_from(&GaussianRational)", T::NAME),
        BenchmarkType::Single,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::try_from(&x).ok()))],
    );
}

fn benchmark_primitive_float_convertible_from_gaussian_rational<
    T: for<'a> ConvertibleFrom<&'a GaussianRational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(&GaussianRational)", T::NAME),
        BenchmarkType::Single,
        gaussian_rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::convertible_from(&x)))],
    );
}
