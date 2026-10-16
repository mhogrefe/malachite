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
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_matrix_convertible_from_natural_matrix);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_convertible_from_natural_matrix_algorithms
    );
    register_unsigned_demos!(runner, demo_unsigned_matrix_try_from_natural_matrix);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_try_from_natural_matrix_evaluation_strategy
    );
}

fn demo_unsigned_matrix_try_from_natural_matrix<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedMatrix::<{}>::try_from({}) = {:?}",
            T::NAME,
            m,
            UnsignedMatrix::<T>::try_from(&m).map(|w| w.to_string())
        );
    }
}

fn benchmark_unsigned_matrix_try_from_natural_matrix_evaluation_strategy<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("UnsignedMatrix::<{}>::try_from(NaturalMatrix)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("m"),
        &mut [
            ("UnsignedMatrix::<T>::try_from(NaturalMatrix)", &mut |m| {
                let _ = UnsignedMatrix::<T>::try_from(m);
            }),
            ("UnsignedMatrix::<T>::try_from(&NaturalMatrix)", &mut |m| {
                let _ = UnsignedMatrix::<T>::try_from(&m);
            }),
        ],
    );
}

fn demo_unsigned_matrix_convertible_from_natural_matrix<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for m in natural_matrix_gen().get(gm, config).take(limit) {
        println!(
            "UnsignedMatrix::<{}>::convertible_from(&{}) = {}",
            T::NAME,
            m,
            UnsignedMatrix::<T>::convertible_from(&m)
        );
    }
}

fn benchmark_unsigned_matrix_convertible_from_natural_matrix_algorithms<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedMatrix::<{}>::convertible_from(&NaturalMatrix)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        natural_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("m"),
        &mut [
            ("standard", &mut |m| {
                no_out!(UnsignedMatrix::<T>::convertible_from(&m));
            }),
            ("using try_from", &mut |m| {
                no_out!(UnsignedMatrix::<T>::try_from(&m).is_ok());
            }),
        ],
    );
}
