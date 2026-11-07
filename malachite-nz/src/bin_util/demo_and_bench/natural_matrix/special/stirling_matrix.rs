// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::number_theory::traits::{
    StirlingMatrixOfTheSecondKind, UnsignedStirlingMatrixOfTheFirstKind,
};
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_28;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_matrix::NaturalMatrix;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_matrix_unsigned_stirling_matrix_of_the_first_kind
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_unsigned_stirling_matrix_of_the_first_kind
    );
    register_demo!(
        runner,
        demo_natural_matrix_stirling_matrix_of_the_second_kind
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_stirling_matrix_of_the_second_kind
    );
}

fn demo_natural_matrix_unsigned_stirling_matrix_of_the_first_kind(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    // An m-by-n matrix holds entries of up to about m log m bits, so the dimensions are kept to u8
    // scale.
    for (height, width) in unsigned_pair_gen_var_28::<u8, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "unsigned_stirling_matrix_of_the_first_kind({}, {}) = {}",
            height,
            width,
            NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(
                u64::from(height),
                u64::from(width)
            )
        );
    }
}

fn benchmark_natural_matrix_unsigned_stirling_matrix_of_the_first_kind(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(u64, u64)",
        BenchmarkType::Single,
        unsigned_pair_gen_var_28::<u8, u8>()
            .get(gm, config)
            .map(|(height, width)| (u64::from(height), u64::from(width))),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("height", "width"),
        &mut [("Malachite", &mut |(height, width)| {
            no_out!(NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(
                height, width
            ));
        })],
    );
}

fn demo_natural_matrix_stirling_matrix_of_the_second_kind(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    // An m-by-n matrix holds entries of up to about m log m bits, so the dimensions are kept to u8
    // scale.
    for (height, width) in unsigned_pair_gen_var_28::<u8, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "stirling_matrix_of_the_second_kind({}, {}) = {}",
            height,
            width,
            NaturalMatrix::stirling_matrix_of_the_second_kind(u64::from(height), u64::from(width))
        );
    }
}

fn benchmark_natural_matrix_stirling_matrix_of_the_second_kind(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix::stirling_matrix_of_the_second_kind(u64, u64)",
        BenchmarkType::Single,
        unsigned_pair_gen_var_28::<u8, u8>()
            .get(gm, config)
            .map(|(height, width)| (u64::from(height), u64::from(width))),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("height", "width"),
        &mut [("Malachite", &mut |(height, width)| {
            no_out!(NaturalMatrix::stirling_matrix_of_the_second_kind(
                height, width
            ));
        })],
    );
}
