// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign};
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_add::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_add);
    register_demo!(runner, demo_unsigned_polynomial_mod_add_val_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_add_ref_val);
    register_demo!(runner, demo_unsigned_polynomial_mod_add_ref_ref);
    register_demo!(runner, demo_unsigned_polynomial_mod_add_assign);
    register_demo!(runner, demo_unsigned_polynomial_mod_add_assign_ref);

    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_add_evaluation_strategy
    );
    register_bench!(runner, benchmark_unsigned_polynomial_mod_add_algorithms);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_add_assign_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_add(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}).mod_add({q_old}, {m}) = {}", p.mod_add(q, m));
    }
}

fn demo_unsigned_polynomial_mod_add_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).mod_add(&({q}), {m}) = {}", p.mod_add(&q, m));
    }
}

fn demo_unsigned_polynomial_mod_add_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let q_old = q.clone();
        println!("(&({p})).mod_add({q_old}, {m}) = {}", (&p).mod_add(q, m));
    }
}

fn demo_unsigned_polynomial_mod_add_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).mod_add(&({q}), {m}) = {}", (&p).mod_add(&q, m));
    }
}

fn demo_unsigned_polynomial_mod_add_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p.mod_add_assign(q, m);
        println!("p := {p_old}; p.mod_add_assign({q_old}, {m}); p = {p}");
    }
}

fn demo_unsigned_polynomial_mod_add_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.mod_add_assign(&q, m);
        println!("p := {p_old}; p.mod_add_assign(&({q}), {m}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_mod_add_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_add(UnsignedPolynomial<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_add(UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, m)| {
                    no_out!(p.mod_add(q, m));
                },
            ),
            (
                "UnsignedPolynomial<u64>.mod_add(&UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, m)| {
                    no_out!(p.mod_add(&q, m));
                },
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_add(UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, m)| {
                    no_out!((&p).mod_add(q, m));
                },
            ),
            (
                "(&UnsignedPolynomial<u64>).mod_add(&UnsignedPolynomial<u64>, u64)",
                &mut |(p, q, m)| {
                    no_out!((&p).mod_add(&q, m));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_add_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_add(UnsignedPolynomial<u64>, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q, m)| {
                no_out!(p.mod_add(q, m));
            }),
            ("naive", &mut |(p, q, m)| {
                no_out!(mod_add_naive(&p, &q, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_add_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_add_assign(UnsignedPolynomial<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            (
                "UnsignedPolynomial<u64>.mod_add_assign(UnsignedPolynomial<u64>, u64)",
                &mut |(mut p, q, m)| p.mod_add_assign(q, m),
            ),
            (
                "UnsignedPolynomial<u64>.mod_add_assign(&UnsignedPolynomial<u64>, u64)",
                &mut |(mut p, q, m)| p.mod_add_assign(&q, m),
            ),
        ],
    );
}
