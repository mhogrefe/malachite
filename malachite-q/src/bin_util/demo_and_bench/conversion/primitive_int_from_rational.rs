// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_rounding_mode_pair_gen_var_3};
use malachite_q::Rational;
use std::fmt::Debug;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_primitive_int_try_from_rational);
    register_primitive_int_demos!(runner, demo_primitive_int_convertible_from_rational);
    register_primitive_int_demos!(runner, demo_primitive_int_rounding_from_rational);

    register_primitive_int_benches!(runner, benchmark_primitive_int_try_from_rational);
    register_primitive_int_benches!(runner, benchmark_primitive_int_convertible_from_rational);
    register_primitive_int_benches!(runner, benchmark_primitive_int_rounding_from_rational);
}

fn demo_primitive_int_try_from_rational<T: for<'a> TryFrom<&'a Rational> + PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> <T as TryFrom<&'a Rational>>::Error: Debug,
{
    for x in rational_gen().get(gm, config).take(limit) {
        println!("{}::try_from({}) = {:?}", T::NAME, x, T::try_from(&x));
    }
}

fn demo_primitive_int_convertible_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            x,
            if T::convertible_from(&x) { "" } else { "not " },
            T::NAME
        );
    }
}

fn demo_primitive_int_rounding_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt + for<'a> RoundingFrom<&'a Rational>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: PartialOrd<T>,
{
    for (x, rm) in rational_rounding_mode_pair_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            x,
            rm,
            T::rounding_from(&x, rm)
        );
    }
}

fn benchmark_primitive_int_try_from_rational<T: for<'a> TryFrom<&'a Rational> + PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::try_from(Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::try_from(&x).ok()))],
    );
}

fn benchmark_primitive_int_convertible_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(Rational)", T::NAME),
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(T::convertible_from(&x)))],
    );
}

fn benchmark_primitive_int_rounding_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt + for<'a> RoundingFrom<&'a Rational>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: PartialOrd<T>,
{
    run_benchmark(
        &format!("{}::rounding_from(Rational)", T::NAME),
        BenchmarkType::Single,
        rational_rounding_mode_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, rm)| {
            no_out!(T::rounding_from(&x, rm))
        })],
    );
}
