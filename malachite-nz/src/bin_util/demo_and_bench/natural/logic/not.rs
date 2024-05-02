// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{pair_2_vec_len_bucketer, vec_len_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vec_pair_gen_var_1};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::not::{limbs_not, limbs_not_in_place, limbs_not_to_out};
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_not);
    register_demo!(runner, demo_limbs_not_to_out);
    register_demo!(runner, demo_limbs_not_in_place);
    register_demo!(runner, demo_natural_not);
    register_demo!(runner, demo_natural_not_ref);

    register_bench!(runner, benchmark_limbs_not);
    register_bench!(runner, benchmark_limbs_not_to_out);
    register_bench!(runner, benchmark_limbs_not_in_place);
    register_bench!(runner, benchmark_natural_not_library_comparison);
    register_bench!(runner, benchmark_natural_not_evaluation_strategy);
}

fn demo_limbs_not(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen().get(gm, config).take(limit) {
        println!("limbs_not({:?}) = {:?}", xs, limbs_not(&xs));
    }
}

fn demo_limbs_not_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_1().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_not_to_out(&mut out, &xs);
        println!("out := {out_old:?}; limbs_not_to_out(&mut out, &{xs:?}); out = {out:?}");
    }
}

fn demo_limbs_not_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        limbs_not_in_place(&mut xs);
        println!("xs := {xs_old:?}; limbs_not_in_place(&mut xs); xs = {xs:?}");
    }
}

fn demo_natural_not(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("!{} = {}", n.clone(), !n);
    }
}

fn demo_natural_not_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("!&{} = {}", n, !&n);
    }
}

fn benchmark_limbs_not(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_not(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_not(&xs)))],
    );
}

fn benchmark_limbs_not_to_out(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_not_to_out(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs)| {
            limbs_not_to_out(&mut out, &xs)
        })],
    );
}

fn benchmark_limbs_not_in_place(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_not_in_place(&mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut xs| limbs_not_in_place(&mut xs))],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_not_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "!Natural",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(_, n)| no_out!(!n)), ("rug", &mut |(n, _)| no_out!(!n))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_not_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "!Natural",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("-Natural", &mut |n| no_out!(!n)), ("-&Natural", &mut |n| no_out!(!&n))],
    );
}
