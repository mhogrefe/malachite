// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, quadruple_3_vec_len_bucketer, quadruple_max_bit_bucketer,
    unsigned_direct_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_quadruple_gen_var_11, unsigned_vec_unsigned_pair_gen_var_23,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::gcd::half_gcd::{
    limbs_gcd_div, limbs_gcd_reduced, limbs_half_gcd_matrix_1_mul_vector, HalfGcdMatrix,
};
use malachite_nz::natural::arithmetic::gcd::limbs_gcd_limb;
use malachite_nz::natural::arithmetic::gcd::matrix_2_2::{
    limbs_matrix_2_2_mul, limbs_matrix_2_2_mul_small, limbs_matrix_2_2_mul_strassen,
    limbs_matrix_mul_2_2_scratch_len,
};
use malachite_nz::test_util::bench::bucketers::{
    limbs_matrix_2_2_mul_bucketer, pair_1_half_gcd_matrix_bucketer, pair_natural_max_bit_bucketer,
    triple_1_half_gcd_matrix_bucketer, triple_3_pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    large_type_gen_var_5, large_type_gen_var_6, large_type_gen_var_7, large_type_gen_var_8,
    natural_pair_gen, natural_pair_gen_nrm, natural_pair_gen_var_4, natural_pair_gen_var_4_nrm,
    unsigned_vec_pair_gen_var_10,
};
use malachite_nz::test_util::natural::arithmetic::gcd::{
    gcd_binary_nz, gcd_euclidean_nz, limbs_gcd_div_alt, limbs_gcd_div_naive, OwnedHalfGcdMatrix,
};
use num::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_gcd_limb);
    register_demo!(runner, demo_half_gcd_matrix_init);
    register_demo!(runner, demo_half_gcd_matrix_update_q);
    register_demo!(runner, demo_half_gcd_matrix_mul_matrix_1);
    register_demo!(runner, demo_half_gcd_matrix_1_mul_vector);
    register_demo!(runner, demo_limbs_matrix_2_2_mul);
    register_demo!(runner, demo_limbs_gcd_div);
    register_demo!(runner, demo_limbs_gcd_reduced);
    register_demo!(runner, demo_natural_gcd);
    register_demo!(runner, demo_natural_gcd_val_ref);
    register_demo!(runner, demo_natural_gcd_ref_val);
    register_demo!(runner, demo_natural_gcd_ref_ref);
    register_demo!(runner, demo_natural_gcd_assign);
    register_demo!(runner, demo_natural_gcd_assign_ref);
    register_demo!(runner, demo_natural_gcd_2);

    register_bench!(runner, benchmark_limbs_gcd_limb);
    register_bench!(runner, benchmark_half_gcd_matrix_init);
    register_bench!(runner, benchmark_half_gcd_matrix_update_q);
    register_bench!(runner, benchmark_half_gcd_matrix_mul_matrix_1);
    register_bench!(runner, benchmark_half_gcd_matrix_1_mul_vector);
    register_bench!(runner, benchmark_limbs_matrix_2_2_mul_algorithms);
    register_bench!(runner, benchmark_limbs_gcd_div_algorithms);
    register_bench!(runner, benchmark_limbs_gcd_reduced);
    register_bench!(runner, benchmark_natural_gcd_algorithms);
    register_bench!(runner, benchmark_natural_gcd_library_comparison);
    register_bench!(runner, benchmark_natural_gcd_evaluation_strategy);
    register_bench!(runner, benchmark_natural_gcd_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_gcd_algorithms_2);
    register_bench!(runner, benchmark_natural_gcd_library_comparison_2);
    register_bench!(runner, benchmark_natural_gcd_evaluation_strategy_2);
}

fn demo_limbs_gcd_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_23()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_gcd_limb({:?}, {}) = {}",
            xs,
            y,
            limbs_gcd_limb(&xs, y)
        );
    }
}

fn demo_half_gcd_matrix_init(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_11().get(gm, config).take(limit) {
        let scratch_len = HalfGcdMatrix::min_init_scratch(n);
        println!(
            "HalfGcdMatrix::init({}, vec![0; {}]) = {:?}",
            n,
            scratch_len,
            OwnedHalfGcdMatrix::init(n, vec![0; scratch_len])
        );
    }
}

fn demo_half_gcd_matrix_update_q(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, qs, column) in large_type_gen_var_5().get(gm, config).take(limit) {
        let old_m = m.clone();
        let mut scratch = vec![0; OwnedHalfGcdMatrix::update_q_scratch_len(&m, qs.len())];
        m.update_q(&qs, column, &mut scratch);
        println!("HalfGcdMatrix::update_q({old_m:?}, {qs:?}, {column}) = {m:?}");
    }
}

fn demo_half_gcd_matrix_mul_matrix_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut m, m_1) in large_type_gen_var_7().get(gm, config).take(limit) {
        let old_m = m.clone();
        let mut scratch = vec![0; m.n];
        m.mul_matrix_1(&m_1, &mut scratch);
        println!("m := {old_m:?}; m.mul_matrix_1({m_1:?}); m = {m:?}");
    }
}

