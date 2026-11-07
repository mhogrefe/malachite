// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::number_theory::traits::UnsignedStirlingNumberOfTheFirstKind;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_28;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::unsigned_stirling_number_of_the_first_kind::*;
use malachite_nz::test_util::natural_polynomial::special::rising_factorial_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_unsigned_stirling_number_of_the_first_kind
    );
    register_bench!(
        runner,
        benchmark_natural_unsigned_stirling_number_of_the_first_kind_algorithms
    );
}

fn demo_natural_unsigned_stirling_number_of_the_first_kind(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    // [n, k] has up to about n log n bits, so the arguments are kept to u8 scale.
    for (n, k) in unsigned_pair_gen_var_28::<u8, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "[{}, {}] = {}",
            n,
            k,
            Natural::unsigned_stirling_number_of_the_first_kind(u64::from(n), u64::from(k))
        );
    }
}

fn benchmark_natural_unsigned_stirling_number_of_the_first_kind_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    // The three algorithms are used only for 2 <= k < n; the closed forms cover the rest.
    run_benchmark(
        "Natural::unsigned_stirling_number_of_the_first_kind(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_28::<u16, u16>()
            .get(gm, config)
            .map(|(n, k)| (u64::from(n), u64::from(k)))
            .filter(|&(n, k)| k >= 2 && k < n),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [
            ("default", &mut |(n, k)| {
                no_out!(Natural::unsigned_stirling_number_of_the_first_kind(n, k));
            }),
            ("second-order Eulerian numbers", &mut |(n, k)| {
                no_out!(stirling_1u_by_small_difference(n, k));
            }),
            ("ordinary generating function", &mut |(n, k)| {
                no_out!(stirling_1u_by_ogf(n, k));
            }),
            ("reversed ordinary generating function", &mut |(n, k)| {
                no_out!(stirling_1u_by_reversed_ogf(n, k));
            }),
            ("naive", &mut |(n, k)| {
                no_out!(
                    rising_factorial_polynomial_truncated_naive(n, k + 1)
                        .coefficient(k)
                        .clone()
                );
            }),
        ],
    );
}
