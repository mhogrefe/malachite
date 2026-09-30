// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::quadruple_1_2_vec_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::schonhage_strassen::{
    mul_middle_to_out_schonhage_strassen, vec_get_fft, vec_set_fft,
};
use malachite_nz::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3,
    integer_vec_unsigned_unsigned_triple_gen_var_1, integer_vec_unsigned_unsigned_triple_gen_var_2,
    integer_vec_unsigned_unsigned_triple_gen_var_3, large_type_gen_var_57, large_type_gen_var_58,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mul_middle_to_out_classical);
    register_demo!(runner, demo_mul_middle_to_out_tiny_1);
    register_demo!(runner, demo_mul_middle_to_out_tiny_2);
    register_demo!(runner, demo_mul_middle_to_out);
    register_demo!(runner, demo_mul_middle_to_out_classical_square);
    register_demo!(runner, demo_mul_middle_to_out_tiny_1_square);
    register_demo!(runner, demo_mul_middle_to_out_tiny_2_square);
    register_demo!(runner, demo_mul_middle_to_out_kronecker);
    register_demo!(runner, demo_mul_middle_to_out_schonhage_strassen);
    register_demo!(runner, demo_mul_middle_to_out_kronecker_square);
    register_demo!(runner, demo_mul_middle_to_out_schonhage_strassen_square);
    register_demo!(runner, demo_mul_middle_to_out_schonhage_strassen_wide);
    register_demo!(
        runner,
        demo_mul_middle_to_out_schonhage_strassen_wide_square
    );
    register_demo!(runner, demo_vec_get_fft);
    register_demo!(runner, demo_vec_set_fft);
    register_demo!(runner, demo_mul_middle_to_out_fft);
    register_demo!(runner, demo_mul_middle_to_out_fft_square);
    register_demo!(runner, demo_mul_middle_to_out_fft_long);
    register_demo!(runner, demo_mul_middle_to_out_fft_long_square);

    register_bench!(runner, benchmark_mul_middle_to_out_algorithms);
    register_bench!(runner, benchmark_mul_middle_to_out_tiny_1_algorithms);
    register_bench!(runner, benchmark_mul_middle_to_out_tiny_2_algorithms);
}

fn demo_mul_middle_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_classical(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_classical(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_1(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_1(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_2(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_2(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_classical_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_classical(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_classical(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_1_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_1(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_1(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_2_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_2(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_2(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn benchmark_mul_middle_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_middle_to_out(&mut [Integer], &[Integer], &[Integer], usize, usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("classical", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_classical(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("default", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("naive", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                out.clone_from_slice(
                    &integers_mul_naive(&xs, &ys)[usize::exact_from(nlo)..usize::exact_from(nhi)],
                );
            }),
            ("Kronecker", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_kronecker(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("Schönhage-Strassen", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_schonhage_strassen(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("FFT", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_fft(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
        ],
    );
}

fn benchmark_mul_middle_to_out_tiny_1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_middle_to_out_tiny_1(&mut [Integer], &[Integer], &[Integer], usize, usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 1", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_tiny_1(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("tiny 2", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_tiny_2(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("classical", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_classical(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("naive", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                out.clone_from_slice(
                    &integers_mul_naive(&xs, &ys)[usize::exact_from(nlo)..usize::exact_from(nhi)],
                );
            }),
        ],
    );
}

fn benchmark_mul_middle_to_out_tiny_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_middle_to_out_tiny_2(&mut [Integer], &[Integer], &[Integer], usize, usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 2", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_tiny_2(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("classical", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_classical(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("naive", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                out.clone_from_slice(
                    &integers_mul_naive(&xs, &ys)[usize::exact_from(nlo)..usize::exact_from(nhi)],
                );
            }),
        ],
    );
}

fn demo_mul_middle_to_out_kronecker(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_kronecker(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_kronecker(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_schonhage_strassen(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_schonhage_strassen(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_schonhage_strassen(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_kronecker_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_kronecker(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_kronecker(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_schonhage_strassen_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_schonhage_strassen(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_schonhage_strassen(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

// The coefficients from `mul_middle_to_out_fft`, or `None` if it declines.
fn fft_middle(xs: &[Integer], ys: &[Integer], nlo: u64, nhi: u64) -> Option<Vec<Integer>> {
    let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
    if mul_middle_to_out_fft(
        &mut out,
        xs,
        ys,
        usize::exact_from(nlo),
        usize::exact_from(nhi),
    ) {
        Some(out)
    } else {
        None
    }
}

// The configuration with long polynomials, unless the caller chose a length, and with coefficients
// small enough that most products fit in the small-prime FFT, unless the caller chose a size. A
// config for coefficients as wide as those that FLINT multiplies by Schönhage–Strassen.
fn wide_config(config: &GenConfig) -> GenConfig {
    let mut config = config.clone();
    if config.get_or("mean_len_n", 0) == 0 {
        config.insert("mean_len_n", 20);
    }
    if config.get_or("mean_bits_n", 0) == 0 {
        config.insert("mean_bits_n", 1000);
    }
    config
}

fn demo_mul_middle_to_out_schonhage_strassen_wide(gm: GenMode, config: &GenConfig, limit: usize) {
    demo_mul_middle_to_out_schonhage_strassen(gm, &wide_config(config), limit);
}

fn demo_mul_middle_to_out_schonhage_strassen_wide_square(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    demo_mul_middle_to_out_schonhage_strassen_square(gm, &wide_config(config), limit);
}

fn long_config(config: &GenConfig) -> GenConfig {
    let mut config = config.clone();
    if config.get_or("mean_len_n", 0) == 0 {
        config.insert("mean_len_n", 300);
    }
    if config.get_or("mean_bits_n", 0) == 0 {
        config.insert("mean_bits_n", 16);
    }
    config
}

fn demo_mul_middle_to_out_fft(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let out = fft_middle(&xs, &ys, nlo, nhi);
        println!("mul_middle_to_out_fft(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_fft_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let out = fft_middle(&xs, &xs, nlo, nhi);
        println!("mul_middle_to_out_fft(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_fft_long(gm: GenMode, config: &GenConfig, limit: usize) {
    demo_mul_middle_to_out_fft(gm, &long_config(config), limit);
}

fn demo_mul_middle_to_out_fft_long_square(gm: GenMode, config: &GenConfig, limit: usize) {
    demo_mul_middle_to_out_fft_square(gm, &long_config(config), limit);
}

fn demo_vec_get_fft(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, limbs) in large_type_gen_var_57().get(gm, config).take(limit) {
        let mut coeffs_f = vec![vec![0; limbs + 1]; xs.len()];
        vec_get_fft(&mut coeffs_f, &xs, limbs);
        println!("vec_get_fft(_, {xs:?}, {limbs}) = {coeffs_f:?}");
    }
}

fn demo_vec_set_fft(gm: GenMode, config: &GenConfig, limit: usize) {
    for (coeffs_f, limbs, sign) in large_type_gen_var_58().get(gm, config).take(limit) {
        let mut out = vec![Integer::ZERO; coeffs_f.len()];
        vec_set_fft(&mut out, &coeffs_f, limbs, sign);
        println!("vec_set_fft(_, {coeffs_f:?}, {limbs}, {sign}) = {out:?}");
    }
}