fn demo_half_gcd_matrix_1_mul_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, mut out, xs, mut ys) in large_type_gen_var_6().get(gm, config).take(limit) {
        let old_out = out.clone();
        let old_ys = ys.clone();
        let out_len = limbs_half_gcd_matrix_1_mul_vector(&m, &mut out, &xs, &mut ys);
        println!(
            "out := {old_out:?}; ys := {old_ys:?}; \
            limbs_half_gcd_matrix_1_mul_vector({m:?}, &mut out, {xs:?}, &mut ys) = {out_len}; \
            out = {out:?}; ys = {ys:?}",
        );
    }
}

fn demo_limbs_matrix_2_2_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs00, mut xs01, mut xs10, mut xs11, xs_len, ys00, ys01, ys10, ys11) in
        large_type_gen_var_8().get(gm, config).take(limit)
    {
        let xs00_old = xs00.clone();
        let xs01_old = xs01.clone();
        let xs10_old = xs10.clone();
        let xs11_old = xs11.clone();
        let mut scratch = vec![0; limbs_matrix_mul_2_2_scratch_len(xs_len, ys00.len())];
        limbs_matrix_2_2_mul(
            &mut xs00,
            &mut xs01,
            &mut xs10,
            &mut xs11,
            xs_len,
            &ys00,
            &ys01,
            &ys10,
            &ys11,
            &mut scratch,
        );
        println!(
            "(xs00, xs01, xs10, xs11) := {:?}; \
            limbs_matrix_2_2_mul(..., {}, {:?}, {:?}, {:?}, {:?}); \
            (xs00, xs01, xs10, xs11) = {:?}",
            (xs00_old, xs01_old, xs10_old, xs11_old),
            xs_len,
            ys00,
            ys01,
            ys10,
            ys11,
            (xs00, xs01, xs10, xs11)
        );
    }
}

fn demo_limbs_gcd_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n1, n0, d1, d0) in unsigned_quadruple_gen_var_11().get(gm, config).take(limit) {
        println!(
            "limbs_gcd_div({}, {}, {}, {}) = {:?}",
            n1,
            n0,
            d1,
            d0,
            limbs_gcd_div(n1, n0, d1, d0)
        );
    }
}

fn demo_limbs_gcd_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen_var_10().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let mut out = vec![0; xs.len()];
        let out_len = limbs_gcd_reduced(&mut out, &mut xs, &mut ys);
        out.resize(out_len, 0);
        println!("limbs_gcd_reduced(&mut out, {xs_old:?}, {ys_old:?}); out = {out:?}");
    }
}

fn demo_natural_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.gcd({}) = {}", x_old, y_old, x.gcd(y));
    }
}

fn demo_natural_gcd_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.gcd(&{}) = {}", x_old, y, x.gcd(&y));
    }
}

fn demo_natural_gcd_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).gcd({}) = {}", x, y_old, (&x).gcd(y));
    }
}

fn demo_natural_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("(&{}).gcd(&{}) = {}", x, y, (&x).gcd(&y));
    }
}

fn demo_natural_gcd_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.gcd_assign(y.clone());
        println!("x := {x_old}; x.gcd_assign({y}); x = {x}");
    }
}

fn demo_natural_gcd_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.gcd_assign(&y);
        println!("x := {x_old}; x.gcd_assign(&{y}); x = {x}");
    }
}

fn demo_natural_gcd_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.gcd({}) = {}", x_old, y_old, x.gcd(y));
    }
}

fn benchmark_limbs_gcd_limb(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "HalfGcdMatrix::init(usize, Vec<Limb>)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_23().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_gcd_limb(&xs, y)))],
    );
}

fn benchmark_half_gcd_matrix_init(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_gcd_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            let scratch_len = HalfGcdMatrix::min_init_scratch(n);
            OwnedHalfGcdMatrix::init(n, vec![0; scratch_len]);
        })],
    );
}

fn benchmark_half_gcd_matrix_update_q(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "HalfGcdMatrix::update_q(&[Limb], u8, &mut [Limb])",
        BenchmarkType::Single,
        large_type_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_half_gcd_matrix_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, qs, column)| {
            let mut scratch = vec![0; OwnedHalfGcdMatrix::update_q_scratch_len(&m, qs.len())];
            m.update_q(&qs, column, &mut scratch);
        })],
    );
}

fn benchmark_half_gcd_matrix_mul_matrix_1(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "HalfGcdMatrix::mul_matrix_1(&HalfGcdMatrix1, &mut [Limb])",
        BenchmarkType::Single,
        large_type_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_half_gcd_matrix_bucketer("m"),
        &mut [("Malachite", &mut |(mut m, m_1)| {
            let mut scratch = vec![0; m.n];
            m.mul_matrix_1(&m_1, &mut scratch);
        })],
    );
}

