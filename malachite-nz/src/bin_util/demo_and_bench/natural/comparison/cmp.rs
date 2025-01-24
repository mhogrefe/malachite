// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_vec_max_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_pair_gen_var_19, unsigned_vec_pair_gen_var_6, unsigned_vec_pair_gen_var_7,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::comparison::cmp::{
    limbs_cmp, limbs_cmp_normalized, limbs_cmp_same_length,
};
use malachite_nz::test_util::bench::bucketers::{
    pair_natural_max_bit_bucketer, triple_3_pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen, natural_pair_gen_nrm, natural_pair_gen_var_9,
};
use malachite_nz::test_util::natural::comparison::cmp::natural_cmp_normalized_naive;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_cmp_same_length);
    register_demo!(runner, demo_limbs_cmp);
    register_demo!(runner, demo_limbs_cmp_normalized);
    register_demo!(runner, demo_natural_cmp);
    register_demo!(runner, demo_natural_cmp_normalized);

    register_bench!(runner, benchmark_limbs_cmp_same_length);
    register_bench!(runner, benchmark_limbs_cmp);
    register_bench!(runner, benchmark_limbs_cmp_normalized);
    register_bench!(runner, benchmark_natural_cmp_library_comparison);
    register_bench!(runner, benchmark_natural_cmp_normalized_algorithms);
}

fn demo_limbs_cmp_same_length(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_6().get(gm, config).take(limit) {
        println!(
            "limbs_cmp_same_length({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_cmp_same_length(&xs, &ys),
        );
    }
}

fn demo_limbs_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_7().get(gm, config).take(limit) {
        println!("limbs_cmp({:?}, {:?}) = {:?}", xs, ys, limbs_cmp(&xs, &ys));
    }
}

fn demo_limbs_cmp_normalized(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_19().get(gm, config).take(limit) {
        println!(
            "limbs_cmp_normalized({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_cmp_normalized(&xs, &ys),
        );
    }
}

fn demo_natural_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        match x.cmp(&y) {
            Less => println!("{x} < {y}"),
            Equal => println!("{x} = {y}"),
            Greater => println!("{x} > {y}"),
        }
    }
}

fn demo_natural_cmp_normalized(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_9().get(gm, config).take(limit) {
        println!("cmp_normalized({}, {}) = {:?}", x, y, x.cmp_normalized(&y));
    }
}

fn benchmark_limbs_cmp_same_length(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_cmp_same_length(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_cmp_same_length(&xs, &ys))
        })],
    );
}

fn benchmark_limbs_cmp(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_cmp(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_cmp(&xs, &ys)))],
    );
}

fn benchmark_limbs_cmp_normalized(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_cmp_normalized(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_cmp_normalized(&xs, &ys))
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_cmp_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.cmp(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.cmp(&y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.cmp(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.cmp(&y))),
        ],
    );
}

fn benchmark_natural_cmp_normalized_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.cmp_normalized(&Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.cmp_normalized(&y))),
            ("naive", &mut |(x, y)| {
                no_out!(natural_cmp_normalized_naive(&x, &y))
            }),
        ],
    );
}
