// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::test_util::bench::bucketers::pair_2_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::fft::{
    limbs_square_to_out_fft, limbs_square_to_out_fft_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use malachite_nz::natural::arithmetic::square::{
    limbs_square_to_out_basecase, limbs_square_to_out_toom_2,
    limbs_square_to_out_toom_2_scratch_len, limbs_square_to_out_toom_3,
    limbs_square_to_out_toom_3_scratch_len, limbs_square_to_out_toom_4,
    limbs_square_to_out_toom_4_scratch_len, limbs_square_to_out_toom_6,
    limbs_square_to_out_toom_6_scratch_len, limbs_square_to_out_toom_8,
    limbs_square_to_out_toom_8_scratch_len,
};
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_gen, unsigned_vec_pair_gen_var_22, unsigned_vec_pair_gen_var_23,
    unsigned_vec_pair_gen_var_24, unsigned_vec_pair_gen_var_26, unsigned_vec_pair_gen_var_27,
    unsigned_vec_pair_gen_var_28,
};
use malachite_nz::test_util::natural::arithmetic::square::limbs_square_to_out_basecase_unrestricted;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_square_to_out_basecase);
    register_demo!(runner, demo_natural_square_assign);
    register_demo!(runner, demo_natural_square);
    register_demo!(runner, demo_natural_square_ref);

    register_bench!(runner, benchmark_limbs_square_to_out_basecase_algorithms);
    register_bench!(runner, benchmark_limbs_square_to_out_toom_2_algorithms);
    register_bench!(runner, benchmark_limbs_square_to_out_toom_3_algorithms);
    register_bench!(runner, benchmark_limbs_square_to_out_toom_4_algorithms);
    register_bench!(runner, benchmark_limbs_square_to_out_toom_6_algorithms);
    register_bench!(runner, benchmark_limbs_square_to_out_toom_8_algorithms);
    register_bench!(runner, benchmark_limbs_square_to_out_fft_algorithms);
    register_bench!(runner, benchmark_natural_square_assign);
    register_bench!(runner, benchmark_natural_square_algorithms);
    register_bench!(runner, benchmark_natural_square_evaluation_strategy);
}

fn demo_limbs_square_to_out_basecase(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs) in unsigned_vec_pair_gen_var_22().get(gm, config).take(limit) {
        let out_old = out.clone();
        limbs_square_to_out_basecase(&mut out, &xs);
        println!(
            "out := {out_old:?}; limbs_square_to_out_basecase(&mut out, {xs:?}); out = {out:?}",
        );
    }
}

fn demo_natural_square_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in natural_gen().get(gm, config).take(limit) {
        let old_n = n.clone();
        n.square_assign();
        println!("n := {n}; n.square_assign(); n = {old_n}");
    }
}

fn demo_natural_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("{} ^ 2 = {}", n.clone(), n.square());
    }
}

fn demo_natural_square_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("&{} ^ 2 = {}", n, (&n).square());
    }
}

fn benchmark_limbs_square_to_out_basecase_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_to_out_basecase(&mut [Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_22().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(mut out, xs)| {
                limbs_square_to_out_basecase(&mut out, &xs)
            }),
            (
                "using limbs_mul_greater_to_out_basecase",
                &mut |(mut out, xs)| limbs_mul_greater_to_out_basecase(&mut out, &xs, &xs),
            ),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_to_out_toom_2(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_23().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs)| {
                limbs_square_to_out_basecase_unrestricted(&mut out, &xs)
            }),
            ("Toom2", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_2_scratch_len(xs.len())];
                limbs_square_to_out_toom_2(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_3_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_to_out_toom_3(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_24().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom2", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_2_scratch_len(xs.len())];
                limbs_square_to_out_toom_2(&mut out, &xs, &mut scratch)
            }),
            ("Toom3", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_3_scratch_len(xs.len())];
                limbs_square_to_out_toom_3(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_4_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_to_out_toom_4(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_26().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom3", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_3_scratch_len(xs.len())];
                limbs_square_to_out_toom_3(&mut out, &xs, &mut scratch)
            }),
            ("Toom4", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_4_scratch_len(xs.len())];
                limbs_square_to_out_toom_4(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_6_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_to_out_toom_6(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom4", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_4_scratch_len(xs.len())];
                limbs_square_to_out_toom_4(&mut out, &xs, &mut scratch)
            }),
            ("Toom6", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_6_scratch_len(xs.len())];
                limbs_square_to_out_toom_6(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_square_to_out_toom_8_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_square_to_out_toom_8(&mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom6", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_6_scratch_len(xs.len())];
                limbs_square_to_out_toom_6(&mut out, &xs, &mut scratch)
            }),
            ("Toom8", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_8_scratch_len(xs.len())];
                limbs_square_to_out_toom_8(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_square_to_out_fft_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb]) for squaring",
        BenchmarkType::Algorithms,
        unsigned_vec_pair_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom8", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_toom_8_scratch_len(xs.len())];
                limbs_square_to_out_toom_8(&mut out, &xs, &mut scratch)
            }),
            ("FFT", &mut |(mut out, xs)| {
                let mut scratch = vec![0; limbs_square_to_out_fft_scratch_len(xs.len())];
                limbs_square_to_out_fft(&mut out, &xs, &mut scratch)
            }),
        ],
    );
}

fn benchmark_natural_square_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.square_assign()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |mut n| n.square_assign())],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_square_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.square()",
        BenchmarkType::Algorithms,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("standard", &mut |n| no_out!(n.square())), ("using *", &mut |n| no_out!(&n * &n))],
    );
}

fn benchmark_natural_square_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.square()",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("Natural.square()", &mut |n| no_out!(n.square())),
            ("(&Natural).square()", &mut |n| no_out!((&n).square())),
        ],
    );
}
