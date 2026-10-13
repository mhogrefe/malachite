// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddDot, AddDotAssign, Dot};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_integer_vector_integer_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_add_dot);
    register_demo!(runner, demo_integer_vector_add_dot_val_val_ref);
    register_demo!(runner, demo_integer_vector_add_dot_val_ref_val);
    register_demo!(runner, demo_integer_vector_add_dot_val_ref_ref);
    register_demo!(runner, demo_integer_vector_add_dot_ref_ref_ref);
    register_demo!(runner, demo_integer_vector_add_dot_assign);
    register_demo!(runner, demo_integer_vector_add_dot_assign_val_ref);
    register_demo!(runner, demo_integer_vector_add_dot_assign_ref_val);
    register_demo!(runner, demo_integer_vector_add_dot_assign_ref_ref);
    register_bench!(runner, benchmark_integer_vector_add_dot_algorithms);
    register_bench!(runner, benchmark_integer_vector_add_dot_evaluation_strategy);
}

fn demo_integer_vector_add_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{x_old}.add_dot({v_old}, {w_old}) = {}", x.add_dot(v, w));
    }
}

fn demo_integer_vector_add_dot_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        println!("{x_old}.add_dot({v_old}, &{w}) = {}", x.add_dot(v, &w));
    }
}

fn demo_integer_vector_add_dot_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let w_old = w.clone();
        println!("{x_old}.add_dot(&{v}, {w_old}) = {}", x.add_dot(&v, w));
    }
}

fn demo_integer_vector_add_dot_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("{x_old}.add_dot(&{v}, &{w}) = {}", x.add_dot(&v, &w));
    }
}

fn demo_integer_vector_add_dot_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}).add_dot(&{v}, &{w}) = {}", (&x).add_dot(&v, &w));
    }
}

fn demo_integer_vector_add_dot_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        let w_old = w.clone();
        x.add_dot_assign(v, w);
        println!("x := {x_old}; x.add_dot_assign({v_old}, {w_old}); x = {x}");
    }
}

fn demo_integer_vector_add_dot_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        x.add_dot_assign(v, &w);
        println!("x := {x_old}; x.add_dot_assign({v_old}, &{w}); x = {x}");
    }
}

fn demo_integer_vector_add_dot_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let w_old = w.clone();
        x.add_dot_assign(&v, w);
        println!("x := {x_old}; x.add_dot_assign(&{v}, {w_old}); x = {x}");
    }
}

fn demo_integer_vector_add_dot_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in integer_vector_integer_vector_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.add_dot_assign(&v, &w);
        println!("x := {x_old}; x.add_dot_assign(&{v}, &{w}); x = {x}");
    }
}

fn benchmark_integer_vector_add_dot_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_dot(&IntegerVector, &IntegerVector)",
        BenchmarkType::Algorithms,
        integer_vector_integer_vector_integer_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w, x)| {
                no_out!(x.add_dot(&v, &w));
            }),
            ("dot, then add", &mut |(v, w, x)| {
                no_out!(x + v.dot(w));
            }),
        ],
    );
}

fn benchmark_integer_vector_add_dot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_dot(IntegerVector, IntegerVector)",
        BenchmarkType::EvaluationStrategy,
        integer_vector_integer_vector_integer_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_vector_bit_bucketer("v"),
        &mut [
            (
                "Integer.add_dot(IntegerVector, IntegerVector)",
                &mut |(v, w, x)| {
                    no_out!(x.add_dot(v, w));
                },
            ),
            (
                "Integer.add_dot(IntegerVector, &IntegerVector)",
                &mut |(v, w, x)| {
                    no_out!(x.add_dot(v, &w));
                },
            ),
            (
                "Integer.add_dot(&IntegerVector, IntegerVector)",
                &mut |(v, w, x)| {
                    no_out!(x.add_dot(&v, w));
                },
            ),
            (
                "Integer.add_dot(&IntegerVector, &IntegerVector)",
                &mut |(v, w, x)| {
                    no_out!(x.add_dot(&v, &w));
                },
            ),
            (
                "(&Integer).add_dot(&IntegerVector, &IntegerVector)",
                &mut |(v, w, x)| {
                    no_out!((&x).add_dot(&v, &w));
                },
            ),
            (
                "Integer.add_dot_assign(IntegerVector, IntegerVector)",
                &mut |(v, w, mut x)| {
                    no_out!(x.add_dot_assign(v, w));
                },
            ),
            (
                "Integer.add_dot_assign(&IntegerVector, &IntegerVector)",
                &mut |(v, w, mut x)| {
                    no_out!(x.add_dot_assign(&v, &w));
                },
            ),
        ],
    );
}
