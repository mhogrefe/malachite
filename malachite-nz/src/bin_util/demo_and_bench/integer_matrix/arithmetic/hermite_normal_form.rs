// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{HermiteNormalForm, HermiteNormalFormAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::hermite_normal_form::*;
use malachite_nz::integer_matrix::arithmetic::hermite_normal_form_pernet_stein::hnf_pernet_stein;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    integer_matrix_bit_bucketer, pair_1_integer_matrix_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4, integer_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_hermite_normal_form);
    register_demo!(runner, demo_integer_matrix_hermite_normal_form_val);
    register_demo!(runner, demo_integer_matrix_hermite_normal_form_assign);
    register_demo!(runner, demo_integer_matrix_hermite_normal_form_low_rank);
    register_demo!(runner, demo_integer_matrix_hnf_classical);
    register_demo!(runner, demo_integer_matrix_hnf_classical_low_rank);
    register_demo!(runner, demo_integer_matrix_hnf_xgcd);
    register_demo!(runner, demo_integer_matrix_hnf_xgcd_low_rank);
    register_demo!(runner, demo_integer_matrix_hnf_minors);
    register_demo!(runner, demo_integer_matrix_hnf_modular);
    register_demo!(runner, demo_integer_matrix_hnf_modular_eldiv);
    register_demo!(runner, demo_integer_matrix_hnf_pernet_stein);
    register_demo!(runner, demo_integer_matrix_hnf_pernet_stein_low_rank);
    register_demo!(runner, demo_extended_gcd_reduced_cofactor);
    register_bench!(
        runner,
        benchmark_integer_matrix_hermite_normal_form_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_hermite_normal_form_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_hermite_normal_form_full_column_rank_algorithms
    );
}

// The determinant of the lattice spanned by the rows of `a`, which has full column rank: the
// product of the pivots of its Hermite normal form.
fn lattice_determinant(a: &IntegerMatrix) -> Natural {
    let h = hnf_classical(a);
    (0..usize::exact_from(h.width()))
        .map(|i| Natural::exact_from(&h[(i, i)]))
        .product()
}

fn demo_integer_matrix_hermite_normal_form(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("hermite_normal_form({a}) = {}", (&a).hermite_normal_form());
    }
}

fn demo_integer_matrix_hermite_normal_form_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "{a_old}.hermite_normal_form() = {}",
            a.hermite_normal_form()
        );
    }
}

fn demo_integer_matrix_hermite_normal_form_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.hermite_normal_form_assign();
        println!("a := {a_old}; a.hermite_normal_form_assign(); a = {a}");
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_hermite_normal_form_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("hermite_normal_form({a}) = {}", (&a).hermite_normal_form());
    }
}

fn demo_integer_matrix_hnf_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("hnf_classical({a}) = {}", hnf_classical(&a));
    }
}

fn demo_integer_matrix_hnf_classical_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("hnf_classical({a}) = {}", hnf_classical(&a));
    }
}

fn demo_integer_matrix_hnf_xgcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("hnf_xgcd({a}) = {}", hnf_xgcd(&a));
    }
}

fn demo_integer_matrix_hnf_xgcd_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("hnf_xgcd({a}) = {}", hnf_xgcd(&a));
    }
}

fn demo_integer_matrix_hnf_minors(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_3().get(gm, config).take(limit) {
        println!("hnf_minors({a}) = {}", hnf_minors(&a));
    }
}

// Each matrix twice: with the determinant of its lattice, and with a multiple of it.
fn demo_integer_matrix_hnf_modular(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_3().get(gm, config).take(limit) {
        let d = lattice_determinant(&a);
        println!("hnf_modular({a}, {d}) = {}", hnf_modular(&a, &d));
        let d = d * Natural::from(6u32);
        println!("hnf_modular({a}, {d}) = {}", hnf_modular(&a, &d));
    }
}

// Each matrix twice: with the determinant of its lattice, which is a multiple of its largest
// elementary divisor, and with a multiple of that.
fn demo_integer_matrix_hnf_modular_eldiv(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_3().get(gm, config).take(limit) {
        let d = lattice_determinant(&a);
        let mut h = a.clone();
        hnf_modular_eldiv(&mut h, &d);
        println!("hnf_modular_eldiv({a}, {d}) = {h}");
        let d = d * Natural::from(6u32);
        let mut h = a.clone();
        hnf_modular_eldiv(&mut h, &d);
        println!("hnf_modular_eldiv({a}, {d}) = {h}");
    }
}

fn demo_integer_matrix_hnf_pernet_stein(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("hnf_pernet_stein({a}) = {}", hnf_pernet_stein(&a));
    }
}

fn demo_integer_matrix_hnf_pernet_stein_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("hnf_pernet_stein({a}) = {}", hnf_pernet_stein(&a));
    }
}

fn demo_extended_gcd_reduced_cofactor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, g) in integer_pair_gen().get(gm, config).take(limit) {
        let (d, a, b) = extended_gcd_reduced_cofactor(&f, &g);
        println!("extended_gcd_reduced_cofactor({f}, {g}) = ({d}, {a}, {b})");
    }
}

fn benchmark_integer_matrix_hermite_normal_form_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.hermite_normal_form()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.hermite_normal_form()", &mut |a| {
                no_out!(a.hermite_normal_form());
            }),
            ("(&IntegerMatrix).hermite_normal_form()", &mut |a| {
                no_out!((&a).hermite_normal_form());
            }),
            (
                "IntegerMatrix.hermite_normal_form_assign()",
                &mut |mut a| {
                    a.hermite_normal_form_assign();
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_hermite_normal_form_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.hermite_normal_form()",
        BenchmarkType::Algorithms,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.hermite_normal_form());
            }),
            ("classical", &mut |a| {
                no_out!(hnf_classical(&a));
            }),
            ("xgcd", &mut |a| {
                no_out!(hnf_xgcd(&a));
            }),
            ("Pernet-Stein", &mut |a| {
                no_out!(hnf_pernet_stein(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_hermite_normal_form_full_column_rank_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.hermite_normal_form()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_3().get(gm, config).map(|a| {
            let d = lattice_determinant(&a);
            (a, d)
        }),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, _)| {
                no_out!(a.hermite_normal_form());
            }),
            ("classical", &mut |(a, _)| {
                no_out!(hnf_classical(&a));
            }),
            ("minors", &mut |(a, _)| {
                no_out!(hnf_minors(&a));
            }),
            ("modular", &mut |(a, d)| {
                no_out!(hnf_modular(&a, &d));
            }),
            ("modular eldiv", &mut |(mut a, d)| {
                hnf_modular_eldiv(&mut a, &d);
            }),
            ("Pernet-Stein", &mut |(a, _)| {
                no_out!(hnf_pernet_stein(&a));
            }),
        ],
    );
}
