// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Solve;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::arithmetic::solve::*;
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_height_bucketer;
use malachite_q::test_util::generators::{
    rational_matrix_pair_gen_var_5, rational_matrix_pair_gen_var_6,
    rational_matrix_rational_vector_pair_gen_var_3,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_solve_fraction_free);
    register_demo!(runner, demo_rational_matrix_solve_fraction_free_low_rank);
    register_demo!(runner, demo_rational_matrix_solve_dixon);
    register_demo!(runner, demo_rational_matrix_solve_dixon_low_rank);
    register_demo!(runner, demo_rational_matrix_solve_multi_mod);
    register_demo!(runner, demo_rational_matrix_solve_multi_mod_low_rank);
    register_demo!(runner, demo_rational_matrix_solve_nonsingular);
    register_demo!(runner, demo_rational_matrix_solve_nonsingular_low_rank);
    register_demo!(runner, demo_rational_matrix_can_solve_fraction_free);
    register_demo!(
        runner,
        demo_rational_matrix_can_solve_fraction_free_low_rank
    );
    register_demo!(runner, demo_rational_matrix_can_solve_multi_mod);
    register_demo!(runner, demo_rational_matrix_can_solve_multi_mod_low_rank);
    register_demo!(runner, demo_rational_matrix_can_solve_dixon);
    register_demo!(runner, demo_rational_matrix_can_solve_dixon_low_rank);
    register_demo!(runner, demo_rational_matrix_can_solve);
    register_demo!(runner, demo_rational_matrix_can_solve_low_rank);
    register_demo!(runner, demo_rational_matrix_solve);
    register_demo!(runner, demo_rational_matrix_solve_low_rank);
    register_demo!(runner, demo_rational_matrix_solve_val_val);
    register_demo!(runner, demo_rational_matrix_solve_vector);
    register_bench!(runner, benchmark_rational_matrix_solve_evaluation_strategy);
    register_bench!(runner, benchmark_rational_matrix_solve_algorithms);
    register_bench!(runner, benchmark_rational_matrix_can_solve_algorithms);
}

pub(crate) fn format_result(x: Option<RationalMatrix>) -> String {
    x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

fn demo_rational_matrix_solve_fraction_free(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "solve_fraction_free({a}, {b}) = {}",
            format_result(solve_fraction_free(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_fraction_free_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "solve_fraction_free({a}, {b}) = {}",
            format_result(solve_fraction_free(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_dixon(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "solve_dixon({a}, {b}) = {}",
            format_result(solve_dixon(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_dixon_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "solve_dixon({a}, {b}) = {}",
            format_result(solve_dixon(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_multi_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "solve_multi_mod({a}, {b}) = {}",
            format_result(solve_multi_mod(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_multi_mod_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "solve_multi_mod({a}, {b}) = {}",
            format_result(solve_multi_mod(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_nonsingular(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "solve_nonsingular({a}, {b}) = {}",
            format_result(solve_nonsingular(&a, &b))
        );
    }
}

fn demo_rational_matrix_solve_nonsingular_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_5().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "solve_nonsingular({a}, {b}) = {}",
            format_result(solve_nonsingular(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve_fraction_free(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        println!(
            "can_solve_fraction_free({a}, {b}) = {}",
            format_result(can_solve_fraction_free(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve_fraction_free_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "can_solve_fraction_free({a}, {b}) = {}",
            format_result(can_solve_fraction_free(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve_multi_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        println!(
            "can_solve_multi_mod({a}, {b}) = {}",
            format_result(can_solve_multi_mod(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve_multi_mod_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "can_solve_multi_mod({a}, {b}) = {}",
            format_result(can_solve_multi_mod(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve_dixon(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        println!(
            "can_solve_dixon({a}, {b}) = {}",
            format_result(can_solve_dixon(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve_dixon_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!(
            "can_solve_dixon({a}, {b}) = {}",
            format_result(can_solve_dixon(&a, &b))
        );
    }
}

fn demo_rational_matrix_can_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        println!("can_solve({a}, {b}) = {}", format_result(can_solve(&a, &b)));
    }
}

fn demo_rational_matrix_can_solve_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("can_solve({a}, {b}) = {}", format_result(can_solve(&a, &b)));
    }
}

fn demo_rational_matrix_solve(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        println!("solve({a}, {b}) = {}", format_result((&a).solve(&b)));
    }
}

fn demo_rational_matrix_solve_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a = rational_matrix_low_rank(&a, &b);
        println!("solve({a}, {b}) = {}", format_result((&a).solve(&b)));
    }
}

fn demo_rational_matrix_solve_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_pair_gen_var_6().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{a_old}.solve({b_old}) = {:?}", a.solve(b));
    }
}

fn demo_rational_matrix_solve_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in rational_matrix_rational_vector_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).solve(&{b}) = {:?}", (&a).solve(&b));
    }
}

fn benchmark_rational_matrix_solve_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.solve(RationalMatrix)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_height_bucketer("a"),
        &mut [
            ("RationalMatrix.solve(RationalMatrix)", &mut |(a, b)| {
                no_out!(a.solve(b));
            }),
            ("RationalMatrix.solve(&RationalMatrix)", &mut |(a, b)| {
                no_out!(a.solve(&b));
            }),
            ("(&RationalMatrix).solve(RationalMatrix)", &mut |(a, b)| {
                no_out!((&a).solve(b));
            }),
            ("(&RationalMatrix).solve(&RationalMatrix)", &mut |(a, b)| {
                no_out!((&a).solve(&b));
            }),
        ],
    );
}

fn benchmark_rational_matrix_solve_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "solve_nonsingular(&RationalMatrix, &RationalMatrix)",
        BenchmarkType::Algorithms,
        rational_matrix_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_height_bucketer("a"),
        &mut [
            ("fraction-free", &mut |(a, b)| {
                no_out!(solve_fraction_free(&a, &b));
            }),
            ("Dixon", &mut |(a, b)| {
                no_out!(solve_dixon(&a, &b));
            }),
            ("multimodular", &mut |(a, b)| {
                no_out!(solve_multi_mod(&a, &b));
            }),
        ],
    );
}

fn benchmark_rational_matrix_can_solve_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "can_solve(&RationalMatrix, &RationalMatrix)",
        BenchmarkType::Algorithms,
        rational_matrix_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_height_bucketer("a"),
        &mut [
            ("fraction-free", &mut |(a, b)| {
                no_out!(can_solve_fraction_free(&a, &b));
            }),
            ("Dixon", &mut |(a, b)| {
                no_out!(can_solve_dixon(&a, &b));
            }),
            ("multimodular", &mut |(a, b)| {
                no_out!(can_solve_multi_mod(&a, &b));
            }),
        ],
    );
}
