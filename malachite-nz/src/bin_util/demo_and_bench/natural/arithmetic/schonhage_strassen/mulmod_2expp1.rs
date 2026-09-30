// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::quadruple_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use malachite_nz::test_util::generators::{
    large_type_gen_var_33, large_type_gen_var_49, large_type_gen_var_50, large_type_gen_var_59,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fft_naive_convolution_1);
    register_demo!(runner, demo_fft_mulmod_2expp1_negacyclic);
    register_demo!(runner, demo_fft_mulmod_2expp1);
    register_demo!(runner, demo_fft_adjust_limbs);

    register_bench!(runner, benchmark_fft_mulmod_2expp1_algorithms);
}

fn demo_fft_naive_convolution_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ii, jj) in large_type_gen_var_33().get(gm, config).take(limit) {
        let mut r = vec![0; ii.len()];
        fft_naive_convolution_1(&mut r, &ii, &jj, ii.len());
        println!(
            "fft_naive_convolution_1(_, {ii:?}, {jj:?}, {}) = {r:?}",
            ii.len()
        );
    }
}

fn demo_fft_mulmod_2expp1_negacyclic(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut r1, i2, r_limbs, depth, w) in large_type_gen_var_50().get(gm, config).take(limit) {
        let r1_old = r1.clone();
        fft_mulmod_2expp1_negacyclic(&mut r1, i2.as_deref(), r_limbs, depth, w);
        println!(
            "fft_mulmod_2expp1_negacyclic({r1_old:?}, {i2:?}, {r_limbs}, {depth}, {w}) = {r1:?}"
        );
    }
}

fn demo_fft_mulmod_2expp1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut r, i2, n, w) in large_type_gen_var_49().get(gm, config).take(limit) {
        let r_old = r.clone();
        let mut tt = vec![0; r.len() << 1];
        fft_mulmod_2expp1(&mut r, i2.as_deref(), n, w, &mut tt);
        println!("fft_mulmod_2expp1({r_old:?}, {i2:?}, {n}, {w}, _) = {r:?}");
    }
}

fn demo_fft_adjust_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for limbs in large_type_gen_var_59().get(gm, config).take(limit) {
        println!("fft_adjust_limbs({limbs}) = {}", fft_adjust_limbs(limbs));
    }
}

fn benchmark_fft_mulmod_2expp1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "fft_mulmod_2expp1(&mut [Limb], Option<&[Limb]>, usize, u64, &mut [Limb])",
        BenchmarkType::Algorithms,
        large_type_gen_var_49().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_vec_len_bucketer("r"),
        &mut [
            ("default", &mut |(mut r, i2, n, w)| {
                let mut tt = vec![0; r.len() << 1];
                fft_mulmod_2expp1(&mut r, i2.as_deref(), n, w, &mut tt);
            }),
            ("basecase", &mut |(mut r, i2, n, w)| {
                let limbs = r.len() - 1;
                let i2_top = i2.as_ref().map_or(r[limbs], |i2| i2[limbs]);
                let c = (r[limbs] << 1) + i2_top;
                let mut tt = vec![0; r.len() << 1];
                r[limbs] = limbs_mul_mod_2expp1_basecase(
                    &mut r,
                    i2.as_deref(),
                    c,
                    u64::exact_from(n) * w,
                    &mut tt,
                );
            }),
        ],
    );
}
