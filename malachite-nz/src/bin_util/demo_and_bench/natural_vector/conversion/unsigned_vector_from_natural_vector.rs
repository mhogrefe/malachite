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
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_vector_convertible_from_natural_vector);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_convertible_from_natural_vector_algorithms
    );
    register_unsigned_demos!(runner, demo_unsigned_vector_try_from_natural_vector);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_vector_try_from_natural_vector_evaluation_strategy
    );
}

fn demo_unsigned_vector_try_from_natural_vector<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedVector::<{}>::try_from({}) = {:?}",
            T::NAME,
            v,
            UnsignedVector::<T>::try_from(&v).map(|w| w.to_string())
        );
    }
}

fn benchmark_unsigned_vector_try_from_natural_vector_evaluation_strategy<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("UnsignedVector::<{}>::try_from(NaturalVector)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [
            ("UnsignedVector::<T>::try_from(NaturalVector)", &mut |v| {
                let _ = UnsignedVector::<T>::try_from(v);
            }),
            ("UnsignedVector::<T>::try_from(&NaturalVector)", &mut |v| {
                let _ = UnsignedVector::<T>::try_from(&v);
            }),
        ],
    );
}

fn demo_unsigned_vector_convertible_from_natural_vector<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedVector::<{}>::convertible_from(&{}) = {}",
            T::NAME,
            v,
            UnsignedVector::<T>::convertible_from(&v)
        );
    }
}

fn benchmark_unsigned_vector_convertible_from_natural_vector_algorithms<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedVector::<{}>::convertible_from(&NaturalVector)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [
            ("standard", &mut |v| {
                no_out!(UnsignedVector::<T>::convertible_from(&v));
            }),
            ("using try_from", &mut |v| {
                no_out!(UnsignedVector::<T>::try_from(&v).is_ok());
            }),
        ],
    );
}
