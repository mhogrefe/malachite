// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::FactoradicDigits;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::conversion::digits::factoradic_digits::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_to_factoradic_digits_asc);
    register_demo!(runner, demo_natural_to_factoradic_digits_desc);
    register_demo!(runner, demo_natural_from_factoradic_digits_asc);
    register_demo!(runner, demo_natural_from_factoradic_digits_desc);
    register_bench!(
        runner,
        benchmark_natural_to_factoradic_digits_asc_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_from_factoradic_digits_asc_algorithms
    );
}

fn demo_natural_to_factoradic_digits_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "{}.to_factoradic_digits_asc() = {:?}",
            x,
            x.to_factoradic_digits_asc()
        );
    }
}

fn demo_natural_to_factoradic_digits_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "{}.to_factoradic_digits_desc() = {:?}",
            x,
            x.to_factoradic_digits_desc()
        );
    }
}

fn demo_natural_from_factoradic_digits_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        println!(
            "Natural::from_factoradic_digits_asc({:?}) = {:?}",
            xs,
            Natural::from_factoradic_digits_asc(xs.iter().copied())
        );
    }
}

fn demo_natural_from_factoradic_digits_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        println!(
            "Natural::from_factoradic_digits_desc({:?}) = {:?}",
            xs,
            Natural::from_factoradic_digits_desc(xs.iter().copied())
        );
    }
}

fn benchmark_natural_to_factoradic_digits_asc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.to_factoradic_digits_asc()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| {
                no_out!(x.to_factoradic_digits_asc());
            }),
            ("naive", &mut |x| {
                no_out!(to_factoradic_digits_asc_naive(&x));
            }),
        ],
    );
}

fn benchmark_natural_from_factoradic_digits_asc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_factoradic_digits_asc(Iterator<Item = u64>)",
        BenchmarkType::Algorithms,
        natural_gen()
            .get(gm, config)
            .map(|x| x.to_factoradic_digits_asc()),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |xs| {
                no_out!(Natural::from_factoradic_digits_asc(xs.into_iter()));
            }),
            ("naive", &mut |xs| {
                no_out!(from_factoradic_digits_asc_naive(&xs));
            }),
        ],
    );
}
