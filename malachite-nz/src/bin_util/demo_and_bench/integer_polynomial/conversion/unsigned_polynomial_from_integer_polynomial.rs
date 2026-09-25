// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_convertible_from_integer_polynomial
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_convertible_from_integer_polynomial_algorithms
    );
    register_unsigned_demos!(runner, demo_unsigned_polynomial_try_from_integer_polynomial);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_try_from_integer_polynomial_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_try_from_integer_polynomial<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedPolynomial::<{}>::try_from({}) = {:?}",
            T::NAME,
            p,
            UnsignedPolynomial::<T>::try_from(&p).map(|q| q.to_string())
        );
    }
}

fn benchmark_unsigned_polynomial_try_from_integer_polynomial_evaluation_strategy<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial::<{}>::try_from(IntegerPolynomial)",
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            (
                "UnsignedPolynomial::<T>::try_from(IntegerPolynomial)",
                &mut |p| {
                    let _ = UnsignedPolynomial::<T>::try_from(p);
                },
            ),
            (
                "UnsignedPolynomial::<T>::try_from(&IntegerPolynomial)",
                &mut |p| {
                    let _ = UnsignedPolynomial::<T>::try_from(&p);
                },
            ),
        ],
    );
}

fn demo_unsigned_polynomial_convertible_from_integer_polynomial<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer> + for<'a> TryFrom<&'a Integer>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedPolynomial::<{}>::convertible_from(&{}) = {}",
            T::NAME,
            p,
            UnsignedPolynomial::<T>::convertible_from(&p)
        );
    }
}

fn benchmark_unsigned_polynomial_convertible_from_integer_polynomial_algorithms<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer> + for<'a> TryFrom<&'a Integer>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial::<{}>::convertible_from(&IntegerPolynomial)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("standard", &mut |p| {
                no_out!(UnsignedPolynomial::<T>::convertible_from(&p));
            }),
            ("using try_from", &mut |p| {
                no_out!(UnsignedPolynomial::<T>::try_from(&p).is_ok());
            }),
        ],
    );
}
