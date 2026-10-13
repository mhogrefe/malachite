// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Dot, SubDot, SubDotAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_rational_vector_rational_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_sub_dot);
    register_demo!(runner, demo_rational_vector_sub_dot_val_val_ref);
    register_demo!(runner, demo_rational_vector_sub_dot_val_ref_val);
    register_demo!(runner, demo_rational_vector_sub_dot_val_ref_ref);
    register_demo!(runner, demo_rational_vector_sub_dot_ref_ref_ref);
    register_demo!(runner, demo_rational_vector_sub_dot_assign);
    register_demo!(runner, demo_rational_vector_sub_dot_assign_val_ref);
    register_demo!(runner, demo_rational_vector_sub_dot_assign_ref_val);
    register_demo!(runner, demo_rational_vector_sub_dot_assign_ref_ref);
    register_bench!(runner, benchmark_rational_vector_sub_dot_algorithms);
    register_bench!(
        runner,
        benchmark_rational_vector_sub_dot_evaluation_strategy
    );
}

fn demo_rational_vector_sub_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{x_old}.sub_dot({v_old}, {w_old}) = {}", x.sub_dot(v, w));
    }
}

fn demo_rational_vector_sub_dot_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        println!("{x_old}.sub_dot({v_old}, &{w}) = {}", x.sub_dot(v, &w));
    }
}

fn demo_rational_vector_sub_dot_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let w_old = w.clone();
        println!("{x_old}.sub_dot(&{v}, {w_old}) = {}", x.sub_dot(&v, w));
    }
}

fn demo_rational_vector_sub_dot_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("{x_old}.sub_dot(&{v}, &{w}) = {}", x.sub_dot(&v, &w));
    }
}

fn demo_rational_vector_sub_dot_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}).sub_dot(&{v}, &{w}) = {}", (&x).sub_dot(&v, &w));
    }
}

fn demo_rational_vector_sub_dot_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        let w_old = w.clone();
        x.sub_dot_assign(v, w);
        println!("x := {x_old}; x.sub_dot_assign({v_old}, {w_old}); x = {x}");
    }
}

fn demo_rational_vector_sub_dot_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        x.sub_dot_assign(v, &w);
        println!("x := {x_old}; x.sub_dot_assign({v_old}, &{w}); x = {x}");
    }
}

fn demo_rational_vector_sub_dot_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let w_old = w.clone();
        x.sub_dot_assign(&v, w);
        println!("x := {x_old}; x.sub_dot_assign(&{v}, {w_old}); x = {x}");
    }
}

fn demo_rational_vector_sub_dot_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.sub_dot_assign(&v, &w);
        println!("x := {x_old}; x.sub_dot_assign(&{v}, &{w}); x = {x}");
    }
}

fn benchmark_rational_vector_sub_dot_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sub_dot(&RationalVector, &RationalVector)",
        BenchmarkType::Algorithms,
        rational_vector_rational_vector_rational_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w, x)| {
                no_out!(x.sub_dot(&v, &w));
            }),
            ("dot, then subtract", &mut |(v, w, x)| {
                no_out!(x - v.dot(w));
            }),
        ],
    );
}

fn benchmark_rational_vector_sub_dot_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sub_dot(RationalVector, RationalVector)",
        BenchmarkType::EvaluationStrategy,
        rational_vector_rational_vector_rational_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_vector_bit_bucketer("v"),
        &mut [
            (
                "Rational.sub_dot(RationalVector, RationalVector)",
                &mut |(v, w, x)| {
                    no_out!(x.sub_dot(v, w));
                },
            ),
            (
                "Rational.sub_dot(RationalVector, &RationalVector)",
                &mut |(v, w, x)| {
                    no_out!(x.sub_dot(v, &w));
                },
            ),
            (
                "Rational.sub_dot(&RationalVector, RationalVector)",
                &mut |(v, w, x)| {
                    no_out!(x.sub_dot(&v, w));
                },
            ),
            (
                "Rational.sub_dot(&RationalVector, &RationalVector)",
                &mut |(v, w, x)| {
                    no_out!(x.sub_dot(&v, &w));
                },
            ),
            (
                "(&Rational).sub_dot(&RationalVector, &RationalVector)",
                &mut |(v, w, x)| {
                    no_out!((&x).sub_dot(&v, &w));
                },
            ),
            (
                "Rational.sub_dot_assign(RationalVector, RationalVector)",
                &mut |(v, w, mut x)| {
                    no_out!(x.sub_dot_assign(v, w));
                },
            ),
            (
                "Rational.sub_dot_assign(&RationalVector, &RationalVector)",
                &mut |(v, w, mut x)| {
                    no_out!(x.sub_dot_assign(&v, &w));
                },
            ),
        ],
    );
}
