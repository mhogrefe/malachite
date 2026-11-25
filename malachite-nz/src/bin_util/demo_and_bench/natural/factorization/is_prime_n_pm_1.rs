// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::IsProbablePrime;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::factorization::is_prime_n_pm_1::{
    natural_divisor_in_residue_class_lenstra, natural_prove_prime,
};
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_gen_var_8, natural_gen_var_10, natural_triple_gen_var_11,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_prove_prime);
    register_demo!(runner, demo_natural_prove_prime_odd);
    register_demo!(runner, demo_natural_divisor_in_residue_class_lenstra);
    register_bench!(runner, benchmark_natural_prove_prime_algorithms);
}

fn demo_natural_prove_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_10().get(gm, config).take(limit) {
        match natural_prove_prime(&n) {
            Some(true) => println!("{n} is proven prime"),
            Some(false) => println!("{n} is proven composite"),
            None => println!("{n} is not settled"),
        }
    }
}

fn demo_natural_prove_prime_odd(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_8().get(gm, config).take(limit) {
        match natural_prove_prime(&n) {
            Some(true) => println!("{n} is proven prime"),
            Some(false) => println!("{n} is proven composite"),
            None => println!("{n} is not settled"),
        }
    }
}

fn demo_natural_divisor_in_residue_class_lenstra(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, r, s) in natural_triple_gen_var_11().get(gm, config).take(limit) {
        match natural_divisor_in_residue_class_lenstra(&n, &r, &s) {
            Some(d) => println!("lenstra({n}, {r}, {s}) = {d}"),
            None => println!("lenstra({n}, {r}, {s}) = none"),
        }
    }
}

fn benchmark_natural_prove_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "natural_prove_prime(&Natural)",
        BenchmarkType::Algorithms,
        natural_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| {
                std::hint::black_box(natural_prove_prime(&n));
            }),
            ("Baillie-PSW", &mut |n| {
                std::hint::black_box(n.is_probable_prime());
            }),
        ],
    );
}
