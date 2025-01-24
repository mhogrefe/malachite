// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ExtendedGcd;
use malachite_base::test_util::bench::bucketers::pair_vec_max_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_pair_gen_var_11;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::gcd::extended_gcd::limbs_extended_gcd;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_pair_gen, natural_pair_gen_rm};
use malachite_nz::test_util::natural::arithmetic::extended_gcd::{
    extended_gcd_binary_natural, extended_gcd_euclidean_natural,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_extended_gcd);
    register_demo!(runner, demo_natural_extended_gcd);
    register_demo!(runner, demo_natural_extended_gcd_val_ref);
    register_demo!(runner, demo_natural_extended_gcd_ref_val);
    register_demo!(runner, demo_natural_extended_gcd_ref_ref);

    register_bench!(runner, benchmark_limbs_extended_gcd);
    register_bench!(runner, benchmark_natural_extended_gcd_algorithms);
    register_bench!(runner, benchmark_natural_extended_gcd_library_comparison);
    register_bench!(runner, benchmark_natural_extended_gcd_evaluation_strategy);
}

fn demo_limbs_extended_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen_var_11().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let mut gs = vec![0; ys.len()];
        let mut ss = vec![0; ys.len() + 1];
        let result = limbs_extended_gcd(&mut gs, &mut ss, &mut xs, &mut ys);
        println!(
            "limbs_gcd_extended_gcd(&mut gs, &mut ss, {xs_old:?}, {ys_old:?}) = {result:?}; \
            gs = {gs:?}, ss = {ss:?}",
        );
    }
}

fn demo_natural_extended_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.extended_gcd({}) = {:?}",
            x_old,
            y_old,
            x.extended_gcd(y)
        );
    }
}

fn demo_natural_extended_gcd_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.extended_gcd(&{}) = {:?}", x_old, y, x.extended_gcd(&y));
    }
}

fn demo_natural_extended_gcd_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).extended_gcd({}) = {:?}",
            x,
            y_old,
            (&x).extended_gcd(y)
        );
    }
}

fn demo_natural_extended_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).extended_gcd(&{}) = {:?}",
            x,
            y,
            (&x).extended_gcd(&y)
        );
    }
}

fn benchmark_limbs_extended_gcd(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_extended_gcd(&mut [Limb], &mut [Limb], &mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut xs, mut ys)| {
            let mut gs = vec![0; ys.len()];
            let mut ss = vec![0; ys.len() + 1];
            limbs_extended_gcd(&mut gs, &mut ss, &mut xs, &mut ys);
        })],
    );
}

fn benchmark_natural_extended_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.extended_gcd(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.extended_gcd(y))),
            ("Euclidean", &mut |(x, y)| {
                no_out!(extended_gcd_euclidean_natural(x, y))
            }),
            ("binary", &mut |(x, y)| {
                no_out!(extended_gcd_binary_natural(x, y))
            }),
        ],
    );
}

fn benchmark_natural_extended_gcd_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.extended_gcd(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.extended_gcd(y))),
            ("rug", &mut |((x, y), _)| {
                no_out!(x.extended_gcd(y, rug::Integer::new()))
            }),
        ],
    );
}

fn benchmark_natural_extended_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.extended_gcd(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.extended_gcd(Natural)", &mut |(x, y)| {
                no_out!(x.extended_gcd(y))
            }),
            ("Natural.extended_gcd(&Natural)", &mut |(x, y)| {
                no_out!(x.extended_gcd(&y))
            }),
            ("&Natural.extended_gcd(Natural)", &mut |(x, y)| {
                no_out!((&x).extended_gcd(y))
            }),
            ("&Natural.extended_gcd(&Natural)", &mut |(x, y)| {
                no_out!((&x).extended_gcd(&y))
            }),
        ],
    );
}
