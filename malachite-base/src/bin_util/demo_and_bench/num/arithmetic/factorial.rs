// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::factorial::checked_multifactorial_naive;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    unsigned_direct_bucketer, usize_convertible_pair_max_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_23, unsigned_gen_var_24, unsigned_gen_var_25,
    unsigned_pair_gen_var_12, unsigned_pair_gen_var_43,
};
use malachite_base::test_util::num::arithmetic::factorial::{
    checked_double_factorial_naive, checked_factorial_naive, checked_subfactorial_naive,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_factorial);
    register_unsigned_demos!(runner, demo_checked_factorial);
    register_unsigned_demos!(runner, demo_double_factorial);
    register_unsigned_demos!(runner, demo_checked_double_factorial);
    register_unsigned_demos!(runner, demo_multifactorial);
    register_unsigned_demos!(runner, demo_checked_multifactorial);
    register_unsigned_demos!(runner, demo_subfactorial);
    register_unsigned_demos!(runner, demo_checked_subfactorial);

    register_unsigned_benches!(runner, benchmark_factorial_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_factorial);
    register_unsigned_benches!(runner, benchmark_double_factorial_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_double_factorial);
    register_unsigned_benches!(runner, benchmark_multifactorial_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_multifactorial);
    register_unsigned_benches!(runner, benchmark_subfactorial_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_subfactorial);
}

fn demo_factorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_23::<T>().get(gm, config).take(limit) {
        println!("{}! = {}", n, T::factorial(n));
    }
}

fn demo_checked_factorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("{}! = {:?}", n, T::checked_factorial(n));
    }
}

fn demo_double_factorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_24::<T>().get(gm, config).take(limit) {
        println!("{}!! = {}", n, T::double_factorial(n));
    }
}

fn demo_checked_double_factorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("{}!! = {:?}", n, T::checked_double_factorial(n));
    }
}

fn demo_multifactorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in unsigned_pair_gen_var_43::<T>().get(gm, config).take(limit) {
        if m <= 5 {
            print!("{n}");
            for _ in 0..m {
                print!("!");
            }
            println!(" = {}", T::multifactorial(n, m));
        } else {
            println!("{}[!^{}] = {}", n, m, T::multifactorial(n, m));
        }
    }
}

fn demo_checked_multifactorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, m) in unsigned_pair_gen_var_12::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        if m <= 5 {
            print!("{n}");
            for _ in 0..m {
                print!("!");
            }
            println!(" = {:?}", T::checked_multifactorial(n, m));
        } else {
            println!("{}[!^{}] = {:?}", n, m, T::checked_multifactorial(n, m));
        }
    }
}

fn demo_subfactorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_25::<T>().get(gm, config).take(limit) {
        println!("!{} = {}", n, T::subfactorial(n));
    }
}

fn demo_checked_subfactorial<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("!{} = {:?}", n, T::checked_subfactorial(n));
    }
}

fn benchmark_factorial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::factorial(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_23::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::factorial(n))),
            ("naive", &mut |n| {
                no_out!(checked_factorial_naive::<T>(n).unwrap())
            }),
        ],
    );
}

fn benchmark_checked_factorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_factorial(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_factorial(n)))],
    );
}

fn benchmark_double_factorial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::double_factorial(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_24::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::double_factorial(n))),
            ("naive", &mut |n| {
                no_out!(checked_double_factorial_naive::<T>(n).unwrap())
            }),
        ],
    );
}

fn benchmark_checked_double_factorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_double_factorial(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(T::checked_double_factorial(n))
        })],
    );
}

fn benchmark_multifactorial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::multifactorial(u64, u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_43::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "m"),
        &mut [
            ("default", &mut |(n, m)| no_out!(T::multifactorial(n, m))),
            ("naive", &mut |(n, m)| {
                no_out!(checked_multifactorial_naive::<T>(n, m).unwrap())
            }),
        ],
    );
}

fn benchmark_checked_multifactorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_multifactorial(u64, u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "m"),
        &mut [("Malachite", &mut |(n, m)| {
            no_out!(T::checked_multifactorial(n, m))
        })],
    );
}

fn benchmark_subfactorial_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::subfactorial(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_25::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::subfactorial(n))),
            ("naive", &mut |n| {
                no_out!(checked_subfactorial_naive::<T>(n).unwrap())
            }),
        ],
    );
}

fn benchmark_checked_subfactorial<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_subfactorial(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_subfactorial(n)))],
    );
}
