// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_27, unsigned_gen_var_28,
};
use malachite_base::test_util::num::arithmetic::primorial::{
    checked_primorial_naive, checked_product_of_first_n_primes_naive,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_primorial);
    register_unsigned_demos!(runner, demo_checked_primorial);
    register_unsigned_demos!(runner, demo_product_of_first_n_primes);
    register_unsigned_demos!(runner, demo_checked_product_of_first_n_primes);

    register_unsigned_benches!(runner, benchmark_primorial_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_primorial);
    register_unsigned_benches!(runner, benchmark_product_of_first_n_primes_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_product_of_first_n_primes);
}

fn demo_primorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_27::<T>().get(gm, config).take(limit) {
        println!("{}# = {}", n, T::primorial(n));
    }
}

fn demo_checked_primorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("{}# = {:?}", n, T::checked_primorial(n));
    }
}

fn demo_product_of_first_n_primes<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_28::<T>().get(gm, config).take(limit) {
        println!("p_{}# = {}", n, T::product_of_first_n_primes(n));
    }
}

fn demo_checked_product_of_first_n_primes<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("p_{}# = {:?}", n, T::checked_product_of_first_n_primes(n));
    }
}

fn benchmark_primorial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::primorial(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::primorial(n))),
            ("naive", &mut |n| {
                no_out!(checked_primorial_naive::<T>(n).unwrap())
            }),
        ],
    );
}

fn benchmark_checked_primorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_primorial(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_primorial(n)))],
    );
}

fn benchmark_product_of_first_n_primes_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::product_of_first_n_primes(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_28::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::product_of_first_n_primes(n))),
            ("naive", &mut |n| {
                no_out!(checked_product_of_first_n_primes_naive::<T>(n).unwrap())
            }),
        ],
    );
}

fn benchmark_checked_product_of_first_n_primes<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_product_of_first_n_primes(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(T::checked_product_of_first_n_primes(n))
        })],
    );
}
