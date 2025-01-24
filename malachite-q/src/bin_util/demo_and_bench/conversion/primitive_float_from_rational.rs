// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, pair_2_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_rm, rational_gen_var_4, rational_rounding_mode_pair_gen_var_5,
};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_rounding_from_rational);
    register_primitive_float_demos!(runner, demo_float_rounding_from_rational_ref);
    register_primitive_float_demos!(runner, demo_float_try_from_rational);
    register_primitive_float_demos!(runner, demo_float_try_from_rational_ref);
    register_primitive_float_demos!(runner, demo_float_exact_from_rational);
    register_primitive_float_demos!(runner, demo_float_exact_from_rational_ref);
    register_primitive_float_demos!(runner, demo_float_convertible_from_rational);
    register_primitive_float_demos!(runner, demo_float_convertible_from_rational_ref);

    register_primitive_float_benches!(
        runner,
        benchmark_float_rounding_from_rational_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_f32_rounding_from_down_rational_library_comparison
    );
    register_bench!(
        runner,
        benchmark_f64_rounding_from_down_rational_library_comparison
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_try_from_rational_evaluation_strategy
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_exact_from_rational_evaluation_strategy
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_convertible_from_rational_evaluation_strategy
    );
}

fn demo_float_rounding_from_rational<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat + RoundingFrom<Rational>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: TryFrom<T>,
{
    for (n, rm) in rational_rounding_mode_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = T::rounding_from(n.clone(), rm);
        println!(
            "{}::rounding_from({}, {}) = {:?}",
            T::NAME,
            n,
            rm,
            (NiceFloat(f), o)
        );
    }
}

fn demo_float_rounding_from_rational_ref<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat + for<'a> RoundingFrom<&'a Rational>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: TryFrom<T>,
{
    for (n, rm) in rational_rounding_mode_pair_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = T::rounding_from(&n, rm);
        println!(
            "{}::rounding_from(&{}, {}) = {:?}",
            T::NAME,
            n,
            rm,
            (NiceFloat(f), o)
        );
    }
}

fn demo_float_try_from_rational<
    T: TryFrom<Rational, Error = FloatConversionError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in rational_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from({}) = {:?}",
            T::NAME,
            n.clone(),
            T::try_from(n).map(NiceFloat)
        );
    }
}

fn demo_float_try_from_rational_ref<
    T: for<'a> TryFrom<&'a Rational, Error = FloatConversionError> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in rational_gen().get(gm, config).take(limit) {
        println!(
            "{}::try_from(&{}) = {:?}",
            T::NAME,
            n,
            T::try_from(&n).map(NiceFloat)
        );
    }
}

fn demo_float_exact_from_rational<T: ExactFrom<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: TryFrom<T>,
{
    for n in rational_gen_var_4::<T>().get(gm, config).take(limit) {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            n.clone(),
            NiceFloat(T::exact_from(n))
        );
    }
}

fn demo_float_exact_from_rational_ref<T: for<'a> ExactFrom<&'a Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: TryFrom<T>,
{
    for n in rational_gen_var_4::<T>().get(gm, config).take(limit) {
        println!(
            "{}::exact_from(&{}) = {}",
            T::NAME,
            n,
            NiceFloat(T::exact_from(&n))
        );
    }
}

fn demo_float_convertible_from_rational<T: ConvertibleFrom<Rational> + PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in rational_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        if T::convertible_from(n) {
            println!("{} is convertible to an {}", n_old, T::NAME);
        } else {
            println!("{} is not convertible to an {}", n_old, T::NAME);
        }
    }
}

fn demo_float_convertible_from_rational_ref<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in rational_gen().get(gm, config).take(limit) {
        if T::convertible_from(&n) {
            println!("{} is convertible to an {}", n, T::NAME);
        } else {
            println!("{} is not convertible to an {}", n, T::NAME);
        }
    }
}

fn benchmark_float_rounding_from_rational_evaluation_strategy<
    T: for<'a> ConvertibleFrom<&'a Rational>
        + PrimitiveFloat
        + RoundingFrom<Rational>
        + for<'a> RoundingFrom<&'a Rational>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::rounding_from(Rational, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_rounding_mode_pair_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [
            (
                &format!("{}::rounding_from(Rational, RoundingMode)", T::NAME),
                &mut |(n, rm)| no_out!(T::rounding_from(n, rm)),
            ),
            (
                &format!("{}::rounding_from(&Rational, RoundingMode)", T::NAME),
                &mut |(n, rm)| no_out!(T::rounding_from(&n, rm)),
            ),
        ],
    );
}

fn benchmark_f32_rounding_from_down_rational_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "f32::rounding_from(Rational, Down)",
        BenchmarkType::LibraryComparison,
        rational_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| {
                no_out!(f32::rounding_from(n, Down))
            }),
            ("rug", &mut |(n, _)| no_out!(n.to_f32())),
        ],
    );
}

fn benchmark_f64_rounding_from_down_rational_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "f64::rounding_from(Rational, Down)",
        BenchmarkType::LibraryComparison,
        rational_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_rational_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| {
                no_out!(f64::rounding_from(n, Down))
            }),
            ("rug", &mut |(n, _)| no_out!(n.to_f64())),
        ],
    );
}

fn benchmark_float_try_from_rational_evaluation_strategy<
    T: TryFrom<Rational> + for<'a> TryFrom<&'a Rational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::try_from(Rational)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("n"),
        &mut [
            (&format!("{}::try_from(Rational)", T::NAME), &mut |n| {
                no_out!(T::try_from(n).ok())
            }),
            (&format!("{}::try_from(&Rational)", T::NAME), &mut |n| {
                no_out!(T::try_from(&n).ok())
            }),
        ],
    );
}

fn benchmark_float_exact_from_rational_evaluation_strategy<
    T: ExactFrom<Rational> + for<'a> ExactFrom<&'a Rational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::exact_from(Rational)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("n"),
        &mut [
            (&format!("{}::exact_from(Rational)", T::NAME), &mut |n| {
                no_out!(T::exact_from(n))
            }),
            (&format!("{}::exact_from(&Rational)", T::NAME), &mut |n| {
                no_out!(T::exact_from(&n))
            }),
        ],
    );
}

fn benchmark_float_convertible_from_rational_evaluation_strategy<
    T: ConvertibleFrom<Rational> + for<'a> ConvertibleFrom<&'a Rational> + PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(Rational)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("n"),
        &mut [
            (
                &format!("{}::convertible_from(Rational)", T::NAME),
                &mut |n| no_out!(T::convertible_from(n)),
            ),
            (
                &format!("{}::convertible_from(&Rational)", T::NAME),
                &mut |n| no_out!(T::convertible_from(&n)),
            ),
        ],
    );
}
