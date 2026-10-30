// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedDiv, DivRem};
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_polynomial::arithmetic::div::div_basecase_wrapper;
use malachite_nz::test_util::bench::bucketers::pair_integer_polynomial_max_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_div);
    register_demo!(runner, demo_integer_polynomial_div_ref_ref);
    register_demo!(runner, demo_integer_polynomial_div_assign);
    register_demo!(runner, demo_integer_polynomial_checked_div);
    register_demo!(runner, demo_integer_polynomial_div_long);
    register_bench!(runner, benchmark_integer_polynomial_div_evaluation_strategy);
    register_bench!(runner, benchmark_integer_polynomial_div_algorithms);
}

fn demo_integer_polynomial_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!("({p_old}) / ({q_old}) = {}", p / q);
    }
}

fn demo_integer_polynomial_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("&({p}) / &({q}) = {}", &p / &q);
    }
}

fn demo_integer_polynomial_div_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        p /= q;
        println!("p := {p_old}; p /= {q_old}; p = {p}");
    }
}

fn demo_integer_polynomial_checked_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).checked_div({q_old}) = {}",
            p.checked_div(q)
                .map_or_else(|| "None".to_string(), |s| s.to_string())
        );
    }
}

// The generator configuration of the long demos: polynomials of about 150 coefficients of about 8
// bits, long enough to reach the divide-and-conquer algorithms.
fn long_config(config: &GenConfig) -> GenConfig {
    let mut config = config.clone();
    if config.get_or("mean_length_n", 0) == 0 {
        config.insert("mean_length_n", 150);
    }
    if config.get_or("mean_bits_n", 0) == 0 {
        config.insert("mean_bits_n", 8);
    }
    config
}

fn demo_integer_polynomial_div_long(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q) in integer_polynomial_pair_gen_var_4()
        .get(gm, &long_config(config))
        .take(limit)
    {
        println!("&({p}) / &({q}) = {}", &p / &q);
    }
}

fn benchmark_integer_polynomial_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial / IntegerPolynomial",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("p / q", &mut |(p, q)| {
                no_out!(p / q);
            }),
            ("p / &q", &mut |(p, q)| {
                no_out!(p / &q);
            }),
            ("&p / q", &mut |(p, q)| {
                no_out!(&p / q);
            }),
            ("&p / &q", &mut |(p, q)| {
                no_out!(&p / &q);
            }),
            ("p /= q", &mut |(mut p, q)| {
                no_out!(p /= q);
            }),
            ("p /= &q", &mut |(mut p, q)| {
                no_out!(p /= &q);
            }),
        ],
    );
}

fn benchmark_integer_polynomial_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial / IntegerPolynomial",
        BenchmarkType::Algorithms,
        integer_polynomial_pair_gen_var_4()
            .get(gm, config)
            .filter(|(p, q)| p.len() >= q.len()),
        gm.name(),
        limit,
        file_name,
        &pair_integer_polynomial_max_bit_bucketer("p", "q"),
        &mut [
            ("default", &mut |(p, q)| {
                no_out!(&p / &q);
            }),
            ("basecase", &mut |(p, q)| {
                no_out!(div_basecase_wrapper(
                    p.coefficients_asc(),
                    q.coefficients_asc()
                ));
            }),
            ("using div_rem", &mut |(p, q)| {
                no_out!((&p).div_rem(&q).0);
            }),
        ],
    );
}
