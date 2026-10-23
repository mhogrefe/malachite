// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Solve;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::can_solve_multi_mod::can_solve_multi_mod_den;
use malachite_nz::integer_matrix::arithmetic::solve::{
    can_solve, can_solve_fflu, solve_cramer, solve_fflu,
};
use malachite_nz::test_util::bench::bucketers::pair_integer_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_integer_vector_pair_gen_var_2, integer_matrix_pair_gen_var_3,
    integer_matrix_pair_gen_var_7, integer_matrix_pair_gen_var_8,
};
use malachite_nz::test_util::integer_matrix::arithmetic::solve::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_solve);
    register_demo!(runner, demo_integer_matrix_solve_low_rank);
    register_demo!(runner, demo_integer_matrix_solve_ref_ref);
    register_demo!(runner, demo_integer_matrix_solve_vector);
    register_demo!(runner, demo_integer_matrix_can_solve_fflu);
    register_demo!(runner, demo_integer_matrix_can_solve_fflu_low_rank);
    register_demo!(runner, demo_integer_matrix_can_solve);
    register_demo!(runner, demo_integer_matrix_can_solve_low_rank);
    register_demo!(runner, demo_integer_matrix_solve_fflu);
    register_demo!(runner, demo_integer_matrix_solve_fflu_low_rank);
    register_demo!(runner, demo_integer_matrix_solve_cramer);
    register_demo!(runner, demo_integer_matrix_solve_fflu_precomp);
    register_demo!(runner, demo_integer_matrix_solve_fflu_precomp_low_rank);
    register_bench!(runner, benchmark_integer_matrix_solve_algorithms);
    register_bench!(
        runner,
        benchmark_integer_matrix_solve_fflu_precomp_algorithms
    );
    register_bench!(runner, benchmark_integer_matrix_solve_small_algorithms);
    register_bench!(runner, benchmark_integer_matrix_solve_evaluation_strategy);
    register_bench!(runner, benchmark_integer_matrix_can_solve_algorithms);
}

pub(crate) fn format_solution(x: Option<(IntegerMatrix, Integer)>) -> String {
    x.map_or_else(
        || "None".to_string(),
        |(x, den)| format!("Some(({x}, {den}))"),
    )
}

fn demo_integer_matrix_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("solve({a_old}, {b_old}) = {}", format_solution(a.solve(b)));
    }
}

// Systems whose matrix has rank at most the width of the right-hand side, when that is smaller than
// its height and width, so that many have no unique solution, and some have none.
fn demo_integer_matrix_solve_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        let a = low_rank(&a, &b);
        println!("solve({a}, {b}) = {}", format_solution((&a).solve(&b)));
    }
}

fn demo_integer_matrix_solve_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        println!("(&{a}).solve(&{b}) = {:?}", (&a).solve(&b));
    }
}

fn demo_integer_matrix_solve_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_integer_vector_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{a_old}.solve({b_old}) = {:?}", a.solve(b));
    }
}

fn demo_integer_matrix_can_solve_fflu(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        println!(
            "can_solve_fflu({a}, {b}) = {}",
            format_solution(can_solve_fflu(&a, &b))
        );
    }
}

fn demo_integer_matrix_can_solve_fflu_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        let a = low_rank(&a, &b);
        println!(
            "can_solve_fflu({a}, {b}) = {}",
            format_solution(can_solve_fflu(&a, &b))
        );
    }
}

fn demo_integer_matrix_can_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        println!(
            "can_solve({a}, {b}) = {}",
            format_solution(can_solve(&a, &b))
        );
    }
}

fn demo_integer_matrix_can_solve_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        let a = low_rank(&a, &b);
        println!(
            "can_solve({a}, {b}) = {}",
            format_solution(can_solve(&a, &b))
        );
    }
}

fn demo_integer_matrix_solve_fflu(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "solve_fflu({a}, {b}) = {}",
            format_solution(solve_fflu(&a, &b))
        );
    }
}

// Systems whose matrix has rank at most the width of the right-hand side, when that is smaller than
// its dimension, so that many are singular.
fn demo_integer_matrix_solve_fflu_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = low_rank(&a, &b);
        println!(
            "solve_fflu({a}, {b}) = {}",
            format_solution(solve_fflu(&a, &b))
        );
    }
}

// A matrix of `a`'s shape with rank at most `b`'s width, if that is smaller than its height and
// width: `b` times the first `b.width()` rows of `a`.
pub(crate) fn low_rank(a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
    let (h, w) = (a.height(), a.width());
    let k = b.width();
    if k >= h.min(w) {
        return a.clone();
    }
    b * a.window(0..k, 0..w)
}

fn demo_integer_matrix_solve_cramer(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_8().get(gm, config).take(limit) {
        println!(
            "solve_cramer({a}, {b}) = {}",
            format_solution(solve_cramer(&a, &b))
        );
    }
}

fn demo_integer_matrix_solve_fflu_precomp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_3().get(gm, config).take(limit) {
        let x = integer_matrix_solve_fflu_precomp_from_matrix(&a, &b);
        println!(
            "solve_fflu_precomp({a}, {b}) = {}",
            x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_solve_fflu_precomp_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = low_rank(&a, &b);
        let x = integer_matrix_solve_fflu_precomp_from_matrix(&a, &b);
        println!(
            "solve_fflu_precomp({a}, {b}) = {}",
            x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn benchmark_integer_matrix_solve_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "solve(&IntegerMatrix, &IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("fraction-free LU", &mut |(a, b)| {
                no_out!(solve_fflu(&a, &b));
            }),
            (
                "fraction-free LU, Integer arithmetic only",
                &mut |(a, b)| {
                    no_out!(integer_matrix_solve_fflu_general(&a, &b));
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_solve_fflu_precomp_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "solve_fflu_precomp(&IntegerMatrix, &IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!(integer_matrix_solve_fflu_precomp_from_matrix(&a, &b));
            }),
            ("Integer arithmetic only", &mut |(a, b)| {
                no_out!(integer_matrix_solve_fflu_precomp_from_matrix_general(
                    &a, &b
                ));
            }),
        ],
    );
}

fn benchmark_integer_matrix_solve_small_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "solve(&IntegerMatrix, &IntegerMatrix) for dimensions up to 3",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("Cramer", &mut |(a, b)| {
                no_out!(solve_cramer(&a, &b));
            }),
            ("fraction-free LU", &mut |(a, b)| {
                no_out!(solve_fflu(&a, &b));
            }),
            ("naive", &mut |(a, b)| {
                no_out!(integer_matrix_solve_naive(&a, &b));
            }),
        ],
    );
}

fn benchmark_integer_matrix_solve_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.solve(IntegerMatrix)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("IntegerMatrix.solve(IntegerMatrix)", &mut |(a, b)| {
                no_out!(a.solve(b));
            }),
            ("IntegerMatrix.solve(&IntegerMatrix)", &mut |(a, b)| {
                no_out!(a.solve(&b));
            }),
            ("(&IntegerMatrix).solve(IntegerMatrix)", &mut |(a, b)| {
                no_out!((&a).solve(b));
            }),
            ("(&IntegerMatrix).solve(&IntegerMatrix)", &mut |(a, b)| {
                no_out!((&a).solve(&b));
            }),
        ],
    );
}

fn benchmark_integer_matrix_can_solve_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "can_solve(&IntegerMatrix, &IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("fraction-free LU", &mut |(a, b)| {
                no_out!(can_solve_fflu(&a, &b));
            }),
            ("multimodular", &mut |(a, b)| {
                no_out!(can_solve_multi_mod_den(&a, &b));
            }),
        ],
    );
}