fn benchmark_half_gcd_matrix_1_mul_vector(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_half_gcd_matrix_1_mul_vector(&HalfGcdMatrix1, &mut [Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        large_type_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(m, mut out, xs, mut ys)| {
            no_out!(limbs_half_gcd_matrix_1_mul_vector(
                &m, &mut out, &xs, &mut ys
            ))
        })],
    );
}

fn benchmark_limbs_matrix_2_2_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_matrix_2_2_mul",
        BenchmarkType::Algorithms,
        large_type_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &limbs_matrix_2_2_mul_bucketer(),
        &mut [
            ("default", &mut |(
                mut xs00,
                mut xs01,
                mut xs10,
                mut xs11,
                xs_len,
                ys00,
                ys01,
                ys10,
                ys11,
            )| {
                let scratch_len = 3 * (xs_len + ys00.len()) + 5;
                let mut scratch = vec![0; scratch_len];
                limbs_matrix_2_2_mul(
                    &mut xs00,
                    &mut xs01,
                    &mut xs10,
                    &mut xs11,
                    xs_len,
                    &ys00,
                    &ys01,
                    &ys10,
                    &ys11,
                    &mut scratch,
                );
            }),
            ("small", &mut |(
                mut xs00,
                mut xs01,
                mut xs10,
                mut xs11,
                xs_len,
                ys00,
                ys01,
                ys10,
                ys11,
            )| {
                let scratch_len = 3 * (xs_len + ys00.len()) + 5;
                let mut scratch = vec![0; scratch_len];
                limbs_matrix_2_2_mul_small(
                    &mut xs00,
                    &mut xs01,
                    &mut xs10,
                    &mut xs11,
                    xs_len,
                    &ys00,
                    &ys01,
                    &ys10,
                    &ys11,
                    &mut scratch,
                );
            }),
            ("Strassen", &mut |(
                mut xs00,
                mut xs01,
                mut xs10,
                mut xs11,
                xs_len,
                ys00,
                ys01,
                ys10,
                ys11,
            )| {
                let scratch_len = 3 * (xs_len + ys00.len()) + 5;
                let mut scratch = vec![0; scratch_len];
                limbs_matrix_2_2_mul_strassen(
                    &mut xs00,
                    &mut xs01,
                    &mut xs10,
                    &mut xs11,
                    xs_len,
                    &ys00,
                    &ys01,
                    &ys10,
                    &ys11,
                    &mut scratch,
                );
            }),
        ],
    );
}

fn benchmark_limbs_gcd_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_gcd_div(Limb, Limb, Limb, Limb)",
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("n1", "n0", "d1", "d0"),
        &mut [
            ("default", &mut |(n1, n0, d1, d0)| {
                no_out!(limbs_gcd_div(n1, n0, d1, d0))
            }),
            ("alt", &mut |(n1, n0, d1, d0)| {
                no_out!(limbs_gcd_div_alt(n1, n0, d1, d0))
            }),
            ("naive", &mut |(n1, n0, d1, d0)| {
                no_out!(limbs_gcd_div_naive(n1, n0, d1, d0))
            }),
        ],
    );
}

fn benchmark_limbs_gcd_reduced(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_gcd_reduced(&mut [Limb], &mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, mut ys)| {
            let mut out = vec![0; xs.len()];
            limbs_gcd_reduced(&mut out, &mut xs, &mut ys);
        })],
    );
}

fn benchmark_natural_gcd_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Euclidean", &mut |(x, y)| no_out!(gcd_euclidean_nz(x, y))),
            ("binary", &mut |(x, y)| no_out!(gcd_binary_nz(x, y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_gcd_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.gcd(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.gcd(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.gcd(&y))),
        ],
    );
}

fn benchmark_natural_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.gcd(Natural)", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Natural.gcd(&Natural)", &mut |(x, y)| no_out!(x.gcd(&y))),
            ("&Natural.gcd(Natural)", &mut |(x, y)| no_out!((&x).gcd(y))),
            (
                "&Natural.gcd(&Natural)",
                &mut |(x, y)| no_out!((&x).gcd(&y)),
            ),
        ],
    );
}

fn benchmark_natural_gcd_algorithms_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Euclidean", &mut |(x, y)| no_out!(gcd_euclidean_nz(x, y))),
            ("binary", &mut |(x, y)| no_out!(gcd_binary_nz(x, y))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_gcd_library_comparison_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_4_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.gcd(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.gcd(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.gcd(&y))),
        ],
    );
}

fn benchmark_natural_gcd_evaluation_strategy_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.gcd(Natural)", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Natural.gcd(&Natural)", &mut |(x, y)| no_out!(x.gcd(&y))),
            ("&Natural.gcd(Natural)", &mut |(x, y)| no_out!((&x).gcd(y))),
            (
                "&Natural.gcd(&Natural)",
                &mut |(x, y)| no_out!((&x).gcd(&y)),
            ),
        ],
    );
}

fn benchmark_natural_gcd_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.gcd(Natural)", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Natural.gcd(&Natural)", &mut |(x, y)| no_out!(x.gcd(&y))),
        ],
    );
}
