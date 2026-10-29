// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{CheckedModDivTruncated, ModDivTruncated, ModDivTruncatedAssign};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1,
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4,
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_5,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_div_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_div_truncated);
    register_demo!(runner, demo_natural_polynomial_mod_div_truncated_ref_ref);
    register_demo!(runner, demo_natural_polynomial_mod_div_truncated_assign);
    register_demo!(runner, demo_natural_polynomial_checked_mod_div_truncated);
    register_demo!(
        runner,
        demo_natural_polynomial_checked_mod_div_truncated_ref_ref
    );
    register_demo!(runner, demo_natural_polynomial_mod_div_truncated_long);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_div_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_div_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_div_truncated_algorithms_long
    );
}

fn show(p: Option<NaturalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_div_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len, m) in natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).mod_div_truncated({q_old}, {len}, {m_old}) = {}",
            p.mod_div_truncated(q, len, m)
        );
    }
}

fn demo_natural_polynomial_mod_div_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_div_truncated(&({q}), {len}, &({m})) = {}",
            (&p).mod_div_truncated(&q, len, &m)
        );
    }
}

fn demo_natural_polynomial_mod_div_truncated_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q, len, m) in natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        p.mod_div_truncated_assign(q, len, m);
        println!("p := {p_old}; p.mod_div_truncated_assign({q_old}, {len}, {m_old}); p = {p}");
    }
}

fn demo_natural_polynomial_checked_mod_div_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        let m_old = m.clone();
        println!(
            "({p_old}).checked_mod_div_truncated({q_old}, {len}, {m_old}) = {}",
            show(p.checked_mod_div_truncated(q, len, m))
        );
    }
}

fn demo_natural_polynomial_checked_mod_div_truncated_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, len, m) in natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).checked_mod_div_truncated(&({q}), {len}, &({m})) = {}",
            show((&p).checked_mod_div_truncated(&q, len, &m))
        );
    }
}

fn demo_natural_polynomial_mod_div_truncated_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len, m) in natural_polynomial_pair_unsigned_natural_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_div_truncated(&({q}), {len}, &({m})) = {}",
            (&p).mod_div_truncated(&q, len, &m)
        );
    }
}

fn benchmark_natural_polynomial_mod_div_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_div_truncated(&NaturalPolynomial, u64, &Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("p.mod_div_truncated(q, len, m)", &mut |(p, q, len, m)| {
                no_out!(p.mod_div_truncated(q, len, m));
            }),
            ("(&p).mod_div_truncated(&q, len, &m)", &mut |(
                p,
                q,
                len,
                m,
            )| {
                no_out!((&p).mod_div_truncated(&q, len, &m));
            }),
            (
                "p.mod_div_truncated_assign(&q, len, &m)",
                &mut |(mut p, q, len, m)| {
                    p.mod_div_truncated_assign(&q, len, &m);
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_div_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_div_truncated(&NaturalPolynomial, u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, m)| {
                no_out!((&p).mod_div_truncated(&q, len, &m));
            }),
            ("naive", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_naive(&p, &q, len, &m));
            }),
            ("basecase", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_basecase_wrapper(&p, &q, len, &m));
            }),
            ("Karp-Markstein", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_newton_wrapper(&p, &q, len, &m));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_div_truncated_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_div_truncated(&NaturalPolynomial, u64, &Natural)",
        BenchmarkType::Algorithms,
        natural_polynomial_pair_unsigned_natural_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("len"),
        &mut [
            ("default", &mut |(p, q, len, m)| {
                no_out!((&p).mod_div_truncated(&q, len, &m));
            }),
            ("basecase", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_basecase_wrapper(&p, &q, len, &m));
            }),
            ("Karp-Markstein", &mut |(p, q, len, m)| {
                no_out!(mod_div_truncated_newton_wrapper(&p, &q, len, &m));
            }),
        ],
    );
}
