// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime};
use malachite_base::test_util::bench::bucketers::{
    primitive_int_direct_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_29, unsigned_gen_var_37,
};
use malachite_base::test_util::num::factorization::is_prime::is_prime_naive;
use malachite_base::test_util::num::factorization::is_prime_flint_3_1_2::{
    u32_is_prime_flint_3_1_2, u64_is_prime_flint_3_1_2,
};
use malachite_base::test_util::num::factorization::is_prime_pseudosquare::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u8_is_prime);
    register_demo!(runner, demo_u16_is_prime);
    register_demo!(runner, demo_u32_is_prime);
    register_demo!(runner, demo_u64_is_prime);
    register_demo!(runner, demo_usize_is_prime);
    register_demo!(runner, demo_u128_is_prime);
    register_demo!(runner, demo_u8_is_prime_primes);
    register_demo!(runner, demo_u16_is_prime_primes);
    register_demo!(runner, demo_u32_is_prime_primes);
    register_demo!(runner, demo_u64_is_prime_primes);
    register_demo!(runner, demo_usize_is_prime_primes);
    register_demo!(runner, demo_u128_is_prime_primes);

    register_bench!(runner, benchmark_u8_is_prime_algorithms);
    register_bench!(runner, benchmark_u16_is_prime_algorithms);
    register_bench!(runner, benchmark_u32_is_prime);
    register_bench!(runner, benchmark_u32_is_prime_algorithms);
    register_bench!(runner, benchmark_u64_is_prime);
    register_bench!(runner, benchmark_u64_is_prime_algorithms);
    register_bench!(runner, benchmark_u32_is_prime_algorithms_primes);
    register_bench!(runner, benchmark_u64_is_prime_algorithms_primes);
    register_bench!(runner, benchmark_usize_is_prime_algorithms);
    register_bench!(runner, benchmark_u128_is_prime_algorithms);
}

fn demo_u8_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<u8>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u16_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<u16>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u32_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<u32>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u64_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<u64>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_usize_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<usize>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u128_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<u128>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u8_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u8>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u16_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u16>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u32_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u32>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u64_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u64>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_usize_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<usize>().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn demo_u128_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_37().get(gm, config).take(limit) {
        if u.is_prime() {
            println!("{u} is prime");
        } else {
            println!("{u} is not prime");
        }
    }
}

fn benchmark_u8_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u8.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| no_out!(u.is_prime())),
            ("naive", &mut |u| no_out!(is_prime_naive(u))),
        ],
    );
}

fn benchmark_u16_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u16.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| no_out!(u.is_prime())),
            ("naive", &mut |u| no_out!(is_prime_naive(u))),
        ],
    );
}

fn benchmark_u32_is_prime(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u32.is_prime()",
        BenchmarkType::Single,
        unsigned_gen::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| {
            std::hint::black_box(u.is_prime());
        })],
    );
}

fn benchmark_u32_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                std::hint::black_box(u.is_prime());
            }),
            ("FLINT 3.1.2", &mut |u| {
                std::hint::black_box(u32_is_prime_flint_3_1_2(u));
            }),
            ("naive", &mut |u| {
                std::hint::black_box(is_prime_naive(u));
            }),
        ],
    );
}

fn benchmark_u64_is_prime(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u64.is_prime()",
        BenchmarkType::Single,
        unsigned_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| {
            std::hint::black_box(u.is_prime());
        })],
    );
}

fn benchmark_u64_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                std::hint::black_box(u.is_prime());
            }),
            ("FLINT 3.1.2", &mut |u| {
                std::hint::black_box(u64_is_prime_flint_3_1_2(u));
            }),
            ("naive", &mut |u| {
                std::hint::black_box(is_prime_naive(u));
            }),
        ],
    );
}

fn benchmark_usize_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "usize.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen::<usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| no_out!(u.is_prime())),
            ("naive", &mut |u| no_out!(is_prime_naive(u))),
        ],
    );
}

fn benchmark_u32_is_prime_algorithms_primes(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_29::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                std::hint::black_box(u.is_prime());
            }),
            ("FLINT 3.1.2", &mut |u| {
                std::hint::black_box(u32_is_prime_flint_3_1_2(u));
            }),
        ],
    );
}

fn benchmark_u64_is_prime_algorithms_primes(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_29::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                std::hint::black_box(u.is_prime());
            }),
            ("FLINT 3.1.2", &mut |u| {
                std::hint::black_box(u64_is_prime_flint_3_1_2(u));
            }),
        ],
    );
}

fn benchmark_u128_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u128.is_prime()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_37().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |u| {
                std::hint::black_box(u.is_prime());
            }),
            ("pseudosquare", &mut |u| {
                std::hint::black_box(u128_is_prime_pseudosquare(u));
            }),
            ("Baillie-PSW", &mut |u| {
                std::hint::black_box(u.is_probable_prime());
            }),
        ],
    );
}
